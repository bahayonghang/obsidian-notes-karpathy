from __future__ import annotations

import json
from datetime import UTC, datetime
from pathlib import Path
from typing import Any

from _vault_common import load_markdown
from _vault_layout import (
    accepted_raw_sources,
    compute_hash,
    detect_companion_skills,
    iso_from_timestamp,
    manifest_path,
    pdf_ingest_plan,
    raw_source_metadata,
    resolve_vault_profile,
    sidecar_for_pdf,
    source_class_for_raw,
    summary_for_raw,
)


MANIFEST_VERSION = 1
MANIFEST_FIELDS = (
    "source_id",
    "path",
    "source_type",
    "capture_origin",
    "source_url_or_handle",
    "content_hash",
    "first_seen_at",
    "last_seen_at",
    "ingest_status",
    "normalized_outputs",
)


def _now_iso() -> str:
    return datetime.now(UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def _yaml_scalar(value: Any) -> str:
    if value is None:
        return '""'
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, (int, float)):
        return str(value)
    return json.dumps(str(value), ensure_ascii=False)


def _parse_manifest_scalar(raw_value: str) -> Any:
    value = raw_value.strip()
    if not value:
        return ""
    if value == "true":
        return True
    if value == "false":
        return False
    if value.isdigit():
        return int(value)
    if value.startswith('"') and value.endswith('"'):
        return json.loads(value)
    if value.startswith("'") and value.endswith("'"):
        return value[1:-1]
    return value


def load_source_manifest(vault_root: Path) -> dict[str, Any]:
    path = manifest_path(vault_root)
    if not path.exists():
        return {
            "version": MANIFEST_VERSION,
            "generated_at": None,
            "profile": resolve_vault_profile(vault_root),
            "sources": [],
        }

    data: dict[str, Any] = {"version": MANIFEST_VERSION, "generated_at": None, "profile": None, "sources": []}
    current: dict[str, Any] | None = None
    in_outputs = False

    for raw_line in path.read_text(encoding="utf-8").splitlines():
        if not raw_line.strip() or raw_line.lstrip().startswith("#"):
            continue

        if raw_line.startswith("version:"):
            data["version"] = _parse_manifest_scalar(raw_line.split(":", 1)[1])
            continue
        if raw_line.startswith("generated_at:"):
            data["generated_at"] = _parse_manifest_scalar(raw_line.split(":", 1)[1])
            continue
        if raw_line.startswith("profile:"):
            data["profile"] = _parse_manifest_scalar(raw_line.split(":", 1)[1])
            continue
        if raw_line.startswith("sources:"):
            continue

        if raw_line.startswith("  - "):
            current = {}
            data["sources"].append(current)
            in_outputs = False
            remainder = raw_line[4:]
            if ":" in remainder:
                key, value = remainder.split(":", 1)
                current[key.strip()] = _parse_manifest_scalar(value)
            continue

        if current is None:
            continue

        if raw_line.startswith("    normalized_outputs:"):
            current["normalized_outputs"] = []
            in_outputs = True
            continue

        if in_outputs and raw_line.startswith("      - "):
            current.setdefault("normalized_outputs", []).append(_parse_manifest_scalar(raw_line[8:]))
            continue

        in_outputs = False
        if raw_line.startswith("    ") and ":" in raw_line:
            key, value = raw_line.strip().split(":", 1)
            current[key.strip()] = _parse_manifest_scalar(value)

    if not data["profile"]:
        data["profile"] = resolve_vault_profile(vault_root)

    for item in data["sources"]:
        item.setdefault("normalized_outputs", [])

    return data


def write_source_manifest(vault_root: Path, payload: dict[str, Any]) -> Path:
    path = manifest_path(vault_root)
    path.parent.mkdir(parents=True, exist_ok=True)

    lines = [
        f"version: {payload.get('version', MANIFEST_VERSION)}",
        f"generated_at: {_yaml_scalar(payload.get('generated_at') or _now_iso())}",
        f"profile: {_yaml_scalar(payload.get('profile') or resolve_vault_profile(vault_root))}",
        "sources:",
    ]
    for item in payload.get("sources", []):
        lines.append(f"  - source_id: {_yaml_scalar(item.get('source_id', ''))}")
        for field in MANIFEST_FIELDS[1:]:
            if field == "normalized_outputs":
                lines.append("    normalized_outputs:")
                normalized_outputs = item.get("normalized_outputs") or []
                if normalized_outputs:
                    lines.extend(f"      - {_yaml_scalar(entry)}" for entry in normalized_outputs)
                else:
                    lines.append('      - ""')
                continue
            lines.append(f"    {field}: {_yaml_scalar(item.get(field, ''))}")
        if item.get("deferred_to"):
            lines.append(f"    deferred_to: {_yaml_scalar(item['deferred_to'])}")
        if item.get("metadata_path"):
            lines.append(f"    metadata_path: {_yaml_scalar(item['metadata_path'])}")
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return path


def _stable_source_id(rel_path: str) -> str:
    source_id = rel_path.replace("\\", "/").replace("/", "--")
    if source_id.endswith(".md"):
        source_id = source_id[:-3]
    return source_id


