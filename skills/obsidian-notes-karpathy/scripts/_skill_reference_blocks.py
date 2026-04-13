from __future__ import annotations

import json
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
ENTRY_SKILL_ROOT = SCRIPT_DIR.parent
REPO_ROOT = ENTRY_SKILL_ROOT.parents[1]
REGISTRY_PATH = SCRIPT_DIR / "skill-contract-registry.json"


def load_registry() -> dict[str, Any]:
    return json.loads(REGISTRY_PATH.read_text(encoding="utf-8"))


def bullet_prefix(skill_name: str) -> str:
    return "./" if skill_name == "obsidian-notes-karpathy" else "../obsidian-notes-karpathy/"


def build_shared_reference_bullets(skill_name: str, registry: dict[str, Any] | None = None) -> list[str]:
    payload = registry or load_registry()
    skill_entry = payload["skills"][skill_name]
    prefix = bullet_prefix(skill_name)
    bullets = [f"- `{prefix}scripts/skill-contract-registry.json`"]
    bullets.extend(f"- `{prefix}references/{reference}`" for reference in skill_entry["reads"])
    return bullets


def render_shared_reference_block(skill_name: str, registry: dict[str, Any] | None = None) -> str:
    return "\n".join(build_shared_reference_bullets(skill_name, registry=registry))
