use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde_json::{json, Value};

const GUIDANCE_CONTRACTS: [(&str, &str, bool); 2] = [
    ("agents", "AGENTS.md", true),
    ("claude", "CLAUDE.md", false),
];

pub fn summarize_local_guidance(file_names: &[String]) -> Value {
    let mut matches_by_name: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for file_name in file_names {
        matches_by_name
            .entry(file_name.to_lowercase())
            .or_default()
            .push(file_name.clone());
    }

    let mut status = serde_json::Map::new();
    let mut warnings: Vec<String> = Vec::new();
    let mut blocking_issues: Vec<String> = Vec::new();

    for (key, canonical_name, required) in GUIDANCE_CONTRACTS {
        let mut matches = matches_by_name
            .get(&canonical_name.to_lowercase())
            .cloned()
            .unwrap_or_default();
        matches.sort_by_key(|value| {
            (
                (value != canonical_name) as u8,
                value.to_lowercase(),
                value.clone(),
            )
        });
        let present = !matches.is_empty();
        let canonical = matches.iter().any(|value| value == canonical_name);
        let selected = if canonical {
            Some(canonical_name.to_string())
        } else {
            matches.first().cloned()
        };

        if matches.len() > 1 {
            let issue = format!("duplicate_{key}_guidance_files");
            warnings.push(issue.clone());
            blocking_issues.push(issue);
        }
        if present && !canonical {
            warnings.push(format!("noncanonical_{key}_guidance_name"));
        }
        if required && !present {
            blocking_issues.push(format!("missing_{key}_guidance"));
        } else if !required && !present {
            warnings.push(format!("missing_{key}_guidance"));
        }

        status.insert(
            key.to_string(),
            json!({
                "present": present,
                "path": selected,
                "canonical": canonical,
                "candidates": matches,
            }),
        );
    }

    warnings.sort();
    warnings.dedup();
    blocking_issues.sort();
    blocking_issues.dedup();
    status.insert("warnings".to_string(), json!(warnings));
    status.insert("blocking_issues".to_string(), json!(blocking_issues));
    Value::Object(status)
}

pub fn inspect_local_guidance(vault_root: &Path) -> Value {
    if !vault_root.exists() {
        return summarize_local_guidance(&[]);
    }
    let file_names = fs::read_dir(vault_root)
        .ok()
        .into_iter()
        .flat_map(|entries| entries.filter_map(Result::ok))
        .filter_map(|entry| {
            entry
                .file_type()
                .ok()
                .filter(|kind| kind.is_file())
                .map(|_| entry.file_name())
        })
        .map(|name| name.to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    summarize_local_guidance(&file_names)
}
