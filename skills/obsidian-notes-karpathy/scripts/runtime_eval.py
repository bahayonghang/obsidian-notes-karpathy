#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
from datetime import UTC, datetime
from pathlib import Path
from typing import Any

from _runtime_eval_grader import (
    detect_root_leakage,
    materialize_files,
    prepare_target_vault,
    resolve_source_vault_root,
    run_checks,
)
from _runtime_eval_paths import (
    DEFAULT_WORKSPACE_ROOT,
    ENTRY_SKILL_ROOT,
    MANIFEST_PATH,
    REPO_ROOT,
    SKILL_PATHS,
    SUPPORTED_MODES,
    WRITABLE_MANIFEST_PATH,
    fixture_root_for_relpath,
    repo_relative,
    resolve_entry_relative,
)
from _runtime_eval_runner import (
    build_prompt,
    claude_command,
    classify_failure,
    codex_command,
    detect_runner,
    execute_attempt,
    execute_run,
    fallback_runner_for,
    resolve_runner_invocation,
)


__all__ = [
    "build_prompt",
    "claude_command",
    "classify_failure",
    "codex_command",
    "detect_root_leakage",
    "detect_runner",
    "execute_attempt",
    "execute_run",
    "fallback_runner_for",
    "fixture_root_for_relpath",
    "load_manifest",
    "materialize_files",
    "prepare_target_vault",
    "repo_relative",
    "resolve_entry_relative",
    "resolve_runner_invocation",
    "resolve_source_vault_root",
    "run_checks",
    "utc_stamp",
    "validate_manifest",
    "MANIFEST_PATH",
    "WRITABLE_MANIFEST_PATH",
    "DEFAULT_WORKSPACE_ROOT",
    "ENTRY_SKILL_ROOT",
    "REPO_ROOT",
    "SKILL_PATHS",
    "SUPPORTED_MODES",
]


def utc_stamp() -> str:
    return datetime.now(UTC).strftime("%Y%m%dT%H%M%SZ")


