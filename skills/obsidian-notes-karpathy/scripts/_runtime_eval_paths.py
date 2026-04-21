from __future__ import annotations

import re
from pathlib import Path


SCRIPT_DIR = Path(__file__).resolve().parent
ENTRY_SKILL_ROOT = SCRIPT_DIR.parent
REPO_ROOT = ENTRY_SKILL_ROOT.parents[1]
MANIFEST_PATH = ENTRY_SKILL_ROOT / "evals" / "runtime-evals.json"
WRITABLE_MANIFEST_PATH = ENTRY_SKILL_ROOT / "evals" / "runtime-evals-writable.json"
DEFAULT_WORKSPACE_ROOT = REPO_ROOT / ".runtime-evals"
SKILL_PATHS = {
    "obsidian-notes-karpathy": ENTRY_SKILL_ROOT / "SKILL.md",
    "kb-init": REPO_ROOT / "skills" / "kb-init" / "SKILL.md",
    "kb-ingest": REPO_ROOT / "skills" / "kb-ingest" / "SKILL.md",
    "kb-compile": REPO_ROOT / "skills" / "kb-compile" / "SKILL.md",
    "kb-review": REPO_ROOT / "skills" / "kb-review" / "SKILL.md",
    "kb-query": REPO_ROOT / "skills" / "kb-query" / "SKILL.md",
    "kb-render": REPO_ROOT / "skills" / "kb-render" / "SKILL.md",
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
MARKDOWN_LINK_RE = re.compile(r"\[([^\]]+)\]\(([^)]+)\)")


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
