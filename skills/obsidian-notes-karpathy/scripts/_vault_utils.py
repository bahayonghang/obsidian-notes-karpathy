from __future__ import annotations

import hashlib
import json
import os
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
ARXIV_ID_RE = re.compile(r"(?<!\d)(\d{4}\.\d{4,5}(?:v\d+)?)(?!\d)")
ARXIV_URL_RE = re.compile(
    r"https?://(?:www\.)?(?:arxiv\.org/(?:abs|pdf)|alphaxiv\.org/(?:overview|abs))/([^?#/]+)"
)
GUIDANCE_CONTRACTS = (
    ("agents", "AGENTS.md", True),
    ("claude", "CLAUDE.md", False),
)
PDF_SIDECAR_SUFFIX = ".source.md"
PDF_COMPANION_SKILLS = ("paper-workbench", "pdf")
REVIEWABLE_DRAFT_ROOTS = (
    "wiki/drafts/summaries",
    "wiki/drafts/concepts",
    "wiki/drafts/entities",
)
TERMINAL_REVIEW_STATES = {"promoted", "rejected", "archived"}
HEALTH_FLAG_KINDS = {
    "duplicate_concept",
    "duplicate_entity",
    "stale_qa",
    "alias_wikilink_in_table",
    "unapproved_live_page",
    "approved_conflict",
    "review_backlog",
}


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
    normalized = rel_path.replace("\\", "/")
    if normalized.startswith("wiki/briefings/"):
        return "briefing"
    if normalized.startswith("outputs/reviews/"):
        return "review"
    if normalized.startswith("wiki/live/concepts/") or normalized.startswith("wiki/drafts/concepts/"):
        return "concept"
    if normalized.startswith("wiki/live/entities/") or normalized.startswith("wiki/drafts/entities/"):
        return "entity"
    if normalized.startswith("wiki/live/summaries/") or normalized.startswith("wiki/drafts/summaries/"):
        return "summary"
    if normalized.startswith("wiki/indices/") or normalized.startswith("wiki/indexes/"):
        return "index"
    if normalized.startswith("outputs/qa/"):
        return "qa"
    if normalized.startswith("raw/"):
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


def summarize_local_guidance(file_names: list[str]) -> dict[str, Any]:
    matches_by_name: dict[str, list[str]] = defaultdict(list)
    for file_name in file_names:
        matches_by_name[file_name.lower()].append(file_name)

    status: dict[str, Any] = {}
    warnings: list[str] = []
    blocking_issues: list[str] = []

    for key, canonical_name, required in GUIDANCE_CONTRACTS:
        matches = sorted(
            matches_by_name.get(canonical_name.lower(), []),
            key=lambda value: (value != canonical_name, value.lower(), value),
        )
        present = bool(matches)
        canonical = canonical_name in matches
        selected = canonical_name if canonical else (matches[0] if matches else None)

        if len(matches) > 1:
            issue = f"duplicate_{key}_guidance_files"
            warnings.append(issue)
            blocking_issues.append(issue)

        if present and not canonical:
            warnings.append(f"noncanonical_{key}_guidance_name")

        if required and not present:
            blocking_issues.append(f"missing_{key}_guidance")
        elif not required and not present:
            warnings.append(f"missing_{key}_guidance")

        status[key] = {
            "present": present,
            "path": selected,
            "canonical": canonical,
            "candidates": matches,
        }

    status["warnings"] = sorted(set(warnings))
    status["blocking_issues"] = sorted(set(blocking_issues))
    return status


def inspect_local_guidance(vault_root: Path) -> dict[str, Any]:
    if not vault_root.exists():
        return summarize_local_guidance([])

    file_names = [path.name for path in vault_root.iterdir() if path.is_file()]
    return summarize_local_guidance(file_names)


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


def parse_number(value: Any) -> float | None:
    if value is None:
        return None
    if isinstance(value, (int, float)):
        return float(value)
    if isinstance(value, str):
        stripped = value.strip()
        if not stripped:
            return None
        try:
            return float(stripped)
        except ValueError:
            return None
    return None


def list_field(frontmatter: dict[str, Any], key: str) -> list[str]:
    raw_value = frontmatter.get(key, [])
    if isinstance(raw_value, list):
        return [item for item in raw_value if isinstance(item, str) and item.strip()]
    if isinstance(raw_value, str) and raw_value.strip():
        return [raw_value.strip()]
    return []


def is_pdf_sidecar(path: Path) -> bool:
    return path.name.endswith(PDF_SIDECAR_SUFFIX)


