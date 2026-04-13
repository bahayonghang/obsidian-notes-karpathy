from __future__ import annotations

from collections import Counter
from pathlib import Path
from typing import Any

from _vault_common import MarkdownRecord, iter_markdown_records, list_field, parse_number


REVIEWABLE_DRAFT_ROOTS = (
    "wiki/drafts/summaries",
    "wiki/drafts/concepts",
    "wiki/drafts/entities",
    "wiki/drafts/procedures",
)
TERMINAL_REVIEW_STATES = {"promoted", "rejected", "archived"}


def reviewable_draft_records(vault_root: Path) -> list[MarkdownRecord]:
    records = iter_markdown_records(vault_root, REVIEWABLE_DRAFT_ROOTS)
    return [record for record in records if not record.basename.startswith("_")]


def live_record_for_draft(vault_root: Path, draft_record: MarkdownRecord) -> str:
    relative = draft_record.path.removeprefix("wiki/drafts/")
    live_path = vault_root / "wiki" / "live" / relative
    return live_path.relative_to(vault_root).as_posix()


def review_record_for_draft(draft_record: MarkdownRecord) -> str:
    suffix = draft_record.path.removeprefix("wiki/drafts/").replace("/", "--")
    return f"outputs/reviews/{suffix}"


def compute_review_score(frontmatter: dict[str, Any]) -> float | None:
    explicit = parse_number(frontmatter.get("review_score"))
    if explicit is not None:
        return round(explicit, 3)

    components: list[float] = []
    for key in ("accuracy", "provenance", "composability"):
        value = parse_number(frontmatter.get(key))
        if value is not None:
            components.append(value)

    conflict_risk = parse_number(frontmatter.get("conflict_risk"))
    if conflict_risk is not None:
        components.append(max(0.0, 1.0 - conflict_risk))

    if not components:
        return None

    return round(sum(components) / len(components), 3)


def scan_review_queue(vault_root: Path) -> dict[str, Any]:
    items: list[dict[str, Any]] = []
    decision_counts = Counter()
    state_counts = Counter()
    pending_count = 0

    for record in reviewable_draft_records(vault_root):
        frontmatter = record.frontmatter
        state = str(frontmatter.get("review_state") or "pending").strip() or "pending"
        score = compute_review_score(frontmatter)
        blocking_flags = list_field(frontmatter, "blocking_flags")
        live_conflict = "live_conflict" in blocking_flags or frontmatter.get("status") == "conflicting"
        hard_flags = [flag for flag in blocking_flags if flag != "live_conflict"]
        live_path = live_record_for_draft(vault_root, record)
        live_exists = (vault_root / live_path).exists()

        alias_candidates: list[str] = []
        duplicate_candidates: list[str] = []
        if state in TERMINAL_REVIEW_STATES:
            pending = False
            decision = "approve" if state == "promoted" else "reject"
            reason = "terminal_state"
        else:
            pending = True
            pending_count += 1
            alias_candidates = list_field(frontmatter, "alias_candidates")
            duplicate_candidates = list_field(frontmatter, "duplicate_candidates")
            if hard_flags:
                decision = "reject"
                reason = "blocking_flags"
            elif score is None:
                decision = "needs-human"
                reason = "missing_review_score"
            elif live_conflict:
                decision = "needs-human"
                reason = "live_conflict"
            elif duplicate_candidates:
                decision = "needs-human"
                reason = "duplicate_candidates"
            elif alias_candidates and score < 0.90:
                decision = "needs-human"
                reason = "alias_alignment_requires_judgment"
            elif score >= 0.85:
                decision = "approve"
                reason = "threshold_met"
            elif score < 0.60:
                decision = "reject"
                reason = "score_below_floor"
            else:
                decision = "needs-human"
                reason = "score_band_requires_judgment"

        decision_counts[decision] += 1
        state_counts[state] += 1
        items.append(
            {
                "path": record.path,
                "kind": record.kind,
                "review_state": state,
                "review_score": score,
                "blocking_flags": blocking_flags,
                "alias_candidates": alias_candidates,
                "duplicate_candidates": duplicate_candidates,
                "decision": decision,
                "reason": reason,
                "pending": pending,
                "live_path": live_path,
                "live_exists": live_exists,
                "review_record_path": review_record_for_draft(record),
            }
        )

    return {
        "vault_root": str(vault_root),
        "counts": {
            "pending": pending_count,
            "approve": decision_counts["approve"],
            "reject": decision_counts["reject"],
            "needs-human": decision_counts["needs-human"],
        },
        "state_counts": dict(sorted(state_counts.items())),
        "items": items,
    }
