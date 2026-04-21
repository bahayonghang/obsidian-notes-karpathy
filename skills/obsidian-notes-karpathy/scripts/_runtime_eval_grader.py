from __future__ import annotations

import hashlib
import shutil
from pathlib import Path
from typing import Any

from _runtime_eval_paths import (
    MARKDOWN_LINK_RE,
    REPO_ROOT,
    repo_relative,
    resolve_entry_relative,
)


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
    scrubbed = MARKDOWN_LINK_RE.sub(r"\1", text)
    normalized = scrubbed.replace("\\", "/").lower()
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
