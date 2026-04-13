from __future__ import annotations

from pathlib import Path

from _vault_common import MarkdownRecord, load_markdown
from _vault_layout import detect_layout_family, resolve_vault_profile


def query_scope(vault_root: Path) -> dict:
    layout_family = detect_layout_family(vault_root)
    profile = resolve_vault_profile(vault_root)
    if layout_family == "review-gated":
        included_roots = ("wiki/live", "outputs/qa")
        if profile != "fast-personal":
            included_roots = (*included_roots[:1], "wiki/briefings", *included_roots[1:])
        candidate_roots = ("outputs/episodes",)
        excluded_roots = ("raw", "wiki/drafts", "outputs/reviews")
    else:
        included_roots = ("wiki", "outputs/qa")
        candidate_roots = ()
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
    candidate_paths = [
        path.relative_to(vault_root).as_posix()
        for rel_root in candidate_roots
        for path in sorted((vault_root / rel_root).rglob("*.md"))
        if (vault_root / rel_root).exists()
    ]
    graph_snapshot = vault_root / "outputs" / "health" / "graph-snapshot.json"
    if graph_snapshot.exists():
        candidate_paths.append(graph_snapshot.relative_to(vault_root).as_posix())
    memory_path = vault_root / "MEMORY.md"
    if memory_path.exists():
        excluded.append(memory_path.relative_to(vault_root).as_posix())
        excluded = sorted(dict.fromkeys(excluded))

    scope_leaks: list[dict[str, str]] = []
    sensitivity_candidates: list[dict[str, str]] = []
    for rel_path in [*included, *candidate_paths]:
        abs_path = vault_root / rel_path
        if not abs_path.exists() or abs_path.suffix != ".md":
            continue
        record = load_markdown(abs_path, vault_root)
        visibility_scope = str(record.frontmatter.get("visibility_scope") or "shared").strip().lower()
        if visibility_scope == "private":
            scope_leaks.append(
                {
                    "path": rel_path,
                    "visibility_scope": "private",
                    "reason": "private_surface_in_default_query_scope",
                }
            )
        sensitivity_level = str(record.frontmatter.get("sensitivity_level") or "").strip().lower()
        if sensitivity_level in {"internal", "restricted", "secret"}:
            sensitivity_candidates.append(
                {
                    "path": rel_path,
                    "sensitivity_level": sensitivity_level,
                    "candidate_kind": "included" if rel_path in included else "candidate",
                }
            )

    return {
        "vault_root": str(vault_root),
        "layout_family": layout_family,
        "profile": profile,
        "included_paths": included,
        "candidate_paths": sorted(dict.fromkeys(candidate_paths)),
        "candidate_policy": "candidate-only",
        "excluded_paths": excluded,
        "excluded_prefixes": list(excluded_roots),
        "scope_leaks": scope_leaks,
        "sensitivity_candidates": sensitivity_candidates,
    }


def live_records(records: list[MarkdownRecord]) -> list[MarkdownRecord]:
    return [record for record in records if record.path.startswith("wiki/live/")]
