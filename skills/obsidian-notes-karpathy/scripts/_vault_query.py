from __future__ import annotations

from pathlib import Path

from _vault_common import MarkdownRecord
from _vault_layout import detect_layout_family


def query_scope(vault_root: Path) -> dict:
    layout_family = detect_layout_family(vault_root)
    if layout_family == "review-gated":
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
    memory_path = vault_root / "MEMORY.md"
    if memory_path.exists():
        excluded.append(memory_path.relative_to(vault_root).as_posix())
        excluded = sorted(dict.fromkeys(excluded))

    return {
        "vault_root": str(vault_root),
        "layout_family": layout_family,
        "included_paths": included,
        "excluded_paths": excluded,
        "excluded_prefixes": list(excluded_roots),
    }


def live_records(records: list[MarkdownRecord]) -> list[MarkdownRecord]:
    return [record for record in records if record.path.startswith("wiki/live/")]
