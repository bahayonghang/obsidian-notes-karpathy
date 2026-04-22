use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use serde_json::{Value, json};

use crate::common::{
    ALIAS_WIKILINK_RE, MarkdownRecord, TABLE_LINE_RE, extract_wikilinks, registry_for_records,
    resolve_target, strip_link_alias,
};

pub fn alias_wikilink_table_issues(records: &[Arc<MarkdownRecord>]) -> Vec<Value> {
    let mut issues = Vec::new();
    for record in records {
        for (index, line) in record.text.lines().enumerate() {
            if !TABLE_LINE_RE.is_match(line) || !ALIAS_WIKILINK_RE.is_match(line) {
                continue;
            }
            issues.push(json!({
                "kind": "alias_wikilink_in_table",
                "path": record.path,
                "line": index + 1,
                "excerpt": line.trim(),
            }));
        }
    }
    issues
}

pub fn broken_wikilink_issues(records: &[Arc<MarkdownRecord>]) -> Vec<Value> {
    let (by_path, by_basename) = registry_for_records(records);
    let mut issues = Vec::new();
    let mut seen = BTreeSet::new();
    for record in records {
        for target in extract_wikilinks(&record.text) {
            let stripped = strip_link_alias(&target);
            let key = format!("{}::{stripped}", record.path);
            if !seen.insert(key) {
                continue;
            }
            if resolve_target(record, &stripped, &by_path, &by_basename).is_empty() {
                issues.push(json!({
                    "kind": "broken_wikilink",
                    "path": record.path,
                    "target": stripped,
                }));
            }
        }
    }
    issues
}

pub fn orphan_page_issues(records: &[Arc<MarkdownRecord>]) -> Vec<Value> {
    let (by_path, by_basename) = registry_for_records(records);
    let mut inbound: BTreeMap<String, i64> = BTreeMap::new();
    let trackable = records
        .iter()
        .filter(|record| {
            matches!(
                record.kind.as_str(),
                "topic" | "concept" | "entity" | "summary" | "qa"
            ) && !record.basename().starts_with('_')
                && !record.path.contains("/drafts/")
        })
        .cloned()
        .collect::<Vec<_>>();

    for record in records {
        for target in extract_wikilinks(&record.text) {
            for resolved in resolve_target(record, &target, &by_path, &by_basename) {
                *inbound.entry(resolved.path_no_ext()).or_insert(0) += 1;
            }
        }
    }

    let mut issues = Vec::new();
    for record in trackable {
        if inbound
            .get(&record.path_no_ext())
            .copied()
            .unwrap_or_default()
            > 0
        {
            continue;
        }
        issues.push(json!({
            "kind": "orphan_page",
            "path": record.path,
        }));
    }
    issues
}
