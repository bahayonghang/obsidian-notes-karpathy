#!/usr/bin/env python3

from __future__ import annotations

import argparse
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

from _vault_utils import audit_vault_mechanics, collect_markdown_records, json_dump, list_field, live_records, slugify_identity


GAP_KINDS = {
    "broken_wikilink",
    "orphan_page",
    "stale_qa",
    "writeback_backlog",
    "weak_live_sources",
    "duplicate_concept",
    "duplicate_entity",
    "duplicate_alias_set",
    "volatile_page_stale",
    "stale_briefing",
}


def _body_open_questions(record) -> list[str]:
    questions: list[str] = []
    in_open_questions = False
    for line in record.body.splitlines():
        stripped = line.strip()
        if stripped.startswith("## "):
            in_open_questions = stripped == "## Open Questions"
            continue
        if not in_open_questions:
            continue
        if stripped.startswith("- [ ] ") or stripped.startswith("- [x] ") or stripped.startswith("- [X] "):
            question = stripped[6:].strip()
            if question:
                questions.append(question)
        elif stripped.startswith("- "):
            question = stripped[2:].strip()
            if question:
                questions.append(question)
        elif stripped:
            in_open_questions = False
    return questions


def build_governance_indices(vault_root: Path) -> dict:
    records = collect_markdown_records(vault_root)
    live = live_records(records)
    audit = audit_vault_mechanics(vault_root)

    questions: list[str] = []
    seen_questions: set[str] = set()
    alias_rows: list[dict[str, object]] = []
    writeback_backlog: list[dict[str, object]] = []
    followup_routes: list[dict[str, object]] = []

    for record in live:
        for question in _body_open_questions(record):
            if question not in seen_questions:
                seen_questions.add(question)
                questions.append(question)
        frontmatter_question = record.frontmatter.get("question")
        if isinstance(frontmatter_question, str) and frontmatter_question.strip():
            question = frontmatter_question.strip()
            if question not in seen_questions:
                seen_questions.add(question)
                questions.append(question)

        canonical = str(record.frontmatter.get("canonical_name") or record.frontmatter.get("concept_id") or record.frontmatter.get("entity_id") or record.basename).strip()
        aliases = sorted({alias for alias in list_field(record.frontmatter, "aliases") if alias.strip()})
        if aliases or canonical:
            alias_rows.append(
                {
                    "path": record.path,
                    "canonical_name": canonical,
                    "canonical_slug": slugify_identity(canonical),
                    "aliases": aliases,
                }
            )

    for record in records:
        is_archived_output = record.path.startswith("outputs/qa/") or record.path.startswith("outputs/content/")
        if not is_archived_output:
            continue

        candidates = list_field(record.frontmatter, "writeback_candidates")
        if candidates:
            status = str(record.frontmatter.get("writeback_status") or "").strip().lower() or None
            if status in {None, "pending"}:
                writeback_backlog.append(
                    {
                        "path": record.path,
                        "candidate_count": len(candidates),
                        "writeback_status": status,
                        "followup_route": str(record.frontmatter.get("followup_route") or "").strip().lower() or None,
                    }
                )

        followup_route = str(record.frontmatter.get("followup_route") or "").strip().lower()
        if followup_route:
            followup_routes.append(
                {
                    "path": record.path,
                    "followup_route": followup_route,
                }
            )

    gap_issues = [issue for issue in audit["issues"] if issue["kind"] in GAP_KINDS]

    questions_md = "# Open Questions\n\n"
    if questions:
        questions_md += "\n".join(f"- [ ] {question}" for question in questions) + "\n"
    else:
        questions_md += "- [ ] No open questions captured yet.\n"

    gaps_md = "# Gap Report\n\n"
    if gap_issues:
        for issue in gap_issues:
            path = issue.get("path", "(no-path)")
            gaps_md += f"- {issue['kind']}: {path}\n"
    else:
        gaps_md += "- No governance gaps detected.\n"

    if writeback_backlog:
        gaps_md += "\n## Writeback Backlog Signals\n\n"
        for item in writeback_backlog:
            status = item["writeback_status"] or "open"
            route = item["followup_route"] or "unspecified"
            gaps_md += f"- {item['path']} | candidates: {item['candidate_count']} | status: {status} | route: {route}\n"

    if followup_routes:
        gaps_md += "\n## Follow-up Routes Seen In Archived Outputs\n\n"
        for item in followup_routes:
            gaps_md += f"- {item['followup_route']}: {item['path']}\n"

    aliases_md = "# Alias Registry\n\n"
    if alias_rows:
        for row in alias_rows:
            aliases_value = row.get("aliases", [])
            aliases = aliases_value if isinstance(aliases_value, list) else []
            alias_list = ", ".join(str(alias) for alias in aliases) if aliases else "(none)"
            aliases_md += f"- `{row['canonical_slug']}` -> {row['canonical_name']} | aliases: {alias_list} | path: {row['path']}\n"
    else:
        aliases_md += "- No alias-bearing live notes found.\n"

    return {
        "vault_root": str(vault_root),
        "questions": questions,
        "gap_issue_kinds": sorted({issue["kind"] for issue in gap_issues}),
        "alias_rows": alias_rows,
        "writeback_backlog": writeback_backlog,
        "followup_routes": followup_routes,
        "files": {
            "QUESTIONS.md": questions_md,
            "GAPS.md": gaps_md,
            "ALIASES.md": aliases_md,
        },
    }


def write_governance_indices(vault_root: Path, payload: dict) -> None:
    indices_root = vault_root / "wiki" / "live" / "indices"
    indices_root.mkdir(parents=True, exist_ok=True)
    for name, content in payload["files"].items():
        (indices_root / name).write_text(content, encoding="utf-8")


def main() -> None:
    parser = argparse.ArgumentParser(description="Build governance indices for optional QUESTIONS/GAPS/ALIASES surfaces.")
    parser.add_argument("vault_root")
    parser.add_argument("--write", action="store_true", help="Write generated indices into wiki/live/indices/")
    args = parser.parse_args()

    vault_root = Path(args.vault_root).resolve()
    payload = build_governance_indices(vault_root)
    if args.write:
        write_governance_indices(vault_root, payload)
    print(json_dump(payload))


if __name__ == "__main__":
    main()
