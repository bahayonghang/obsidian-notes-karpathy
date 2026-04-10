#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
ENTRY_SKILL_ROOT = SCRIPT_DIR.parent
REPO_ROOT = ENTRY_SKILL_ROOT.parents[1]
REGISTRY_PATH = SCRIPT_DIR / "skill-contract-registry.json"
TRIGGER_EVALS_PATH = ENTRY_SKILL_ROOT / "evals" / "trigger-evals.json"
RUNTIME_EVALS_PATH = ENTRY_SKILL_ROOT / "evals" / "runtime-evals.json"
WRITABLE_RUNTIME_EVALS_PATH = ENTRY_SKILL_ROOT / "evals" / "runtime-evals-writable.json"
SKILL_PATHS = {
    "obsidian-notes-karpathy": ENTRY_SKILL_ROOT / "SKILL.md",
    "kb-init": REPO_ROOT / "skills" / "kb-init" / "SKILL.md",
    "kb-compile": REPO_ROOT / "skills" / "kb-compile" / "SKILL.md",
    "kb-review": REPO_ROOT / "skills" / "kb-review" / "SKILL.md",
    "kb-query": REPO_ROOT / "skills" / "kb-query" / "SKILL.md",
    "kb-health": REPO_ROOT / "skills" / "kb-health" / "SKILL.md",
}
DESCRIPTION_RE = re.compile(r"^description:\s*(.+)$", re.MULTILINE)
NAME_RE = re.compile(r"^name:\s*(.+)$", re.MULTILINE)
USE_TRIGGER_PATTERNS = ("Use this skill when", "Use this skill whenever")
BOUNDARY_PATTERNS = ("Do not use", "Prefer ", "only route", "not ")
OUTPUT_SECTION_PATTERNS = ("## Output", "## Outputs", "## Report output")


def load_json(path: Path) -> dict[str, Any] | list[Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def parse_frontmatter_field(text: str, pattern: re.Pattern[str]) -> str | None:
    match = pattern.search(text)
    return match.group(1).strip() if match else None


def load_registry() -> dict[str, Any]:
    return load_json(REGISTRY_PATH)  # type: ignore[return-value]


def load_eval_skill_sets() -> tuple[set[str], set[str], set[str]]:
    trigger_data = load_json(TRIGGER_EVALS_PATH)
    runtime_data = load_json(RUNTIME_EVALS_PATH)
    writable_data = load_json(WRITABLE_RUNTIME_EVALS_PATH)
    trigger_skills = {
        item["expected_skill"]
        for item in trigger_data  # type: ignore[arg-type]
        if isinstance(item, dict) and item.get("expected_skill")
    }
    runtime_skills = {
        item["skill"]
        for item in runtime_data["evals"]  # type: ignore[index]
        if isinstance(item, dict) and item.get("skill")
    }
    writable_skills = {
        item["skill"]
        for item in writable_data["evals"]  # type: ignore[index]
        if isinstance(item, dict) and item.get("skill")
    }
    return trigger_skills, runtime_skills, writable_skills


def audit_skill(
    skill_name: str,
    skill_path: Path,
    registry: dict[str, Any],
    trigger_skills: set[str],
    runtime_skills: set[str],
    writable_skills: set[str],
) -> dict[str, Any]:
    text = skill_path.read_text(encoding="utf-8")
    description = parse_frontmatter_field(text, DESCRIPTION_RE)
    frontmatter_name = parse_frontmatter_field(text, NAME_RE)
    registry_entry = registry["skills"].get(skill_name)
    role = registry_entry["role"] if registry_entry else "unknown"

    checks = {
        "frontmatter_name_matches": frontmatter_name == skill_name,
        "description_present": bool(description),
        "description_has_trigger_phrase": bool(description) and any(token in description for token in USE_TRIGGER_PATTERNS),
        "description_has_boundary_language": bool(description) and any(token in description for token in BOUNDARY_PATTERNS),
        "description_has_multilingual_trigger": bool(description) and any(ord(char) > 127 for char in description),
        "has_read_before_section": ("## Read before" in text) or ("## Scope" in text),
        "has_output_section": any(token in text for token in OUTPUT_SECTION_PATTERNS),
        "mentions_registry": "skill-contract-registry.json" in text,
        "mentions_baseline_script": bool(registry_entry) and registry_entry["baseline_script"] in text,
        "trigger_eval_covered": skill_name in trigger_skills,
        "runtime_eval_covered": skill_name in runtime_skills,
        "writable_runtime_covered": (role != "operation") or (skill_name in writable_skills),
    }

    blocking_issues: list[str] = []
    warnings: list[str] = []
    for key in (
        "frontmatter_name_matches",
        "description_present",
        "description_has_trigger_phrase",
        "description_has_boundary_language",
        "has_read_before_section",
        "has_output_section",
        "mentions_registry",
        "mentions_baseline_script",
        "trigger_eval_covered",
        "runtime_eval_covered",
        "writable_runtime_covered",
    ):
        if not checks[key]:
            blocking_issues.append(key)

    if not checks["description_has_multilingual_trigger"]:
        warnings.append("description_has_multilingual_trigger")
    if len(text.splitlines()) > 180:
        warnings.append("body_is_long")

    passed_checks = sum(1 for value in checks.values() if value)
    score = round(passed_checks / len(checks), 3)
    return {
        "name": skill_name,
        "path": skill_path.relative_to(REPO_ROOT).as_posix(),
        "role": role,
        "line_count": len(text.splitlines()),
        "description": description,
        "checks": checks,
        "blocking_issues": blocking_issues,
        "warnings": warnings,
        "score": score,
    }


def build_payload() -> dict[str, Any]:
    registry = load_registry()
    trigger_skills, runtime_skills, writable_skills = load_eval_skill_sets()
    audits = [
        audit_skill(skill_name, skill_path, registry, trigger_skills, runtime_skills, writable_skills)
        for skill_name, skill_path in SKILL_PATHS.items()
    ]
    blocking_issue_count = sum(len(item["blocking_issues"]) for item in audits)
    warning_count = sum(len(item["warnings"]) for item in audits)
    payload = {
        "status": "ok" if blocking_issue_count == 0 else "error",
        "summary": {
            "skill_count": len(audits),
            "trigger_eval_covered": sum(1 for item in audits if item["checks"]["trigger_eval_covered"]),
            "runtime_eval_covered": sum(1 for item in audits if item["checks"]["runtime_eval_covered"]),
            "writable_runtime_covered": sum(1 for item in audits if item["checks"]["writable_runtime_covered"]),
            "blocking_issue_count": blocking_issue_count,
            "warning_count": warning_count,
            "average_score": round(sum(item["score"] for item in audits) / len(audits), 3),
        },
        "skills": audits,
    }
    return payload


def main() -> None:
    parser = argparse.ArgumentParser(description="Audit shipped skills for contract hygiene and evaluation coverage.")
    parser.add_argument("--json", action="store_true", help="Print JSON payload. JSON is also the default output.")
    args = parser.parse_args()
    payload = build_payload()
    if args.json or True:
        print(json.dumps(payload, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
