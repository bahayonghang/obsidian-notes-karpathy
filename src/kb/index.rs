use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::sync::Arc;

use crate::kb::markdown::{MarkdownRecord, list_field, strip_link_alias};
use crate::support::collapse_posix;

pub type RecordByPath = HashMap<String, Arc<MarkdownRecord>>;
pub type RecordByBasename = HashMap<String, Vec<Arc<MarkdownRecord>>>;

pub fn registry_for_records(records: &[Arc<MarkdownRecord>]) -> (RecordByPath, RecordByBasename) {
    let mut by_path: RecordByPath = HashMap::new();
    let mut by_basename: RecordByBasename = HashMap::new();
    for record in records {
        by_path.insert(record.path_no_ext(), Arc::clone(record));
        by_basename
            .entry(record.basename())
            .or_default()
            .push(Arc::clone(record));
    }
    (by_path, by_basename)
}

pub fn resolve_target(
    source_record: &MarkdownRecord,
    target: &str,
    by_path: &RecordByPath,
    by_basename: &RecordByBasename,
) -> Vec<Arc<MarkdownRecord>> {
    let stripped = strip_link_alias(target);
    if stripped.is_empty() {
        return Vec::new();
    }

    let mut candidates: Vec<Arc<MarkdownRecord>> = Vec::new();
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
            candidates.push(Arc::clone(record));
        }
    } else if let Some(record) = by_path.get(&normalized) {
        candidates.push(Arc::clone(record));
    } else {
        let basename = Path::new(&normalized)
            .file_name()
            .map(|value| value.to_string_lossy().into_owned())
            .unwrap_or_default();
        if let Some(values) = by_basename.get(&basename) {
            candidates.extend(values.iter().map(Arc::clone));
        }
    }

    let mut deduped: BTreeMap<String, Arc<MarkdownRecord>> = BTreeMap::new();
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
    if let Some(serde_json::Value::String(title)) = record.frontmatter.get("title")
        && !title.trim().is_empty()
    {
        identities.push(title.trim().to_string());
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
