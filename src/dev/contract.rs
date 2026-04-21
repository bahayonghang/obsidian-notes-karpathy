use std::path::Path;

use anyhow::Result;
use serde_json::{json, Value};

use super::reference_blocks::build_shared_reference_bullets;
use super::skill_audit::build_payload as build_audit_payload;
use super::{list_repo_files, load_registry, read_utf8, skill_paths};

const KB_INIT_REQUIRED_ASSETS: [&str; 13] = [
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
fn blocked_legacy_markers() -> Vec<String> {
    let py_short = ['p', 'y'].iter().collect::<String>();
    let py_full = ['p', 'y', 't', 'h', 'o', 'n'].iter().collect::<String>();
    let ext = format!(".{py_short}");
    let quoted_legacy_key = format!("\"{}_{}\"", "baseline", "script");
    let legacy_runner = ["run", "repo", py_full.as_str(), "script"].join("_");
    vec![
        format!("{py_short} -3"),
        format!("{py_full}3"),
        legacy_runner,
        quoted_legacy_key,
        format!("detect_lifecycle{ext}"),
        format!("scan_ingest_delta{ext}"),
        format!("scan_compile_delta{ext}"),
        format!("scan_review_queue{ext}"),
        format!("scan_query_scope{ext}"),
        format!("render_live_artifact{ext}"),
        format!("bootstrap_review_gated_vault{ext}"),
        format!("migrate_legacy_vault{ext}"),
        format!("vault_status{ext}"),
        format!("audit_skills{ext}"),
        format!("validate_bundle_contract{ext}"),
        format!("runtime_eval{ext}"),
        format!("trigger_eval{ext}"),
        format!("render_reference_block{ext}"),
        format!("sync_source_manifest{ext}"),
    ]
}

fn scan_forbidden_legacy_references(repo_root: &Path) -> Result<Vec<String>> {
    let files = list_repo_files(
        repo_root,
        &[
            "src",
            "tests",
            "skills",
            "docs",
            "README.md",
            "README_CN.md",
            "CLAUDE.md",
            "justfile",
            "AGENTS.md",
        ],
    )?;
    let mut issues = Vec::new();
    for path in files {
        let relative = super::relative_to_repo(repo_root, &path);
        if relative.starts_with("docs/node_modules/")
            || relative.starts_with("docs/.vitepress/dist/")
            || relative == "src/dev/contract.rs"
            || relative.ends_with(".png")
            || relative.ends_with(".pdf")
        {
            continue;
        }
        let text = match read_utf8(&path) {
            Ok(value) => value,
            Err(_) => continue,
        };
        for pattern in blocked_legacy_markers() {
            if text.contains(&pattern) {
                issues.push(format!(
                    "{relative}: forbidden legacy reference `{pattern}`"
                ));
            }
        }
    }
    Ok(issues)
}

fn forbidden_script_files_present(repo_root: &Path) -> Result<Vec<String>> {
    Ok(list_repo_files(
        repo_root,
        &["skills/obsidian-notes-karpathy/scripts", "tests"],
    )?
    .into_iter()
    .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("py"))
    .map(|path| super::relative_to_repo(repo_root, &path))
    .collect())
}

