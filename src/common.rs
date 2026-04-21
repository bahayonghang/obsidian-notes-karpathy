use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::{DateTime, Timelike, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;
use serde_json::{Map, Number, Value};
use walkdir::WalkDir;

pub static WIKILINK_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\[\[([^\]]+)\]\]").expect("valid wikilink regex"));
pub static MARKDOWN_LINK_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\[[^\]]+\]\(([^)]+)\)").expect("valid markdown link regex"));
pub static ALIAS_WIKILINK_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\[\[[^|\]]+\|[^\]]+\]\]").expect("valid alias wikilink regex"));
pub static TABLE_LINE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\s*\|.*\|\s*$").expect("valid table regex"));
pub static FRONTMATTER_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)^---\n(.*?)\n---\n?(.*)$").expect("valid frontmatter regex"));

#[derive(Clone, Debug, Serialize)]
pub struct MarkdownRecord {
    pub path: String,
    pub kind: String,
    #[serde(skip_serializing)]
    pub text: String,
    #[serde(skip_serializing)]
    pub body: String,
    #[serde(skip_serializing)]
    pub frontmatter: Map<String, Value>,
}

impl MarkdownRecord {
    pub fn path_no_ext(&self) -> String {
        self.path
            .strip_suffix(".md")
            .unwrap_or(&self.path)
            .to_string()
    }

    pub fn basename(&self) -> String {
        Path::new(&self.path_no_ext())
            .file_name()
            .map(|value| value.to_string_lossy().into_owned())
            .unwrap_or_default()
    }
}

pub fn load_markdown(path: &Path, vault_root: Option<&Path>) -> Result<MarkdownRecord> {
    let rel_path = match vault_root {
        Some(root) if path.is_absolute() => relative_posix(path, root),
        _ => normalize_path_string(path.to_string_lossy().as_ref()),
    };
    let text =
        fs::read_to_string(path).with_context(|| format!("read markdown {}", path.display()))?;
    let (frontmatter, body) = parse_frontmatter(&text);
    Ok(MarkdownRecord {
        path: rel_path.clone(),
        kind: classify_markdown_path(&rel_path),
        text,
        body,
        frontmatter,
    })
}

pub fn classify_markdown_path(rel_path: &str) -> String {
    let normalized = rel_path.replace('\\', "/");
    if normalized == "MEMORY.md" {
        return "memory".to_string();
    }
    if normalized.starts_with("wiki/briefings/") {
        return "briefing".to_string();
    }
    if normalized.starts_with("outputs/reviews/") {
        return "review".to_string();
    }
    if normalized.starts_with("outputs/content/") {
        return "content_output".to_string();
    }
    if normalized.starts_with("outputs/episodes/") {
        return "episode".to_string();
    }
    if normalized.starts_with("wiki/live/concepts/")
        || normalized.starts_with("wiki/drafts/concepts/")
    {
        return "concept".to_string();
    }
    if normalized.starts_with("wiki/live/entities/")
        || normalized.starts_with("wiki/drafts/entities/")
    {
        return "entity".to_string();
    }
    if normalized.starts_with("wiki/live/procedures/")
        || normalized.starts_with("wiki/drafts/procedures/")
    {
        return "procedure".to_string();
    }
    if normalized.starts_with("wiki/live/topics/") || normalized.starts_with("wiki/drafts/topics/")
    {
        return "topic".to_string();
    }
    if normalized.starts_with("wiki/live/overviews/")
        || normalized.starts_with("wiki/drafts/overviews/")
    {
        return "overview".to_string();
    }
    if normalized.starts_with("wiki/live/comparisons/")
        || normalized.starts_with("wiki/drafts/comparisons/")
    {
        return "comparison".to_string();
    }
    if normalized.starts_with("wiki/live/summaries/")
        || normalized.starts_with("wiki/drafts/summaries/")
    {
        return "summary".to_string();
    }
    if normalized.starts_with("wiki/live/indices/")
        || normalized.starts_with("wiki/drafts/indices/")
        || normalized.starts_with("wiki/indices/")
        || normalized.starts_with("wiki/indexes/")
    {
        return "index".to_string();
    }
    if normalized.starts_with("outputs/qa/") {
        return "qa".to_string();
    }
    if normalized.starts_with("outputs/reports/") {
        return "report_output".to_string();
    }
    if normalized.starts_with("outputs/slides/") {
        return "slide_output".to_string();
    }
    if normalized.starts_with("outputs/charts/") {
        return "chart_output".to_string();
    }
    if normalized.starts_with("raw/") {
        return "raw".to_string();
    }
    "other".to_string()
}

pub fn parse_frontmatter(text: &str) -> (Map<String, Value>, String) {
    if let Some(captures) = FRONTMATTER_RE.captures(text) {
        let raw_frontmatter = captures
            .get(1)
            .map(|value| value.as_str())
            .unwrap_or_default();
        let body = captures
            .get(2)
            .map(|value| value.as_str())
            .unwrap_or_default()
            .to_string();
        return (parse_simple_yaml(raw_frontmatter), body);
    }
    (Map::new(), text.to_string())
}