def _source_url_or_handle(vault_root: Path, raw_path: Path, source_class: str, plan: dict[str, Any] | None) -> str:
    if plan:
        for key in ("source_url", "paper_handle"):
            value = plan.get(key)
            if isinstance(value, str) and value.strip():
                return value.strip()

    if source_class == "markdown":
        record = load_markdown(raw_path, vault_root)
        for key in ("source", "paper_id"):
            value = record.frontmatter.get(key)
            if isinstance(value, str) and value.strip():
                return value.strip()

    sidecar_path = sidecar_for_pdf(raw_path)
    if sidecar_path.exists():
        record = load_markdown(sidecar_path, vault_root)
        for key in ("source", "paper_id"):
            value = record.frontmatter.get(key)
            if isinstance(value, str) and value.strip():
                return value.strip()
    return ""


def _manifest_entry_for_source(
    vault_root: Path,
    raw_path: Path,
    existing: dict[str, Any] | None,
    companion_status: dict[str, bool],
    now_iso: str,
) -> dict[str, Any]:
    rel_path = raw_path.relative_to(vault_root).as_posix()
    source_class = source_class_for_raw(raw_path)
    summary_path = summary_for_raw(vault_root, raw_path).relative_to(vault_root).as_posix()
    source_meta = raw_source_metadata(vault_root, raw_path)
    plan = pdf_ingest_plan(vault_root, raw_path, companion_status) if source_class == "paper_pdf" else None

    if source_class == "paper_pdf":
        ingest_status = "deferred" if plan and plan.get("ingest_plan") == "paper-workbench" else "deferred-missing-skill"
    else:
        ingest_status = "ready-for-compile"

    entry = {
        "source_id": existing.get("source_id") if existing else _stable_source_id(rel_path),
        "path": rel_path,
        "source_type": source_class,
        "capture_origin": source_meta["capture_source"],
        "source_url_or_handle": _source_url_or_handle(vault_root, raw_path, source_class, plan),
        "content_hash": compute_hash(raw_path),
        "first_seen_at": existing.get("first_seen_at") if existing else now_iso,
        "last_seen_at": now_iso,
        "ingest_status": ingest_status,
        "normalized_outputs": [summary_path],
    }

    if plan and plan.get("ingest_plan") == "paper-workbench":
        entry["deferred_to"] = "paper-workbench"
    elif plan and plan.get("ingest_plan") == "skip":
        entry["deferred_to"] = "paper-workbench"

    metadata_path = plan.get("metadata_path") if plan else None
    if isinstance(metadata_path, str) and metadata_path.strip():
        entry["metadata_path"] = metadata_path.strip()

    return entry


def scan_ingest_delta(vault_root: Path) -> dict[str, Any]:
    manifest = load_source_manifest(vault_root)
    companion_status = detect_companion_skills()["skills"]
    existing_by_path = {
        str(item.get("path")): item
        for item in manifest.get("sources", [])
        if isinstance(item, dict) and item.get("path")
    }
    counts = {"new": 0, "changed": 0, "unchanged": 0, "removed": 0}
    items: list[dict[str, Any]] = []
    now_iso = _now_iso()
    seen_paths: set[str] = set()

    for raw_path in accepted_raw_sources(vault_root):
        rel_path = raw_path.relative_to(vault_root).as_posix()
        seen_paths.add(rel_path)
        existing = existing_by_path.get(rel_path)
        candidate = _manifest_entry_for_source(vault_root, raw_path, existing, companion_status, now_iso)
        if existing is None:
            status = "new"
            counts["new"] += 1
        elif any(existing.get(field) != candidate.get(field) for field in ("content_hash", "ingest_status", "normalized_outputs", "source_url_or_handle")):
            status = "changed"
            counts["changed"] += 1
        else:
            status = "unchanged"
            counts["unchanged"] += 1
        items.append({"status": status, **candidate})

    for rel_path, existing in sorted(existing_by_path.items()):
        if rel_path in seen_paths:
            continue
        counts["removed"] += 1
        removed_item = dict(existing)
        removed_item["status"] = "removed"
        items.append(removed_item)

    manifest_present = manifest_path(vault_root).exists()
    needs_ingest = manifest_present and any(counts[key] for key in ("new", "changed", "removed"))
    return {
        "vault_root": str(vault_root),
        "profile": manifest.get("profile") or resolve_vault_profile(vault_root),
        "manifest_path": manifest_path(vault_root).relative_to(vault_root).as_posix(),
        "manifest_present": manifest_present,
        "manifest_status": "current" if manifest_present and not needs_ingest else ("stale" if manifest_present else "missing"),
        "bootstrap_manifest_required": bool(not manifest_present and items),
        "needs_ingest": needs_ingest,
        "counts": counts,
        "items": items,
    }


def sync_source_manifest(vault_root: Path) -> dict[str, Any]:
    delta = scan_ingest_delta(vault_root)
    manifest = load_source_manifest(vault_root)
    current_entries = [item for item in delta["items"] if item.get("status") != "removed"]
    payload = {
        "version": MANIFEST_VERSION,
        "generated_at": _now_iso(),
        "profile": manifest.get("profile") or resolve_vault_profile(vault_root),
        "sources": [
            {field: entry.get(field, [] if field == "normalized_outputs" else "") for field in MANIFEST_FIELDS}
            | ({key: entry[key] for key in ("deferred_to", "metadata_path") if key in entry})
            for entry in sorted(current_entries, key=lambda item: str(item.get("path", "")))
        ],
    }
    output_path = write_source_manifest(vault_root, payload)
    return {
        **delta,
        "written_manifest": output_path.relative_to(vault_root).as_posix(),
        "manifest_present": True,
        "manifest_status": "current",
        "needs_ingest": False,
        "bootstrap_manifest_required": False,
    }
