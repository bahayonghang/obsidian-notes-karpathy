from __future__ import annotations

import json
import shutil
import subprocess
import sys
import time
from pathlib import Path
from typing import Any

from _runtime_eval_paths import (
    INFRA_FAILURE_PATTERNS,
    REPO_ROOT,
    SKILL_PATHS,
)


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


def codex_command(output_path: Path, sandbox_mode: str) -> list[str]:
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
        "-",
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
        command = codex_command(output_path, sandbox_mode)
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
            input=prompt if runner == "codex" else None,
            stdin=None if runner == "codex" else subprocess.DEVNULL,
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
