use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use anyhow::{Context, Result, anyhow};
use serde_json::{Value, json};

use crate::audit_log;
use crate::common::{
    MarkdownRecord, json_string, list_field, normalize_path_string, write_markdown,
};
use crate::layout::collect_markdown_records;

const DRAFT_ROOT: &str = "wiki/drafts/summaries/writeback";
const PACKAGE_ROOT: &str = "wiki/drafts/indices/packages/writeback";
const ARCHIVE_ROOTS: [&str; 2] = ["outputs/qa/", "outputs/content/"];

struct ScaffoldPlan {
    source_path: String,
    draft_path: String,
    package_path: String,
    grounding: Vec<String>,
    draft_content: Vec<String>,
    package_content: Vec<String>,
}

fn scalar_lower(record: &MarkdownRecord, key: &str) -> Option<String> {
    record
        .frontmatter
        .get(key)
        .and_then(Value::as_str)
        .map(|value| value.trim().to_lowercase())
        .filter(|value| !value.is_empty())
}

fn grounding_refs(record: &MarkdownRecord) -> Vec<String> {
    let mut refs = list_field(&record.frontmatter, "source_live_pages");
    if refs.is_empty() {
        refs = list_field(&record.frontmatter, "sources")
            .into_iter()
            .filter(|value| {
                let inner = value.trim().trim_start_matches("[[");
                inner.starts_with("wiki/live/") || inner.starts_with("raw/")
            })
            .collect();
    }
    refs.into_iter()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect()
}

fn artifact_title(record: &MarkdownRecord, slug: &str) -> String {
    for key in ["question", "title"] {
        if let Some(Value::String(value)) = record.frontmatter.get(key)
            && !value.trim().is_empty()
        {
            return value.trim().to_string();
        }
    }
    slug.to_string()
}

fn push_ref_list(lines: &mut Vec<String>, key: &str, values: &[String]) {
    lines.push(format!("{key}:"));
    for value in values {
        lines.push(format!("  - {}", json_string(value)));
    }
}

fn scaffold_plan(record: &MarkdownRecord) -> ScaffoldPlan {
    let slug = record.basename();
    let source_ref = format!("[[{}]]", record.path_no_ext());
    let draft_path = format!("{DRAFT_ROOT}/{slug}.md");
    let package_path = format!("{PACKAGE_ROOT}/{slug}.md");
    let draft_ref = format!("[[{}]]", draft_path.trim_end_matches(".md"));
    let package_ref = format!("[[{}]]", package_path.trim_end_matches(".md"));
    let grounding = grounding_refs(record);
    let candidates = list_field(&record.frontmatter, "writeback_candidates");
    let title = format!("Writeback: {}", artifact_title(record, &slug));
    let draft_id = format!("writeback--{slug}");

    let mut draft_lines = vec![
        "---".to_string(),
        format!("title: {}", json_string(&title)),
        format!("draft_id: {}", json_string(&draft_id)),
    ];
    push_ref_list(&mut draft_lines, "compiled_from", &grounding);
    push_ref_list(&mut draft_lines, "capture_sources", &grounding);
    draft_lines.push(format!("writeback_source: {}", json_string(&source_ref)));
    push_ref_list(&mut draft_lines, "writeback_candidates", &candidates);
    draft_lines.extend([
        r#"review_state: "pending""#.to_string(),
        r#"review_score: "0.70""#.to_string(),
        "blocking_flags: []".to_string(),
        r#"evidence_coverage: "0.50""#.to_string(),
        r#"uncertainty_level: "medium""#.to_string(),
        r#"promotion_target: "semantic""#.to_string(),
        format!("review_package_meta: {}", json_string(&package_ref)),
        "---".to_string(),
        String::new(),
        format!("# {title}"),
        String::new(),
        "## Writeback Provenance".to_string(),
        String::new(),
        format!("- Triggered by: {source_ref} (archived artifact - trigger, not truth)"),
        "- Grounded in:".to_string(),
    ]);
    for value in &grounding {
        draft_lines.push(format!("  - {value}"));
    }
    draft_lines.extend([
        String::new(),
        "## Core Conclusions".to_string(),
        String::new(),
        "- No core conclusions distilled yet.".to_string(),
        String::new(),
        "## Key Evidence".to_string(),
        String::new(),
        "- No key evidence extracted yet.".to_string(),
        String::new(),
        "## Writeback Candidates".to_string(),
        String::new(),
    ]);
    for value in &candidates {
        draft_lines.push(format!("- {value}"));
    }

    let package_title = format!("Review Package: {draft_id}");
    let mut package_lines = vec![
        "---".to_string(),
        format!("title: {}", json_string(&package_title)),
        format!("source_id: {}", json_string(&draft_id)),
        r#"review_state: "pending""#.to_string(),
        format!("summary_path: {}", json_string(&draft_ref)),
        r#"promotion_target: "semantic""#.to_string(),
        format!("writeback_source: {}", json_string(&source_ref)),
        "---".to_string(),
        String::new(),
        format!("# {package_title}"),
        String::new(),
        format!("- summary: {draft_ref}"),
        format!("- writeback_source: {source_ref}"),
        "- followup_route: `draft`".to_string(),
    ];
    package_lines.push(String::new());
    package_lines.push("## Writeback Candidates".to_string());
    package_lines.push(String::new());
    for value in &candidates {
        package_lines.push(format!("- {value}"));
    }

    ScaffoldPlan {
        source_path: record.path.clone(),
        draft_path,
        package_path,
        grounding,
        draft_content: draft_lines,
        package_content: package_lines,
    }
}

