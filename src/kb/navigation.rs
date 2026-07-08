use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde_json::{Value, json};

use crate::audit_log;
use crate::common::{
    MarkdownRecord, list_field, normalize_path_string, parse_datetime, slugify_identity,
    strip_link_alias,
};
use crate::layout::collect_markdown_records;
use crate::query::live_records;

const MANAGED_BEGIN: &str = "<!-- onkb:indices:begin -->";
const MANAGED_END: &str = "<!-- onkb:indices:end -->";
const INDEX_HOME_PATH: &str = "wiki/index.md";
const RECENT_PATH: &str = "wiki/live/indices/RECENT.md";
const EDITORIAL_PRIORITIES_PATH: &str = "wiki/live/indices/EDITORIAL-PRIORITIES.md";
const RECENT_LIMIT: usize = 20;
const INDEX_FAMILY_LINKS: [&str; 5] = [
    "wiki/live/indices/CONCEPTS",
    "wiki/live/indices/SOURCES",
    "wiki/live/indices/TOPICS",
    "wiki/live/indices/RECENT",
    "wiki/live/indices/EDITORIAL-PRIORITIES",
];
const LIVE_PAGE_FAMILIES: [(&str, &str); 7] = [
    ("comparisons", "comparison"),
    ("concepts", "concept"),
    ("entities", "entity"),
    ("overviews", "overview"),
    ("procedures", "procedure"),
    ("summaries", "summary"),
    ("topics", "topic"),
];

struct NavigationState {
    files: Vec<(String, String)>,
    index_home_block: String,
    unlisted_count: usize,
}

fn normalize_newlines(text: &str) -> String {
    text.replace("\r\n", "\n")
}

fn page_title(record: &MarkdownRecord) -> String {
    record
        .frontmatter
        .get("title")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
        .unwrap_or_else(|| record.basename())
}

fn canonical_slug(record: &MarkdownRecord) -> String {
    let canonical = record
        .frontmatter
        .get("canonical_name")
        .and_then(Value::as_str)
        .or_else(|| record.frontmatter.get("concept_id").and_then(Value::as_str))
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| record.basename());
    slugify_identity(&canonical)
}

fn provenance_labels(record: &MarkdownRecord) -> Vec<String> {
    let mut sources = list_field(&record.frontmatter, "compiled_from");
    if sources.is_empty() {
        sources = list_field(&record.frontmatter, "capture_sources");
    }
    sources
        .iter()
        .map(|value| strip_link_alias(value))
        .filter(|value| !value.is_empty())
        .collect()
}

fn index_document(title: &str, intro: &str, body_lines: &[String]) -> String {
    let mut lines = vec![
        "---".to_string(),
        format!("title: \"{title}\""),
        "managed_by: \"onkb review indices\"".to_string(),
        "---".to_string(),
        String::new(),
        format!("# {title}"),
        String::new(),
        intro.to_string(),
        String::new(),
    ];
    lines.extend_from_slice(body_lines);
    format!("{}\n", lines.join("\n"))
}