def sidecar_for_pdf(raw_path: Path) -> Path:
    return raw_path.with_name(f"{raw_path.stem}{PDF_SIDECAR_SUFFIX}")


def detect_contract_version(vault_root: Path) -> str:
    has_v2 = any(
        (vault_root / rel_path).exists()
        for rel_path in (
            "wiki/drafts",
            "wiki/live",
            "wiki/briefings",
            "outputs/reviews",
            "raw/human",
            "raw/agents",
        )
    )
    if has_v2:
        return "v2"

    has_v1 = any(
        (vault_root / rel_path).exists()
        for rel_path in (
            "wiki/summaries",
            "wiki/concepts",
            "wiki/entities",
            "raw/articles",
            "raw/papers",
            "raw/podcasts",
            "raw/repos",
        )
    )
    if has_v1:
        return "v1"

    return "none"


def raw_source_metadata(vault_root: Path, raw_path: Path) -> dict[str, Any]:
    rel_parts = raw_path.relative_to(vault_root).parts
    metadata = {
        "capture_source": "legacy",
        "capture_trust": "legacy",
        "agent_role": None,
        "legacy_layout": True,
    }

    if len(rel_parts) >= 3 and rel_parts[1] == "human":
        metadata.update(
            {
                "capture_source": "human",
                "capture_trust": "curated",
                "legacy_layout": False,
            }
        )
    elif len(rel_parts) >= 4 and rel_parts[1] == "agents":
        metadata.update(
            {
                "capture_source": "agent",
                "capture_trust": "untrusted",
                "agent_role": rel_parts[2],
                "legacy_layout": False,
            }
        )

    return metadata


def source_class_for_raw(raw_path: Path) -> str:
    return "paper_pdf" if raw_path.suffix.lower() == ".pdf" else "markdown"


def configured_skill_roots() -> list[Path]:
    override = os.environ.get("KB_COMPANION_SKILL_PATHS")
    candidates: list[Path] = []

    if override is not None:
        candidates.extend(
            Path(part).expanduser()
            for part in override.split(os.pathsep)
            if part.strip()
        )
    else:
        codex_home = os.environ.get("CODEX_HOME")
        if codex_home:
            candidates.append(Path(codex_home).expanduser() / "skills")

        home = Path.home()
        candidates.extend(
            [
                home / ".codex" / "skills",
                home / ".claude" / "skills",
                home / ".agents" / "skills",
            ]
        )

    deduped: list[Path] = []
    seen: set[str] = set()
    for candidate in candidates:
        key = str(candidate)
        if key in seen:
            continue
        seen.add(key)
        deduped.append(candidate)
    return deduped


def detect_companion_skills(skill_roots: list[Path] | None = None) -> dict[str, Any]:
    roots = skill_roots if skill_roots is not None else configured_skill_roots()
    availability = {
        skill_name: any((root / skill_name / "SKILL.md").exists() for root in roots)
        for skill_name in PDF_COMPANION_SKILLS
    }
    return {
        "search_roots": [str(root) for root in roots],
        "skills": availability,
    }


def extract_paper_handle(candidate: Any) -> str | None:
    if not isinstance(candidate, str):
        return None

    text = candidate.strip()
    if not text:
        return None

    url_match = ARXIV_URL_RE.search(text)
    if url_match:
        url_tail = url_match.group(1).removesuffix(".pdf")
        handle_match = ARXIV_ID_RE.search(url_tail)
        if handle_match:
            return handle_match.group(1)

    handle_match = ARXIV_ID_RE.search(text)
    if handle_match:
        return handle_match.group(1)

    return None


def pdf_ingest_plan(
    vault_root: Path,
    raw_path: Path,
    companion_status: dict[str, bool],
) -> dict[str, Any]:
    plan: dict[str, Any] = {
        "source_class": "paper_pdf",
        "paper_handle": None,
        "paper_handle_source": None,
        "ingest_plan": None,
        "ingest_reason": None,
        "companion_status": dict(companion_status),
    }

    sidecar_path = sidecar_for_pdf(raw_path)
    if sidecar_path.exists():
        sidecar_record = load_markdown(sidecar_path, vault_root)
        plan["metadata_path"] = sidecar_record.path

        title = sidecar_record.frontmatter.get("title")
        if isinstance(title, str) and title.strip():
            plan["paper_title"] = title.strip()

        source_url = sidecar_record.frontmatter.get("source")
        if isinstance(source_url, str) and source_url.strip():
            plan["source_url"] = source_url.strip()

        for source_name, candidate in (
            ("paper_id", sidecar_record.frontmatter.get("paper_id")),
            ("source", sidecar_record.frontmatter.get("source")),
        ):
            handle = extract_paper_handle(candidate)
            if handle:
                plan["paper_handle"] = handle
                plan["paper_handle_source"] = source_name
                break

    if not plan["paper_handle"]:
        handle = extract_paper_handle(raw_path.stem)
        if handle:
            plan["paper_handle"] = handle
            plan["paper_handle_source"] = "filename"

    if companion_status["paper-workbench"]:
        plan["ingest_plan"] = "paper-workbench"
        plan["ingest_reason"] = "paper_workbench_directory_policy"
        return plan

    plan["ingest_plan"] = "skip"
    plan["ingest_reason"] = "paper_workbench_required_for_raw_papers"
    return plan


