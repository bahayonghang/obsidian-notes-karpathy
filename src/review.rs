use std::collections::BTreeMap;
use std::path::Path;

use anyhow::Result;
use serde_json::{json, Value};

use crate::common::{iter_markdown_records, list_field, parse_number, MarkdownRecord};

pub const REVIEWABLE_DRAFT_ROOTS: [&str; 5] = [
    "wiki/drafts/summaries",
    "wiki/drafts/topics",
    "wiki/drafts/concepts",
    "wiki/drafts/entities",
    "wiki/drafts/procedures",
];

pub fn reviewable_draft_records(vault_root: &Path) -> Result<Vec<MarkdownRecord>> {
    let records = iter_markdown_records(vault_root, &REVIEWABLE_DRAFT_ROOTS)?;
    Ok(records
        .into_iter()
        .filter(|record| !record.basename().starts_with('_'))
        .collect())
}

pub fn live_record_for_draft(vault_root: &Path, draft_record: &MarkdownRecord) -> String {
    let relative = draft_record
        .path
        .strip_prefix("wiki/drafts/")
        .unwrap_or(&draft_record.path);
    crate::common::relative_posix(
        &vault_root.join("wiki").join("live").join(relative),
        vault_root,
    )
}

pub fn review_record_for_draft(draft_record: &MarkdownRecord) -> String {
    let suffix = draft_record
        .path
        .strip_prefix("wiki/drafts/")
        .unwrap_or(&draft_record.path)
        .replace('/', "--");
    format!("outputs/reviews/{suffix}")
}

pub fn compute_review_score(frontmatter: &serde_json::Map<String, Value>) -> Option<f64> {
    if let Some(explicit) = parse_number(frontmatter.get("review_score")) {
        return Some((explicit * 1000.0).round() / 1000.0);
    }
    let mut components = Vec::new();
    for key in ["accuracy", "provenance", "composability"] {
        if let Some(value) = parse_number(frontmatter.get(key)) {
            components.push(value);
        }
    }
    if let Some(conflict_risk) = parse_number(frontmatter.get("conflict_risk")) {
        components.push((1.0 - conflict_risk).max(0.0));
    }
    if components.is_empty() {
        return None;
    }
    let score = components.iter().sum::<f64>() / components.len() as f64;
    Some((score * 1000.0).round() / 1000.0)
}

pub fn scan_review_queue(vault_root: &Path) -> Result<Value> {
    let mut items = Vec::new();
    let mut decision_counts: BTreeMap<String, i64> = BTreeMap::new();
    let mut state_counts: BTreeMap<String, i64> = BTreeMap::new();
    let mut pending_count = 0i64;

    for record in reviewable_draft_records(vault_root)? {
        let frontmatter = &record.frontmatter;
        let state = frontmatter
            .get("review_state")
            .and_then(Value::as_str)
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "pending".to_string());
        let score = compute_review_score(frontmatter);
        let blocking_flags = list_field(frontmatter, "blocking_flags");
        let live_conflict = blocking_flags.iter().any(|flag| flag == "live_conflict")
            || frontmatter.get("status").and_then(Value::as_str) == Some("conflicting");
        let hard_flags = blocking_flags
            .iter()
            .filter(|flag| flag.as_str() != "live_conflict")
            .cloned()
            .collect::<Vec<_>>();
        let live_path = live_record_for_draft(vault_root, &record);
        let live_exists = vault_root.join(&live_path).exists();

        let (pending, alias_candidates, duplicate_candidates, decision, reason) =
            if matches!(state.as_str(), "promoted" | "rejected" | "archived") {
                (
                    false,
                    Vec::new(),
                    Vec::new(),
                    if state == "promoted" {
                        "approve"
                    } else {
                        "reject"
                    }
                    .to_string(),
                    "terminal_state".to_string(),
                )
            } else {
                pending_count += 1;
                let alias_candidates = list_field(frontmatter, "alias_candidates");
                let duplicate_candidates = list_field(frontmatter, "duplicate_candidates");
                let (decision, reason) = if !hard_flags.is_empty() {
                    ("reject", "blocking_flags")
                } else if score.is_none() {
                    ("needs-human", "missing_review_score")
                } else if live_conflict {
                    ("needs-human", "live_conflict")
                } else if !duplicate_candidates.is_empty() {
                    ("needs-human", "duplicate_candidates")
                } else if !alias_candidates.is_empty() && score.unwrap_or_default() < 0.90 {
                    ("needs-human", "alias_alignment_requires_judgment")
                } else if score.unwrap_or_default() >= 0.85 {
                    ("approve", "threshold_met")
                } else if score.unwrap_or_default() < 0.60 {
                    ("reject", "score_below_floor")
                } else {
                    ("needs-human", "score_band_requires_judgment")
                };
                (
                    true,
                    alias_candidates,
                    duplicate_candidates,
                    decision.to_string(),
                    reason.to_string(),
                )
            };

        *decision_counts.entry(decision.clone()).or_insert(0) += 1;
        *state_counts.entry(state.clone()).or_insert(0) += 1;
        items.push(json!({
            "path": record.path,
            "kind": record.kind,
            "review_state": state,
            "review_score": score,
            "blocking_flags": blocking_flags,
            "alias_candidates": alias_candidates,
            "duplicate_candidates": duplicate_candidates,
            "decision": decision,
            "reason": reason,
            "pending": pending,
            "live_path": live_path,
            "live_exists": live_exists,
            "review_record_path": review_record_for_draft(&record),
        }));
    }

    Ok(json!({
        "vault_root": crate::common::normalize_path_string(vault_root.to_string_lossy().as_ref()),
        "counts": {
            "pending": pending_count,
            "approve": decision_counts.get("approve").copied().unwrap_or_default(),
            "reject": decision_counts.get("reject").copied().unwrap_or_default(),
            "needs-human": decision_counts.get("needs-human").copied().unwrap_or_default(),
        },
        "state_counts": state_counts,
        "items": items,
    }))
}
