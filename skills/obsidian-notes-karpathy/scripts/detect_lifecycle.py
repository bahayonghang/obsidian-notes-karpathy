#!/usr/bin/env python3

from __future__ import annotations

import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

from _vault_utils import (
    audit_vault_mechanics,
    inspect_local_guidance,
    json_dump,
    scan_compile_delta,
)


CORE_SUPPORT_FILES = [
    "wiki/index.md",
    "wiki/log.md",
]


def detect_lifecycle(vault_root: Path) -> dict:
    signals: list[str] = []
    missing = [
        rel_path
        for rel_path in CORE_SUPPORT_FILES
        if not (vault_root / rel_path).exists()
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
    compiled_content_exists = any(
        root.exists() and any(root.rglob("*.md"))
        for root in (
            vault_root / "wiki" / "concepts",
            vault_root / "wiki" / "summaries",
            vault_root / "outputs" / "qa",
        )
    )

    def build_payload(
        *,
        state: str,
        route: str,
        missing_support_files: list[str],
        compile_delta: dict,
        health_flags: list[str],
    ) -> dict:
        return {
            "vault_root": str(vault_root),
            "state": state,
            "route": route,
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
            state="fresh",
            route="kb-init",
            missing_support_files=missing,
            compile_delta={"new_count": 0, "changed_count": 0, "unchanged_count": 0},
            health_flags=[],
        )

    structural_partial = not has_wiki or ((missing or not has_raw) and not compiled_content_exists)

    if blocking_guidance_issues or structural_partial:
        signals.extend(blocking_guidance_issues)
        if missing:
            signals.append("missing_support_layer")
        if not has_raw:
            signals.append("missing_raw")
        if not has_wiki:
            signals.append("missing_wiki")
        missing_support_files = list(missing)
        if "missing_agents_guidance" in blocking_guidance_issues:
            missing_support_files.append("AGENTS.md")
        return build_payload(
            state="partial",
            route="kb-init",
            missing_support_files=sorted(dict.fromkeys(missing_support_files)),
            compile_delta={"new_count": 0, "changed_count": 0, "unchanged_count": 0},
            health_flags=[],
        )

    compile_delta = scan_compile_delta(vault_root)
    new_count = compile_delta["counts"]["new"]
    changed_count = compile_delta["counts"]["changed"]
    unchanged_count = compile_delta["counts"]["unchanged"]

    if new_count or changed_count:
        if new_count:
            signals.append("new_raw_sources")
        if changed_count:
            signals.append("changed_raw_sources")
        return build_payload(
            state="compile-ready",
            route="kb-compile",
            missing_support_files=[],
            compile_delta={
                "new_count": new_count,
                "changed_count": changed_count,
                "unchanged_count": unchanged_count,
            },
            health_flags=[],
        )

    audit = audit_vault_mechanics(vault_root)
    health_flags = [
        issue["kind"]
        for issue in audit["issues"]
        if issue["kind"] in {"duplicate_concept", "duplicate_entity", "stale_qa", "alias_wikilink_in_table"}
    ]

    if health_flags:
        signals.append("compiled_layer_drift")
        return build_payload(
            state="health-first",
            route="kb-health",
            missing_support_files=[],
            compile_delta={
                "new_count": 0,
                "changed_count": 0,
                "unchanged_count": unchanged_count,
            },
            health_flags=sorted(set(health_flags)),
        )

    signals.append("compiled_layer_ready")
    return build_payload(
        state="query-ready",
        route="kb-query",
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
