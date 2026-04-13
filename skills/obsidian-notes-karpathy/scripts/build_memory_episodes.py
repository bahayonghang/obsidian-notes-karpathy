#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
from datetime import UTC, datetime
from pathlib import Path

from _vault_utils import collect_markdown_records, json_dump, list_field


def _append_audit_event(vault_root: Path, action: str, payload: dict[str, object]) -> None:
    audit_path = vault_root / "outputs" / "audit" / "operations.jsonl"
    audit_path.parent.mkdir(parents=True, exist_ok=True)
    entry = {
        "timestamp": datetime.now(UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "action": action,
        "payload": payload,
    }
    with audit_path.open("a", encoding="utf-8") as handle:
        handle.write(json.dumps(entry, ensure_ascii=False) + "\n")


def _episode_path(vault_root: Path, rel_output_path: str) -> Path:
    name = Path(rel_output_path).with_suffix(".md").name
    return vault_root / "outputs" / "episodes" / name


def build_memory_episodes(vault_root: Path) -> dict[str, object]:
    records = collect_markdown_records(vault_root)
    proposals: list[dict[str, object]] = []
    for record in records:
        if record.kind not in {"qa", "content_output"}:
            continue
        target_path = _episode_path(vault_root, record.path)
        proposals.append(
            {
                "source_path": record.path,
                "episode_path": target_path.relative_to(vault_root).as_posix(),
                "source_live_pages": list_field(record.frontmatter, "source_live_pages"),
                "open_questions_touched": list_field(record.frontmatter, "open_questions_touched"),
                "writeback_candidates": list_field(record.frontmatter, "writeback_candidates"),
                "followup_route": str(record.frontmatter.get("followup_route") or "").strip().lower() or "none",
                "visibility_scope": str(record.frontmatter.get("visibility_scope") or "shared").strip(),
            }
        )
    return {
        "vault_root": str(vault_root),
        "proposal_count": len(proposals),
        "proposals": proposals,
    }


def write_memory_episodes(vault_root: Path, payload: dict[str, object]) -> list[str]:
    written: list[str] = []
    for proposal in payload["proposals"]:
        target_path = vault_root / proposal["episode_path"]
        target_path.parent.mkdir(parents=True, exist_ok=True)
        source_path = proposal["source_path"]
        basename = Path(source_path).stem
        frontmatter_lines = [
            "---",
            f'title: "Episode: {basename}"',
            f'episode_id: "{basename}"',
            'memory_tier: episodic',
            f'captured_at: "{datetime.now(UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")}"',
            f'episode_scope: "{Path(source_path).parts[1]}"',
            f'source_artifacts:',
            f'  - "[[{source_path[:-3]}]]"',
        ]
        for key in ("source_live_pages", "open_questions_touched", "writeback_candidates"):
            values = proposal[key]
            frontmatter_lines.append(f"{key}:")
            if values:
                frontmatter_lines.extend(f'  - "{value}"' for value in values)
            else:
                frontmatter_lines.append("  - \"\"")
        frontmatter_lines.extend(
            [
                'consolidation_status: pending',
                f'followup_route: {proposal["followup_route"]}',
                f'visibility_scope: {proposal["visibility_scope"]}',
                "---",
                "",
                f"# Episode: {basename}",
                "",
                "## Source Artifact",
                "",
                f'- [[{source_path[:-3]}]]',
                "",
                "## Durable Signals",
                "",
                "- Promote reusable knowledge through draft -> review -> live.",
                "- Keep this page as episodic memory, not approved topic truth.",
                "",
            ]
        )
        target_path.write_text("\n".join(frontmatter_lines), encoding="utf-8")
        written.append(target_path.relative_to(vault_root).as_posix())

    if written:
        _append_audit_event(vault_root, "build_memory_episodes", {"written_paths": written})
    return written


def main() -> None:
    parser = argparse.ArgumentParser(description="Build episodic archive notes from archived Q&A and content outputs.")
    parser.add_argument("vault_root")
    parser.add_argument("--write", action="store_true", help="Write outputs/episodes/*.md proposals")
    args = parser.parse_args()

    vault_root = Path(args.vault_root).resolve()
    payload = build_memory_episodes(vault_root)
    if args.write:
        payload["written_paths"] = write_memory_episodes(vault_root, payload)
    print(json_dump(payload))


if __name__ == "__main__":
    main()
