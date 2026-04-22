use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::Result;
use include_dir::{Dir, include_dir};
use serde_json::{Value, json};

use crate::common::{now_iso, relative_posix, write_markdown};
use crate::compile::scan_compile_delta;
use crate::guidance::inspect_local_guidance;
use crate::ingest::{load_source_manifest, scan_ingest_delta, sync_source_manifest};
use crate::layout::{accepted_raw_sources, detect_layout_family, resolve_vault_profile};
use crate::review::scan_review_queue;

static KB_INIT_ASSETS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/skills/kb-init/assets");

const REQUIRED_DIRECTORIES: [&str; 24] = [
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
];
const REQUIRED_DIRECTORIES_2: [&str; 3] = ["wiki/briefings", "outputs", "outputs/reviews"];
const FULL_OUTPUT_DIRECTORIES: [&str; 8] = [
    "outputs/qa",
    "outputs/health",
    "outputs/reports",
    "outputs/slides",
    "outputs/charts",
    "outputs/web",
    "outputs/content/articles",
    "outputs/content/threads",
];
const FULL_OUTPUT_DIRECTORIES_2: [&str; 1] = ["outputs/content/talks"];
const LATEST_OUTPUT_DIRECTORIES: [&str; 2] = ["outputs/episodes", "outputs/audit"];
const BASE_ASSET_FILES: [&str; 13] = [
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
];
const GOVERNANCE_ASSET_FILES: [&str; 5] = [
    "wiki/live/indices/QUESTIONS.md",
    "wiki/live/indices/GAPS.md",
    "wiki/live/indices/ALIASES.md",
    "wiki/live/indices/ENTITIES.md",
    "wiki/live/indices/RELATIONSHIPS.md",
];
const LATEST_ASSET_FILES: [&str; 1] = ["outputs/audit/operations.jsonl"];
const LEGACY_RAW_DIR_MAP: [(&str, &str); 4] = [
    ("articles", "raw/human/articles"),
    ("papers", "raw/human/papers"),
    ("podcasts", "raw/human/podcasts"),
    ("repos", "raw/human/repos"),
];
const LEGACY_WIKI_DIR_MAP: [(&str, &str); 7] = [
    ("wiki/summaries", "wiki/live/summaries"),
    ("wiki/concepts", "wiki/live/concepts"),
    ("wiki/entities", "wiki/live/entities"),
    ("wiki/topics", "wiki/live/topics"),
    ("wiki/overviews", "wiki/live/overviews"),
    ("wiki/comparisons", "wiki/live/comparisons"),
    ("wiki/procedures", "wiki/live/procedures"),
];
const LEGACY_WIKI_DIR_MAP_2: [(&str, &str); 1] = [("wiki/queries", "outputs/qa")];
const CORE_SUPPORT_FILES: [&str; 2] = ["wiki/index.md", "wiki/log.md"];
const REVIEW_GATED_SUPPORT_PATHS: [&str; 4] = [
    "wiki/drafts",
    "wiki/live",
    "wiki/briefings",
    "outputs/reviews",
];

fn template_context(vault_root: &Path, profile: &str) -> BTreeMap<String, String> {
    let generated_at = now_iso();
    BTreeMap::from([
        ("GENERATED_AT".to_string(), generated_at.clone()),
        ("KB_PROFILE".to_string(), profile.to_string()),
        (
            "VAULT_NAME".to_string(),
            vault_root
                .file_name()
                .map(|value| value.to_string_lossy().into_owned())
                .unwrap_or_default(),
        ),
        (
            "NEXT_ACTION".to_string(),
            "kb-ingest or kb-compile".to_string(),
        ),
        (
            "UTC_DATE".to_string(),
            generated_at
                .split('T')
                .next()
                .unwrap_or_default()
                .to_string(),
        ),
    ])
}