def accepted_raw_sources(vault_root: Path) -> list[Path]:
    raw_root = vault_root / "raw"
    if not raw_root.exists():
        return []

    sources: list[Path] = []
    for path in sorted(raw_root.rglob("*")):
        if not path.is_file():
            continue

        rel = path.relative_to(vault_root)
        if any(part.startswith(".") for part in rel.parts):
            continue
        if path.name.startswith("_"):
            continue
        if is_pdf_sidecar(path):
            continue
        if "assets" in rel.parts:
            continue

        suffix = path.suffix.lower()
        if suffix not in {".md", ".pdf"}:
            continue

        if suffix == ".pdf" and "papers" not in rel.parts:
            continue

        sources.append(path)

    return sources


def draft_summary_for_raw(vault_root: Path, raw_path: Path) -> Path:
    rel = raw_path.relative_to(vault_root / "raw")
    return vault_root / "wiki" / "drafts" / "summaries" / rel.with_suffix(".md")


def legacy_summary_for_raw(vault_root: Path, raw_path: Path) -> Path:
    summary_name = raw_path.name if raw_path.suffix.lower() == ".md" else f"{raw_path.stem}.md"
    return vault_root / "wiki" / "summaries" / summary_name


def summary_for_raw(vault_root: Path, raw_path: Path) -> Path:
    contract_version = detect_contract_version(vault_root)
    if contract_version == "v1":
        return legacy_summary_for_raw(vault_root, raw_path)
    return draft_summary_for_raw(vault_root, raw_path)


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
    ingest_counts = Counter()
    companion_skills = detect_companion_skills()
    companion_status = companion_skills["skills"]
    contract_version = detect_contract_version(vault_root)

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
            "source_class": source_class_for_raw(raw_path),
            "contract_version": contract_version if contract_version != "none" else "v2",
        }
        item.update(raw_source_metadata(vault_root, raw_path))

        if item["source_class"] == "paper_pdf":
            item.update(pdf_ingest_plan(vault_root, raw_path, companion_status))
        else:
            item["ingest_plan"] = "markdown"
            item["ingest_reason"] = "markdown_source"

        ingest_counts[item["ingest_plan"]] += 1

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
        "contract_version": contract_version if contract_version != "none" else "v2",
        "counts": payload_counts,
        "ingest_counts": dict(sorted(ingest_counts.items())),
        "companion_skills": companion_skills,
        "items": items,
    }


def iter_markdown_records(vault_root: Path, roots: tuple[str, ...]) -> list[MarkdownRecord]:
    records: list[MarkdownRecord] = []
    for rel_root in roots:
        root = vault_root / rel_root
        if not root.exists():
            continue
        for path in sorted(root.rglob("*.md")):
            rel = path.relative_to(vault_root)
            if any(part.startswith(".") for part in rel.parts):
                continue
            records.append(load_markdown(path, vault_root))
    return records


def collect_markdown_records(vault_root: Path) -> list[MarkdownRecord]:
    contract_version = detect_contract_version(vault_root)
    if contract_version == "v2":
        return iter_markdown_records(
            vault_root,
            ("raw", "wiki/live", "wiki/briefings", "outputs/qa", "outputs/reviews"),
        )
    return iter_markdown_records(vault_root, ("raw", "wiki", "outputs/qa"))


def reviewable_draft_records(vault_root: Path) -> list[MarkdownRecord]:
    records = iter_markdown_records(vault_root, REVIEWABLE_DRAFT_ROOTS)
    return [record for record in records if not record.basename.startswith("_")]


def extract_wikilinks(text: str) -> list[str]:
    return WIKILINK_RE.findall(text)


def extract_markdown_links(text: str) -> list[str]:
    return MARKDOWN_LINK_RE.findall(text)