def load_manifest(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def validate_manifest(manifest: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    evals = manifest.get("evals")
    if not isinstance(evals, list):
        return ["Manifest must contain an 'evals' list."]

    for item in evals:
        eval_id = item.get("id", "<missing-id>")
        for field in ("id", "skill", "prompt", "files", "vault_root"):
            if field not in item:
                errors.append(f"{eval_id}: missing required field '{field}'.")
        if item.get("skill") not in SKILL_PATHS:
            errors.append(f"{eval_id}: unsupported skill '{item.get('skill')}'.")
        if not isinstance(item.get("files"), list) or not item.get("files"):
            errors.append(f"{eval_id}: files must be a non-empty list.")
            continue
        mode = item.get("mode", "read-only")
        if mode not in SUPPORTED_MODES:
            errors.append(f"{eval_id}: unsupported mode '{mode}'.")

        vault_root = item.get("vault_root")
        vault_fixture_root = fixture_root_for_relpath(vault_root) if isinstance(vault_root, str) else None
        if vault_fixture_root is None:
            errors.append(f"{eval_id}: vault_root must point at a fixture root under evals/fixtures/.")
        fixture_roots = {fixture_root_for_relpath(file_path) for file_path in item["files"]}
        if None in fixture_roots:
            errors.append(f"{eval_id}: all files must live under evals/fixtures/.")
        elif len(fixture_roots) != 1:
            errors.append(f"{eval_id}: files must belong to exactly one fixture root.")
        elif vault_fixture_root is not None and vault_fixture_root not in fixture_roots:
            errors.append(f"{eval_id}: files do not match the declared vault_root.")

        checks = item.get("checks", [])
        if checks and not isinstance(checks, list):
            errors.append(f"{eval_id}: checks must be a list when provided.")
    return errors


def build_plan_payload(manifest: dict[str, Any], workspace_root: Path, runner: str | None, status: str, reason: str | None = None) -> dict[str, Any]:
    return {
        "status": status,
        "runner": runner,
        "workspace": str(workspace_root),
        "eval_count": len(manifest["evals"]),
        "reason": reason,
        "skills": sorted({item["skill"] for item in manifest["evals"]}),
        "modes": sorted({item.get("mode", "read-only") for item in manifest["evals"]}),
    }


def main() -> None:
    parser = argparse.ArgumentParser(description="Run runtime evals for shipped skills with explicit target vault roots.")
    parser.add_argument("--manifest", type=Path, default=MANIFEST_PATH)
    parser.add_argument("--runner", help="Runner to use. Defaults to codex, then claude if available.")
    parser.add_argument("--workspace", type=Path, help="Workspace root for outputs.")
    parser.add_argument("--dry-run", action="store_true", help="Print the execution plan without invoking the runner.")
    parser.add_argument("--limit", type=int, help="Only run the first N evals from the manifest.")
    parser.add_argument("--timeout-sec", type=int, default=180, help="Per-attempt timeout in seconds.")
    args = parser.parse_args()

    manifest = load_manifest(args.manifest)
    if args.limit is not None:
        manifest = {**manifest, "evals": manifest["evals"][: max(args.limit, 0)]}

    validation_errors = validate_manifest(manifest)
    workspace_root = (args.workspace or DEFAULT_WORKSPACE_ROOT / utc_stamp()).resolve()
    if validation_errors:
        print(
            json.dumps(
                {
                    "status": "error",
                    "workspace": str(workspace_root),
                    "errors": validation_errors,
                },
                ensure_ascii=False,
                indent=2,
            )
        )
        raise SystemExit(1)

    runner = detect_runner(args.runner)
    if runner is None:
        print(json.dumps(build_plan_payload(manifest, workspace_root, None, "skipped", "No supported runner found."), ensure_ascii=False, indent=2))
        return

    if args.dry_run:
        print(json.dumps(build_plan_payload(manifest, workspace_root, runner, "planned"), ensure_ascii=False, indent=2))
        return

    workspace_root.mkdir(parents=True, exist_ok=True)
    summary: list[dict[str, Any]] = []
    for item in manifest["evals"]:
        eval_dir = workspace_root / item["skill"] / item["id"]
        with_skill_dir = eval_dir / "with_skill"
        baseline_dir = eval_dir / "baseline"
        mode = item.get("mode", "read-only")
        source_vault_root = resolve_source_vault_root(item)

        metadata = {
            "id": item["id"],
            "skill": item["skill"],
            "files": item["files"],
            "prompt": item["prompt"],
            "mode": mode,
            "source_vault_root": repo_relative(source_vault_root),
            "checks": item.get("checks", []),
        }
        eval_dir.mkdir(parents=True, exist_ok=True)
        (eval_dir / "metadata.json").write_text(json.dumps(metadata, ensure_ascii=False, indent=2), encoding="utf-8")

        with_skill_target_root = prepare_target_vault(source_vault_root, eval_dir, "with_skill", mode)
        baseline_target_root = prepare_target_vault(source_vault_root, eval_dir, "baseline", mode)
        with_skill_files = materialize_files(source_vault_root, with_skill_target_root, item["files"])
        baseline_files = materialize_files(source_vault_root, baseline_target_root, item["files"])
        sandbox_mode = "read-only" if mode == "read-only" else "workspace-write"

        with_skill_result = execute_run(
            runner,
            build_prompt(
                use_skill=True,
                skill=item["skill"],
                prompt=item["prompt"],
                files=[repo_relative(path) for path in with_skill_files],
                vault_root=repo_relative(with_skill_target_root),
                mode=mode,
            ),
            with_skill_dir,
            args.timeout_sec,
            sandbox_mode,
        )
        baseline_result = execute_run(
            runner,
            build_prompt(
                use_skill=False,
                skill=item["skill"],
                prompt=item["prompt"],
                files=[repo_relative(path) for path in baseline_files],
                vault_root=repo_relative(baseline_target_root),
                mode=mode,
            ),
            baseline_dir,
            args.timeout_sec,
            sandbox_mode,
        )

        for run_label, run_dir, result_payload, target_vault_root in (
            ("with_skill", with_skill_dir, with_skill_result, with_skill_target_root),
            ("baseline", baseline_dir, baseline_result, baseline_target_root),
        ):
            last_message = (run_dir / "last_message.txt").read_text(encoding="utf-8")
            leakage = detect_root_leakage(last_message, target_vault_root)
            (run_dir / "root_leakage.json").write_text(json.dumps(leakage, ensure_ascii=False, indent=2), encoding="utf-8")
            result_payload["root_leakage"] = leakage

            checks_payload = run_checks(target_vault_root, source_vault_root, item.get("checks", []))
            (run_dir / "checks.json").write_text(json.dumps(checks_payload, ensure_ascii=False, indent=2), encoding="utf-8")
            result_payload["checks"] = checks_payload
            result_payload["target_vault_root"] = repo_relative(target_vault_root)
            result_payload["mode"] = mode
            (run_dir / "result.json").write_text(json.dumps(result_payload, ensure_ascii=False, indent=2), encoding="utf-8")

        summary.append(
            {
                "id": item["id"],
                "skill": item["skill"],
                "mode": mode,
                "source_vault_root": repo_relative(source_vault_root),
                "with_skill": with_skill_result,
                "baseline": baseline_result,
            }
        )

    payload = build_plan_payload(manifest, workspace_root, runner, "completed")
    payload["runs"] = summary
    print(json.dumps(payload, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