fn classify_record(
    vault_root: &Path,
    record: &MarkdownRecord,
    planned_paths: &mut BTreeSet<String>,
) -> (Value, Option<ScaffoldPlan>) {
    let path = record.path.clone();
    let skip = |reason: &str| json!({ "path": path, "status": "skipped", "reason": reason });

    match scalar_lower(record, "writeback_status") {
        Some(status) if status == "pending" => {}
        Some(status) => return (skip(&format!("writeback_status_{status}")), None),
        None => return (skip("writeback_status_missing"), None),
    }
    match scalar_lower(record, "followup_route") {
        Some(route) if route == "draft" => {}
        Some(route) => return (skip(&format!("followup_route_{route}")), None),
        None => return (skip("followup_route_missing"), None),
    }
    if list_field(&record.frontmatter, "writeback_candidates").is_empty() {
        return (skip("no_writeback_candidates"), None);
    }
    let plan = scaffold_plan(record);
    if plan.grounding.is_empty() {
        return (skip("missing_live_grounding"), None);
    }
    if !planned_paths.insert(plan.draft_path.clone()) {
        return (skip("draft_path_conflict"), None);
    }
    if vault_root.join(&plan.draft_path).exists() || vault_root.join(&plan.package_path).exists() {
        return (skip("draft_already_exists"), None);
    }
    let item = json!({
        "path": record.path,
        "status": "eligible",
        "draft_path": plan.draft_path,
        "package_path": plan.package_path,
        "grounding": plan.grounding,
    });
    (item, Some(plan))
}

fn compute_writeback_plans(vault_root: &Path) -> Result<(Vec<Value>, Vec<ScaffoldPlan>)> {
    let records = collect_markdown_records(vault_root)?;
    let mut archived: Vec<&MarkdownRecord> = records
        .iter()
        .map(|record| record.as_ref())
        .filter(|record| {
            ARCHIVE_ROOTS
                .iter()
                .any(|root| record.path.starts_with(root))
        })
        .collect();
    archived.sort_by(|a, b| a.path.cmp(&b.path));

    let mut items = Vec::new();
    let mut plans = Vec::new();
    let mut planned_paths = BTreeSet::new();
    for record in archived {
        let (item, plan) = classify_record(vault_root, record, &mut planned_paths);
        items.push(item);
        if let Some(plan) = plan {
            plans.push(plan);
        }
    }
    Ok((items, plans))
}