fn compute_navigation_state(vault_root: &Path) -> Result<NavigationState> {
    let records = collect_markdown_records(vault_root)?;
    let live = live_records(&records);
    let pages: Vec<Arc<MarkdownRecord>> = live
        .iter()
        .filter(|record| !record.path.starts_with("wiki/live/indices/"))
        .cloned()
        .collect();

    let mut family_counts: BTreeMap<String, usize> = LIVE_PAGE_FAMILIES
        .iter()
        .map(|(label, _)| (label.to_string(), 0))
        .collect();
    let mut concept_rows: Vec<(String, String, String)> = Vec::new();
    let mut source_rows: Vec<(String, String)> = Vec::new();
    let mut topic_rows: Vec<(String, String)> = Vec::new();
    let mut recent_rows: Vec<(DateTime<Utc>, String, String)> = Vec::new();
    let mut unlisted_count = 0usize;

    for record in &pages {
        if let Some((label, _)) = LIVE_PAGE_FAMILIES
            .iter()
            .find(|(_, kind)| record.kind == *kind)
        {
            *family_counts.entry(label.to_string()).or_insert(0) += 1;
        }

        let link = record.path_no_ext();
        let title = page_title(record);
        match record.kind.as_str() {
            "concept" => {
                let summary = record
                    .frontmatter
                    .get("summary")
                    .and_then(Value::as_str)
                    .map(str::trim)
                    .filter(|value| !value.is_empty());
                let line = match summary {
                    Some(summary) => format!("- [[{link}]] | {title} | {summary}"),
                    None => format!("- [[{link}]] | {title}"),
                };
                concept_rows.push((canonical_slug(record), record.path.clone(), line));
            }
            "summary" => {
                let provenance = provenance_labels(record);
                let line = if provenance.is_empty() {
                    format!("- [[{link}]] | {title}")
                } else {
                    let joined = provenance
                        .iter()
                        .map(|value| format!("`{value}`"))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("- [[{link}]] | {title} | from: {joined}")
                };
                source_rows.push((record.path.clone(), line));
            }
            "topic" => {
                topic_rows.push((record.path.clone(), format!("- [[{link}]] | {title}")));
            }
            _ => {}
        }

        let approved = record.frontmatter.get("approved_at");
        let reviewed = record.frontmatter.get("last_reviewed_at");
        if let Some(instant) = parse_datetime(approved) {
            let raw = approved.and_then(Value::as_str).unwrap_or_default().trim();
            recent_rows.push((
                instant,
                record.path.clone(),
                format!("- [[{link}]] | {title} | approved: {raw}"),
            ));
        } else if let Some(instant) = parse_datetime(reviewed) {
            let raw = reviewed.and_then(Value::as_str).unwrap_or_default().trim();
            recent_rows.push((
                instant,
                record.path.clone(),
                format!("- [[{link}]] | {title} | reviewed: {raw}"),
            ));
        } else {
            unlisted_count += 1;
        }
    }

    concept_rows.sort();
    source_rows.sort();
    topic_rows.sort();
    recent_rows.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    recent_rows.truncate(RECENT_LIMIT);

    let mut index_body = vec!["## Index Family".to_string(), String::new()];
    for link in INDEX_FAMILY_LINKS {
        index_body.push(format!("- [[{link}]]"));
    }
    index_body.push(String::new());
    index_body.push("## Approved Page Counts".to_string());
    index_body.push(String::new());
    for (label, count) in &family_counts {
        index_body.push(format!("- {label}: {count}"));
    }

    let concept_lines = if concept_rows.is_empty() {
        vec!["- No approved concept pages yet.".to_string()]
    } else {
        concept_rows.into_iter().map(|(_, _, line)| line).collect()
    };
    let source_lines = if source_rows.is_empty() {
        vec!["- No approved source summaries yet.".to_string()]
    } else {
        source_rows.into_iter().map(|(_, line)| line).collect()
    };
    let topic_lines = if topic_rows.is_empty() {
        vec!["- No approved topic pages yet.".to_string()]
    } else {
        topic_rows.into_iter().map(|(_, line)| line).collect()
    };
    let recent_lines = if recent_rows.is_empty() {
        vec!["- No approved live pages recorded yet.".to_string()]
    } else {
        recent_rows.into_iter().map(|(_, _, line)| line).collect()
    };

    let files = vec![
        (
            "wiki/live/indices/INDEX.md".to_string(),
            index_document(
                "Approved Knowledge Index",
                "Use this page as the browse-layer home for approved knowledge.",
                &index_body,
            ),
        ),
        (
            "wiki/live/indices/CONCEPTS.md".to_string(),
            index_document(
                "Approved Concepts",
                "Approved concept pages under `wiki/live/concepts/`.",
                &concept_lines,
            ),
        ),
        (
            "wiki/live/indices/SOURCES.md".to_string(),
            index_document(
                "Approved Sources",
                "Approved source summaries under `wiki/live/summaries/`.",
                &source_lines,
            ),
        ),
        (
            "wiki/live/indices/TOPICS.md".to_string(),
            index_document(
                "Approved Topics",
                "Approved topic pages under `wiki/live/topics/`.",
                &topic_lines,
            ),
        ),
        (
            RECENT_PATH.to_string(),
            index_document(
                "Recently Approved Changes",
                "Most recently approved live pages, newest first (top 20).",
                &recent_lines,
            ),
        ),
    ];

    let mut block_lines = vec![
        String::new(),
        "## Approved Live Indices".to_string(),
        String::new(),
        "- [[wiki/live/indices/INDEX]]".to_string(),
    ];
    for link in INDEX_FAMILY_LINKS {
        block_lines.push(format!("- [[{link}]]"));
    }
    block_lines.push(String::new());
    block_lines.push(format!("Approved live pages: {}", pages.len()));
    let index_home_block = format!("{}\n", block_lines.join("\n"));

    Ok(NavigationState {
        files,
        index_home_block,
        unlisted_count,
    })
}