pub fn validate_bundle(repo_root: &Path) -> Result<Value> {
    let mut errors = Vec::new();
    let registry = load_registry(repo_root)?;
    if registry.contract_family != "review-gated" {
        errors.push("Registry contract_family must be review-gated.".to_string());
    }

    let skill_paths = skill_paths(repo_root);
    if registry.skills.keys().cloned().collect::<Vec<_>>()
        != skill_paths.keys().cloned().collect::<Vec<_>>()
    {
        errors.push("Registry skill keys must match the shipped core skills.".to_string());
    }

    for (skill_name, skill_path) in &skill_paths {
        if !skill_path.exists() {
            errors.push(format!(
                "Missing SKILL.md for {}: {}",
                skill_name,
                super::relative_to_repo(repo_root, skill_path)
            ));
            continue;
        }
        let skill_text = read_utf8(skill_path)?;
        if !skill_text.contains("name:") || !skill_text.contains("description:") {
            errors.push(format!(
                "{} is missing required frontmatter fields.",
                super::relative_to_repo(repo_root, skill_path)
            ));
        }
        let Some(expected) = registry.skills.get(skill_name) else {
            continue;
        };
        let referenced_paths = super::reference_blocks::extract_reference_bullets(&skill_text);
        for bullet in build_shared_reference_bullets(skill_name, &registry)? {
            let target = bullet
                .trim_start_matches("- `")
                .trim_end_matches('`')
                .to_string();
            if !referenced_paths.contains(&target) {
                errors.push(format!(
                    "{skill_name} is missing reference bullet for {target}."
                ));
            }
        }
        if !expected.baseline_command.is_empty() && !skill_text.contains(&expected.baseline_command)
        {
            errors.push(format!(
                "{skill_name} does not mention baseline command {}.",
                expected.baseline_command
            ));
        }
        if !expected.writes_mechanical_fix_only.is_empty() {
            let writes = expected
                .writes
                .iter()
                .cloned()
                .collect::<std::collections::BTreeSet<_>>();
            let overlap = expected
                .writes_mechanical_fix_only
                .iter()
                .filter(|item| writes.contains(*item))
                .cloned()
                .collect::<Vec<_>>();
            if !overlap.is_empty() {
                errors.push(format!(
                    "{skill_name} has writes_mechanical_fix_only entries also in writes: {:?}.",
                    overlap
                ));
            }
            if !skill_text.to_ascii_lowercase().contains("mechanical fix") {
                errors.push(format!(
                    "{skill_name} declares writes_mechanical_fix_only but SKILL.md lacks 'mechanical fix' policy text."
                ));
            }
        }
    }

    let kb_init_assets_root = repo_root.join("skills").join("kb-init").join("assets");
    for asset in KB_INIT_REQUIRED_ASSETS {
        if !kb_init_assets_root.join(asset).exists() {
            errors.push(format!("kb-init asset is missing: {asset}."));
        }
    }

    let audit_payload = build_audit_payload(repo_root)?;
    if audit_payload.get("status").and_then(Value::as_str) != Some("ok") {
        errors.push("Skill audit must pass before bundle validation succeeds.".to_string());
    }

    errors.extend(forbidden_script_files_present(repo_root)?);
    errors.extend(scan_forbidden_legacy_references(repo_root)?);

    let readme = read_utf8(&repo_root.join("README.md"))?;
    let readme_cn = read_utf8(&repo_root.join("README_CN.md"))?;
    let claude = read_utf8(&repo_root.join("CLAUDE.md"))?;
    for (name, text) in [
        ("README.md", readme),
        ("README_CN.md", readme_cn),
        ("CLAUDE.md", claude),
    ] {
        for needle in [
            "wiki/drafts",
            "wiki/live",
            "wiki/briefings",
            "outputs/reviews",
        ] {
            if !text.contains(needle) {
                errors.push(format!("{name} is missing contract term {needle}."));
            }
        }
    }

    let docs_overview = read_utf8(&repo_root.join("docs").join("skills").join("overview.md"))?;
    let docs_overview_zh = read_utf8(
        &repo_root
            .join("docs")
            .join("zh")
            .join("skills")
            .join("overview.md"),
    )?;
    if !docs_overview.contains("one package entry skill and six operational skills") {
        errors.push("docs/skills/overview.md must describe the 1+6 core bundle shape.".to_string());
    }
    if !docs_overview_zh.contains("1 个入口技能和 6 个操作技能") {
        errors.push(
            "docs/zh/skills/overview.md must describe the 1+6 core bundle shape.".to_string(),
        );
    }

    let workflow_overview =
        read_utf8(&repo_root.join("docs").join("workflow").join("overview.md"))?;
    let workflow_overview_zh = read_utf8(
        &repo_root
            .join("docs")
            .join("zh")
            .join("workflow")
            .join("overview.md"),
    )?;
    if workflow_overview.contains("```mermaid") || workflow_overview_zh.contains("```mermaid") {
        errors.push("Workflow overview pages must not use Mermaid fences anymore.".to_string());
    }
    if !workflow_overview.contains("<WorkflowLifecycleDiagram")
        || !workflow_overview_zh.contains("<WorkflowLifecycleDiagram")
    {
        errors.push(
            "Workflow overview pages must render the native WorkflowLifecycleDiagram component."
                .to_string(),
        );
    }
    if !repo_root
        .join("docs")
        .join(".vitepress")
        .join("theme")
        .join("components")
        .join("WorkflowLifecycleDiagram.vue")
        .exists()
    {
        errors.push(
            "WorkflowLifecycleDiagram.vue must exist under docs/.vitepress/theme/components/."
                .to_string(),
        );
    }

    Ok(json!({
        "status": if errors.is_empty() { "ok" } else { "error" },
        "checks": skill_paths.len() + KB_INIT_REQUIRED_ASSETS.len() + 6,
        "errors": errors,
    }))
}
