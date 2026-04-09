#!/usr/bin/env python3

from __future__ import annotations

import argparse
import hashlib
import json
import shutil
import subprocess
import sys
import time
from datetime import UTC, datetime
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
ENTRY_SKILL_ROOT = SCRIPT_DIR.parent
REPO_ROOT = ENTRY_SKILL_ROOT.parents[1]
MANIFEST_PATH = ENTRY_SKILL_ROOT / "evals" / "runtime-evals.json"
WRITABLE_MANIFEST_PATH = ENTRY_SKILL_ROOT / "evals" / "runtime-evals-writable.json"
DEFAULT_WORKSPACE_ROOT = REPO_ROOT / ".runtime-evals"
SKILL_PATHS = {
    "kb-init": REPO_ROOT / "skills" / "kb-init" / "SKILL.md",
    "kb-compile": REPO_ROOT / "skills" / "kb-compile" / "SKILL.md",
    "kb-review": REPO_ROOT / "skills" / "kb-review" / "SKILL.md",
    "kb-query": REPO_ROOT / "skills" / "kb-query" / "SKILL.md",
    "kb-health": REPO_ROOT / "skills" / "kb-health" / "SKILL.md",
}
SUPPORTED_MODES = {"read-only", "writable-copy"}
INFRA_FAILURE_PATTERNS = (
    "stream disconnected before completion",
    "reconnecting...",
    "timed out",
    "profile.ps1",
    "microsoft.powershell_profile.ps1",
    "cannot dot-source this command",
    "cannot set property",
    "os error 123",
)


def utc_stamp() -> str:
    return datetime.now(UTC).strftime("%Y%m%dT%H%M%SZ")


