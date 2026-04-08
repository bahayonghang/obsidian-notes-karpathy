#!/usr/bin/env python3

from __future__ import annotations

import argparse
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
DEFAULT_WORKSPACE_ROOT = REPO_ROOT / ".runtime-evals"
SKILL_PATHS = {
    "kb-init": REPO_ROOT / "skills" / "kb-init" / "SKILL.md",
    "kb-compile": REPO_ROOT / "skills" / "kb-compile" / "SKILL.md",
    "kb-review": REPO_ROOT / "skills" / "kb-review" / "SKILL.md",
    "kb-query": REPO_ROOT / "skills" / "kb-query" / "SKILL.md",
    "kb-health": REPO_ROOT / "skills" / "kb-health" / "SKILL.md",
}
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


def build_prompt(*, use_skill: bool, skill: str, prompt: str, files: list[str]) -> str:
    file_block = "\n".join(f"- {file_path}" for file_path in files)
    common = (
        "Work in read-only mode. Do not modify any files.\n"
        "Use only repository-local evidence from the listed files.\n"
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
            f"{common}\n"
            f"Task:\n{prompt}\n\n"
            f"Relevant files:\n{file_block}\n"
        )
    return (
        "Do not use any external skill instructions beyond the repository itself.\n"
        f"{common}\n"
        f"Task:\n{prompt}\n\n"
        f"Relevant files:\n{file_block}\n"
    )


def codex_command(prompt: str, output_path: Path) -> list[str]:
    return [
        *resolve_runner_invocation("codex"),
        "exec",
        "--ephemeral",
        "-s",
        "read-only",
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


def execute_attempt(runner: str, prompt: str, run_dir: Path, timeout_sec: int) -> dict[str, Any]:
    run_dir.mkdir(parents=True, exist_ok=True)
    output_path = run_dir / "last_message.txt"
    started = time.monotonic()
    if runner == "codex":
        command = codex_command(prompt, output_path)
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


def execute_run(runner: str, prompt: str, run_dir: Path, timeout_sec: int) -> dict[str, Any]:
    run_dir.mkdir(parents=True, exist_ok=True)
    attempts: list[dict[str, Any]] = []

    primary_dir = run_dir / f"attempt-1-{runner}"
    primary = execute_attempt(runner, prompt, primary_dir, timeout_sec)
    attempts.append(primary)
    final_attempt = primary

    fallback_runner = fallback_runner_for(runner)
    if primary["failure_kind"] == "infra_failure" and fallback_runner:
        fallback_dir = run_dir / f"attempt-2-{fallback_runner}"
        fallback = execute_attempt(fallback_runner, prompt, fallback_dir, timeout_sec)
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
    }


def main() -> None:
    parser = argparse.ArgumentParser(description="Run or scaffold read-only runtime evals for shipped skills.")
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
    runner = detect_runner(args.runner)
    workspace_root = (args.workspace or DEFAULT_WORKSPACE_ROOT / utc_stamp()).resolve()

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
        metadata = {
            "id": item["id"],
            "skill": item["skill"],
            "files": item["files"],
            "prompt": item["prompt"],
        }
        eval_dir.mkdir(parents=True, exist_ok=True)
        (eval_dir / "metadata.json").write_text(json.dumps(metadata, ensure_ascii=False, indent=2), encoding="utf-8")
        with_skill_result = execute_run(
            runner,
            build_prompt(use_skill=True, skill=item["skill"], prompt=item["prompt"], files=item["files"]),
            with_skill_dir,
            args.timeout_sec,
        )
        baseline_result = execute_run(
            runner,
            build_prompt(use_skill=False, skill=item["skill"], prompt=item["prompt"], files=item["files"]),
            baseline_dir,
            args.timeout_sec,
        )
        summary.append(
            {
                "id": item["id"],
                "skill": item["skill"],
                "with_skill": with_skill_result,
                "baseline": baseline_result,
            }
        )

    payload = build_plan_payload(manifest, workspace_root, runner, "completed")
    payload["runs"] = summary
    print(json.dumps(payload, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