fn render_template(text: &str, context: &BTreeMap<String, String>) -> String {
    let mut rendered = text.to_string();
    for (key, value) in context {
        rendered = rendered.replace(&format!("{{{{{key}}}}}"), value);
    }
    rendered
}

fn asset_paths(
    include_governance: bool,
    include_latest_outputs: bool,
    skip_memory: bool,
) -> Vec<&'static str> {
    let mut assets = BASE_ASSET_FILES.to_vec();
    if include_governance {
        assets.extend(GOVERNANCE_ASSET_FILES);
    }
    if include_latest_outputs {
        assets.extend(LATEST_ASSET_FILES);
    }
    if skip_memory {
        assets.retain(|path| *path != "MEMORY.md");
    }
    assets
}

fn directory_paths(include_full_outputs: bool, include_latest_outputs: bool) -> Vec<&'static str> {
    let mut dirs = REQUIRED_DIRECTORIES.to_vec();
    dirs.extend(REQUIRED_DIRECTORIES_2);
    if include_full_outputs {
        dirs.extend(FULL_OUTPUT_DIRECTORIES);
        dirs.extend(FULL_OUTPUT_DIRECTORIES_2);
    }
    if include_latest_outputs {
        dirs.extend(LATEST_OUTPUT_DIRECTORIES);
    }
    dirs
}

pub(crate) fn write_asset_file(
    vault_root: &Path,
    asset_rel_path: &str,
    context: &BTreeMap<String, String>,
    overwrite: bool,
) -> Result<&'static str> {
    let asset_file = KB_INIT_ASSETS
        .get_file(asset_rel_path)
        .ok_or_else(|| anyhow::anyhow!("missing embedded asset {asset_rel_path}"))?;
    let target_path = vault_root.join(asset_rel_path);
    if target_path.exists() && !overwrite {
        return Ok("preserved");
    }
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let rendered = render_template(asset_file.contents_utf8().unwrap_or_default(), context);
    fs::write(target_path, rendered)?;
    Ok("written")
}

pub(crate) fn append_log_event(vault_root: &Path, message: &str) -> Result<()> {
    let log_path = vault_root.join("wiki").join("log.md");
    if !log_path.exists() {
        return Ok(());
    }
    let mut existing = fs::read_to_string(&log_path)?;
    existing.push_str(&format!("- [{}] {}\n", now_iso(), message));
    fs::write(log_path, existing)?;
    Ok(())
}

pub fn scaffold_review_gated_vault(
    vault_root: &Path,
    profile: &str,
    include_governance: bool,
    include_full_outputs: bool,
    include_latest_outputs: bool,
    overwrite: bool,
    skip_memory: bool,
) -> Result<Value> {
    let context = template_context(vault_root, profile);
    let mut created_dirs = Vec::new();
    let mut preserved_files = Vec::new();
    let mut written_files = Vec::new();

    for rel_dir in directory_paths(include_full_outputs, include_latest_outputs) {
        let target_dir = vault_root.join(rel_dir);
        if !target_dir.exists() {
            fs::create_dir_all(&target_dir)?;
            created_dirs.push(rel_dir.to_string());
        }
    }

    for asset_rel_path in asset_paths(include_governance, include_latest_outputs, skip_memory) {
        match write_asset_file(vault_root, asset_rel_path, &context, overwrite)? {
            "written" => written_files.push(asset_rel_path.to_string()),
            _ => preserved_files.push(asset_rel_path.to_string()),
        }
    }

    Ok(json!({
        "vault_root": crate::common::normalize_path_string(vault_root.to_string_lossy().as_ref()),
        "profile": profile,
        "created_dirs": created_dirs,
        "written_files": written_files,
        "preserved_files": preserved_files,
        "skip_memory": skip_memory,
    }))
}

