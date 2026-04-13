#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import py_compile
import re
import sys
import tempfile
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
ENTRY_SKILL_ROOT = SCRIPT_DIR.parent
REPO_ROOT = ENTRY_SKILL_ROOT.parents[1]
TESTS_DIR = REPO_ROOT / "tests"
RUNTIME_EVALS_PATH = ENTRY_SKILL_ROOT / "evals" / "runtime-evals.json"
WRITABLE_RUNTIME_EVALS_PATH = ENTRY_SKILL_ROOT / "evals" / "runtime-evals-writable.json"
KB_INIT_ASSETS_ROOT = REPO_ROOT / "skills" / "kb-init" / "assets"
REGISTRY_PATH = SCRIPT_DIR / "skill-contract-registry.json"
SKILL_PATHS = {
    "obsidian-notes-karpathy": ENTRY_SKILL_ROOT / "SKILL.md",
    "kb-init": REPO_ROOT / "skills" / "kb-init" / "SKILL.md",
    "kb-ingest": REPO_ROOT / "skills" / "kb-ingest" / "SKILL.md",
    "kb-compile": REPO_ROOT / "skills" / "kb-compile" / "SKILL.md",
    "kb-review": REPO_ROOT / "skills" / "kb-review" / "SKILL.md",
    "kb-query": REPO_ROOT / "skills" / "kb-query" / "SKILL.md",
    "kb-render": REPO_ROOT / "skills" / "kb-render" / "SKILL.md",
}
KB_INIT_REQUIRED_ASSETS = (
    "AGENTS.md",
    "CLAUDE.md",
    "MEMORY.md",
    "raw/_manifest.yaml",
    "wiki/index.md",
    "wiki/log.md",
    "wiki/briefings/researcher.md",
    "wiki/live/indices/INDEX.md",
    "wiki/live/indices/CONCEPTS.md",
    "wiki/live/indices/SOURCES.md",
    "wiki/live/indices/TOPICS.md",
    "wiki/live/indices/RECENT.md",
    "wiki/live/indices/EDITORIAL-PRIORITIES.md",
)
HELPER_SCRIPTS = (
    "bootstrap_review_gated_vault.py",
    "migrate_legacy_vault.py",
    "vault_status.py",
    "render_reference_block.py",
)
REFERENCE_BULLETS_RE = re.compile(r"- `([^`]+)`")
READ_BEFORE_RE = re.compile(r"## Read before .*?(?=\n## |\Z)", re.DOTALL)
SCOPE_READ_RE = re.compile(r"## Scope\n\nBefore checking the vault, read these files first:.*?(?=\n## |\Z)", re.DOTALL)
# Fallback: any section whose body contains a bullet referencing the registry or shared references
GENERIC_REF_SECTION_RE = re.compile(
    r"## [^\n]*\n\n(?:[^\n]*\n)*?(?=- `\.\./|.*skill-contract-registry).*?(?=\n## |\Z)",
    re.DOTALL,
)

from _skill_reference_blocks import build_shared_reference_bullets


def load_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def extract_reference_bullets(skill_text: str) -> list[str]:
    section_match = (
        READ_BEFORE_RE.search(skill_text)
        or SCOPE_READ_RE.search(skill_text)
        or GENERIC_REF_SECTION_RE.search(skill_text)
    )
    if not section_match:
        return []
    return REFERENCE_BULLETS_RE.findall(section_match.group(0))


def compile_python_files() -> tuple[int, list[str]]:
    checked = 0
    errors: list[str] = []
    original_prefix = sys.pycache_prefix
    with tempfile.TemporaryDirectory() as tmp:
        sys.pycache_prefix = tmp
        try:
            for root in (SCRIPT_DIR, TESTS_DIR):
                for path in sorted(root.rglob("*.py")):
                    if "__pycache__" in path.parts:
                        continue
                    checked += 1
                    try:
                        py_compile.compile(str(path), doraise=True)
                    except py_compile.PyCompileError as exc:
                        errors.append(f"{path.relative_to(REPO_ROOT).as_posix()}: {exc.msg}")
        finally:
            sys.pycache_prefix = original_prefix
    return checked, errors


def check_registry_and_skills() -> list[str]:
    errors: list[str] = []
    registry = load_json(REGISTRY_PATH)
    if registry.get("contract_family") != "review-gated":
        errors.append("Registry contract_family must be review-gated.")

    registry_skills = registry.get("skills", {})
    if sorted(registry_skills.keys()) != sorted(SKILL_PATHS.keys()):
        errors.append("Registry skill keys must match the shipped core skills.")

    for skill_name, skill_path in SKILL_PATHS.items():
        if not skill_path.exists():
            errors.append(f"Missing SKILL.md for {skill_name}: {skill_path.relative_to(REPO_ROOT).as_posix()}")
            continue

        skill_text = skill_path.read_text(encoding="utf-8")
        if "name:" not in skill_text or "description:" not in skill_text:
            errors.append(f"{skill_path.relative_to(REPO_ROOT).as_posix()} is missing required frontmatter fields.")

        expected = registry_skills.get(skill_name)
        if expected is None:
            continue

        referenced_paths = extract_reference_bullets(skill_text)
        for bullet in build_shared_reference_bullets(skill_name, registry):
            target = bullet.removeprefix("- `").removesuffix("`")
            if target not in referenced_paths:
                errors.append(f"{skill_name} is missing reference bullet for {target}.")
        if expected["baseline_script"] not in skill_text:
            errors.append(f"{skill_name} does not mention baseline script {expected['baseline_script']}.")

    for asset_rel_path in KB_INIT_REQUIRED_ASSETS:
        if not (KB_INIT_ASSETS_ROOT / asset_rel_path).exists():
            errors.append(f"kb-init asset is missing: {asset_rel_path}.")
    for script_name in HELPER_SCRIPTS:
        if not (SCRIPT_DIR / script_name).exists():
            errors.append(f"Helper script is missing: {script_name}.")

    return errors


