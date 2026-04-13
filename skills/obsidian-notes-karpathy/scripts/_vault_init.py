from __future__ import annotations

import shutil
from datetime import UTC, datetime
from pathlib import Path
from typing import Any

from _vault_ingest import load_source_manifest, sync_source_manifest
from _vault_layout import DEFAULT_KB_PROFILE, LEGACY_RAW_DIR_MAP, accepted_raw_sources, detect_layout_family
from _vault_review import scan_review_queue
from _vault_utils import json_dump


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parents[2]
KB_INIT_ASSETS_ROOT = REPO_ROOT / "skills" / "kb-init" / "assets"

REQUIRED_DIRECTORIES = (
    "raw",
    "raw/human/articles",
    "raw/human/papers",
    "raw/human/podcasts",
    "raw/human/repos",
    "raw/human/assets",
    "raw/human/data",
    "raw/agents",
    "wiki/drafts/summaries",
    "wiki/drafts/topics",
    "wiki/drafts/concepts",
    "wiki/drafts/entities",
    "wiki/drafts/procedures",
    "wiki/drafts/overviews",
    "wiki/drafts/comparisons",
    "wiki/drafts/indices",
    "wiki/live/summaries",
    "wiki/live/topics",
    "wiki/live/concepts",
    "wiki/live/entities",
    "wiki/live/procedures",
    "wiki/live/overviews",
    "wiki/live/comparisons",
    "wiki/live/indices",
    "wiki/briefings",
    "outputs",
    "outputs/reviews",
)

FULL_OUTPUT_DIRECTORIES = (
    "outputs/qa",
    "outputs/health",
    "outputs/reports",
    "outputs/slides",
    "outputs/charts",
    "outputs/web",
    "outputs/content/articles",
    "outputs/content/threads",
    "outputs/content/talks",
)

LATEST_OUTPUT_DIRECTORIES = (
    "outputs/episodes",
    "outputs/audit",
)