pub fn parse_simple_yaml(raw_text: &str) -> Map<String, Value> {
    let mut data = Map::new();
    let mut current_key: Option<String> = None;

    for raw_line in raw_text.lines() {
        let line = raw_line.trim_end();
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }

        if let Some(stripped) = line.strip_prefix("  - ") {
            if let Some(key) = current_key.clone() {
                let value = parse_scalar(stripped.trim());
                match data.get_mut(&key) {
                    Some(Value::Array(items)) => items.push(value),
                    _ => {
                        data.insert(key, Value::Array(vec![value]));
                    }
                }
            }
            continue;
        }

        let Some((key_raw, value_raw)) = line.split_once(':') else {
            current_key = None;
            continue;
        };
        let key = key_raw.trim().to_string();
        let value = value_raw.trim();

        if value.is_empty() {
            data.insert(key.clone(), Value::Array(Vec::new()));
            current_key = Some(key);
            continue;
        }

        current_key = None;
        data.insert(key, parse_scalar(value));
    }

    data
}

pub fn parse_scalar(value: &str) -> Value {
    let stripped = value.trim();
    if matches!(stripped, "[]" | "[ ]") {
        return Value::Array(Vec::new());
    }

    if stripped.starts_with('[') && stripped.ends_with(']') {
        let inner = stripped[1..stripped.len() - 1].trim();
        if inner.is_empty() {
            return Value::Array(Vec::new());
        }
        let parts = inner
            .split(',')
            .map(|item| parse_scalar(item.trim()))
            .collect::<Vec<_>>();
        return Value::Array(parts);
    }

    if stripped.starts_with('"') && stripped.ends_with('"') && stripped.len() >= 2 {
        return Value::String(stripped[1..stripped.len() - 1].to_string());
    }
    if stripped.starts_with('\'') && stripped.ends_with('\'') && stripped.len() >= 2 {
        return Value::String(stripped[1..stripped.len() - 1].to_string());
    }

    if let Ok(number) = stripped.parse::<i64>() {
        return Value::Number(number.into());
    }
    if let Ok(number) = stripped.parse::<f64>() {
        if let Some(value) = Number::from_f64(number) {
            return Value::Number(value);
        }
    }
    if stripped == "true" {
        return Value::Bool(true);
    }
    if stripped == "false" {
        return Value::Bool(false);
    }

    Value::String(stripped.to_string())
}

pub fn collapse_posix(path_like: &str) -> String {
    let mut parts: Vec<String> = Vec::new();
    for part in path_like.replace('\\', "/").split('/') {
        if part.is_empty() || part == "." {
            continue;
        }
        if part == ".." {
            parts.pop();
            continue;
        }
        parts.push(part.to_string());
    }
    parts.join("/")
}

pub fn parse_number(value: Option<&Value>) -> Option<f64> {
    match value {
        None | Some(Value::Null) => None,
        Some(Value::Number(number)) => number.as_f64(),
        Some(Value::String(text)) => {
            let stripped = text.trim();
            if stripped.is_empty() {
                None
            } else {
                stripped.parse::<f64>().ok()
            }
        }
        _ => None,
    }
}

pub fn list_field(frontmatter: &Map<String, Value>, key: &str) -> Vec<String> {
    match frontmatter.get(key) {
        Some(Value::Array(items)) => items
            .iter()
            .filter_map(|item| match item {
                Value::String(text) if !text.trim().is_empty() => Some(text.trim().to_string()),
                Value::Number(number) => Some(number.to_string()),
                Value::Bool(flag) => Some(flag.to_string()),
                _ => None,
            })
            .collect(),
        Some(Value::String(text)) if !text.trim().is_empty() => vec![text.trim().to_string()],
        _ => Vec::new(),
    }
}

pub fn parse_datetime(value: Option<&Value>) -> Option<DateTime<Utc>> {
    let candidate = match value {
        Some(Value::String(text)) if !text.trim().is_empty() => text.trim().to_string(),
        _ => return None,
    };
    let normalized = if candidate.ends_with('Z') {
        format!("{}+00:00", &candidate[..candidate.len() - 1])
    } else if Regex::new(r"^\d{4}-\d{2}-\d{2}$")
        .expect("valid date regex")
        .is_match(&candidate)
    {
        format!("{candidate}T00:00:00+00:00")
    } else {
        candidate
    };
    chrono::DateTime::parse_from_rfc3339(&normalized)
        .ok()
        .map(|value| value.with_timezone(&Utc))
}