fn copy_tree_files(
    source_root: &Path,
    target_root: &Path,
    backup_root: Option<&Path>,
) -> Result<(Vec<Value>, Vec<Value>)> {
    let mut migrated = Vec::new();
    let mut skipped = Vec::new();
    if !source_root.exists() {
        return Ok((migrated, skipped));
    }
    for entry in walkdir::WalkDir::new(source_root).sort_by_file_name() {
        let entry = match entry {
            Ok(value) => value,
            Err(_) => continue,
        };
        if !entry.file_type().is_file() {
            continue;
        }
        let rel_path = entry
            .path()
            .strip_prefix(source_root)
            .unwrap_or(entry.path());
        let target_path = target_root.join(rel_path);
        if target_path.exists() {
            skipped.push(json!({
                "source": crate::common::normalize_path_string(entry.path().to_string_lossy().as_ref()),
                "target": crate::common::normalize_path_string(target_path.to_string_lossy().as_ref()),
                "reason": "target_exists",
            }));
            continue;
        }
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(entry.path(), &target_path)?;
        if let Some(backup_root) = backup_root {
            let backup_path = backup_root.join(rel_path);
            if let Some(parent) = backup_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(entry.path(), backup_path)?;
        }
        migrated.push(json!({
            "source": crate::common::normalize_path_string(entry.path().to_string_lossy().as_ref()),
            "target": crate::common::normalize_path_string(target_path.to_string_lossy().as_ref()),
        }));
    }
    Ok((migrated, skipped))
}

