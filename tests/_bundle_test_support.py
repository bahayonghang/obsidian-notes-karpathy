import json
import os
import re
import subprocess
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
ENTRY_SKILL_ROOT = REPO_ROOT / "skills" / "obsidian-notes-karpathy"
SCRIPTS_DIR = ENTRY_SKILL_ROOT / "scripts"
FIXTURES_DIR = ENTRY_SKILL_ROOT / "evals" / "fixtures"
REGISTRY_PATH = SCRIPTS_DIR / "skill-contract-registry.json"
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
REFERENCE_BULLETS_RE = re.compile(r"- `([^`]+)`")
READ_BEFORE_RE = re.compile(r"## Read before .*?(?=\n## |\Z)", re.DOTALL)
SCOPE_READ_RE = re.compile(r"## Scope\n\nBefore checking the vault, read these files first:.*?(?=\n## |\Z)", re.DOTALL)
sys.path.insert(0, str(SCRIPTS_DIR))

from _vault_utils import accepted_raw_sources, summarize_local_guidance


def load_registry() -> dict:
    return json.loads(REGISTRY_PATH.read_text(encoding="utf-8"))


def extract_reference_bullets(skill_text: str) -> list[str]:
    section_match = READ_BEFORE_RE.search(skill_text) or SCOPE_READ_RE.search(skill_text)
    if not section_match:
        return []
    return REFERENCE_BULLETS_RE.findall(section_match.group(0))


def run_json_script(script_name: str, *args: str, env: dict[str, str] | None = None) -> dict:
    script_path = SCRIPTS_DIR / script_name
    command_env = os.environ.copy()
    if env:
        command_env.update(env)
    result = subprocess.run(
        [sys.executable, str(script_path), *args],
        cwd=REPO_ROOT,
        check=True,
        capture_output=True,
        text=True,
        env=command_env,
    )
    return json.loads(result.stdout)


def run_repo_command(*args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        list(args),
        cwd=REPO_ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