pub fn iter_markdown_records(vault_root: &Path, roots: &[&str]) -> Result<Vec<MarkdownRecord>> {
    let mut records = Vec::new();
    for rel_root in roots {
        let root = vault_root.join(rel_root);
        if !root.exists() {
            continue;
        }
        for entry in WalkDir::new(&root).sort_by_file_name() {
            let entry = entry?;
            if !entry.file_type().is_file() {
                continue;
            }
            if entry.path().extension().and_then(|value| value.to_str()) != Some("md") {
                continue;
            }
            let rel = relative_posix(entry.path(), vault_root);
            if rel.split('/').any(|part| part.starts_with('.')) {
                continue;
            }
            records.push(load_markdown(entry.path(), Some(vault_root))?);
        }
    }
    Ok(records)
}

pub fn extract_wikilinks(text: &str) -> Vec<String> {
    WIKILINK_RE
        .captures_iter(text)
        .filter_map(|captures| captures.get(1).map(|value| value.as_str().to_string()))
        .collect()
}

pub fn extract_markdown_links(text: &str) -> Vec<String> {
    MARKDOWN_LINK_RE
        .captures_iter(text)
        .filter_map(|captures| captures.get(1).map(|value| value.as_str().to_string()))
        .collect()
}

pub fn strip_link_alias(target: &str) -> String {
    let mut candidate = target.trim().to_string();
    if candidate.starts_with("[[") && candidate.ends_with("]]") {
        candidate = candidate[2..candidate.len() - 2].trim().to_string();
    }
    candidate = candidate
        .split('|')
        .next()
        .unwrap_or_default()
        .trim()
        .to_string();
    candidate
        .split('#')
        .next()
        .unwrap_or_default()
        .trim()
        .to_string()
}

pub fn registry_for_records(
    records: &[MarkdownRecord],
) -> (
    HashMap<String, MarkdownRecord>,
    HashMap<String, Vec<MarkdownRecord>>,
) {
    let mut by_path = HashMap::new();
    let mut by_basename: HashMap<String, Vec<MarkdownRecord>> = HashMap::new();
    for record in records {
        by_path.insert(record.path_no_ext(), record.clone());
        by_basename
            .entry(record.basename())
            .or_default()
            .push(record.clone());
    }
    (by_path, by_basename)
}

pub fn resolve_target(
    source_record: &MarkdownRecord,
    target: &str,
    by_path: &HashMap<String, MarkdownRecord>,
    by_basename: &HashMap<String, Vec<MarkdownRecord>>,
) -> Vec<MarkdownRecord> {
    let stripped = strip_link_alias(target);
    if stripped.is_empty() {
        return Vec::new();
    }

    let mut candidates: Vec<MarkdownRecord> = Vec::new();
    let normalized = stripped
        .strip_suffix(".md")
        .unwrap_or(&stripped)
        .trim_start_matches('/')
        .to_string();
    if normalized.starts_with("./") || normalized.starts_with("../") {
        let parent = Path::new(&source_record.path)
            .parent()
            .map(|value| value.to_string_lossy().into_owned())
            .unwrap_or_default();
        let joined = collapse_posix(&format!("{parent}/{normalized}"));
        if let Some(record) = by_path.get(&joined) {
            candidates.push(record.clone());
        }
    } else if let Some(record) = by_path.get(&normalized) {
        candidates.push(record.clone());
    } else {
        let basename = Path::new(&normalized)
            .file_name()
            .map(|value| value.to_string_lossy().into_owned())
            .unwrap_or_default();
        if let Some(values) = by_basename.get(&basename) {
            candidates.extend(values.iter().cloned());
        }
    }

    let mut deduped = BTreeMap::new();
    for candidate in candidates {
        deduped.insert(candidate.path_no_ext(), candidate);
    }
    deduped.into_values().collect()
}

pub fn normalize_identity(value: &str) -> String {
    value
        .to_lowercase()
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn slugify_identity(value: &str) -> String {
    normalize_identity(value).replace(' ', "-")
}

pub fn record_identities(record: &MarkdownRecord) -> Vec<String> {
    let mut identities = Vec::new();
    if let Some(Value::String(title)) = record.frontmatter.get("title") {
        if !title.trim().is_empty() {
            identities.push(title.trim().to_string());
        }
    }
    identities.push(record.basename().replace('-', " "));
    identities.push(record.basename());
    for alias in list_field(&record.frontmatter, "aliases") {
        identities.push(alias);
    }
    identities
        .into_iter()
        .filter(|identity| !normalize_identity(identity).is_empty())
        .collect()
}

pub fn relative_posix(path: &Path, root: &Path) -> String {
    match path.strip_prefix(root) {
        Ok(value) => normalize_path_string(value.to_string_lossy().as_ref()),
        Err(_) => normalize_path_string(path.to_string_lossy().as_ref()),
    }
}

pub fn normalize_path_string(value: &str) -> String {
    value.replace('\\', "/")
}

pub fn now_iso() -> String {
    let now = Utc::now();
    now.with_nanosecond(0)
        .unwrap_or(now)
        .to_rfc3339()
        .replace("+00:00", "Z")
}

pub fn write_markdown(path: &Path, lines: &[String]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    fs::write(path, format!("{}\n", lines.join("\n")))
        .with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

pub fn json_string(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"\"".to_string())
}