fn write_migration_report(
    vault_root: &Path,
    profile: &str,
    scaffold_result: &Value,
    migrated_raw: &[Value],
    migrated_live: &[Value],
    skipped: &[Value],
) -> Result<String> {
    let report_path = vault_root
        .join("outputs")
        .join("reviews")
        .join("migration-report.md");
    let mut lines = vec![
        "---".to_string(),
        r#"title: "Legacy Migration Report""#.to_string(),
        format!(r#"generated_at: "{}""#, now_iso()),
        format!(r#"profile: "{}""#, profile),
        format!(r#"layout_family: "{}""#, detect_layout_family(vault_root)),
        "---".to_string(),
        String::new(),
        "# Legacy Migration Report".to_string(),
        String::new(),
        "## Support Layer".to_string(),
        String::new(),
        format!(
            "- created_dirs: {}",
            scaffold_result
                .get("created_dirs")
                .and_then(Value::as_array)
                .map(Vec::len)
                .unwrap_or_default()
        ),
        format!(
            "- written_files: {}",
            scaffold_result
                .get("written_files")
                .and_then(Value::as_array)
                .map(Vec::len)
                .unwrap_or_default()
        ),
        String::new(),
        "## Raw Captures Migrated".to_string(),
        String::new(),
    ];
    if migrated_raw.is_empty() {
        lines.push("- No legacy raw captures required migration.".to_string());
    } else {
        for item in migrated_raw {
            lines.push(format!(
                "- `{}` -> `{}`",
                item.get("source")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
                item.get("target")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
            ));
        }
    }
    lines.push(String::new());
    lines.push("## Wiki Pages Migrated".to_string());
    lines.push(String::new());
    if migrated_live.is_empty() {
        lines.push("- No legacy wiki pages required migration.".to_string());
    } else {
        for item in migrated_live {
            lines.push(format!(
                "- `{}` -> `{}`",
                item.get("source")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
                item.get("target")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
            ));
        }
    }
    lines.push(String::new());
    lines.push("## Preserved Legacy Paths".to_string());
    lines.push(String::new());
    lines.push("- Originals remain on disk under their legacy paths for auditability.".to_string());
    lines.push("- Active review-gated workflows should now use `raw/human/**`, `wiki/live/**`, and `raw/_manifest.yaml`.".to_string());
    lines.push(String::new());
    lines.push("## Skipped Paths".to_string());
    lines.push(String::new());
    if skipped.is_empty() {
        lines.push("- None.".to_string());
    } else {
        for item in skipped {
            lines.push(format!(
                "- `{}` skipped because `{}` already exists.",
                item.get("source")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
                item.get("target")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
            ));
        }
    }
    lines.push(String::new());
    lines.push("## Next Step".to_string());
    lines.push(String::new());
    lines.push(
        "- Run `kb-ingest` if raw migration introduced new sources into `raw/human/**`."
            .to_string(),
    );
    lines.push(
        "- Then run `kb-compile` and `kb-review` before normal query work resumes.".to_string(),
    );
    write_markdown(&report_path, &lines)?;
    Ok(relative_posix(&report_path, vault_root))
}

pub fn migrate_legacy_vault(
    vault_root: &Path,
    profile: &str,
    include_governance: bool,
    include_full_outputs: bool,
    include_latest_outputs: bool,
) -> Result<Value> {
    let scaffold_result = scaffold_review_gated_vault(
        vault_root,
        profile,
        include_governance,
        include_full_outputs,
        include_latest_outputs,
        false,
        false,
    )?;
    let backup_root = vault_root
        .join("outputs")
        .join("reviews")
        .join("migration-backups");
    let mut migrated_raw = Vec::new();
    let mut migrated_live = Vec::new();
    let mut skipped = Vec::new();

    for (legacy_dir, review_gated_target) in LEGACY_RAW_DIR_MAP {
        let source_root = vault_root.join("raw").join(legacy_dir);
        let target_root = vault_root.join(review_gated_target);
        let (moved, ignored) = copy_tree_files(
            &source_root,
            &target_root,
            Some(&backup_root.join("raw").join(legacy_dir)),
        )?;
        migrated_raw.extend(moved);
        skipped.extend(ignored);
    }
    for (legacy_root, target_root) in LEGACY_WIKI_DIR_MAP.into_iter().chain(LEGACY_WIKI_DIR_MAP_2) {
        let source_root = vault_root.join(legacy_root);
        let target_dir = vault_root.join(target_root);
        let (moved, ignored) = copy_tree_files(
            &source_root,
            &target_dir,
            Some(&backup_root.join(legacy_root)),
        )?;
        migrated_live.extend(moved);
        skipped.extend(ignored);
    }

    let manifest_result = sync_source_manifest(vault_root)?;
    let report_path = write_migration_report(
        vault_root,
        profile,
        &scaffold_result,
        &migrated_raw,
        &migrated_live,
        &skipped,
    )?;
    append_log_event(vault_root, "kb-init migration completed")?;
    Ok(json!({
        "vault_root": crate::common::normalize_path_string(vault_root.to_string_lossy().as_ref()),
        "profile": profile,
        "scaffold": scaffold_result,
        "migrated_raw": migrated_raw,
        "migrated_live": migrated_live,
        "skipped": skipped,
        "written_manifest": manifest_result.get("written_manifest").cloned().unwrap_or_else(|| json!("")),
        "migration_report": report_path,
    }))
}

pub fn detect_lifecycle(vault_root: &Path) -> Result<Value> {
    let mut signals = Vec::new();
    let layout_family = detect_layout_family(vault_root);
    let profile = resolve_vault_profile(vault_root);
    let missing = CORE_SUPPORT_FILES
        .iter()
        .filter(|rel| !vault_root.join(rel).exists())
        .map(|rel| rel.to_string())
        .collect::<Vec<_>>();
    let missing_support_paths = REVIEW_GATED_SUPPORT_PATHS
        .iter()
        .filter(|rel| layout_family == "review-gated" && !vault_root.join(rel).exists())
        .map(|rel| rel.to_string())
        .collect::<Vec<_>>();
    let guidance = inspect_local_guidance(vault_root);
    let guidance_status = json!({
        "agents": guidance.get("agents").cloned().unwrap_or(Value::Null),
        "claude": guidance.get("claude").cloned().unwrap_or(Value::Null),
    });
    let guidance_warnings = guidance
        .get("warnings")
        .cloned()
        .unwrap_or_else(|| json!([]));
    let blocking_guidance_issues = guidance
        .get("blocking_issues")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|value| value.as_str().map(ToString::to_string))
        .collect::<Vec<_>>();
    let has_raw = vault_root.join("raw").exists();
    let has_wiki = vault_root.join("wiki").exists();
    let has_outputs = vault_root.join("outputs").exists();

    let build_payload = |state: &str,
                         route: &str,
                         route_mode: Option<&str>,
                         signals: &[String],
                         missing_support_files: Vec<String>,
                         compile_delta: Value,
                         health_flags: Vec<String>|
     -> Value {
        json!({
            "vault_root": crate::common::normalize_path_string(vault_root.to_string_lossy().as_ref()),
            "layout_family": layout_family,
            "profile": profile,
            "state": state,
            "route": route,
            "route_mode": route_mode,
            "signals": signals,
            "missing_support_files": missing_support_files,
            "compile_delta": compile_delta,
            "health_flags": health_flags,
            "guidance_status": guidance_status,
            "guidance_warnings": guidance_warnings,
        })
    };

    let agents_present = guidance_status
        .get("agents")
        .and_then(|value| value.get("present"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let claude_present = guidance_status
        .get("claude")
        .and_then(|value| value.get("present"))
        .and_then(Value::as_bool)
        .unwrap_or(false);

    if !has_raw && !has_wiki && !has_outputs && !agents_present && !claude_present {
        signals.push("missing_core_directories".to_string());
        return Ok(build_payload(
            "needs-setup",
            "kb-init",
            None,
            &signals,
            missing,
            json!({"new_count": 0, "changed_count": 0, "unchanged_count": 0}),
            Vec::new(),
        ));
    }

    if layout_family == "legacy-layout" && blocking_guidance_issues.is_empty() {
        signals.push("legacy_layout_detected".to_string());
        return Ok(build_payload(
            "needs-migration",
            "kb-init",
            None,
            &signals,
            missing,
            json!({"new_count": 0, "changed_count": 0, "unchanged_count": 0}),
            Vec::new(),
        ));
    }

    let structural_partial =
        !has_wiki || !missing.is_empty() || !has_raw || !missing_support_paths.is_empty();
    if !blocking_guidance_issues.is_empty() || structural_partial {
        signals.extend(blocking_guidance_issues.clone());
        if !missing.is_empty() {
            signals.push("missing_support_layer".to_string());
        }
        if !missing_support_paths.is_empty() {
            signals.push("missing_review_gate_support".to_string());
        }
        if !has_raw {
            signals.push("missing_raw".to_string());
        }
        if !has_wiki {
            signals.push("missing_wiki".to_string());
        }
        let mut missing_support_files = missing.clone();
        missing_support_files.extend(missing_support_paths.clone());
        if blocking_guidance_issues
            .iter()
            .any(|issue| issue == "missing_agents_guidance")
        {
            missing_support_files.push("AGENTS.md".to_string());
        }
        missing_support_files.sort();
        missing_support_files.dedup();
        return Ok(build_payload(
            "needs-repair",
            "kb-init",
            None,
            &signals,
            missing_support_files,
            json!({"new_count": 0, "changed_count": 0, "unchanged_count": 0}),
            Vec::new(),
        ));
    }

    let compile_delta = scan_compile_delta(vault_root)?;
    let ingest_delta = scan_ingest_delta(vault_root)?;
    let new_count = compile_delta
        .get("counts")
        .and_then(|value| value.get("new"))
        .and_then(Value::as_i64)
        .unwrap_or_default();
    let changed_count = compile_delta
        .get("counts")
        .and_then(|value| value.get("changed"))
        .and_then(Value::as_i64)
        .unwrap_or_default();
    let unchanged_count = compile_delta
        .get("counts")
        .and_then(|value| value.get("unchanged"))
        .and_then(Value::as_i64)
        .unwrap_or_default();

    if ingest_delta
        .get("needs_ingest")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        if ingest_delta
            .get("counts")
            .and_then(|value| value.get("new"))
            .and_then(Value::as_i64)
            .unwrap_or_default()
            > 0
        {
            signals.push("new_sources_not_registered".to_string());
        }
        if ingest_delta
            .get("counts")
            .and_then(|value| value.get("changed"))
            .and_then(Value::as_i64)
            .unwrap_or_default()
            > 0
        {
            signals.push("registered_sources_changed".to_string());
        }
        if ingest_delta
            .get("counts")
            .and_then(|value| value.get("removed"))
            .and_then(Value::as_i64)
            .unwrap_or_default()
            > 0
        {
            signals.push("manifest_sources_removed".to_string());
        }
        return Ok(build_payload(
            "needs-ingest",
            "kb-ingest",
            None,
            &signals,
            Vec::new(),
            json!({"new_count": new_count, "changed_count": changed_count, "unchanged_count": unchanged_count}),
            Vec::new(),
        ));
    }

    if new_count > 0 || changed_count > 0 {
        if new_count > 0 {
            signals.push("new_raw_sources".to_string());
        }
        if changed_count > 0 {
            signals.push("changed_raw_sources".to_string());
        }
        return Ok(build_payload(
            "needs-compilation",
            "kb-compile",
            None,
            &signals,
            Vec::new(),
            json!({"new_count": new_count, "changed_count": changed_count, "unchanged_count": unchanged_count}),
            Vec::new(),
        ));
    }

    let review_queue = scan_review_queue(vault_root)?;
    if review_queue
        .get("counts")
        .and_then(|value| value.get("pending"))
        .and_then(Value::as_i64)
        .unwrap_or_default()
        > 0
    {
        signals.push("drafts_pending_review".to_string());
        return Ok(build_payload(
            "needs-review",
            "kb-review",
            Some("gate"),
            &signals,
            Vec::new(),
            json!({"new_count": 0, "changed_count": 0, "unchanged_count": unchanged_count}),
            Vec::new(),
        ));
    }

    let stale_briefings = if profile == "fast-personal" {
        Vec::new()
    } else {
        crate::health::briefing_staleness_issues(vault_root)?
    };
    if !stale_briefings.is_empty() {
        signals.push("briefing_refresh_required".to_string());
        return Ok(build_payload(
            "needs-briefing-refresh",
            "kb-review",
            Some("gate"),
            &signals,
            Vec::new(),
            json!({"new_count": 0, "changed_count": 0, "unchanged_count": unchanged_count}),
            vec!["stale_briefing".to_string()],
        ));
    }

    let audit = crate::health::audit_vault_mechanics(vault_root)?;
    let mut health_flags = audit
        .get("issues")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|issue| {
            issue
                .get("kind")
                .and_then(Value::as_str)
                .map(ToString::to_string)
        })
        .filter(|kind| {
            [
                "duplicate_concept",
                "duplicate_entity",
                "stale_qa",
                "alias_wikilink_in_table",
                "unapproved_live_page",
                "approved_conflict",
                "review_backlog",
                "memory_knowledge_mix",
                "writeback_backlog",
                "weak_live_sources",
                "broken_wikilink",
                "stale_briefing",
                "duplicate_alias_set",
                "volatile_page_stale",
                "missing_confidence_metadata",
                "confidence_decay_due",
                "supersession_gap",
                "episodic_backlog",
                "procedural_promotion_gap",
                "graph_gap",
                "audit_trail_gap",
            ]
            .contains(&kind.as_str())
        })
        .collect::<Vec<_>>();
    health_flags.sort();
    health_flags.dedup();
    if profile == "standard" {
        health_flags
            .retain(|flag| !["audit_trail_gap", "episodic_backlog"].contains(&flag.as_str()));
    } else if profile == "fast-personal" {
        health_flags.retain(|flag| {
            ![
                "stale_briefing",
                "stale_qa",
                "writeback_backlog",
                "confidence_decay_due",
                "episodic_backlog",
                "audit_trail_gap",
                "graph_gap",
            ]
            .contains(&flag.as_str())
        });
    }
    if !health_flags.is_empty() {
        signals.push("maintenance_needed".to_string());
        return Ok(build_payload(
            "needs-maintenance",
            "kb-review",
            Some("maintenance"),
            &signals,
            Vec::new(),
            json!({"new_count": 0, "changed_count": 0, "unchanged_count": unchanged_count}),
            health_flags,
        ));
    }

    signals.push("approved_knowledge_ready".to_string());
    Ok(build_payload(
        "ready-for-query",
        "kb-query",
        None,
        &signals,
        Vec::new(),
        json!({"new_count": 0, "changed_count": 0, "unchanged_count": unchanged_count}),
        Vec::new(),
    ))
}

pub fn describe_vault_status(vault_root: &Path) -> Result<Value> {
    let lifecycle = detect_lifecycle(vault_root)?;
    let manifest = load_source_manifest(vault_root)?;
    let review_queue = scan_review_queue(vault_root)?;
    let draft_pages = count_md_files(&vault_root.join("wiki").join("drafts"));
    let live_pages = count_md_files(&vault_root.join("wiki").join("live"));
    let briefings = count_md_files(&vault_root.join("wiki").join("briefings"));
    let review_records = count_md_files(&vault_root.join("outputs").join("reviews"));
    let counts = json!({
        "raw_sources": accepted_raw_sources(vault_root).len(),
        "manifest_sources": manifest.get("sources").and_then(Value::as_array).map(Vec::len).unwrap_or_default(),
        "draft_pages": draft_pages,
        "live_pages": live_pages,
        "briefings": briefings,
        "review_records": review_records,
        "pending_reviews": review_queue.get("counts").and_then(|value| value.get("pending")).and_then(Value::as_i64).unwrap_or_default(),
    });
    let mut summary_lines = vec![
        format!(
            "Vault: {}",
            crate::common::normalize_path_string(vault_root.to_string_lossy().as_ref())
        ),
        format!(
            "Profile: {}",
            lifecycle
                .get("profile")
                .and_then(Value::as_str)
                .unwrap_or_default()
        ),
        format!(
            "Stage: {}",
            lifecycle
                .get("state")
                .and_then(Value::as_str)
                .unwrap_or_default()
        ),
        format!(
            "Next step: {}",
            lifecycle
                .get("route")
                .and_then(Value::as_str)
                .unwrap_or_default()
        ),
        "Signals:".to_string(),
    ];
    if let Some(signals) = lifecycle.get("signals").and_then(Value::as_array) {
        if signals.is_empty() {
            summary_lines.push("- none".to_string());
        } else {
            for signal in signals {
                if let Some(signal) = signal.as_str() {
                    summary_lines.push(format!("- {signal}"));
                }
            }
        }
    }
    summary_lines.push("Counts:".to_string());
    if let Some(counts_map) = counts.as_object() {
        for (key, value) in counts_map {
            summary_lines.push(format!("- {key}: {}", value.as_i64().unwrap_or_default()));
        }
    }

    Ok(json!({
        "vault_root": crate::common::normalize_path_string(vault_root.to_string_lossy().as_ref()),
        "profile": lifecycle.get("profile").cloned().unwrap_or_else(|| json!("")),
        "state": lifecycle.get("state").cloned().unwrap_or_else(|| json!("")),
        "route": lifecycle.get("route").cloned().unwrap_or_else(|| json!("")),
        "signals": lifecycle.get("signals").cloned().unwrap_or_else(|| json!([])),
        "counts": counts,
        "summary": summary_lines.join("\n"),
    }))
}

fn count_md_files(root: &Path) -> usize {
    if !root.exists() {
        return 0;
    }
    walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.file_type().is_file()
                && entry.path().extension().and_then(|value| value.to_str()) == Some("md")
        })
        .count()
}
