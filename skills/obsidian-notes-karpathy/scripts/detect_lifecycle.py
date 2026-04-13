#!/usr/bin/env python3

from __future__ import annotations

import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

from _vault_utils import (
    audit_vault_mechanics,
    briefing_staleness_issues,
    detect_layout_family,
    inspect_local_guidance,
    json_dump,
    resolve_vault_profile,
    scan_ingest_delta,
    scan_review_queue,
    scan_compile_delta,
)


CORE_SUPPORT_FILES = ["wiki/index.md", "wiki/log.md"]
REVIEW_GATED_SUPPORT_PATHS = [
    "wiki/drafts",
    "wiki/live",
    "wiki/briefings",
    "outputs/reviews",
]


def detect_lifecycle(vault_root: Path) -> dict:
    signals: list[str] = []
    layout_family = detect_layout_family(vault_root)
    profile = resolve_vault_profile(vault_root)
    missing = [
        rel_path
        for rel_path in CORE_SUPPORT_FILES
        if not (vault_root / rel_path).exists()
    ]
    missing_support_paths = [
        rel_path
        for rel_path in REVIEW_GATED_SUPPORT_PATHS
        if layout_family == "review-gated" and not (vault_root / rel_path).exists()
    ]
    guidance = inspect_local_guidance(vault_root)
    guidance_status = {
        "agents": guidance["agents"],
        "claude": guidance["claude"],
    }
    guidance_warnings = guidance["warnings"]
    blocking_guidance_issues = guidance["blocking_issues"]

    has_raw = (vault_root / "raw").exists()
    has_wiki = (vault_root / "wiki").exists()
    has_outputs = (vault_root / "outputs").exists()
    def build_payload(
        *,
        state: str,
        route: str,
        route_mode: str | None,
        missing_support_files: list[str],
        compile_delta: dict,
        health_flags: list[str],
    ) -> dict:
        return {
            "vault_root": str(vault_root),
            "layout_family": layout_family,
            "profile": profile,
            "state": state,
            "route": route,
            "route_mode": route_mode,
            "signals": signals,
            "missing_support_files": missing_support_files,
            "compile_delta": compile_delta,
            "health_flags": health_flags,
            "guidance_status": guidance_status,
            "guidance_warnings": guidance_warnings,
        }

    if (
        not has_raw
        and not has_wiki
        and not has_outputs
        and not guidance_status["agents"]["present"]
        and not guidance_status["claude"]["present"]
    ):
        signals.append("missing_core_directories")
        return build_payload(
            state="needs-setup",
            route="kb-init",
            route_mode=None,
            missing_support_files=missing,
            compile_delta={"new_count": 0, "changed_count": 0, "unchanged_count": 0},
            health_flags=[],
        )

    if layout_family == "legacy-layout" and not blocking_guidance_issues:
        signals.append("legacy_layout_detected")
        return build_payload(
            state="needs-migration",
            route="kb-init",
            route_mode=None,
            missing_support_files=sorted(set(missing)),
            compile_delta={"new_count": 0, "changed_count": 0, "unchanged_count": 0},
            health_flags=[],
        )

    structural_partial = bool(
        not has_wiki
        or missing
        or not has_raw
        or missing_support_paths
    )

    if blocking_guidance_issues or structural_partial:
        signals.extend(blocking_guidance_issues)
        if missing:
            signals.append("missing_support_layer")
        if missing_support_paths:
            signals.append("missing_review_gate_support")
        if not has_raw:
            signals.append("missing_raw")
        if not has_wiki:
            signals.append("missing_wiki")
        missing_support_files = list(missing) + list(missing_support_paths)
        if "missing_agents_guidance" in blocking_guidance_issues:
            missing_support_files.append("AGENTS.md")
        return build_payload(
            state="needs-repair",
            route="kb-init",
            route_mode=None,
            missing_support_files=sorted(dict.fromkeys(missing_support_files)),
            compile_delta={"new_count": 0, "changed_count": 0, "unchanged_count": 0},
            health_flags=[],
        )

    compile_delta = scan_compile_delta(vault_root)
    ingest_delta = scan_ingest_delta(vault_root)
    new_count = compile_delta["counts"]["new"]
    changed_count = compile_delta["counts"]["changed"]
    unchanged_count = compile_delta["counts"]["unchanged"]

    if ingest_delta["needs_ingest"]:
        if ingest_delta["counts"]["new"]:
            signals.append("new_sources_not_registered")
        if ingest_delta["counts"]["changed"]:
            signals.append("registered_sources_changed")
        if ingest_delta["counts"]["removed"]:
            signals.append("manifest_sources_removed")
        return build_payload(
            state="needs-ingest",
            route="kb-ingest",
            route_mode=None,
            missing_support_files=[],
            compile_delta={
                "new_count": new_count,
                "changed_count": changed_count,
                "unchanged_count": unchanged_count,
            },
            health_flags=[],
        )

    if new_count or changed_count:
        if new_count:
            signals.append("new_raw_sources")
        if changed_count:
            signals.append("changed_raw_sources")
        return build_payload(
            state="needs-compilation",
            route="kb-compile",
            route_mode=None,
            missing_support_files=[],
            compile_delta={
                "new_count": new_count,
                "changed_count": changed_count,
                "unchanged_count": unchanged_count,
            },
            health_flags=[],
        )

    review_queue = scan_review_queue(vault_root)
    if review_queue["counts"]["pending"]:
        signals.append("drafts_pending_review")
        return build_payload(
            state="needs-review",
            route="kb-review",
            route_mode="gate",
            missing_support_files=[],
            compile_delta={
                "new_count": 0,
                "changed_count": 0,
                "unchanged_count": unchanged_count,
            },
            health_flags=[],
        )

    stale_briefings = [] if profile == "fast-personal" else briefing_staleness_issues(vault_root)
    if stale_briefings:
        signals.append("briefing_refresh_required")
        return build_payload(
            state="needs-briefing-refresh",
            route="kb-review",
            route_mode="gate",
            missing_support_files=[],
            compile_delta={
                "new_count": 0,
                "changed_count": 0,
                "unchanged_count": unchanged_count,
            },
            health_flags=["stale_briefing"],
        )

    audit = audit_vault_mechanics(vault_root)
    health_flags = [
        issue["kind"]
        for issue in audit["issues"]
        if issue["kind"] in {
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
        }
    ]

    profile_filtered_flags = sorted(set(health_flags))
    if profile == "standard":
        suppressed = {"audit_trail_gap", "episodic_backlog"}
        profile_filtered_flags = [flag for flag in profile_filtered_flags if flag not in suppressed]
    elif profile == "fast-personal":
        suppressed = {"stale_briefing", "stale_qa", "writeback_backlog", "confidence_decay_due", "episodic_backlog", "audit_trail_gap", "graph_gap"}
        profile_filtered_flags = [flag for flag in profile_filtered_flags if flag not in suppressed]

    if profile_filtered_flags:
        signals.append("maintenance_needed")
        return build_payload(
            state="needs-maintenance",
            route="kb-review",
            route_mode="maintenance",
            missing_support_files=[],
            compile_delta={
                "new_count": 0,
                "changed_count": 0,
                "unchanged_count": unchanged_count,
            },
            health_flags=profile_filtered_flags,
        )

    signals.append("approved_knowledge_ready")
    return build_payload(
        state="ready-for-query",
        route="kb-query",
        route_mode=None,
        missing_support_files=[],
        compile_delta={
            "new_count": 0,
            "changed_count": 0,
            "unchanged_count": unchanged_count,
        },
        health_flags=[],
    )


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit("Usage: detect_lifecycle.py <vault-root>")

    vault_root = Path(sys.argv[1]).resolve()
    payload = detect_lifecycle(vault_root)
    print(json_dump(payload))


if __name__ == "__main__":
    main()
