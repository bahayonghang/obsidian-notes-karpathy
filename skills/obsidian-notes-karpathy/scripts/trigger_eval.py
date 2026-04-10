#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import re
import shutil
import subprocess
import sys
import time
from pathlib import Path
from typing import Any

from runtime_eval import classify_failure, detect_runner, fallback_runner_for, resolve_runner_invocation, utc_stamp


SCRIPT_DIR = Path(__file__).resolve().parent
ENTRY_SKILL_ROOT = SCRIPT_DIR.parent
REPO_ROOT = ENTRY_SKILL_ROOT.parents[1]
TRIGGER_EVALS_PATH = ENTRY_SKILL_ROOT / "evals" / "trigger-evals.json"
DEFAULT_WORKSPACE_ROOT = REPO_ROOT / ".trigger-evals"
SKILL_PATHS = {
    "obsidian-notes-karpathy": ENTRY_SKILL_ROOT / "SKILL.md",
    "kb-init": REPO_ROOT / "skills" / "kb-init" / "SKILL.md",
    "kb-compile": REPO_ROOT / "skills" / "kb-compile" / "SKILL.md",
    "kb-review": REPO_ROOT / "skills" / "kb-review" / "SKILL.md",
    "kb-query": REPO_ROOT / "skills" / "kb-query" / "SKILL.md",
    "kb-health": REPO_ROOT / "skills" / "kb-health" / "SKILL.md",
}
DESCRIPTION_RE = re.compile(r"^description:\s*(.+)$", re.MULTILINE)


def load_eval_set(path: Path) -> list[dict[str, Any]]:
    return json.loads(path.read_text(encoding="utf-8"))


def load_skill_catalog() -> dict[str, str]:
    catalog: dict[str, str] = {}
    for skill_name, skill_path in SKILL_PATHS.items():
        text = skill_path.read_text(encoding="utf-8")
        match = DESCRIPTION_RE.search(text)
        if not match:
            raise ValueError(f"Missing description frontmatter in {skill_path}")
        catalog[skill_name] = match.group(1).strip()
    return catalog


def build_trigger_prompt(query: str, catalog: dict[str, str]) -> str:
    skill_lines = "\n".join(f"- {name}: {description}" for name, description in catalog.items())
    return (
        "You are running an offline trigger-classification benchmark.\n"
        "Do not answer the user request. Do not give policy advice. Do not explain how skills usually work.\n"
        "Classify which single skill should be consulted first for the request below.\n"
        "Available skills:\n"
        f"{skill_lines}\n\n"
        "Choose exactly one skill from the list above, or `none` if none should be consulted first.\n"
        "Prefer operation-specific skills over the router when the operation is already clear.\n"
        "Do not invent companion skills that are not in the list.\n"
        "Return exactly one minified JSON object and nothing else.\n"
        'Required schema: {"selected_skill":"<skill-name-or-none>","reason":"<short reason>"}\n'
        "If you output anything before or after the JSON object, the benchmark run is invalid.\n\n"
        f"User request:\n{query}\n"
    )


def codex_command(output_path: Path) -> list[str]:
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


def parse_selected_skill(text: str) -> str | None:
    stripped = text.strip()
    if not stripped:
        return None
    try:
        payload = json.loads(stripped)
    except json.JSONDecodeError:
        payload = None

    if isinstance(payload, dict):
        selected = payload.get("selected_skill")
        if selected in {*SKILL_PATHS.keys(), "none"}:
            return None if selected == "none" else selected

    pattern = re.compile(r'"selected_skill"\s*:\s*"(obsidian-notes-karpathy|kb-init|kb-compile|kb-review|kb-query|kb-health|none)"')
    match = pattern.search(stripped)
    if match:
        value = match.group(1)
        return None if value == "none" else value
    return None


def summarize_trigger_results(results: list[dict[str, Any]], skill_names: list[str]) -> dict[str, Any]:
    total = len(results)
    matched = sum(1 for item in results if item["matched"])
    per_skill: dict[str, dict[str, Any]] = {}

    for skill_name in [*skill_names, "none"]:
        true_positive = sum(1 for item in results if item["expected_skill"] == skill_name and item["actual_skill"] == skill_name)
        false_positive = sum(1 for item in results if item["expected_skill"] != skill_name and item["actual_skill"] == skill_name)
        false_negative = sum(1 for item in results if item["expected_skill"] == skill_name and item["actual_skill"] != skill_name)
        precision = true_positive / (true_positive + false_positive) if (true_positive + false_positive) else None
        recall = true_positive / (true_positive + false_negative) if (true_positive + false_negative) else None
        per_skill[skill_name] = {
            "expected": sum(1 for item in results if item["expected_skill"] == skill_name),
            "actual": sum(1 for item in results if item["actual_skill"] == skill_name),
            "true_positive": true_positive,
            "false_positive": false_positive,
            "false_negative": false_negative,
            "precision": precision,
            "recall": recall,
        }

    return {
        "total": total,
        "matched": matched,
        "accuracy": matched / total if total else None,
        "per_skill": per_skill,
        "false_positive_count": sum(1 for item in results if item["expected_skill"] != item["actual_skill"] and item["actual_skill"] is not None),
        "false_negative_count": sum(1 for item in results if item["expected_skill"] is not None and item["expected_skill"] != item["actual_skill"]),
    }


