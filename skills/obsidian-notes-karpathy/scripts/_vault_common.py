from __future__ import annotations

import json
import re
from collections import defaultdict
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
    normalized = rel_path.replace("\\", "/")
    if normalized == "MEMORY.md":
        return "memory"
    if normalized.startswith("wiki/briefings/"):
        return "briefing"
    if normalized.startswith("outputs/reviews/"):
        return "review"
    if normalized.startswith("outputs/content/"):
        return "content_output"
    if normalized.startswith("wiki/live/concepts/") or normalized.startswith("wiki/drafts/concepts/"):
        return "concept"
    if normalized.startswith("wiki/live/entities/") or normalized.startswith("wiki/drafts/entities/"):
        return "entity"
    if normalized.startswith("wiki/live/overviews/") or normalized.startswith("wiki/drafts/overviews/"):
        return "overview"
    if normalized.startswith("wiki/live/comparisons/") or normalized.startswith("wiki/drafts/comparisons/"):
        return "comparison"
    if normalized.startswith("wiki/live/summaries/") or normalized.startswith("wiki/drafts/summaries/"):
        return "summary"
    if (
        normalized.startswith("wiki/live/indices/")
        or normalized.startswith("wiki/drafts/indices/")
        or normalized.startswith("wiki/indices/")
        or normalized.startswith("wiki/indexes/")
    ):
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
    """Parse flat YAML frontmatter into a dict.

    Known limitations (intentional YAGNI trade-off):
    - Only supports flat key: value pairs and single-level lists (  - item).
    - Does not handle nested objects, multi-line values, or block scalars (| / >).
    - Assumes 2-space indented list items under the parent key.
    - Sufficient for current Obsidian frontmatter; revisit if schemas grow nested.
    """
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