def check_docs_and_evals() -> list[str]:
    errors: list[str] = []
    readme = (REPO_ROOT / "README.md").read_text(encoding="utf-8")
    readme_cn = (REPO_ROOT / "README_CN.md").read_text(encoding="utf-8")
    claude_md = (REPO_ROOT / "CLAUDE.md").read_text(encoding="utf-8")
    docs_overview = (REPO_ROOT / "docs" / "skills" / "overview.md").read_text(encoding="utf-8")
    docs_overview_zh = (REPO_ROOT / "docs" / "zh" / "skills" / "overview.md").read_text(encoding="utf-8")
    trigger_evals = load_json(ENTRY_SKILL_ROOT / "evals" / "trigger-evals.json")
    runtime_evals = load_json(RUNTIME_EVALS_PATH)
    writable_runtime_evals = load_json(WRITABLE_RUNTIME_EVALS_PATH)

    for text_name, text in {
        "README.md": readme,
        "README_CN.md": readme_cn,
        "CLAUDE.md": claude_md,
    }.items():
        for needle in ("wiki/drafts", "wiki/live", "wiki/briefings", "outputs/reviews"):
            if needle not in text:
                errors.append(f"{text_name} is missing contract term {needle}.")

    if "one package entry skill and six operational skills" not in docs_overview:
        errors.append("docs/skills/overview.md must describe the 1+6 core bundle shape.")
    if "1 个入口技能和 6 个操作技能" not in docs_overview_zh:
        errors.append("docs/zh/skills/overview.md must describe the 1+6 core bundle shape.")
    if "Companion skills" not in docs_overview or "搭配技能" not in docs_overview_zh:
        errors.append("Skills overview docs must explicitly distinguish companion skills from the core bundle.")
    if "Companion skill matrix" not in readme or "搭配技能矩阵" not in readme_cn:
        errors.append("Top-level READMEs must include the companion skill matrix.")
    for helper_name in ("bootstrap_review_gated_vault.py", "migrate_legacy_vault.py", "vault_status.py"):
        if helper_name not in readme:
            errors.append(f"README.md must mention helper script {helper_name}.")
        if helper_name not in readme_cn:
            errors.append(f"README_CN.md must mention helper script {helper_name}.")

    if len(trigger_evals) < 20:
        errors.append("trigger-evals.json must contain at least 20 cases.")
    expected_trigger_keys = {*SKILL_PATHS.keys(), "none"}
    actual_trigger_keys = {
        item["expected_skill"] if item["expected_skill"] is not None else "none"
        for item in trigger_evals
    }
    missing_trigger_keys = sorted(expected_trigger_keys - actual_trigger_keys)
    if missing_trigger_keys:
        errors.append(f"trigger-evals.json is missing coverage for: {', '.join(missing_trigger_keys)}.")

    runtime_counts: dict[str, int] = {}
    for item in runtime_evals["evals"]:
        runtime_counts[item["skill"]] = runtime_counts.get(item["skill"], 0) + 1
        if "vault_root" not in item:
            errors.append(f"runtime-evals.json item {item.get('id')} is missing vault_root.")
        if item.get("mode", "read-only") != "read-only":
            errors.append(f"runtime-evals.json item {item.get('id')} must stay read-only.")
    operational_skills = [name for name, entry in load_json(REGISTRY_PATH)["skills"].items() if entry.get("role") == "operation"]
    for skill_name in operational_skills:
        if runtime_counts.get(skill_name, 0) < 2:
            errors.append(f"runtime-evals.json must include at least two evals for {skill_name}.")

    writable_counts: dict[str, int] = {}
    for item in writable_runtime_evals["evals"]:
        writable_counts[item["skill"]] = writable_counts.get(item["skill"], 0) + 1
        if "vault_root" not in item:
            errors.append(f"runtime-evals-writable.json item {item.get('id')} is missing vault_root.")
        if item.get("mode") != "writable-copy":
            errors.append(f"runtime-evals-writable.json item {item.get('id')} must use writable-copy mode.")
        if not item.get("checks"):
            errors.append(f"runtime-evals-writable.json item {item.get('id')} must define checks.")
    writable_required = [
        name
        for name, entry in load_json(REGISTRY_PATH)["skills"].items()
        if entry.get("role") == "operation" and entry.get("writes")
    ]
    for skill_name in writable_required:
        if writable_counts.get(skill_name, 0) < 1:
            errors.append(f"runtime-evals-writable.json must include at least one eval for {skill_name}.")

    return errors


def build_report() -> dict[str, Any]:
    checks = 0
    errors: list[str] = []

    compiled, compile_errors = compile_python_files()
    checks += compiled
    errors.extend(compile_errors)

    registry_errors = check_registry_and_skills()
    checks += len(SKILL_PATHS) + 1
    errors.extend(registry_errors)

    docs_errors = check_docs_and_evals()
    checks += 4
    errors.extend(docs_errors)

    return {
        "status": "ok" if not errors else "error",
        "checks": checks,
        "errors": errors,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description="Validate the shipped skill bundle contract.")
    parser.add_argument("--json", action="store_true", help="Emit a JSON report instead of plain text.")
    args = parser.parse_args()

    report = build_report()
    if args.json:
        print(json.dumps(report, ensure_ascii=False, indent=2))
    elif report["status"] == "ok":
        print(f"Validated skill bundle successfully ({report['checks']} checks).")
    else:
        print("Skill bundle validation failed:")
        for error in report["errors"]:
            print(f"- {error}")

    if report["status"] != "ok":
        raise SystemExit(1)


if __name__ == "__main__":
    main()