def strip_link_alias(target: str) -> str:
    candidate = target.strip()
    if candidate.startswith("[[") and candidate.endswith("]]"):
        candidate = candidate[2:-2].strip()
    candidate = candidate.split("|", 1)[0].strip()
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
        if "/drafts/" in record.path:
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
        and "/drafts/" not in record.path
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

        sources = list_field(record.frontmatter, "sources")
        newer_sources: list[str] = []
        for source in sources:
            resolved = resolve_target(record, strip_link_alias(source), by_path, by_basename)
            if not resolved:
                continue
            candidate = resolved[0]
            updated_at = (
                parse_datetime(candidate.frontmatter.get("updated_at"))
                or parse_datetime(candidate.frontmatter.get("approved_at"))
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


def live_record_for_draft(vault_root: Path, draft_record: MarkdownRecord) -> str:
    relative = draft_record.path.removeprefix("wiki/drafts/")
    live_path = vault_root / "wiki" / "live" / relative
    return live_path.relative_to(vault_root).as_posix()


def review_record_for_draft(draft_record: MarkdownRecord) -> str:
    suffix = draft_record.path.removeprefix("wiki/drafts/").replace("/", "--")
    return f"outputs/reviews/{suffix}"


def compute_review_score(frontmatter: dict[str, Any]) -> float | None:
    explicit = parse_number(frontmatter.get("review_score"))
    if explicit is not None:
        return round(explicit, 3)

    components: list[float] = []
    for key in ("accuracy", "provenance", "composability"):
        value = parse_number(frontmatter.get(key))
        if value is not None:
            components.append(value)

    conflict_risk = parse_number(frontmatter.get("conflict_risk"))
    if conflict_risk is not None:
        components.append(max(0.0, 1.0 - conflict_risk))

    if not components:
        return None

    return round(sum(components) / len(components), 3)


def scan_review_queue(vault_root: Path) -> dict[str, Any]:
    items: list[dict[str, Any]] = []
    decision_counts = Counter()
    state_counts = Counter()
    pending_count = 0

    for record in reviewable_draft_records(vault_root):
        frontmatter = record.frontmatter
        state = str(frontmatter.get("review_state") or "pending").strip() or "pending"
        score = compute_review_score(frontmatter)
        blocking_flags = list_field(frontmatter, "blocking_flags")
        live_conflict = "live_conflict" in blocking_flags or frontmatter.get("status") == "conflicting"
        hard_flags = [flag for flag in blocking_flags if flag != "live_conflict"]
        live_path = live_record_for_draft(vault_root, record)
        live_exists = (vault_root / live_path).exists()

        if state in TERMINAL_REVIEW_STATES:
            pending = False
            decision = "approve" if state == "promoted" else "reject"
            reason = "terminal_state"
        else:
            pending = True
            pending_count += 1
            if hard_flags:
                decision = "reject"
                reason = "blocking_flags"
            elif score is None:
                decision = "needs-human"
                reason = "missing_review_score"
            elif live_conflict:
                decision = "needs-human"
                reason = "live_conflict"
            elif score >= 0.85:
                decision = "approve"
                reason = "threshold_met"
            elif score < 0.60:
                decision = "reject"
                reason = "score_below_floor"
            else:
                decision = "needs-human"
                reason = "score_band_requires_judgment"

        decision_counts[decision] += 1
        state_counts[state] += 1
        items.append(
            {
                "path": record.path,
                "kind": record.kind,
                "review_state": state,
                "review_score": score,
                "blocking_flags": blocking_flags,
                "decision": decision,
                "reason": reason,
                "pending": pending,
                "live_path": live_path,
                "live_exists": live_exists,
                "review_record_path": review_record_for_draft(record),
            }
        )

    return {
        "vault_root": str(vault_root),
        "counts": {
            "pending": pending_count,
            "approve": decision_counts["approve"],
            "reject": decision_counts["reject"],
            "needs-human": decision_counts["needs-human"],
        },
        "state_counts": dict(sorted(state_counts.items())),
        "items": items,
    }


def query_scope(vault_root: Path) -> dict[str, Any]:
    contract_version = detect_contract_version(vault_root)
    if contract_version == "v2":
        included_roots = ("wiki/live", "wiki/briefings", "outputs/qa")
        excluded_roots = ("raw", "wiki/drafts", "outputs/reviews")
    else:
        included_roots = ("wiki", "outputs/qa")
        excluded_roots = ("raw",)

    included = [
        path.relative_to(vault_root).as_posix()
        for rel_root in included_roots
        for path in sorted((vault_root / rel_root).rglob("*.md"))
        if (vault_root / rel_root).exists()
    ]
    excluded = [
        path.relative_to(vault_root).as_posix()
        for rel_root in excluded_roots
        for path in sorted((vault_root / rel_root).rglob("*.md"))
        if (vault_root / rel_root).exists()
    ]

    return {
        "vault_root": str(vault_root),
        "contract_version": contract_version,
        "included_paths": included,
        "excluded_paths": excluded,
        "excluded_prefixes": list(excluded_roots),
    }


def live_records(records: list[MarkdownRecord]) -> list[MarkdownRecord]:
    return [record for record in records if record.path.startswith("wiki/live/")]


def briefing_staleness_issues(vault_root: Path) -> list[dict[str, Any]]:
    records = collect_markdown_records(vault_root)
    by_path, by_basename = registry_for_records(records)
    issues: list[dict[str, Any]] = []

    for record in records:
        if record.kind != "briefing":
            continue

        updated_at = parse_datetime(record.frontmatter.get("updated_at"))
        staleness_after = parse_datetime(record.frontmatter.get("staleness_after"))
        source_live_pages = list_field(record.frontmatter, "source_live_pages")
        resolved_sources: list[MarkdownRecord] = []
        for source in source_live_pages:
            resolved_sources.extend(resolve_target(record, source, by_path, by_basename))

        newest_source_time: datetime | None = None
        newest_source_path: str | None = None
        for source in resolved_sources:
            candidate_time = (
                parse_datetime(source.frontmatter.get("updated_at"))
                or parse_datetime(source.frontmatter.get("approved_at"))
                or parse_datetime(source.frontmatter.get("compiled_at"))
            )
            if candidate_time and (newest_source_time is None or candidate_time > newest_source_time):
                newest_source_time = candidate_time
                newest_source_path = source.path

        stale = False
        reason = None
        if newest_source_time and updated_at and newest_source_time > updated_at:
            stale = True
            reason = "source_live_page_newer_than_briefing"
        elif newest_source_time and staleness_after and newest_source_time > staleness_after:
            stale = True
            reason = "staleness_threshold_crossed"

        if stale:
            issues.append(
                {
                    "kind": "stale_briefing",
                    "path": record.path,
                    "reason": reason,
                    "newer_source": newest_source_path,
                }
            )

    return issues


def unapproved_live_issues(records: list[MarkdownRecord]) -> list[dict[str, Any]]:
    issues: list[dict[str, Any]] = []
    for record in live_records(records):
        if record.kind not in {"summary", "concept", "entity"}:
            continue
        trust_level = record.frontmatter.get("trust_level")
        approved_at = record.frontmatter.get("approved_at")
        review_record = record.frontmatter.get("review_record")
        if trust_level == "approved" and approved_at and review_record:
            continue
        issues.append(
            {
                "kind": "unapproved_live_page",
                "path": record.path,
            }
        )
    return issues


def approved_conflict_issues(records: list[MarkdownRecord]) -> list[dict[str, Any]]:
    issues: list[dict[str, Any]] = []
    for record in live_records(records):
        if record.kind not in {"concept", "entity"}:
            continue
        if record.frontmatter.get("trust_level") == "approved" and record.frontmatter.get("status") == "conflicting":
            issues.append(
                {
                    "kind": "approved_conflict",
                    "path": record.path,
                }
            )
    return issues


def review_backlog_issues(vault_root: Path) -> list[dict[str, Any]]:
    queue = scan_review_queue(vault_root)
    return [
        {
            "kind": "review_backlog",
            "path": item["path"],
            "decision": item["decision"],
        }
        for item in queue["items"]
        if item["pending"]
    ]


def audit_vault_mechanics(vault_root: Path) -> dict[str, Any]:
    records = collect_markdown_records(vault_root)
    issues: list[dict[str, Any]] = []
    issues.extend(alias_wikilink_table_issues(records))
    issues.extend(duplicate_identity_issues(records))
    issues.extend(stale_qa_issues(records))
    issues.extend(broken_wikilink_issues(records))
    issues.extend(orphan_page_issues(records))
    issues.extend(unapproved_live_issues(records))
    issues.extend(approved_conflict_issues(records))
    issues.extend(review_backlog_issues(vault_root))
    issues.extend(briefing_staleness_issues(vault_root))

    counts = Counter(issue["kind"] for issue in issues)
    return {
        "vault_root": str(vault_root),
        "contract_version": detect_contract_version(vault_root),
        "issue_counts": dict(sorted(counts.items())),
        "issues": sorted(issues, key=lambda issue: (issue["kind"], issue.get("path", ""), issue.get("line", 0))),
    }
