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
from _vault_review import scan_review_queue


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
        if record.kind not in {"summary", "concept", "entity"}:
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

        if record.path.startswith("wiki/live/") and record.kind in {"summary", "concept", "entity"}:
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


def duplicate_alias_set_issues(records: list[MarkdownRecord]) -> list[dict[str, Any]]:
    alias_map: dict[tuple[str, ...], list[str]] = defaultdict(list)
    for record in live_records(records):
        if record.kind not in {"concept", "entity"}:
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
        if record.kind not in {"summary", "concept", "entity"}:
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
    issues.extend(stale_qa_issues(records))
    issues.extend(writeback_backlog_issues(records))
    issues.extend(broken_wikilink_issues(records))
    issues.extend(orphan_page_issues(records))
    issues.extend(duplicate_alias_set_issues(records))
    issues.extend(volatile_page_stale_issues(records))
    issues.extend(memory_knowledge_mix_issues(records))
    issues.extend(unapproved_live_issues(records))
    issues.extend(weak_live_sources_issues(records))
    issues.extend(approved_conflict_issues(records))
    issues.extend(review_backlog_issues(vault_root))
    issues.extend(briefing_staleness_issues(vault_root))

    counts = Counter(issue["kind"] for issue in issues)
    return {
        "vault_root": str(vault_root),
        "layout_family": detect_layout_family(vault_root),
        "issue_counts": dict(sorted(counts.items())),
        "issues": sorted(issues, key=lambda issue: (issue["kind"], issue.get("path", ""), issue.get("line", 0))),
    }
