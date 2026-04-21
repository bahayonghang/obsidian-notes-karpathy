from __future__ import annotations

from datetime import UTC, datetime
from pathlib import Path

from _vault_common import MarkdownRecord, list_field
from _vault_layout import collect_markdown_records


RENDER_MODES = ("slides", "charts", "canvas", "report", "web")
ARCHIVED_RENDER_SOURCE_KINDS = {
    "briefing",
    "qa",
    "content_output",
    "report_output",
    "slide_output",
    "chart_output",
}


def records_by_path(vault_root: Path) -> dict[str, MarkdownRecord]:
    return {record.path: record for record in collect_markdown_records(vault_root)}


def resolve_sources(vault_root: Path, source_paths: list[str]) -> tuple[list[MarkdownRecord], list[str]]:
    records = records_by_path(vault_root)
    resolved: list[MarkdownRecord] = []
    rejected: list[str] = []
    for path in source_paths:
        normalized = path.replace("\\", "/")
        candidate = records.get(normalized)
        if candidate is None:
            rejected.append(normalized)
            continue
        if is_allowed_render_source(candidate):
            resolved.append(candidate)
        else:
            rejected.append(normalized)
    return resolved, rejected


def is_allowed_render_source(record: MarkdownRecord) -> bool:
    if record.path.startswith("wiki/live/"):
        return True
    if record.kind in ARCHIVED_RENDER_SOURCE_KINDS:
        return bool(source_live_pages_from_records([record]))
    return False


def source_live_pages_from_records(records: list[MarkdownRecord]) -> list[str]:
    pages: set[str] = set()
    for record in records:
        if record.path.startswith("wiki/live/"):
            pages.add(f"[[{record.path_no_ext}]]")
            continue
        for source in list_field(record.frontmatter, "source_live_pages"):
            pages.add(source)
    return sorted(pages)


def default_output_path(mode: str, title_slug: str) -> str:
    if mode == "slides":
        return f"outputs/slides/{title_slug}.md"
    if mode == "report":
        return f"outputs/reports/{title_slug}.md"
    if mode == "charts":
        return f"outputs/charts/{title_slug}.md"
    if mode == "web":
        return f"outputs/web/{title_slug}/index.html"
    return f"outputs/charts/{title_slug}.canvas"


def resolve_title(records: list[MarkdownRecord], explicit_title: str | None) -> str:
    if explicit_title and explicit_title.strip():
        return explicit_title.strip()
    if records:
        raw_title = records[0].frontmatter.get("title")
        if isinstance(raw_title, str) and raw_title.strip():
            return raw_title.strip()
        return records[0].basename.replace("-", " ").title()
    return "Rendered Artifact"


def slugify_title(title: str) -> str:
    return "-".join(part for part in title.lower().replace("/", " ").replace("_", " ").split() if part)


def write_markdown(path: Path, lines: list[str]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def now_iso() -> str:
    return datetime.now(UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")
