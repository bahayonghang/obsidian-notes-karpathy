use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use serde_json::{Value, json};

use crate::common::{MarkdownRecord, normalize_identity, record_identities, slugify_identity};

pub fn duplicate_identity_issues(records: &[Arc<MarkdownRecord>]) -> Vec<Value> {
    let mut identity_map: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut identity_type: BTreeMap<String, String> = BTreeMap::new();
    for record in records {
        if !matches!(record.kind.as_str(), "concept" | "entity") || record.path.contains("/drafts/")
        {
            continue;
        }
        for identity in record_identities(record) {
            let normalized = normalize_identity(&identity);
            if normalized.is_empty() {
                continue;
            }
            identity_map
                .entry(normalized.clone())
                .or_default()
                .insert(record.path.clone());
            identity_type.insert(normalized, record.kind.clone());
        }
    }
    let mut issues = Vec::new();
    for (normalized, paths) in identity_map {
        if paths.len() < 2 {
            continue;
        }
        issues.push(json!({
            "kind": format!("duplicate_{}", identity_type.get(&normalized).cloned().unwrap_or_else(|| "concept".to_string())),
            "normalized_key": slugify_identity(&normalized),
            "paths": paths.into_iter().collect::<Vec<_>>(),
        }));
    }
    issues
}
