from __future__ import annotations

from collections import defaultdict
from pathlib import Path
from typing import Any


GUIDANCE_CONTRACTS = (
    ("agents", "AGENTS.md", True),
    ("claude", "CLAUDE.md", False),
)


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
