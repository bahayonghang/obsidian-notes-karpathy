from __future__ import annotations

from collections import Counter, defaultdict
from datetime import UTC, datetime
from pathlib import Path
from typing import Any

from _vault_common import (
    ALIAS_WIKILINK_RE,
    TABLE_LINE_RE,
    MarkdownRecord,
    extract_wikilinks,
    list_field,
    load_markdown,
    normalize_identity,
    parse_datetime,
    record_identities,
    registry_for_records,
    resolve_target,
    slugify_identity,
    strip_link_alias,
)
from _vault_layout import collect_markdown_records, detect_layout_family
from _vault_query import live_records
from _vault_review import reviewable_draft_records, scan_review_queue
from _vault_guidance import inspect_local_guidance


HEALTH_FLAG_KINDS = {
    "duplicate_concept",
    "duplicate_entity",
    "stale_qa",
    "alias_wikilink_in_table",
    "unapproved_live_page",
    "approved_conflict",
    "review_backlog",
    "memory_knowledge_mix",
    "writeback_backlog",
    "weak_live_sources",
    "broken_wikilink",
    "orphan_page",
    "stale_briefing",
    "duplicate_alias_set",
    "volatile_page_stale",
    "missing_confidence_metadata",
    "confidence_decay_due",
    "supersession_gap",
    "episodic_backlog",
    "procedural_promotion_gap",
    "graph_gap",
    "audit_trail_gap",
    "private_scope_leak",
    "sensitivity_metadata_gap",
    "editorial_drift",
    "profile_conflict",
    "reuse_gap",
    "underused_sources",
}


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
        if record.kind in {"topic", "concept", "entity", "summary", "qa"}
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


def weak_live_sources_issues(records: list[MarkdownRecord]) -> list[dict[str, Any]]:
    issues: list[dict[str, Any]] = []
    for record in live_records(records):
        if record.kind not in {"summary", "topic", "concept", "entity", "procedure"}:
            continue
        if record.frontmatter.get("trust_level") != "approved":
            continue
        if list_field(record.frontmatter, "sources"):
            continue
        issues.append(
            {
                "kind": "weak_live_sources",
                "path": record.path,
            }
        )
    return issues


def memory_knowledge_mix_issues(records: list[MarkdownRecord]) -> list[dict[str, Any]]:
    issues: list[dict[str, Any]] = []
    knowledge_keys = {
        "sources",
        "source_file",
        "review_record",
        "approved_from",
        "trust_level",
        "compiled_from",
        "capture_sources",
        "draft_id",
        "review_state",
        "decision",
        "approved_at",
    }
    knowledge_headings = (
        "## Thesis",
        "## Definition",
        "## Evidence",
        "## Key Takeaways",
        "## Established",
        "## Source Claims",
    )
    collaboration_headings = (
        "## Preferences",
        "## Collaboration Rules",
        "## Working Agreements",
        "## Current Tasks",
        "## Active Tasks",
    )

    for record in records:
        if record.kind == "memory":
            reasons = sorted(key for key in knowledge_keys if key in record.frontmatter)
            if any(heading in record.body for heading in knowledge_headings):
                reasons.append("knowledge_heading")
            if reasons:
                issues.append(
                    {
                        "kind": "memory_knowledge_mix",
                        "path": record.path,
                        "reasons": reasons,
                    }
                )
            continue

        if record.path.startswith("wiki/live/") and record.kind in {"summary", "topic", "concept", "entity"}:
            reasons = sorted(key for key in ("preferences", "working_style", "collaboration_rules") if key in record.frontmatter)
            if any(heading in record.body for heading in collaboration_headings):
                reasons.append("collaboration_heading")
            if reasons:
                issues.append(
                    {
                        "kind": "memory_knowledge_mix",
                        "path": record.path,
                        "reasons": reasons,
                    }
                )

    return issues


def writeback_backlog_issues(records: list[MarkdownRecord]) -> list[dict[str, Any]]:
    issues: list[dict[str, Any]] = []
    for record in records:
        if not (record.path.startswith("outputs/qa/") or record.path.startswith("outputs/content/")):
            continue

        candidates = list_field(record.frontmatter, "writeback_candidates")
        status = str(record.frontmatter.get("writeback_status") or "").strip().lower()
        if not candidates:
            continue
        if status in {"compiled", "rejected"}:
            continue

        reason = "pending_writeback_candidates" if status == "pending" else "missing_or_open_writeback_status"
        issues.append(
            {
                "kind": "writeback_backlog",
                "path": record.path,
                "candidate_count": len(candidates),
                "writeback_status": status or None,
                "reason": reason,
            }
        )

    return issues


