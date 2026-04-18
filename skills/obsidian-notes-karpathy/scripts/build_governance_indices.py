#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
from datetime import UTC, datetime
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

from _vault_utils import audit_vault_mechanics, collect_markdown_records, json_dump, list_field, live_records, slugify_identity


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
    "weakly_connected_live_page",
    "hub_candidate_backlog",
    "missing_confidence_metadata",
    "confidence_decay_due",
    "supersession_gap",
    "episodic_backlog",
    "procedural_promotion_gap",
    "graph_gap",
    "audit_trail_gap",
    "editorial_drift",
    "profile_conflict",
    "reuse_gap",
    "underused_sources",
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


def _append_question(questions: list[str], seen_questions: set[str], value: object) -> None:
    if not isinstance(value, str):
        return

    question = value.strip()
    if not question or question in seen_questions:
        return

    seen_questions.add(question)
    questions.append(question)


def build_governance_indices(vault_root: Path) -> dict:
    records = collect_markdown_records(vault_root)
    live = live_records(records)
    audit = audit_vault_mechanics(vault_root)

    questions: list[str] = []
    seen_questions: set[str] = set()
    alias_rows: list[dict[str, object]] = []
    entity_rows: list[dict[str, object]] = []
    relationship_rows: list[dict[str, object]] = []
    writeback_backlog: list[dict[str, object]] = []
    confidence_maintenance: list[dict[str, object]] = []
    closure_signals: list[dict[str, object]] = []
    followup_routes: list[dict[str, object]] = []
    route_counts: dict[str, int] = {}
    hub_candidates: list[dict[str, object]] = []

    for record in live:
        for question in _body_open_questions(record):
            _append_question(questions, seen_questions, question)
        _append_question(questions, seen_questions, record.frontmatter.get("question"))

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
        if record.kind == "entity":
            entity_rows.append(
                {
                    "path": record.path,
                    "entity_id": str(record.frontmatter.get("entity_id") or record.basename).strip(),
                    "entity_type": str(record.frontmatter.get("entity_type") or "other").strip(),
                    "visibility_scope": str(record.frontmatter.get("visibility_scope") or "shared").strip(),
                    "aliases": aliases,
                }
            )
        if record.kind in {"summary", "concept", "entity", "procedure"}:
            related = list_field(record.frontmatter, "related")
            for target in related:
                relationship_rows.append(
                    {
                        "source": record.path,
                        "relation": "related",
                        "target": target,
                    }
                )
            for target in list_field(record.frontmatter, "supersedes"):
                relationship_rows.append(
                    {
                        "source": record.path,
                        "relation": "supersedes",
                        "target": target,
                    }
                )
            for target in list_field(record.frontmatter, "superseded_by"):
                relationship_rows.append(
                    {
                        "source": record.path,
                        "relation": "superseded_by",
                        "target": target,
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
            route_counts[followup_route] = route_counts.get(followup_route, 0) + 1

    for record in live:
        related = list_field(record.frontmatter, "related")
        topic_hub = str(record.frontmatter.get("topic_hub") or "").strip()
        question_links = list_field(record.frontmatter, "question_links")
        open_questions = list_field(record.frontmatter, "open_questions")
        if len(related) <= 1 and not topic_hub and not question_links and not open_questions:
            hub_candidates.append(
                {
                    "path": record.path,
                    "reason": "weakly connected live page",
                }
            )

    gap_issues = [issue for issue in audit["issues"] if issue["kind"] in GAP_KINDS]
    confidence_maintenance = [
        {
            "path": issue.get("path"),
            "issue_kind": issue["kind"],
            "recommended_action": issue.get("recommended_action"),
            "missing_keys": issue.get("missing_keys", []),
            "next_review_due_at": issue.get("next_review_due_at"),
            "overdue_days": issue.get("overdue_days"),
            "last_confirmed_at": issue.get("last_confirmed_at"),
        }
        for issue in gap_issues
        if issue["kind"] in {"missing_confidence_metadata", "confidence_decay_due"}
    ]
    closure_signals = [
        {
            "path": issue.get("path"),
            "issue_kind": issue["kind"],
            "recommended_action": issue.get("recommended_action"),
            "followup_route": issue.get("followup_route"),
            "reasons": issue.get("reasons", []),
            "consolidation_status": issue.get("consolidation_status"),
            "expected_procedure_path": issue.get("expected_procedure_path"),
        }
        for issue in gap_issues
        if issue["kind"] in {"supersession_gap", "episodic_backlog", "procedural_promotion_gap"}
    ]

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

    if confidence_maintenance:
        gaps_md += "\n## Confidence Maintenance Signals\n\n"
        for item in confidence_maintenance:
            action = item["recommended_action"] or "review_confidence_metadata"
            if item["issue_kind"] == "missing_confidence_metadata":
                missing_value = item["missing_keys"]
                missing_keys = missing_value if isinstance(missing_value, list) else []
                missing = ", ".join(str(key) for key in missing_keys) or "(unspecified)"
                gaps_md += f"- {item['path']} | issue: missing_confidence_metadata | missing: {missing} | action: {action}\n"
            else:
                overdue = item["overdue_days"] if item["overdue_days"] is not None else "unknown"
                due_at = item["next_review_due_at"] or "unknown"
                gaps_md += f"- {item['path']} | issue: confidence_decay_due | due: {due_at} | overdue_days: {overdue} | action: {action}\n"

    if closure_signals:
        gaps_md += "\n## Closure Signals\n\n"
        for item in closure_signals:
            action = item["recommended_action"] or "review_followup"
            route = item["followup_route"] or "unspecified"
            if item["issue_kind"] == "supersession_gap":
                reasons_value = item["reasons"]
                reasons = reasons_value if isinstance(reasons_value, list) else []
                reason_text = ", ".join(str(reason) for reason in reasons) or "(unspecified)"
                gaps_md += f"- {item['path']} | issue: supersession_gap | route: {route} | reasons: {reason_text} | action: {action}\n"
            elif item["issue_kind"] == "episodic_backlog":
                status = item["consolidation_status"] or "open"
                gaps_md += f"- {item['path']} | issue: episodic_backlog | status: {status} | route: {route} | action: {action}\n"
            else:
                expected = item["expected_procedure_path"] or "(unspecified)"
                gaps_md += f"- {item['path']} | issue: procedural_promotion_gap | expected: {expected} | route: {route} | action: {action}\n"

    creator_issues = [issue for issue in gap_issues if issue["kind"] in {"editorial_drift", "profile_conflict"}]
    if creator_issues:
        gaps_md += "\n## Creator Consistency Signals\n\n"
        for issue in creator_issues:
            detail = issue.get("reason") or issue.get("field") or "unspecified"
            account_key = issue.get("account_key") or "n/a"
            gaps_md += f"- {issue['path']} | issue: {issue['kind']} | account: {account_key} | detail: {detail}\n"

    reuse_issues = [issue for issue in gap_issues if issue["kind"] in {"reuse_gap", "underused_sources"}]
    if reuse_issues:
        gaps_md += "\n## Reuse Signals\n\n"
        for issue in reuse_issues:
            action = issue.get("recommended_action") or "review_reuse_signal"
            gaps_md += f"- {issue['path']} | issue: {issue['kind']} | action: {action}\n"

    if followup_routes:
        gaps_md += "\n## Follow-up Routes Seen In Archived Outputs\n\n"
        for item in followup_routes:
            gaps_md += f"- {item['followup_route']}: {item['path']}\n"

    if route_counts:
        gaps_md += "\n## Follow-up Route Clusters\n\n"
        for route, count in sorted(route_counts.items()):
            gaps_md += f"- {route}: {count} archived outputs\n"

    if hub_candidates:
        gaps_md += "\n## Hub And Relationship Hints\n\n"
        for item in hub_candidates:
            gaps_md += f"- {item['reason']}: {item['path']}\n"

    aliases_md = "# Alias Registry\n\n"
    if alias_rows:
        for row in alias_rows:
            aliases_value = row.get("aliases", [])
            aliases = aliases_value if isinstance(aliases_value, list) else []
            alias_list = ", ".join(str(alias) for alias in aliases) if aliases else "(none)"
            aliases_md += f"- `{row['canonical_slug']}` -> {row['canonical_name']} | aliases: {alias_list} | path: {row['path']}\n"
    else:
        aliases_md += "- No alias-bearing live notes found.\n"

    entities_md = "# Entity Registry\n\n"
    if entity_rows:
        for row in entity_rows:
            aliases = row.get("aliases", [])
            alias_list = ", ".join(str(alias) for alias in aliases) if isinstance(aliases, list) and aliases else "(none)"
            entities_md += (
                f"- `{row['entity_id']}` | type: {row['entity_type']} | scope: {row['visibility_scope']} | "
                f"aliases: {alias_list} | path: {row['path']}\n"
            )
    else:
        entities_md += "- No entity pages found.\n"

    relationships_md = "# Relationship Registry\n\n"
    if relationship_rows:
        for row in relationship_rows:
            relationships_md += f"- {row['source']} --{row['relation']}--> {row['target']}\n"
    else:
        relationships_md += "- No explicit relationship edges found.\n"

    return {
        "vault_root": str(vault_root),
        "questions": questions,
        "gap_issue_kinds": sorted({issue["kind"] for issue in gap_issues}),
        "alias_rows": alias_rows,
        "entity_rows": entity_rows,
        "relationship_rows": relationship_rows,
        "writeback_backlog": writeback_backlog,
        "confidence_maintenance": confidence_maintenance,
        "closure_signals": closure_signals,
        "followup_routes": followup_routes,
        "route_counts": route_counts,
        "hub_candidates": hub_candidates,
        "files": {
            "QUESTIONS.md": questions_md,
            "GAPS.md": gaps_md,
            "ALIASES.md": aliases_md,
            "ENTITIES.md": entities_md,
            "RELATIONSHIPS.md": relationships_md,
        },
    }


def write_governance_indices(vault_root: Path, payload: dict) -> None:
    indices_root = vault_root / "wiki" / "live" / "indices"
    indices_root.mkdir(parents=True, exist_ok=True)
    written_files: list[str] = []
    for name, content in payload["files"].items():
        path = indices_root / name
        path.write_text(content, encoding="utf-8")
        written_files.append(path.relative_to(vault_root).as_posix())
    _append_audit_event(
        vault_root,
        "build_governance_indices",
        {"written_paths": written_files, "question_count": len(payload["questions"]), "gap_issue_kinds": payload["gap_issue_kinds"]},
    )


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