def execute_attempt(runner: str, prompt: str, run_dir: Path, timeout_sec: int) -> dict[str, Any]:
    run_dir.mkdir(parents=True, exist_ok=True)
    output_path = run_dir / "last_message.txt"
    started = time.monotonic()
    if runner == "codex":
        command = codex_command(output_path)
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
        stderr = f"{stderr}\nTrigger eval attempt timed out after {timeout_sec} seconds.".strip()

    if runner == "claude" and not output_path.exists():
        output_path.write_text(stdout, encoding="utf-8")
    if not output_path.exists():
        output_path.write_text("", encoding="utf-8")

    duration_ms = round((time.monotonic() - started) * 1000)
    payload = {
        "runner": runner,
        "returncode": returncode,
        "duration_ms": duration_ms,
        "failure_kind": classify_failure(returncode, stdout, stderr),
        "output_captured": bool(output_path.read_text(encoding="utf-8").strip()),
    }
    (run_dir / "stdout.txt").write_text(stdout, encoding="utf-8")
    (run_dir / "stderr.txt").write_text(stderr, encoding="utf-8")
    (run_dir / "result.json").write_text(json.dumps(payload, ensure_ascii=False, indent=2), encoding="utf-8")
    return payload


def execute_with_fallback(runner: str, prompt: str, run_dir: Path, timeout_sec: int) -> dict[str, Any]:
    primary = execute_attempt(runner, prompt, run_dir / f"attempt-1-{runner}", timeout_sec)
    attempts = [primary]
    final = primary
    fallback = fallback_runner_for(runner)
    if primary["failure_kind"] == "infra_failure" and fallback:
        secondary = execute_attempt(fallback, prompt, run_dir / f"attempt-2-{fallback}", timeout_sec)
        attempts.append(secondary)
        final = secondary

    final_run_dir = run_dir / f"attempt-{len(attempts)}-{final['runner']}"
    for file_name in ("last_message.txt", "stdout.txt", "stderr.txt"):
        source = final_run_dir / file_name
        if source.exists():
            shutil.copy2(source, run_dir / file_name)
    payload = {
        "requested_runner": runner,
        "runner": final["runner"],
        "returncode": final["returncode"],
        "duration_ms": final["duration_ms"],
        "failure_kind": final["failure_kind"],
        "fallback_used": len(attempts) > 1,
        "attempts": attempts,
    }
    (run_dir / "result.json").write_text(json.dumps(payload, ensure_ascii=False, indent=2), encoding="utf-8")
    return payload


def main() -> None:
    parser = argparse.ArgumentParser(description="Run report-only trigger precision evals for shipped skills.")
    parser.add_argument("--eval-set", type=Path, default=TRIGGER_EVALS_PATH)
    parser.add_argument("--runner", help="Runner to use. Defaults to codex, then claude if available.")
    parser.add_argument("--workspace", type=Path, help="Workspace root for outputs.")
    parser.add_argument("--dry-run", action="store_true", help="Print the planned run without invoking a runner.")
    parser.add_argument("--limit", type=int, help="Only run the first N evals.")
    parser.add_argument("--timeout-sec", type=int, default=90, help="Per-attempt timeout in seconds.")
    args = parser.parse_args()

    evals = load_eval_set(args.eval_set)
    if args.limit is not None:
        evals = evals[: max(args.limit, 0)]
    runner = detect_runner(args.runner)
    workspace_root = (args.workspace or DEFAULT_WORKSPACE_ROOT / utc_stamp()).resolve()
    catalog = load_skill_catalog()

    if runner is None:
        print(
            json.dumps(
                {
                    "status": "skipped",
                    "workspace": str(workspace_root),
                    "reason": "No supported runner found.",
                    "eval_count": len(evals),
                },
                ensure_ascii=False,
                indent=2,
            )
        )
        return

    if args.dry_run:
        print(
            json.dumps(
                {
                    "status": "planned",
                    "workspace": str(workspace_root),
                    "runner": runner,
                    "eval_count": len(evals),
                    "skills": sorted(catalog.keys()),
                },
                ensure_ascii=False,
                indent=2,
            )
        )
        return

    workspace_root.mkdir(parents=True, exist_ok=True)
    results: list[dict[str, Any]] = []
    for index, item in enumerate(evals, start=1):
        run_dir = workspace_root / f"{index:02d}"
        prompt = build_trigger_prompt(item["query"], catalog)
        result_payload = execute_with_fallback(runner, prompt, run_dir, args.timeout_sec)
        output_text = (run_dir / "last_message.txt").read_text(encoding="utf-8")
        actual_skill = parse_selected_skill(output_text)
        expected_skill = item["expected_skill"]
        run_result = {
            "query": item["query"],
            "expected_skill": expected_skill,
            "actual_skill": actual_skill,
            "matched": expected_skill == actual_skill,
            "runner": result_payload["runner"],
            "reason": item.get("reason"),
        }
        (run_dir / "selection.json").write_text(json.dumps(run_result, ensure_ascii=False, indent=2), encoding="utf-8")
        results.append(run_result)

    summary = summarize_trigger_results(results, sorted(catalog.keys()))
    payload = {
        "status": "completed",
        "workspace": str(workspace_root),
        "runner": runner,
        "eval_count": len(evals),
        "results": results,
        "summary": summary,
    }
    (workspace_root / "summary.json").write_text(json.dumps(payload, ensure_ascii=False, indent=2), encoding="utf-8")
    print(json.dumps(payload, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
