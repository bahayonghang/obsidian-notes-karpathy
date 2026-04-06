from __future__ import annotations

import hashlib
import json
import re
from collections import Counter, defaultdict
from dataclasses import dataclass
from datetime import UTC, datetime
from pathlib import Path, PurePosixPath
from typing import Any


WIKILINK_RE = re.compile(r"\[\[([^\]]+)\]\]")
MARKDOWN_LINK_RE = re.compile(r"\[[^\]]+\]\(([^)]+)\)")
ALIAS_WIKILINK_RE = re.compile(r"\[\[[^|\]]+\|[^\]]+\]\]")
TABLE_LINE_RE = re.compile(r"^\s*\|.*\|\s*$")
FRONTMATTER_RE = re.compile(r"^---\n(.*?)\n---\n?(.*)$", re.DOTALL)


@dataclass(frozen=True)
class MarkdownRecord:
    path: str
    kind: str
    text: str
    body: str
    frontmatter: dict[str, Any]

    @property
    def path_no_ext(self) -> str:
        return self.path[:-3] if self.path.endswith(".md") else self.path

    @property
    def basename(self) -> str:
        return PurePosixPath(self.path_no_ext).name


def load_markdown(path: Path, vault_root: Path | None = None) -> MarkdownRecord:
    rel_path = path.relative_to(vault_root).as_posix() if vault_root and path.is_absolute() else path.as_posix()
    text = path.read_text(encoding="utf-8")
    frontmatter, body = parse_frontmatter(text)
    return MarkdownRecord(
        path=rel_path,
        kind=classify_markdown_path(rel_path),
        text=text,
        body=body,
        frontmatter=frontmatter,
    )


def classify_markdown_path(rel_path: str) -> str:
    if rel_path.startswith("wiki/concepts/"):
        return "concept"
    if rel_path.startswith("wiki/entities/"):
        return "entity"
    if rel_path.startswith("wiki/summaries/"):
        return "summary"
    if rel_path.startswith("wiki/indices/") or rel_path.startswith("wiki/indexes/"):
        return "index"
    if rel_path.startswith("outputs/qa/"):
        return "qa"
    if rel_path.startswith("raw/"):
        return "raw"
    return "other"


def parse_frontmatter(text: str) -> tuple[dict[str, Any], str]:
    match = FRONTMATTER_RE.match(text)
    if not match:
        return {}, text

    raw_frontmatter, body = match.groups()
    return parse_simple_yaml(raw_frontmatter), body


def parse_simple_yaml(raw_text: str) -> dict[str, Any]:
    data: dict[str, Any] = {}
    current_key: str | None = None

    for raw_line in raw_text.splitlines():
        line = raw_line.rstrip()
        if not line or line.lstrip().startswith("#"):
            continue

        if line.startswith("  - ") and current_key:
            data.setdefault(current_key, [])
            data[current_key].append(parse_scalar(line[4:].strip()))
            continue

        if ":" not in line:
            current_key = None
            continue

        key, value = line.split(":", 1)
        key = key.strip()
        value = value.strip()

        if not value:
            data[key] = []
            current_key = key
            continue

        current_key = None
        data[key] = parse_scalar(value)

    return data


def parse_scalar(value: str) -> Any:
    stripped = value.strip()

    if stripped in {"[]", "[ ]"}:
        return []

    if stripped.startswith("[") and stripped.endswith("]"):
        inner = stripped[1:-1].strip()
        if not inner:
            return []
        return [parse_scalar(part.strip()) for part in inner.split(",")]

    if stripped.startswith('"') and stripped.endswith('"'):
        return stripped[1:-1]

    if stripped.startswith("'") and stripped.endswith("'"):
        return stripped[1:-1]

    return stripped


def json_dump(payload: dict[str, Any]) -> str:
    return json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True)