def load_manifest(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def detect_runner(preferred: str | None) -> str | None:
    if preferred:
        return preferred if shutil.which(preferred) else None
    for candidate in ("codex", "claude"):
        if shutil.which(candidate):
            return candidate
    return None


def resolve_runner_invocation(runner: str) -> list[str]:
    resolved = shutil.which(runner)
    if not resolved:
        raise FileNotFoundError(f"Runner not found: {runner}")

    path = Path(resolved)
    suffix = path.suffix.lower()
    if sys.platform.startswith("win") and suffix == ".ps1":
        return ["powershell", "-NoProfile", "-ExecutionPolicy", "Bypass", "-File", str(path)]
    if sys.platform.startswith("win") and suffix in {".cmd", ".bat"}:
        return [str(path)]
    return [resolved]


def repo_relative(path: Path) -> str:
    try:
        return path.resolve().relative_to(REPO_ROOT).as_posix()
    except ValueError:
        return path.resolve().as_posix()


def resolve_entry_relative(path_str: str) -> Path:
    return (ENTRY_SKILL_ROOT / Path(path_str)).resolve()


def fixture_root_for_relpath(path_str: str) -> str | None:
    rel_path = Path(path_str)
    parts = rel_path.parts
    if len(parts) < 3 or parts[0] != "evals" or parts[1] != "fixtures":
        return None
    return Path(*parts[:3]).as_posix()


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


def build_prompt(*, use_skill: bool, skill: str, prompt: str, files: list[str], vault_root: str, mode: str) -> str:
    file_block = "\n".join(f"- {file_path}" for file_path in files)
    root_instruction = (
        f"Treat `{vault_root}` as the only target vault root for this task.\n"
        "The listed files are evidence from that vault, not alternate roots.\n"
        "Do not reason about the repository root as if it were the vault.\n"
    )
    if mode == "writable-copy":
        common = (
            "You may modify files only under the declared vault root.\n"
            "Do not modify repository-tracked files outside that target vault copy.\n"
            "Use only repository-local evidence from the declared vault root and the listed files.\n"
            "Prefer the listed files over open-ended repo exploration.\n"
            "If search is required, prefer rg over grep for repository-local searches.\n"
            "On Windows, avoid shell globs inside literal paths. Prefer reading the exact listed files, and if search is required use PowerShell-native commands instead of rg against wildcarded absolute paths.\n"
            "Return a concise answer with:\n"
            "1. the conclusion\n"
            "2. the files used\n"
            "3. any assumptions\n"
        )
    else:
        common = (
            "Work in read-only mode. Do not modify any files.\n"
            "Use only repository-local evidence from the declared vault root and the listed files.\n"
            "Prefer the listed files over open-ended repo exploration.\n"
            "If search is required, prefer rg over grep for repository-local searches.\n"
            "On Windows, avoid shell globs inside literal paths. Prefer reading the exact listed files, and if search is required use PowerShell-native commands instead of rg against wildcarded absolute paths.\n"
            "Return a concise answer with:\n"
            "1. the conclusion\n"
            "2. the files used\n"
            "3. any assumptions\n"
        )

    if use_skill:
        return (
            f"Read and follow the skill at `{SKILL_PATHS[skill]}` for this task.\n"
            f"{root_instruction}"
            f"{common}\n"
            f"Task:\n{prompt}\n\n"
            f"Relevant files:\n{file_block}\n"
        )
    return (
        "Do not use any external skill instructions beyond the repository itself.\n"
        f"{root_instruction}"
        f"{common}\n"
        f"Task:\n{prompt}\n\n"
        f"Relevant files:\n{file_block}\n"
    )


def codex_command(prompt: str, output_path: Path, sandbox_mode: str) -> list[str]:
    return [
        *resolve_runner_invocation("codex"),
        "exec",
        "--ephemeral",
        "-s",
        sandbox_mode,
        "-C",
        str(REPO_ROOT),
        "--output-last-message",
        str(output_path),
        prompt,
    ]


def claude_command(prompt: str) -> list[str]:
    return [
        *resolve_runner_invocation("claude"),
        "-p",
        "--permission-mode",
        "default",
        prompt,
    ]


def classify_failure(returncode: int, stdout: str, stderr: str) -> str:
    if returncode == 0:
        return "ok"
    combined = f"{stdout}\n{stderr}".lower()
    if any(pattern in combined for pattern in INFRA_FAILURE_PATTERNS):
        return "infra_failure"
    return "runner_failure"


def fallback_runner_for(runner: str) -> str | None:
    if runner == "codex" and detect_runner("claude"):
        return "claude"
    return None


def execute_attempt(runner: str, prompt: str, run_dir: Path, timeout_sec: int, sandbox_mode: str) -> dict[str, Any]:
    run_dir.mkdir(parents=True, exist_ok=True)
    output_path = run_dir / "last_message.txt"
    started = time.monotonic()
    if runner == "codex":
        command = codex_command(prompt, output_path, sandbox_mode)
    elif runner == "claude":
        command = claude_command(prompt)
    else:
        raise ValueError(f"Unsupported runner: {runner}")

    try:
        result = subprocess.run(
            command,
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
            encoding="utf-8",
            errors="replace",
            timeout=timeout_sec,
        )
        returncode = result.returncode
        stdout = result.stdout or ""
        stderr = result.stderr or ""
    except subprocess.TimeoutExpired as exc:
        returncode = 124
        stdout = exc.stdout or ""
        stderr = exc.stderr or ""
        if not isinstance(stdout, str):
            stdout = stdout.decode("utf-8", errors="replace")
        if not isinstance(stderr, str):
            stderr = stderr.decode("utf-8", errors="replace")
        stderr = f"{stderr}\nRuntime eval attempt timed out after {timeout_sec} seconds.".strip()

    duration_ms = round((time.monotonic() - started) * 1000)
    if runner == "claude" and not output_path.exists():
        output_path.write_text(stdout, encoding="utf-8")
    if not output_path.exists():
        output_path.write_text("", encoding="utf-8")
    (run_dir / "stdout.txt").write_text(stdout, encoding="utf-8")
    (run_dir / "stderr.txt").write_text(stderr, encoding="utf-8")

    payload = {
        "runner": runner,
        "returncode": returncode,
        "duration_ms": duration_ms,
        "output_path": output_path.name,
        "output_captured": bool(output_path.read_text(encoding="utf-8").strip()),
        "failure_kind": classify_failure(returncode, stdout, stderr),
    }
    (run_dir / "result.json").write_text(json.dumps(payload, ensure_ascii=False, indent=2), encoding="utf-8")
    return payload


def copy_attempt_outputs(attempt_dir: Path, run_dir: Path) -> None:
    for file_name in ("last_message.txt", "stdout.txt", "stderr.txt"):
        source = attempt_dir / file_name
        if source.exists():
            shutil.copy2(source, run_dir / file_name)


def execute_run(runner: str, prompt: str, run_dir: Path, timeout_sec: int, sandbox_mode: str) -> dict[str, Any]:
    run_dir.mkdir(parents=True, exist_ok=True)
    attempts: list[dict[str, Any]] = []

    primary_dir = run_dir / f"attempt-1-{runner}"
    primary = execute_attempt(runner, prompt, primary_dir, timeout_sec, sandbox_mode)
    attempts.append(primary)
    final_attempt = primary

    fallback_runner = fallback_runner_for(runner)
    if primary["failure_kind"] == "infra_failure" and fallback_runner:
        fallback_dir = run_dir / f"attempt-2-{fallback_runner}"
        fallback = execute_attempt(fallback_runner, prompt, fallback_dir, timeout_sec, sandbox_mode)
        attempts.append(fallback)
        final_attempt = fallback

    copy_attempt_outputs(run_dir / f"attempt-{len(attempts)}-{final_attempt['runner']}", run_dir)
    final_payload = {
        "requested_runner": runner,
        "runner": final_attempt["runner"],
        "returncode": final_attempt["returncode"],
        "duration_ms": final_attempt["duration_ms"],
        "output_path": "last_message.txt",
        "output_captured": bool((run_dir / "last_message.txt").read_text(encoding="utf-8").strip()),
        "failure_kind": final_attempt["failure_kind"],
        "fallback_used": len(attempts) > 1,
        "attempts": attempts,
    }
    (run_dir / "result.json").write_text(json.dumps(final_payload, ensure_ascii=False, indent=2), encoding="utf-8")
    return final_payload


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


def resolve_source_vault_root(item: dict[str, Any]) -> Path:
    return resolve_entry_relative(item["vault_root"])


def prepare_target_vault(source_vault_root: Path, eval_dir: Path, run_label: str, mode: str) -> Path:
    if mode == "read-only":
        return source_vault_root

    target_vault_root = (eval_dir / "targets" / run_label / source_vault_root.name).resolve()
    if target_vault_root.exists():
        shutil.rmtree(target_vault_root)
    target_vault_root.parent.mkdir(parents=True, exist_ok=True)
    shutil.copytree(source_vault_root, target_vault_root)
    return target_vault_root


def materialize_files(source_vault_root: Path, target_vault_root: Path, file_paths: list[str]) -> list[Path]:
    resolved: list[Path] = []
    for file_path in file_paths:
        source_file = resolve_entry_relative(file_path)
        if target_vault_root == source_vault_root:
            resolved.append(source_file)
            continue

        relative_inside_vault = source_file.relative_to(source_vault_root)
        resolved.append((target_vault_root / relative_inside_vault).resolve())
    return resolved


def detect_root_leakage(text: str, vault_root: Path) -> dict[str, Any]:
    normalized = text.replace("\\", "/").lower()
    repo_root = REPO_ROOT.resolve().as_posix().lower()
    declared_root = vault_root.resolve().as_posix().lower()
    reasons: list[str] = []

    if declared_root != repo_root and repo_root in normalized:
        reasons.append("repo_root_path_mentioned")
    if declared_root != repo_root and "current workspace root" in normalized:
        reasons.append("current_workspace_root_referenced")

    return {
        "detected": bool(reasons),
        "reasons": reasons,
    }


def file_digest(path: Path) -> str:
    digest = hashlib.sha256()
    digest.update(path.read_bytes())
    return digest.hexdigest()


def tree_snapshot(root: Path) -> dict[str, str]:
    if not root.exists():
        return {}
    snapshot: dict[str, str] = {}
    for path in sorted(root.rglob("*")):
        if not path.is_file():
            continue
        snapshot[path.relative_to(root).as_posix()] = file_digest(path)
    return snapshot


def run_checks(target_vault_root: Path, source_vault_root: Path, checks: list[dict[str, Any]]) -> dict[str, Any]:
    results: list[dict[str, Any]] = []
    for check in checks:
        kind = check["kind"]
        name = check["name"]
        if kind == "exists":
            target = target_vault_root / check["path"]
            passed = target.exists()
            evidence = repo_relative(target) if passed else f"missing:{check['path']}"
        elif kind == "dir_non_empty":
            target = target_vault_root / check["path"]
            passed = target.exists() and any(target.iterdir())
            evidence = repo_relative(target) if passed else f"empty:{check['path']}"
        elif kind == "same_tree":
            source = source_vault_root / check["path"]
            target = target_vault_root / check["path"]
            passed = tree_snapshot(source) == tree_snapshot(target)
            evidence = check["path"] if passed else f"tree_mismatch:{check['path']}"
        elif kind == "glob_count_gte":
            matches = sorted((target_vault_root).glob(check["pattern"]))
            minimum = int(check["min_count"])
            passed = len(matches) >= minimum
            evidence = f"matches={len(matches)} pattern={check['pattern']}"
        elif kind == "file_contains":
            target = target_vault_root / check["path"]
            needle = check["needle"]
            passed = target.exists() and needle in target.read_text(encoding="utf-8")
            evidence = repo_relative(target) if passed else f"missing_needle:{check['path']}"
        else:
            passed = False
            evidence = f"unsupported_check_kind:{kind}"
        results.append(
            {
                "name": name,
                "kind": kind,
                "passed": passed,
                "evidence": evidence,
            }
        )

    return {
        "total": len(results),
        "passed": sum(1 for result in results if result["passed"]),
        "results": results,
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