def missing_confidence_metadata_issues(records: list[MarkdownRecord]) -> list[dict[str, Any]]:
    required_keys = ("confidence_score", "confidence_band", "support_count", "contradiction_count")
    latest_signal_keys = {
        "confidence_score",
        "confidence_band",
        "support_count",
        "contradiction_count",
        "last_confirmed_at",
        "next_review_due_at",
        "decay_class",
        "supersedes",
        "superseded_by",
        "superseded_at",
        "supersession_reason",
        "visibility_scope",
    }
    issues: list[dict[str, Any]] = []
    for record in live_records(records):
        if record.kind not in {"summary", "topic", "concept", "entity", "procedure"}:
            continue
        if record.frontmatter.get("trust_level") != "approved":
            continue
        if not any(key in record.frontmatter for key in latest_signal_keys):
            continue
        missing = [key for key in required_keys if key not in record.frontmatter]
        if not missing:
            continue
        issues.append(
            {
                "kind": "missing_confidence_metadata",
                "path": record.path,
                "missing_keys": missing,
                "recommended_action": "fill_core_confidence_metadata",
            }
        )
    return issues


def confidence_decay_due_issues(records: list[MarkdownRecord]) -> list[dict[str, Any]]:
    now = datetime.now(UTC)
    issues: list[dict[str, Any]] = []
    for record in live_records(records):
        if record.kind not in {"summary", "topic", "concept", "entity", "procedure"}:
            continue
        due_at = parse_datetime(record.frontmatter.get("next_review_due_at"))
        if due_at is None or due_at > now:
            continue
        overdue_days = int((now - due_at).total_seconds() // 86400)
        issues.append(
            {
                "kind": "confidence_decay_due",
                "path": record.path,
                "next_review_due_at": record.frontmatter.get("next_review_due_at"),
                "overdue_days": overdue_days,
                "last_confirmed_at": record.frontmatter.get("last_confirmed_at"),
                "recommended_action": "refresh_confidence_review",
            }
        )
    return issues


def supersession_gap_issues(records: list[MarkdownRecord]) -> list[dict[str, Any]]:
    by_path, by_basename = registry_for_records(records)
    issues: list[dict[str, Any]] = []
    for record in live_records(records):
        if record.kind not in {"summary", "topic", "concept", "entity", "procedure"}:
            continue
        supersedes = list_field(record.frontmatter, "supersedes")
        superseded_by = list_field(record.frontmatter, "superseded_by")
        if not supersedes and not superseded_by:
            continue

        reasons: list[str] = []
        if superseded_by and not record.frontmatter.get("superseded_at"):
            reasons.append("missing_superseded_at")
        if superseded_by and not record.frontmatter.get("supersession_reason"):
            reasons.append("missing_supersession_reason")

        for target in supersedes + superseded_by:
            resolved = resolve_target(record, target, by_path, by_basename)
            if not resolved:
                reasons.append(f"unresolved_target:{strip_link_alias(target)}")

        for target in superseded_by:
            resolved = resolve_target(record, target, by_path, by_basename)
            if not resolved:
                continue
            backrefs = []
            for target_ref in list_field(resolved[0].frontmatter, "supersedes"):
                for backref in resolve_target(resolved[0], target_ref, by_path, by_basename):
                    backrefs.append(backref.path_no_ext)
            if record.path_no_ext not in backrefs:
                reasons.append(f"missing_reciprocal_supersedes:{resolved[0].path}")

        if reasons:
            issues.append(
                {
                    "kind": "supersession_gap",
                    "path": record.path,
                    "reasons": sorted(dict.fromkeys(reasons)),
                    "recommended_action": "reconcile_supersession_chain",
                    "followup_route": "review",
                }
            )
    return issues


def episodic_backlog_issues(records: list[MarkdownRecord]) -> list[dict[str, Any]]:
    issues: list[dict[str, Any]] = []
    for record in records:
        if record.kind != "episode":
            continue
        status = str(record.frontmatter.get("consolidation_status") or "").strip().lower()
        if status in {"reviewed", "completed"}:
            continue
        issues.append(
            {
                "kind": "episodic_backlog",
                "path": record.path,
                "consolidation_status": status or None,
                "recommended_action": "consolidate_episode_followup",
                "followup_route": str(record.frontmatter.get("followup_route") or "draft").strip().lower() or "draft",
            }
        )
    return issues


def graph_gap_issues(records: list[MarkdownRecord]) -> list[dict[str, Any]]:
    issues: list[dict[str, Any]] = []
    for record in live_records(records):
        if record.kind not in {"summary", "topic", "concept", "entity", "procedure"}:
            continue
        graph_required = str(record.frontmatter.get("graph_required") or "").strip().lower() in {"true", "yes", "1"}
        relationship_notes = list_field(record.frontmatter, "relationship_notes")
        if not graph_required and not relationship_notes:
            continue
        if list_field(record.frontmatter, "related"):
            continue
        issues.append(
            {
                "kind": "graph_gap",
                "path": record.path,
                "reason": "graph_signals_without_related_edges",
            }
        )
    return issues


def _creator_account_key(record: MarkdownRecord) -> str:
    for key in ("account_id", "brief_for", "creator_profile", "source_profile"):
        value = record.frontmatter.get(key)
        if isinstance(value, str) and value.strip():
            return slugify_identity(value)
    basename = record.basename
    if basename.endswith("_style-guide"):
        basename = basename[: -len("_style-guide")]
    return slugify_identity(basename)


def _creator_rules(record: MarkdownRecord) -> dict[str, Any]:
    return {
        "account_key": _creator_account_key(record),
        "voice_profile": str(record.frontmatter.get("voice_profile") or "").strip(),
        "target_audience": str(record.frontmatter.get("target_audience") or "").strip(),
        "forbidden_terms": sorted({value for value in list_field(record.frontmatter, "forbidden_terms") if value.strip()}),
        "required_constraints": sorted(
            {
                *list_field(record.frontmatter, "required_constraints"),
                *list_field(record.frontmatter, "publishing_constraints"),
            }
        ),
    }


def creator_guidance_issues(vault_root: Path, records: list[MarkdownRecord]) -> list[dict[str, Any]]:
    issues: list[dict[str, Any]] = []
    guidance = inspect_local_guidance(vault_root)
    claude_text = (vault_root / "CLAUDE.md").read_text(encoding="utf-8") if (vault_root / "CLAUDE.md").exists() else ""
    memory_text = (vault_root / "MEMORY.md").read_text(encoding="utf-8") if (vault_root / "MEMORY.md").exists() else ""
    style_guides = [load_markdown(path, vault_root) for path in sorted(vault_root.glob("*_style-guide.md"))]
    briefings = [record for record in records if record.kind == "briefing"]
    briefings_by_account = {_creator_account_key(record): record for record in briefings}

    if style_guides and not guidance["claude"]["present"]:
        issues.append(
            {
                "kind": "editorial_drift",
                "path": "CLAUDE.md",
                "reason": "missing_claude_guidance_for_creator_profiles",
            }
        )
    if style_guides and not (vault_root / "MEMORY.md").exists():
        issues.append(
            {
                "kind": "editorial_drift",
                "path": "MEMORY.md",
                "reason": "missing_memory_context_for_creator_profiles",
            }
        )

    for style_guide in style_guides:
        account_key = _creator_account_key(style_guide)
        style_rules = _creator_rules(style_guide)
        briefing = briefings_by_account.get(account_key)
        if briefing is None:
            issues.append(
                {
                    "kind": "editorial_drift",
                    "path": style_guide.path,
                    "reason": "missing_account_briefing",
                    "account_key": account_key,
                }
            )
            continue

        briefing_rules = _creator_rules(briefing)
        for field in ("voice_profile", "target_audience"):
            if style_rules[field] and not briefing_rules[field]:
                issues.append(
                    {
                        "kind": "editorial_drift",
                        "path": briefing.path,
                        "reason": f"missing_{field}",
                        "account_key": account_key,
                    }
                )
            elif style_rules[field] and briefing_rules[field] and style_rules[field] != briefing_rules[field]:
                issues.append(
                    {
                        "kind": "profile_conflict",
                        "path": briefing.path,
                        "account_key": account_key,
                        "field": field,
                        "style_value": style_rules[field],
                        "briefing_value": briefing_rules[field],
                    }
                )

        missing_terms = [term for term in style_rules["forbidden_terms"] if term not in briefing_rules["forbidden_terms"]]
        if missing_terms:
            issues.append(
                {
                    "kind": "editorial_drift",
                    "path": briefing.path,
                    "reason": "missing_forbidden_terms",
                    "account_key": account_key,
                    "missing_terms": missing_terms,
                }
            )

        missing_constraints = [value for value in style_rules["required_constraints"] if value not in briefing_rules["required_constraints"]]
        if missing_constraints:
            issues.append(
                {
                    "kind": "editorial_drift",
                    "path": briefing.path,
                    "reason": "missing_required_constraints",
                    "account_key": account_key,
                    "missing_constraints": missing_constraints,
                }
            )

        if account_key and account_key not in slugify_identity(claude_text) and account_key not in slugify_identity(memory_text):
            issues.append(
                {
                    "kind": "editorial_drift",
                    "path": style_guide.path,
                    "reason": "account_profile_not_reflected_in_claude_or_memory",
                    "account_key": account_key,
                }
            )

    return issues


def reuse_gap_issues(records: list[MarkdownRecord]) -> list[dict[str, Any]]:
    issues: list[dict[str, Any]] = []
    for record in records:
        if record.kind != "content_output":
            continue
        if not list_field(record.frontmatter, "source_live_pages"):
            continue
        if list_field(record.frontmatter, "reused_prior_coverage") or list_field(record.frontmatter, "derived_from"):
            continue
        issues.append(
            {
                "kind": "reuse_gap",
                "path": record.path,
                "recommended_action": "record_reused_prior_coverage",
            }
        )
    return issues


def underused_source_issues(records: list[MarkdownRecord]) -> list[dict[str, Any]]:
    used_sources: set[str] = set()
    for record in records:
        if record.kind == "summary" and record.path.startswith("wiki/live/"):
            continue
        for key in ("sources", "source_live_pages", "reused_prior_coverage"):
            for target in list_field(record.frontmatter, key):
                stripped = strip_link_alias(target)
                if stripped:
                    used_sources.add(stripped)

    issues: list[dict[str, Any]] = []
    for record in live_records(records):
        if record.kind != "summary":
            continue
        if record.frontmatter.get("trust_level") != "approved":
            continue
        if record.path_no_ext in used_sources:
            continue
        issues.append(
            {
                "kind": "underused_sources",
                "path": record.path,
                "recommended_action": "surface_source_for_creator_reuse",
            }
        )
    return issues


def unapproved_live_issues(records: list[MarkdownRecord]) -> list[dict[str, Any]]:
    issues: list[dict[str, Any]] = []
    for record in live_records(records):
        if record.kind not in {"summary", "topic", "concept", "entity", "procedure"}:
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
        if record.kind not in {"topic", "concept", "entity", "procedure"}:
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


def sensitivity_metadata_gap_issues(records: list[MarkdownRecord]) -> list[dict[str, Any]]:
    issues: list[dict[str, Any]] = []
    for record in records:
        if record.kind not in {"qa", "content_output", "report_output", "slide_output", "chart_output", "episode", "topic", "concept", "entity", "procedure"}:
            continue
        visibility_scope = str(record.frontmatter.get("visibility_scope") or "shared").strip().lower()
        sensitivity_level = str(record.frontmatter.get("sensitivity_level") or "").strip().lower()
        if visibility_scope == "private" and not sensitivity_level:
            issues.append(
                {
                    "kind": "sensitivity_metadata_gap",
                    "path": record.path,
                    "recommended_action": "add_sensitivity_level",
                }
            )
    return issues


def private_scope_leak_issues(records: list[MarkdownRecord]) -> list[dict[str, Any]]:
    issues: list[dict[str, Any]] = []
    for record in records:
        if record.kind not in {"qa", "briefing"}:
            continue
        visibility_scope = str(record.frontmatter.get("visibility_scope") or "shared").strip().lower()
        if visibility_scope != "private":
            continue
        issues.append(
            {
                "kind": "private_scope_leak",
                "path": record.path,
                "recommended_action": "move_private_surface_out_of_default_query_scope",
            }
        )
    return issues


def procedural_promotion_gap_issues(vault_root: Path) -> list[dict[str, Any]]:
    issues: list[dict[str, Any]] = []
    for record in reviewable_draft_records(vault_root):
        if str(record.frontmatter.get("promotion_target") or "").strip().lower() != "procedural":
            continue
        expected_path = vault_root / "wiki" / "drafts" / "procedures" / f"{record.basename}.md"
        if expected_path.exists():
            continue
        issues.append(
            {
                "kind": "procedural_promotion_gap",
                "path": record.path,
                "expected_procedure_path": expected_path.relative_to(vault_root).as_posix(),
                "recommended_action": "draft_procedure_candidate",
                "followup_route": "draft",
            }
        )
    return issues


def audit_trail_gap_issues(vault_root: Path) -> list[dict[str, Any]]:
    audit_root = vault_root / "outputs" / "audit"
    if not audit_root.exists():
        return []
    operations_path = audit_root / "operations.jsonl"
    if operations_path.exists() and operations_path.stat().st_size > 0:
        return []
    return [
        {
            "kind": "audit_trail_gap",
            "path": "outputs/audit/operations.jsonl",
        }
    ]


def duplicate_alias_set_issues(records: list[MarkdownRecord]) -> list[dict[str, Any]]:
    alias_map: dict[tuple[str, ...], list[str]] = defaultdict(list)
    for record in live_records(records):
        if record.kind not in {"topic", "concept", "entity"}:
            continue
        aliases = sorted({slugify_identity(alias) for alias in list_field(record.frontmatter, "aliases") if slugify_identity(alias)})
        if len(aliases) < 2:
            continue
        alias_map[tuple(aliases)].append(record.path)

    issues: list[dict[str, Any]] = []
    for alias_set, paths in sorted(alias_map.items()):
        if len(paths) < 2:
            continue
        issues.append({
            "kind": "duplicate_alias_set",
            "aliases": list(alias_set),
            "paths": sorted(paths),
        })
    return issues


def volatile_page_stale_issues(records: list[MarkdownRecord]) -> list[dict[str, Any]]:
    thresholds = {"high": 90, "medium": 180, "low": 365}
    now = datetime.now(UTC)
    issues: list[dict[str, Any]] = []
    for record in live_records(records):
        if record.kind not in {"summary", "topic", "concept", "entity", "procedure"}:
            continue
        volatility = str(record.frontmatter.get("domain_volatility") or "").strip().lower()
        if volatility not in thresholds:
            continue
        last_reviewed = parse_datetime(record.frontmatter.get("last_reviewed_at") or record.frontmatter.get("updated_at") or record.frontmatter.get("approved_at"))
        if last_reviewed is None:
            continue
        age_days = int((now - last_reviewed).total_seconds() // 86400)
        if age_days > thresholds[volatility]:
            issues.append({
                "kind": "volatile_page_stale",
                "path": record.path,
                "domain_volatility": volatility,
                "age_days": age_days,
                "threshold_days": thresholds[volatility],
            })
    return issues


def audit_vault_mechanics(vault_root: Path) -> dict[str, Any]:
    records = collect_markdown_records(vault_root)
    issues: list[dict[str, Any]] = []
    issues.extend(alias_wikilink_table_issues(records))
    issues.extend(duplicate_identity_issues(records))
    issues.extend(missing_confidence_metadata_issues(records))
    issues.extend(confidence_decay_due_issues(records))
    issues.extend(supersession_gap_issues(records))
    issues.extend(stale_qa_issues(records))
    issues.extend(writeback_backlog_issues(records))
    issues.extend(episodic_backlog_issues(records))
    issues.extend(broken_wikilink_issues(records))
    issues.extend(orphan_page_issues(records))
    issues.extend(duplicate_alias_set_issues(records))
    issues.extend(volatile_page_stale_issues(records))
    issues.extend(memory_knowledge_mix_issues(records))
    issues.extend(graph_gap_issues(records))
    issues.extend(creator_guidance_issues(vault_root, records))
    issues.extend(reuse_gap_issues(records))
    issues.extend(underused_source_issues(records))
    issues.extend(unapproved_live_issues(records))
    issues.extend(weak_live_sources_issues(records))
    issues.extend(approved_conflict_issues(records))
    issues.extend(review_backlog_issues(vault_root))
    issues.extend(private_scope_leak_issues(records))
    issues.extend(sensitivity_metadata_gap_issues(records))
    issues.extend(procedural_promotion_gap_issues(vault_root))
    issues.extend(audit_trail_gap_issues(vault_root))
    issues.extend(briefing_staleness_issues(vault_root))

    counts = Counter(issue["kind"] for issue in issues)
    return {
        "vault_root": str(vault_root),
        "layout_family": detect_layout_family(vault_root),
        "issue_counts": dict(sorted(counts.items())),
        "issues": sorted(issues, key=lambda issue: (issue["kind"], issue.get("path", ""), issue.get("line", 0))),
    }