def collapse_posix(path_like: str) -> str:
    parts: list[str] = []
    for part in PurePosixPath(path_like).parts:
        if part in {"", "."}:
            continue
        if part == "..":
            if parts:
                parts.pop()
            continue
        parts.append(part)
    return "/".join(parts)


def accepted_raw_sources(vault_root: Path) -> list[Path]:
    raw_root = vault_root / "raw"
    if not raw_root.exists():
        return []

    sources: list[Path] = []
    for path in sorted(raw_root.rglob("*.md")):
        rel = path.relative_to(vault_root)
        if any(part.startswith(".") for part in rel.parts):
            continue
        if path.name.startswith("_"):
            continue
        if "assets" in rel.parts:
            continue
        if len(rel.parts) == 2:
            sources.append(path)
            continue
        if rel.parts[1] in {"articles", "papers", "podcasts", "repos"}:
            sources.append(path)
    return sources


def summary_for_raw(vault_root: Path, raw_path: Path) -> Path:
    return vault_root / "wiki" / "summaries" / raw_path.name


def iso_from_timestamp(timestamp: float) -> str:
    return datetime.fromtimestamp(timestamp, UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def parse_datetime(value: Any) -> datetime | None:
    if not value or not isinstance(value, str):
        return None

    candidate = value.strip()
    if not candidate:
        return None

    if re.fullmatch(r"\d{4}-\d{2}-\d{2}", candidate):
        candidate = f"{candidate}T00:00:00Z"

    if candidate.endswith("Z"):
        candidate = candidate[:-1] + "+00:00"

    try:
        return datetime.fromisoformat(candidate).astimezone(UTC)
    except ValueError:
        return None


def compute_hash(path: Path) -> str:
    digest = hashlib.sha256()
    digest.update(path.read_bytes())
    return digest.hexdigest()


def scan_compile_delta(vault_root: Path) -> dict[str, Any]:
    items: list[dict[str, Any]] = []
    counts = Counter()

    for raw_path in accepted_raw_sources(vault_root):
        rel_path = raw_path.relative_to(vault_root).as_posix()
        summary_path = summary_for_raw(vault_root, raw_path)
        raw_hash = compute_hash(raw_path)
        raw_mtime = iso_from_timestamp(raw_path.stat().st_mtime)
        item = {
            "path": rel_path,
            "summary_path": summary_path.relative_to(vault_root).as_posix(),
            "raw_hash": raw_hash,
            "raw_mtime": raw_mtime,
        }

        if not summary_path.exists():
            item["status"] = "new"
            item["reason"] = "missing_summary"
            counts["new"] += 1
            items.append(item)
            continue

        summary_record = load_markdown(summary_path, vault_root)
        summary_fm = summary_record.frontmatter
        summary_hash = summary_fm.get("source_hash")
        summary_mtime = parse_datetime(summary_fm.get("source_mtime"))
        raw_mtime_dt = parse_datetime(raw_mtime)

        if summary_hash:
            item["recorded_source_hash"] = summary_hash
            if summary_hash == raw_hash:
                item["status"] = "unchanged"
                item["reason"] = "source_hash_match"
                counts["unchanged"] += 1
            else:
                item["status"] = "changed"
                item["reason"] = "source_hash_mismatch"
                counts["changed"] += 1
            items.append(item)
            continue

        if summary_mtime and raw_mtime_dt and raw_mtime_dt > summary_mtime:
            item["status"] = "changed"
            item["reason"] = "source_mtime_outdated"
            item["recorded_source_mtime"] = summary_fm.get("source_mtime")
            counts["changed"] += 1
        else:
            item["status"] = "unchanged"
            item["reason"] = "source_mtime_current"
            item["recorded_source_mtime"] = summary_fm.get("source_mtime")
            counts["unchanged"] += 1

        items.append(item)

    payload_counts = {
        "new": counts["new"],
        "changed": counts["changed"],
        "unchanged": counts["unchanged"],
        "skipped": counts["skipped"],
    }

    return {
        "vault_root": str(vault_root),
        "counts": payload_counts,
        "items": items,
    }


def collect_markdown_records(vault_root: Path) -> list[MarkdownRecord]:
    records: list[MarkdownRecord] = []
    for rel_root in ("raw", "wiki", "outputs/qa"):
        root = vault_root / rel_root
        if not root.exists():
            continue
        for path in sorted(root.rglob("*.md")):
            rel = path.relative_to(vault_root)
            if any(part.startswith(".") for part in rel.parts):
                continue
            records.append(load_markdown(path, vault_root))
    return records


def extract_wikilinks(text: str) -> list[str]:
    return WIKILINK_RE.findall(text)


def extract_markdown_links(text: str) -> list[str]:
    return MARKDOWN_LINK_RE.findall(text)


def strip_link_alias(target: str) -> str:
    candidate = target.split("|", 1)[0].strip()
    return candidate.split("#", 1)[0].strip()


def registry_for_records(records: list[MarkdownRecord]) -> tuple[dict[str, MarkdownRecord], dict[str, list[MarkdownRecord]]]:
    by_path = {record.path_no_ext: record for record in records}
    by_basename: dict[str, list[MarkdownRecord]] = defaultdict(list)
    for record in records:
        by_basename[record.basename].append(record)
    return by_path, by_basename


def resolve_target(
    source_record: MarkdownRecord,
    target: str,
    by_path: dict[str, MarkdownRecord],
    by_basename: dict[str, list[MarkdownRecord]],
) -> list[MarkdownRecord]:
    stripped = strip_link_alias(target)
    if not stripped:
        return []

    normalized = stripped[:-3] if stripped.endswith(".md") else stripped
    normalized = normalized.lstrip("/")

    candidates: list[MarkdownRecord] = []
    if normalized.startswith("./") or normalized.startswith("../"):
        joined = collapse_posix(f"{PurePosixPath(source_record.path).parent.as_posix()}/{normalized}")
        if joined in by_path:
            candidates.append(by_path[joined])
    elif normalized in by_path:
        candidates.append(by_path[normalized])
    else:
        basename = PurePosixPath(normalized).name
        candidates.extend(by_basename.get(basename, []))

    deduped: dict[str, MarkdownRecord] = {candidate.path_no_ext: candidate for candidate in candidates}
    return list(deduped.values())


def normalize_identity(value: str) -> str:
    lowered = value.lower()
    normalized = re.sub(r"[^a-z0-9]+", " ", lowered).strip()
    return normalized


def slugify_identity(value: str) -> str:
    normalized = normalize_identity(value)
    return normalized.replace(" ", "-")


def record_identities(record: MarkdownRecord) -> set[str]:
    identities: set[str] = set()
    title = record.frontmatter.get("title")
    if isinstance(title, str) and title.strip():
        identities.add(title)

    identities.add(record.basename.replace("-", " "))
    identities.add(record.basename)

    aliases = record.frontmatter.get("aliases", [])
    if isinstance(aliases, list):
        for alias in aliases:
            if isinstance(alias, str) and alias.strip():
                identities.add(alias)

    return {identity for identity in identities if normalize_identity(identity)}


def duplicate_identity_issues(records: list[MarkdownRecord]) -> list[dict[str, Any]]:
    identity_map: dict[str, set[str]] = defaultdict(set)
    identity_type: dict[str, str] = {}

    for record in records:
        if record.kind not in {"concept", "entity"}:
            continue
        for identity in record_identities(record):
            normalized = normalize_identity(identity)
            if not normalized:
                continue
            identity_map[normalized].add(record.path)
            identity_type[normalized] = record.kind

    issues: list[dict[str, Any]] = []
    for normalized, paths in sorted(identity_map.items()):
        if len(paths) < 2:
            continue
        issues.append(
            {
                "kind": f"duplicate_{identity_type[normalized]}",
                "normalized_key": slugify_identity(normalized),
                "paths": sorted(paths),
            }
        )
    return issues


def alias_wikilink_table_issues(records: list[MarkdownRecord]) -> list[dict[str, Any]]:
    issues: list[dict[str, Any]] = []
    for record in records:
        for index, line in enumerate(record.text.splitlines(), start=1):
            if not TABLE_LINE_RE.match(line):
                continue
            if not ALIAS_WIKILINK_RE.search(line):
                continue
            issues.append(
                {
                    "kind": "alias_wikilink_in_table",
                    "path": record.path,
                    "line": index,
                    "excerpt": line.strip(),
                }
            )
    return issues


def broken_wikilink_issues(records: list[MarkdownRecord]) -> list[dict[str, Any]]:
    by_path, by_basename = registry_for_records(records)
    issues: list[dict[str, Any]] = []
    seen: set[tuple[str, str]] = set()

    for record in records:
        for target in extract_wikilinks(record.text):
            stripped = strip_link_alias(target)
            key = (record.path, stripped)
            if key in seen:
                continue
            seen.add(key)
            resolved = resolve_target(record, stripped, by_path, by_basename)
            if resolved:
                continue
            issues.append(
                {
                    "kind": "broken_wikilink",
                    "path": record.path,
                    "target": stripped,
                }
            )
    return issues


def orphan_page_issues(records: list[MarkdownRecord]) -> list[dict[str, Any]]:
    by_path, by_basename = registry_for_records(records)
    inbound = Counter()
    trackable = {
        record.path_no_ext: record
        for record in records
        if record.kind in {"concept", "entity", "summary", "qa"}
        and not record.basename.startswith("_")
    }

    for record in records:
        for target in extract_wikilinks(record.text):
            for resolved in resolve_target(record, target, by_path, by_basename):
                inbound[resolved.path_no_ext] += 1

    issues: list[dict[str, Any]] = []
    for key, record in sorted(trackable.items()):
        if inbound[key] > 0:
            continue
        issues.append(
            {
                "kind": "orphan_page",
                "path": record.path,
            }
        )
    return issues


def stale_qa_issues(records: list[MarkdownRecord]) -> list[dict[str, Any]]:
    by_path, by_basename = registry_for_records(records)
    issues: list[dict[str, Any]] = []

    for record in records:
        if record.kind != "qa":
            continue

        asked_at = parse_datetime(record.frontmatter.get("asked_at"))
        if not asked_at:
            continue

        sources = record.frontmatter.get("sources", [])
        if not isinstance(sources, list):
            continue

        newer_sources: list[str] = []
        for source in sources:
            if not isinstance(source, str):
                continue
            resolved = resolve_target(record, strip_link_alias(source), by_path, by_basename)
            if not resolved:
                continue
            candidate = resolved[0]
            updated_at = (
                parse_datetime(candidate.frontmatter.get("updated_at"))
                or parse_datetime(candidate.frontmatter.get("compiled_at"))
                or parse_datetime(candidate.frontmatter.get("date"))
            )
            if updated_at and updated_at > asked_at:
                newer_sources.append(candidate.path)

        if newer_sources:
            issues.append(
                {
                    "kind": "stale_qa",
                    "path": record.path,
                    "newer_sources": sorted(newer_sources),
                }
            )

    return issues


def audit_vault_mechanics(vault_root: Path) -> dict[str, Any]:
    records = collect_markdown_records(vault_root)
    issues = []
    issues.extend(alias_wikilink_table_issues(records))
    issues.extend(duplicate_identity_issues(records))
    issues.extend(stale_qa_issues(records))
    issues.extend(broken_wikilink_issues(records))
    issues.extend(orphan_page_issues(records))

    counts = Counter(issue["kind"] for issue in issues)
    return {
        "vault_root": str(vault_root),
        "issue_counts": dict(sorted(counts.items())),
        "issues": sorted(issues, key=lambda issue: (issue["kind"], issue.get("path", ""), issue.get("line", 0))),
    }