BASE_ASSET_FILES = (
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

GOVERNANCE_ASSET_FILES = (
    "wiki/live/indices/QUESTIONS.md",
    "wiki/live/indices/GAPS.md",
    "wiki/live/indices/ALIASES.md",
    "wiki/live/indices/ENTITIES.md",
    "wiki/live/indices/RELATIONSHIPS.md",
)

LATEST_ASSET_FILES = (
    "outputs/audit/operations.jsonl",
)

LEGACY_WIKI_DIR_MAP = {
    "wiki/summaries": "wiki/live/summaries",
    "wiki/concepts": "wiki/live/concepts",
    "wiki/entities": "wiki/live/entities",
    "wiki/topics": "wiki/live/topics",
    "wiki/overviews": "wiki/live/overviews",
    "wiki/comparisons": "wiki/live/comparisons",
    "wiki/procedures": "wiki/live/procedures",
    "wiki/queries": "outputs/qa",
}


def _now_iso() -> str:
    return datetime.now(UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def _render_template(text: str, context: dict[str, str]) -> str:
    rendered = text
    for key, value in context.items():
        rendered = rendered.replace(f"{{{{{key}}}}}", value)
    return rendered


def _template_context(vault_root: Path, profile: str) -> dict[str, str]:
    generated_at = _now_iso()
    return {
        "GENERATED_AT": generated_at,
        "KB_PROFILE": profile,
        "VAULT_NAME": vault_root.name,
        "NEXT_ACTION": "kb-ingest or kb-compile",
        "UTC_DATE": generated_at.split("T", 1)[0],
    }


def _asset_paths(
    *,
    include_governance: bool,
    include_latest_outputs: bool,
) -> tuple[str, ...]:
    asset_paths = list(BASE_ASSET_FILES)
    if include_governance:
        asset_paths.extend(GOVERNANCE_ASSET_FILES)
    if include_latest_outputs:
        asset_paths.extend(LATEST_ASSET_FILES)
    return tuple(asset_paths)


def _directory_paths(
    *,
    include_full_outputs: bool,
    include_latest_outputs: bool,
) -> tuple[str, ...]:
    dir_paths = list(REQUIRED_DIRECTORIES)
    if include_full_outputs:
        dir_paths.extend(FULL_OUTPUT_DIRECTORIES)
    if include_latest_outputs:
        dir_paths.extend(LATEST_OUTPUT_DIRECTORIES)
    return tuple(dir_paths)


def _write_text_if_missing(
    target_path: Path,
    text: str,
    *,
    overwrite: bool,
) -> str:
    if target_path.exists() and not overwrite:
        return "preserved"
    target_path.parent.mkdir(parents=True, exist_ok=True)
    target_path.write_text(text, encoding="utf-8")
    return "written"


def _write_asset_file(
    vault_root: Path,
    asset_rel_path: str,
    context: dict[str, str],
    *,
    overwrite: bool,
) -> str:
    asset_path = KB_INIT_ASSETS_ROOT / asset_rel_path
    target_path = vault_root / asset_rel_path
    rendered = _render_template(asset_path.read_text(encoding="utf-8"), context)
    return _write_text_if_missing(target_path, rendered, overwrite=overwrite)


def _append_log_event(vault_root: Path, message: str) -> None:
    log_path = vault_root / "wiki" / "log.md"
    if not log_path.exists():
        return
    entry = f"- [{_now_iso()}] {message}\n"
    with log_path.open("a", encoding="utf-8") as handle:
        handle.write(entry)


def scaffold_review_gated_vault(
    vault_root: Path,
    *,
    profile: str = DEFAULT_KB_PROFILE,
    include_governance: bool = False,
    include_full_outputs: bool = False,
    include_latest_outputs: bool = False,
    overwrite: bool = False,
) -> dict[str, Any]:
    context = _template_context(vault_root, profile)
    created_dirs: list[str] = []
    preserved_files: list[str] = []
    written_files: list[str] = []

    for rel_dir in _directory_paths(
        include_full_outputs=include_full_outputs,
        include_latest_outputs=include_latest_outputs,
    ):
        target_dir = vault_root / rel_dir
        if not target_dir.exists():
            target_dir.mkdir(parents=True, exist_ok=True)
            created_dirs.append(rel_dir)

    for asset_rel_path in _asset_paths(
        include_governance=include_governance,
        include_latest_outputs=include_latest_outputs,
    ):
        status = _write_asset_file(vault_root, asset_rel_path, context, overwrite=overwrite)
        if status == "written":
            written_files.append(asset_rel_path)
        else:
            preserved_files.append(asset_rel_path)

    return {
        "vault_root": str(vault_root),
        "profile": profile,
        "created_dirs": created_dirs,
        "written_files": written_files,
        "preserved_files": preserved_files,
    }


def _copy_tree_files(
    source_root: Path,
    target_root: Path,
    *,
    backup_root: Path | None,
) -> tuple[list[dict[str, str]], list[dict[str, str]]]:
    migrated: list[dict[str, str]] = []
    skipped: list[dict[str, str]] = []
    if not source_root.exists():
        return migrated, skipped

    for source_path in sorted(source_root.rglob("*")):
        if not source_path.is_file():
            continue
        rel_path = source_path.relative_to(source_root)
        target_path = target_root / rel_path
        if target_path.exists():
            skipped.append(
                {
                    "source": source_path.as_posix(),
                    "target": target_path.as_posix(),
                    "reason": "target_exists",
                }
            )
            continue
        target_path.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(source_path, target_path)
        if backup_root is not None:
            backup_path = backup_root / rel_path
            backup_path.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(source_path, backup_path)
        migrated.append(
            {
                "source": source_path.as_posix(),
                "target": target_path.as_posix(),
            }
        )
    return migrated, skipped


def _write_migration_report(
    vault_root: Path,
    *,
    profile: str,
    scaffold_result: dict[str, Any],
    migrated_raw: list[dict[str, str]],
    migrated_live: list[dict[str, str]],
    skipped: list[dict[str, str]],
) -> str:
    report_path = vault_root / "outputs" / "reviews" / "migration-report.md"
    lines = [
        "---",
        f'title: "Legacy Migration Report"',
        f'generated_at: "{_now_iso()}"',
        f'profile: "{profile}"',
        f'layout_family: "{detect_layout_family(vault_root)}"',
        "---",
        "",
        "# Legacy Migration Report",
        "",
        "## Support Layer",
        "",
        f"- created_dirs: {len(scaffold_result['created_dirs'])}",
        f"- written_files: {len(scaffold_result['written_files'])}",
        "",
        "## Raw Captures Migrated",
        "",
    ]
    if migrated_raw:
        lines.extend(f"- `{item['source']}` -> `{item['target']}`" for item in migrated_raw)
    else:
        lines.append("- No legacy raw captures required migration.")
    lines.extend(
        [
            "",
            "## Wiki Pages Migrated",
            "",
        ]
    )
    if migrated_live:
        lines.extend(f"- `{item['source']}` -> `{item['target']}`" for item in migrated_live)
    else:
        lines.append("- No legacy wiki pages required migration.")
    lines.extend(
        [
            "",
            "## Preserved Legacy Paths",
            "",
            "- Originals remain on disk under their legacy paths for auditability.",
            "- Active review-gated workflows should now use `raw/human/**`, `wiki/live/**`, and `raw/_manifest.yaml`.",
            "",
            "## Skipped Paths",
            "",
        ]
    )
    if skipped:
        lines.extend(
            f"- `{item['source']}` skipped because `{item['target']}` already exists."
            for item in skipped
        )
    else:
        lines.append("- None.")
    lines.extend(
        [
            "",
            "## Next Step",
            "",
            "- Run `kb-ingest` if raw migration introduced new sources into `raw/human/**`.",
            "- Then run `kb-compile` and `kb-review` before normal query work resumes.",
        ]
    )
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return report_path.relative_to(vault_root).as_posix()


def migrate_legacy_vault(
    vault_root: Path,
    *,
    profile: str = DEFAULT_KB_PROFILE,
    include_governance: bool = True,
    include_full_outputs: bool = False,
    include_latest_outputs: bool = False,
) -> dict[str, Any]:
    scaffold_result = scaffold_review_gated_vault(
        vault_root,
        profile=profile,
        include_governance=include_governance,
        include_full_outputs=include_full_outputs,
        include_latest_outputs=include_latest_outputs,
        overwrite=False,
    )

    backup_root = vault_root / "outputs" / "reviews" / "migration-backups"
    migrated_raw: list[dict[str, str]] = []
    migrated_live: list[dict[str, str]] = []
    skipped: list[dict[str, str]] = []

    for legacy_dir, review_gated_target in LEGACY_RAW_DIR_MAP.items():
        source_root = vault_root / "raw" / legacy_dir
        target_root = vault_root / review_gated_target
        moved, ignored = _copy_tree_files(source_root, target_root, backup_root=backup_root / "raw" / legacy_dir)
        migrated_raw.extend(moved)
        skipped.extend(ignored)

    for legacy_root, target_root in LEGACY_WIKI_DIR_MAP.items():
        source_root = vault_root / legacy_root
        target_dir = vault_root / target_root
        moved, ignored = _copy_tree_files(source_root, target_dir, backup_root=backup_root / legacy_root)
        migrated_live.extend(moved)
        skipped.extend(ignored)

    manifest_result = sync_source_manifest(vault_root)
    report_path = _write_migration_report(
        vault_root,
        profile=profile,
        scaffold_result=scaffold_result,
        migrated_raw=migrated_raw,
        migrated_live=migrated_live,
        skipped=skipped,
    )
    _append_log_event(vault_root, "kb-init migration completed")
    return {
        "vault_root": str(vault_root),
        "profile": profile,
        "scaffold": scaffold_result,
        "migrated_raw": migrated_raw,
        "migrated_live": migrated_live,
        "skipped": skipped,
        "written_manifest": manifest_result["written_manifest"],
        "migration_report": report_path,
    }


def describe_vault_status(vault_root: Path) -> dict[str, Any]:
    from detect_lifecycle import detect_lifecycle

    lifecycle = detect_lifecycle(vault_root)
    manifest = load_source_manifest(vault_root)
    review_queue = scan_review_queue(vault_root)
    counts = {
        "raw_sources": len(accepted_raw_sources(vault_root)),
        "manifest_sources": len(manifest.get("sources", [])),
        "draft_pages": sum(1 for _ in (vault_root / "wiki" / "drafts").rglob("*.md")) if (vault_root / "wiki" / "drafts").exists() else 0,
        "live_pages": sum(1 for _ in (vault_root / "wiki" / "live").rglob("*.md")) if (vault_root / "wiki" / "live").exists() else 0,
        "briefings": sum(1 for _ in (vault_root / "wiki" / "briefings").rglob("*.md")) if (vault_root / "wiki" / "briefings").exists() else 0,
        "review_records": sum(1 for _ in (vault_root / "outputs" / "reviews").rglob("*.md")) if (vault_root / "outputs" / "reviews").exists() else 0,
        "pending_reviews": review_queue["counts"]["pending"],
    }
    summary_lines = [
        f"Vault: {vault_root}",
        f"Profile: {lifecycle['profile']}",
        f"Stage: {lifecycle['state']}",
        f"Next step: {lifecycle['route']}",
        "Signals:",
    ]
    summary_lines.extend(f"- {signal}" for signal in lifecycle["signals"]) if lifecycle["signals"] else summary_lines.append("- none")
    summary_lines.extend(
        [
            "Counts:",
            *[f"- {key}: {value}" for key, value in counts.items()],
        ]
    )
    return {
        "vault_root": str(vault_root),
        "profile": lifecycle["profile"],
        "state": lifecycle["state"],
        "route": lifecycle["route"],
        "signals": lifecycle["signals"],
        "counts": counts,
        "summary": "\n".join(summary_lines),
    }


def json_status(payload: dict[str, Any]) -> str:
    return json_dump(payload)