pub fn build_writeback_scaffolds(vault_root: &Path) -> Result<Value> {
    let (items, plans) = compute_writeback_plans(vault_root)?;
    let skipped = items.len() - plans.len();
    let counts = BTreeMap::from([
        ("eligible".to_string(), plans.len()),
        ("skipped".to_string(), skipped),
        ("total".to_string(), items.len()),
    ]);
    let payload = crate::payload::CompileWriteback {
        action: "compile_writeback".to_string(),
        vault_root: normalize_path_string(vault_root.to_string_lossy().as_ref()),
        write: false,
        counts,
        items,
        written_paths: Vec::new(),
        advanced_sources: Vec::new(),
    };
    Ok(serde_json::to_value(&payload)?)
}

/// 外科式推进归档产物的 `writeback_status`，其余内容逐字节保留。
///
/// 仅替换 frontmatter 里的 `writeback_status:` 行，并在其后补
/// `writeback_draft:` 指针（键已存在则不重复插入）。
fn advance_writeback_status(vault_root: &Path, rel_path: &str, draft_ref: &str) -> Result<Value> {
    let path = vault_root.join(rel_path);
    let text =
        fs::read_to_string(&path).with_context(|| format!("read archive {}", path.display()))?;
    let mut lines: Vec<String> = text.split('\n').map(ToString::to_string).collect();

    let mut fence_count = 0usize;
    let mut status_line = None;
    let mut has_draft_pointer = false;
    for (index, line) in lines.iter().enumerate() {
        if line.trim_end_matches('\r') == "---" {
            fence_count += 1;
            if fence_count == 2 {
                break;
            }
            continue;
        }
        if fence_count != 1 {
            continue;
        }
        if line.starts_with("writeback_status:") && status_line.is_none() {
            status_line = Some(index);
        }
        if line.starts_with("writeback_draft:") {
            has_draft_pointer = true;
        }
    }
    let status_index = status_line
        .ok_or_else(|| anyhow!("no writeback_status line in frontmatter of {rel_path}"))?;

    let previous = lines[status_index]
        .trim_end_matches('\r')
        .trim_start_matches("writeback_status:")
        .trim()
        .trim_matches('"')
        .to_string();
    let line_ending = if lines[status_index].ends_with('\r') {
        "\r"
    } else {
        ""
    };
    lines[status_index] = format!(r#"writeback_status: "drafted"{line_ending}"#);
    if !has_draft_pointer {
        lines.insert(
            status_index + 1,
            format!("writeback_draft: {}{line_ending}", json_string(draft_ref)),
        );
    }
    fs::write(&path, lines.join("\n"))
        .with_context(|| format!("write archive {}", path.display()))?;
    Ok(json!({
        "path": rel_path,
        "from": previous,
        "to": "drafted",
        "draft": draft_ref,
    }))
}

pub fn write_writeback_scaffolds(vault_root: &Path) -> Result<(Value, Value)> {
    let (_, plans) = compute_writeback_plans(vault_root)?;
    let mut written_paths = Vec::new();
    let mut advanced_sources = Vec::new();
    for plan in &plans {
        let draft_path = vault_root.join(&plan.draft_path);
        write_markdown(&draft_path, &plan.draft_content)?;
        written_paths.push(plan.draft_path.clone());
        let package_path = vault_root.join(&plan.package_path);
        write_markdown(&package_path, &plan.package_content)?;
        written_paths.push(plan.package_path.clone());
        let draft_ref = format!("[[{}]]", plan.draft_path.trim_end_matches(".md"));
        advanced_sources.push(advance_writeback_status(
            vault_root,
            &plan.source_path,
            &draft_ref,
        )?);
    }
    written_paths.sort();
    if !plans.is_empty() {
        audit_log::append_event(
            vault_root,
            "compile_writeback",
            &json!({
                "written_paths": written_paths,
                "advanced_sources": advanced_sources,
                "eligible_count": plans.len(),
            }),
        )?;
    }
    Ok((json!(written_paths), json!(advanced_sources)))
}