fn file_status(vault_root: &Path, rel_path: &str, generated: &str) -> Result<&'static str> {
    let path = vault_root.join(rel_path);
    if !path.exists() {
        return Ok("missing");
    }
    let existing =
        fs::read_to_string(&path).with_context(|| format!("read index {}", path.display()))?;
    if normalize_newlines(&existing) == generated {
        Ok("in_sync")
    } else {
        Ok("drifted")
    }
}

fn managed_region(text: &str) -> Option<(usize, usize)> {
    let begin = text.find(MANAGED_BEGIN)?;
    let content_start = begin + MANAGED_BEGIN.len();
    let end = content_start + text[content_start..].find(MANAGED_END)?;
    Some((content_start, end))
}

fn index_home_state(
    vault_root: &Path,
    generated_block: &str,
) -> Result<(&'static str, Option<String>)> {
    let path = vault_root.join(INDEX_HOME_PATH);
    if !path.exists() {
        return Ok(("unmanaged", None));
    }
    let existing = normalize_newlines(
        &fs::read_to_string(&path)
            .with_context(|| format!("read index home {}", path.display()))?,
    );
    let Some((start, end)) = managed_region(&existing) else {
        return Ok(("unmanaged", None));
    };
    let generated_inner = format!("\n{generated_block}");
    if existing[start..end] == generated_inner {
        return Ok(("in_sync", None));
    }
    let updated = format!(
        "{}{}{}",
        &existing[..start],
        generated_inner,
        &existing[end..]
    );
    Ok(("drifted", Some(updated)))
}

pub fn build_navigation_indices(vault_root: &Path) -> Result<Value> {
    let state = compute_navigation_state(vault_root)?;
    let mut files: BTreeMap<String, Value> = BTreeMap::new();
    let mut drift_count = 0usize;
    for (rel_path, content) in &state.files {
        let status = file_status(vault_root, rel_path, content)?;
        if status != "in_sync" {
            drift_count += 1;
        }
        let mut entry = json!({ "status": status });
        if rel_path == RECENT_PATH {
            entry["unlisted_count"] = json!(state.unlisted_count);
        }
        files.insert(rel_path.clone(), entry);
    }
    files.insert(
        EDITORIAL_PRIORITIES_PATH.to_string(),
        json!({ "status": "excluded" }),
    );
    let (home_status, _) = index_home_state(vault_root, &state.index_home_block)?;
    if home_status == "drifted" {
        drift_count += 1;
    }
    files.insert(
        INDEX_HOME_PATH.to_string(),
        json!({ "status": home_status }),
    );

    let payload = crate::payload::NavigationIndices {
        action: "review_indices".to_string(),
        vault: normalize_path_string(vault_root.to_string_lossy().as_ref()),
        write: false,
        files,
        drift_count,
        written_paths: Vec::new(),
    };
    Ok(serde_json::to_value(&payload)?)
}

pub fn write_navigation_indices(vault_root: &Path, payload: &Value) -> Result<Vec<String>> {
    let state = compute_navigation_state(vault_root)?;
    let mut written_paths = Vec::new();
    for (rel_path, content) in &state.files {
        if file_status(vault_root, rel_path, content)? == "in_sync" {
            continue;
        }
        let path = vault_root.join(rel_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
        }
        fs::write(&path, content).with_context(|| format!("write {}", path.display()))?;
        written_paths.push(rel_path.clone());
    }
    if let ("drifted", Some(updated)) = index_home_state(vault_root, &state.index_home_block)? {
        let path = vault_root.join(INDEX_HOME_PATH);
        fs::write(&path, updated).with_context(|| format!("write {}", path.display()))?;
        written_paths.push(INDEX_HOME_PATH.to_string());
    }
    written_paths.sort();
    audit_log::append_event(
        vault_root,
        "rebuild_navigation_indices",
        &json!({
            "written_paths": written_paths,
            "drift_count": payload.get("drift_count").and_then(Value::as_u64).unwrap_or_default(),
        }),
    )?;
    Ok(written_paths)
}
