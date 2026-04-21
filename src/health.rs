use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use anyhow::Result;
use serde_json::{json, Value};

use crate::common::{
    extract_wikilinks, list_field, load_markdown, normalize_identity, parse_datetime,
    record_identities, registry_for_records, resolve_target, slugify_identity, strip_link_alias,
    ALIAS_WIKILINK_RE, TABLE_LINE_RE,
};
use crate::guidance::inspect_local_guidance;
use crate::layout::collect_markdown_records;
use crate::query::live_records;
use crate::review::{reviewable_draft_records, scan_review_queue};

pub fn duplicate_identity_issues(records: &[crate::common::MarkdownRecord]) -> Vec<Value> {
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

pub fn alias_wikilink_table_issues(records: &[crate::common::MarkdownRecord]) -> Vec<Value> {
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

pub fn broken_wikilink_issues(records: &[crate::common::MarkdownRecord]) -> Vec<Value> {
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

pub fn orphan_page_issues(records: &[crate::common::MarkdownRecord]) -> Vec<Value> {
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

pub fn stale_qa_issues(records: &[crate::common::MarkdownRecord]) -> Vec<Value> {
    let (by_path, by_basename) = registry_for_records(records);
    let mut issues = Vec::new();
    for record in records {
        if record.kind != "qa" {
            continue;
        }
        let Some(asked_at) = parse_datetime(record.frontmatter.get("asked_at")) else {
            continue;
        };
        let mut newer_sources = Vec::new();
        for source in list_field(&record.frontmatter, "sources") {
            let resolved =
                resolve_target(record, &strip_link_alias(&source), &by_path, &by_basename);
            let Some(candidate) = resolved.first() else {
                continue;
            };
            let updated_at = parse_datetime(candidate.frontmatter.get("updated_at"))
                .or_else(|| parse_datetime(candidate.frontmatter.get("approved_at")))
                .or_else(|| parse_datetime(candidate.frontmatter.get("compiled_at")))
                .or_else(|| parse_datetime(candidate.frontmatter.get("date")));
            if let Some(updated_at) = updated_at {
                if updated_at > asked_at {
                    newer_sources.push(candidate.path.clone());
                }
            }
        }
        if !newer_sources.is_empty() {
            newer_sources.sort();
            newer_sources.dedup();
            issues.push(json!({
                "kind": "stale_qa",
                "path": record.path,
                "newer_sources": newer_sources,
            }));
        }
    }
    issues
}

pub fn briefing_staleness_issues(vault_root: &Path) -> Result<Vec<String>> {
    let records = collect_markdown_records(vault_root)?;
    Ok(briefing_staleness_issue_values(vault_root, &records)?
        .into_iter()
        .filter_map(|item| {
            item.get("kind")
                .and_then(Value::as_str)
                .map(ToString::to_string)
        })
        .collect())
}

fn briefing_staleness_issue_values(
    _vault_root: &Path,
    records: &[crate::common::MarkdownRecord],
) -> Result<Vec<Value>> {
    let (by_path, by_basename) = registry_for_records(records);
    let mut issues = Vec::new();
    for record in records {
        if record.kind != "briefing" {
            continue;
        }
        let updated_at = parse_datetime(record.frontmatter.get("updated_at"));
        let staleness_after = parse_datetime(record.frontmatter.get("staleness_after"));
        let source_live_pages = list_field(&record.frontmatter, "source_live_pages");
        let mut resolved_sources = Vec::new();
        for source in source_live_pages {
            resolved_sources.extend(resolve_target(record, &source, &by_path, &by_basename));
        }
        let mut newest_source_time = None;
        let mut newest_source_path: Option<String> = None;
        for source in resolved_sources {
            let candidate_time = parse_datetime(source.frontmatter.get("updated_at"))
                .or_else(|| parse_datetime(source.frontmatter.get("approved_at")))
                .or_else(|| parse_datetime(source.frontmatter.get("compiled_at")));
            if let Some(candidate_time) = candidate_time {
                if newest_source_time
                    .map(|current| candidate_time > current)
                    .unwrap_or(true)
                {
                    newest_source_time = Some(candidate_time);
                    newest_source_path = Some(source.path.clone());
                }
            }
        }

        let mut stale = false;
        let mut reason = None;
        if let (Some(newest_source_time), Some(updated_at)) = (newest_source_time, updated_at) {
            if newest_source_time > updated_at {
                stale = true;
                reason = Some("source_live_page_newer_than_briefing");
            }
        }
        if !stale {
            if let (Some(newest_source_time), Some(staleness_after)) =
                (newest_source_time, staleness_after)
            {
                if newest_source_time > staleness_after {
                    stale = true;
                    reason = Some("staleness_threshold_crossed");
                }
            }
        }
        if stale {
            issues.push(json!({
                "kind": "stale_briefing",
                "path": record.path,
                "reason": reason,
                "newer_source": newest_source_path,
            }));
        }
    }
    Ok(issues)
}

pub fn weak_live_sources_issues(records: &[crate::common::MarkdownRecord]) -> Vec<Value> {
    live_records(records)
        .into_iter()
        .filter(|record| {
            matches!(
                record.kind.as_str(),
                "summary" | "topic" | "concept" | "entity" | "procedure"
            )
        })
        .filter(|record| {
            record
                .frontmatter
                .get("trust_level")
                .and_then(Value::as_str)
                == Some("approved")
        })
        .filter(|record| list_field(&record.frontmatter, "sources").is_empty())
        .map(|record| json!({"kind": "weak_live_sources", "path": record.path}))
        .collect()
}

pub fn memory_knowledge_mix_issues(records: &[crate::common::MarkdownRecord]) -> Vec<Value> {
    let knowledge_keys = [
        "sources",
        "source_file",
        "review_record",
        "approved_from",
        "trust_level",
        "compiled_from",
        "capture_sources",
        "draft_id",
        "review_state",
        "decision",
        "approved_at",
    ];
    let knowledge_headings = [
        "## Thesis",
        "## Definition",
        "## Evidence",
        "## Key Takeaways",
        "## Established",
        "## Source Claims",
    ];
    let collaboration_headings = [
        "## Preferences",
        "## Collaboration Rules",
        "## Working Agreements",
        "## Current Tasks",
        "## Active Tasks",
    ];
    let mut issues = Vec::new();
    for record in records {
        if record.kind == "memory" {
            let mut reasons = knowledge_keys
                .iter()
                .filter(|key| record.frontmatter.contains_key(**key))
                .map(|key| key.to_string())
                .collect::<Vec<_>>();
            if knowledge_headings
                .iter()
                .any(|heading| record.body.contains(heading))
            {
                reasons.push("knowledge_heading".to_string());
            }
            if !reasons.is_empty() {
                issues.push(json!({"kind": "memory_knowledge_mix", "path": record.path, "reasons": reasons}));
            }
            continue;
        }
        if record.path.starts_with("wiki/live/")
            && matches!(
                record.kind.as_str(),
                "summary" | "topic" | "concept" | "entity"
            )
        {
            let mut reasons = ["preferences", "working_style", "collaboration_rules"]
                .iter()
                .filter(|key| record.frontmatter.contains_key(**key))
                .map(|key| key.to_string())
                .collect::<Vec<_>>();
            if collaboration_headings
                .iter()
                .any(|heading| record.body.contains(heading))
            {
                reasons.push("collaboration_heading".to_string());
            }
            if !reasons.is_empty() {
                issues.push(json!({"kind": "memory_knowledge_mix", "path": record.path, "reasons": reasons}));
            }
        }
    }
    issues
}

pub fn writeback_backlog_issues(records: &[crate::common::MarkdownRecord]) -> Vec<Value> {
    let mut issues = Vec::new();
    for record in records {
        if !(record.path.starts_with("outputs/qa/") || record.path.starts_with("outputs/content/"))
        {
            continue;
        }
        let candidates = list_field(&record.frontmatter, "writeback_candidates");
        let status = record
            .frontmatter
            .get("writeback_status")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .trim()
            .to_lowercase();
        if candidates.is_empty() || matches!(status.as_str(), "compiled" | "rejected") {
            continue;
        }
        let reason = if status == "pending" {
            "pending_writeback_candidates"
        } else {
            "missing_or_open_writeback_status"
        };
        issues.push(json!({
            "kind": "writeback_backlog",
            "path": record.path,
            "candidate_count": candidates.len(),
            "writeback_status": if status.is_empty() { Value::Null } else { json!(status) },
            "reason": reason,
        }));
    }
    issues
}

pub fn missing_confidence_metadata_issues(records: &[crate::common::MarkdownRecord]) -> Vec<Value> {
    let required_keys = [
        "confidence_score",
        "confidence_band",
        "support_count",
        "contradiction_count",
    ];
    let latest_signal_keys = [
        "confidence_score",
        "confidence_band",
        "support_count",
        "contradiction_count",
        "last_confirmed_at",
        "next_review_due_at",
        "decay_class",
        "supersedes",
        "superseded_by",
        "superseded_at",
        "supersession_reason",
        "visibility_scope",
    ];
    let mut issues = Vec::new();
    for record in live_records(records) {
        if !matches!(
            record.kind.as_str(),
            "summary" | "topic" | "concept" | "entity" | "procedure"
        ) {
            continue;
        }
        if record
            .frontmatter
            .get("trust_level")
            .and_then(Value::as_str)
            != Some("approved")
        {
            continue;
        }
        if !latest_signal_keys
            .iter()
            .any(|key| record.frontmatter.contains_key(*key))
        {
            continue;
        }
        let missing = required_keys
            .iter()
            .filter(|key| !record.frontmatter.contains_key(**key))
            .map(|key| key.to_string())
            .collect::<Vec<_>>();
        if missing.is_empty() {
            continue;
        }
        issues.push(json!({
            "kind": "missing_confidence_metadata",
            "path": record.path,
            "missing_keys": missing,
            "recommended_action": "fill_core_confidence_metadata",
        }));
    }
    issues
}

pub fn confidence_decay_due_issues(records: &[crate::common::MarkdownRecord]) -> Vec<Value> {
    let now = chrono::Utc::now();
    let mut issues = Vec::new();
    for record in live_records(records) {
        if !matches!(
            record.kind.as_str(),
            "summary" | "topic" | "concept" | "entity" | "procedure"
        ) {
            continue;
        }
        let Some(due_at) = parse_datetime(record.frontmatter.get("next_review_due_at")) else {
            continue;
        };
        if due_at > now {
            continue;
        }
        let overdue_days = (now - due_at).num_seconds() / 86_400;
        issues.push(json!({
            "kind": "confidence_decay_due",
            "path": record.path,
            "next_review_due_at": record.frontmatter.get("next_review_due_at").cloned().unwrap_or(Value::Null),
            "overdue_days": overdue_days,
            "last_confirmed_at": record.frontmatter.get("last_confirmed_at").cloned().unwrap_or(Value::Null),
            "recommended_action": "refresh_confidence_review",
        }));
    }
    issues
}

pub fn supersession_gap_issues(records: &[crate::common::MarkdownRecord]) -> Vec<Value> {
    let (by_path, by_basename) = registry_for_records(records);
    let mut issues = Vec::new();
    for record in live_records(records) {
        if !matches!(
            record.kind.as_str(),
            "summary" | "topic" | "concept" | "entity" | "procedure"
        ) {
            continue;
        }
        let supersedes = list_field(&record.frontmatter, "supersedes");
        let superseded_by = list_field(&record.frontmatter, "superseded_by");
        if supersedes.is_empty() && superseded_by.is_empty() {
            continue;
        }
        let mut reasons = BTreeSet::new();
        if !superseded_by.is_empty() && !record.frontmatter.contains_key("superseded_at") {
            reasons.insert("missing_superseded_at".to_string());
        }
        if !superseded_by.is_empty() && !record.frontmatter.contains_key("supersession_reason") {
            reasons.insert("missing_supersession_reason".to_string());
        }
        for target in supersedes.iter().chain(superseded_by.iter()) {
            if resolve_target(&record, target, &by_path, &by_basename).is_empty() {
                reasons.insert(format!("unresolved_target:{}", strip_link_alias(target)));
            }
        }
        for target in superseded_by {
            let resolved = resolve_target(&record, &target, &by_path, &by_basename);
            let Some(target_record) = resolved.first() else {
                continue;
            };
            let mut backrefs = BTreeSet::new();
            for target_ref in list_field(&target_record.frontmatter, "supersedes") {
                for backref in resolve_target(target_record, &target_ref, &by_path, &by_basename) {
                    backrefs.insert(backref.path_no_ext());
                }
            }
            if !backrefs.contains(&record.path_no_ext()) {
                reasons.insert(format!(
                    "missing_reciprocal_supersedes:{}",
                    target_record.path
                ));
            }
        }
        if !reasons.is_empty() {
            issues.push(json!({
                "kind": "supersession_gap",
                "path": record.path,
                "reasons": reasons.into_iter().collect::<Vec<_>>(),
                "recommended_action": "reconcile_supersession_chain",
                "followup_route": "review",
            }));
        }
    }
    issues
}

pub fn episodic_backlog_issues(records: &[crate::common::MarkdownRecord]) -> Vec<Value> {
    records
        .iter()
        .filter(|record| record.kind == "episode")
        .filter(|record| {
            let status = record
                .frontmatter
                .get("consolidation_status")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .trim()
                .to_lowercase();
            !matches!(status.as_str(), "reviewed" | "completed")
        })
        .map(|record| {
            json!({
                "kind": "episodic_backlog",
                "path": record.path,
                "consolidation_status": record.frontmatter.get("consolidation_status").cloned().unwrap_or(Value::Null),
                "recommended_action": "consolidate_episode_followup",
                "followup_route": record.frontmatter.get("followup_route").and_then(Value::as_str).map(|value| value.trim().to_lowercase()).unwrap_or_else(|| "draft".to_string()),
            })
        })
        .collect()
}

pub fn graph_gap_issues(records: &[crate::common::MarkdownRecord]) -> Vec<Value> {
    let mut issues = Vec::new();
    for record in live_records(records) {
        if !matches!(
            record.kind.as_str(),
            "summary" | "topic" | "concept" | "entity" | "procedure"
        ) {
            continue;
        }
        let graph_required = matches!(
            record
                .frontmatter
                .get("graph_required")
                .and_then(Value::as_str)
                .map(|value| value.trim().to_lowercase())
                .as_deref(),
            Some("true" | "yes" | "1")
        );
        let relationship_notes = list_field(&record.frontmatter, "relationship_notes");
        if !graph_required && relationship_notes.is_empty() {
            continue;
        }
        if !list_field(&record.frontmatter, "related").is_empty() {
            continue;
        }
        issues.push(json!({"kind": "graph_gap", "path": record.path, "reason": "graph_signals_without_related_edges"}));
    }
    issues
}

fn creator_account_key(record: &crate::common::MarkdownRecord) -> String {
    for key in [
        "account_id",
        "brief_for",
        "creator_profile",
        "source_profile",
    ] {
        if let Some(Value::String(value)) = record.frontmatter.get(key) {
            if !value.trim().is_empty() {
                return slugify_identity(value);
            }
        }
    }
    let basename = record.basename();
    let basename = basename.strip_suffix("_style-guide").unwrap_or(&basename);
    slugify_identity(basename)
}

fn creator_rules(record: &crate::common::MarkdownRecord) -> Value {
    let mut forbidden_terms = list_field(&record.frontmatter, "forbidden_terms");
    forbidden_terms.sort();
    forbidden_terms.dedup();
    let mut required_constraints = list_field(&record.frontmatter, "required_constraints");
    required_constraints.extend(list_field(&record.frontmatter, "publishing_constraints"));
    required_constraints.sort();
    required_constraints.dedup();
    json!({
        "account_key": creator_account_key(record),
        "voice_profile": record.frontmatter.get("voice_profile").and_then(Value::as_str).unwrap_or_default().trim(),
        "target_audience": record.frontmatter.get("target_audience").and_then(Value::as_str).unwrap_or_default().trim(),
        "forbidden_terms": forbidden_terms,
        "required_constraints": required_constraints,
    })
}

pub fn creator_guidance_issues(
    vault_root: &Path,
    records: &[crate::common::MarkdownRecord],
) -> Result<Vec<Value>> {
    let mut issues = Vec::new();
    let guidance = inspect_local_guidance(vault_root);
    let claude_text = fs::read_to_string(vault_root.join("CLAUDE.md")).unwrap_or_default();
    let memory_text = fs::read_to_string(vault_root.join("MEMORY.md")).unwrap_or_default();
    let style_guides = std::fs::read_dir(vault_root)
        .ok()
        .into_iter()
        .flat_map(|entries| entries.filter_map(Result::ok))
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("md"))
        .filter(|path| {
            path.file_name()
                .map(|value| value.to_string_lossy().ends_with("_style-guide.md"))
                .unwrap_or(false)
        })
        .map(|path| load_markdown(&path, Some(vault_root)))
        .collect::<Result<Vec<_>>>()?;
    let briefings = records
        .iter()
        .filter(|record| record.kind == "briefing")
        .cloned()
        .collect::<Vec<_>>();
    let briefings_by_account = briefings
        .into_iter()
        .map(|record| (creator_account_key(&record), record))
        .collect::<BTreeMap<_, _>>();

    if !style_guides.is_empty()
        && guidance
            .get("claude")
            .and_then(|value| value.get("present"))
            .and_then(Value::as_bool)
            != Some(true)
    {
        issues.push(json!({"kind": "editorial_drift", "path": "CLAUDE.md", "reason": "missing_claude_guidance_for_creator_profiles"}));
    }
    if !style_guides.is_empty() && !vault_root.join("MEMORY.md").exists() {
        issues.push(json!({"kind": "editorial_drift", "path": "MEMORY.md", "reason": "missing_memory_context_for_creator_profiles"}));
    }

    for style_guide in style_guides {
        let account_key = creator_account_key(&style_guide);
        let style_rules = creator_rules(&style_guide);
        let Some(briefing) = briefings_by_account.get(&account_key) else {
            issues.push(json!({"kind": "editorial_drift", "path": style_guide.path, "reason": "missing_account_briefing", "account_key": account_key}));
            continue;
        };
        let briefing_rules = creator_rules(briefing);
        for field in ["voice_profile", "target_audience"] {
            let style_value = style_rules
                .get(field)
                .and_then(Value::as_str)
                .unwrap_or_default();
            let briefing_value = briefing_rules
                .get(field)
                .and_then(Value::as_str)
                .unwrap_or_default();
            if !style_value.is_empty() && briefing_value.is_empty() {
                issues.push(json!({"kind": "editorial_drift", "path": briefing.path, "reason": format!("missing_{field}"), "account_key": account_key}));
            } else if !style_value.is_empty() && style_value != briefing_value {
                issues.push(json!({
                    "kind": "profile_conflict",
                    "path": briefing.path,
                    "account_key": account_key,
                    "field": field,
                    "style_value": style_value,
                    "briefing_value": briefing_value,
                }));
            }
        }
        let style_terms = style_rules
            .get("forbidden_terms")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let briefing_terms = briefing_rules
            .get("forbidden_terms")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let missing_terms = style_terms
            .iter()
            .filter_map(Value::as_str)
            .filter(|value| {
                !briefing_terms
                    .iter()
                    .any(|item| item.as_str() == Some(*value))
            })
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        if !missing_terms.is_empty() {
            issues.push(json!({
                "kind": "editorial_drift",
                "path": briefing.path,
                "reason": "missing_forbidden_terms",
                "account_key": account_key,
                "missing_terms": missing_terms,
            }));
        }
        let style_constraints = style_rules
            .get("required_constraints")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let briefing_constraints = briefing_rules
            .get("required_constraints")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let missing_constraints = style_constraints
            .iter()
            .filter_map(Value::as_str)
            .filter(|value| {
                !briefing_constraints
                    .iter()
                    .any(|item| item.as_str() == Some(*value))
            })
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        if !missing_constraints.is_empty() {
            issues.push(json!({
                "kind": "editorial_drift",
                "path": briefing.path,
                "reason": "missing_required_constraints",
                "account_key": account_key,
                "missing_constraints": missing_constraints,
            }));
        }
        let account_key_slug = slugify_identity(&account_key);
        if !account_key_slug.is_empty()
            && !slugify_identity(&claude_text).contains(&account_key_slug)
            && !slugify_identity(&memory_text).contains(&account_key_slug)
        {
            issues.push(json!({
                "kind": "editorial_drift",
                "path": style_guide.path,
                "reason": "account_profile_not_reflected_in_claude_or_memory",
                "account_key": account_key,
            }));
        }
    }
    Ok(issues)
}

pub fn reuse_gap_issues(records: &[crate::common::MarkdownRecord]) -> Vec<Value> {
    records
        .iter()
        .filter(|record| record.kind == "content_output")
        .filter(|record| !list_field(&record.frontmatter, "source_live_pages").is_empty())
        .filter(|record| list_field(&record.frontmatter, "reused_prior_coverage").is_empty())
        .filter(|record| list_field(&record.frontmatter, "derived_from").is_empty())
        .map(|record| json!({"kind": "reuse_gap", "path": record.path, "recommended_action": "record_reused_prior_coverage"}))
        .collect()
}

pub fn underused_source_issues(records: &[crate::common::MarkdownRecord]) -> Vec<Value> {
    let mut used_sources = BTreeSet::new();
    for record in records {
        if record.kind == "summary" && record.path.starts_with("wiki/live/") {
            continue;
        }
        for key in ["sources", "source_live_pages", "reused_prior_coverage"] {
            for target in list_field(&record.frontmatter, key) {
                let stripped = strip_link_alias(&target);
                if !stripped.is_empty() {
                    used_sources.insert(stripped);
                }
            }
        }
    }
    live_records(records)
        .into_iter()
        .filter(|record| record.kind == "summary")
        .filter(|record| record.frontmatter.get("trust_level").and_then(Value::as_str) == Some("approved"))
        .filter(|record| !used_sources.contains(&record.path_no_ext()))
        .map(|record| json!({"kind": "underused_sources", "path": record.path, "recommended_action": "surface_source_for_creator_reuse"}))
        .collect()
}

pub fn unapproved_live_issues(records: &[crate::common::MarkdownRecord]) -> Vec<Value> {
    live_records(records)
        .into_iter()
        .filter(|record| {
            matches!(
                record.kind.as_str(),
                "summary" | "topic" | "concept" | "entity" | "procedure"
            )
        })
        .filter(|record| {
            !(record
                .frontmatter
                .get("trust_level")
                .and_then(Value::as_str)
                == Some("approved")
                && record.frontmatter.contains_key("approved_at")
                && record.frontmatter.contains_key("review_record"))
        })
        .map(|record| json!({"kind": "unapproved_live_page", "path": record.path}))
        .collect()
}

pub fn approved_conflict_issues(records: &[crate::common::MarkdownRecord]) -> Vec<Value> {
    live_records(records)
        .into_iter()
        .filter(|record| {
            matches!(
                record.kind.as_str(),
                "topic" | "concept" | "entity" | "procedure"
            )
        })
        .filter(|record| {
            record
                .frontmatter
                .get("trust_level")
                .and_then(Value::as_str)
                == Some("approved")
                && record.frontmatter.get("status").and_then(Value::as_str) == Some("conflicting")
        })
        .map(|record| json!({"kind": "approved_conflict", "path": record.path}))
        .collect()
}

pub fn review_backlog_issues(vault_root: &Path) -> Result<Vec<Value>> {
    let queue = scan_review_queue(vault_root)?;
    Ok(queue
        .get("items")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter(|item| item.get("pending").and_then(Value::as_bool) == Some(true))
        .map(|item| json!({"kind": "review_backlog", "path": item.get("path").cloned().unwrap_or(Value::Null), "decision": item.get("decision").cloned().unwrap_or(Value::Null)}))
        .collect())
}

pub fn sensitivity_metadata_gap_issues(records: &[crate::common::MarkdownRecord]) -> Vec<Value> {
    let allowed = [
        "qa",
        "content_output",
        "report_output",
        "slide_output",
        "chart_output",
        "episode",
        "topic",
        "concept",
        "entity",
        "procedure",
    ];
    records
        .iter()
        .filter(|record| allowed.contains(&record.kind.as_str()))
        .filter(|record| record.frontmatter.get("visibility_scope").and_then(Value::as_str).unwrap_or("shared").trim().to_lowercase() == "private")
        .filter(|record| !record.frontmatter.contains_key("sensitivity_level"))
        .map(|record| json!({"kind": "sensitivity_metadata_gap", "path": record.path, "recommended_action": "add_sensitivity_level"}))
        .collect()
}

pub fn private_scope_leak_issues(records: &[crate::common::MarkdownRecord]) -> Vec<Value> {
    records
        .iter()
        .filter(|record| matches!(record.kind.as_str(), "qa" | "briefing"))
        .filter(|record| record.frontmatter.get("visibility_scope").and_then(Value::as_str).unwrap_or("shared").trim().to_lowercase() == "private")
        .map(|record| json!({"kind": "private_scope_leak", "path": record.path, "recommended_action": "move_private_surface_out_of_default_query_scope"}))
        .collect()
}

pub fn procedural_promotion_gap_issues(vault_root: &Path) -> Result<Vec<Value>> {
    Ok(reviewable_draft_records(vault_root)?
        .into_iter()
        .filter(|record| record.frontmatter.get("promotion_target").and_then(Value::as_str).unwrap_or_default().trim().to_lowercase() == "procedural")
        .filter_map(|record| {
            let expected_path = vault_root.join("wiki").join("drafts").join("procedures").join(format!("{}.md", record.basename()));
            if expected_path.exists() {
                None
            } else {
                Some(json!({
                    "kind": "procedural_promotion_gap",
                    "path": record.path,
                    "expected_procedure_path": crate::common::relative_posix(&expected_path, vault_root),
                    "recommended_action": "draft_procedure_candidate",
                    "followup_route": "draft",
                }))
            }
        })
        .collect())
}

pub fn audit_trail_gap_issues(vault_root: &Path) -> Vec<Value> {
    let audit_root = vault_root.join("outputs").join("audit");
    let operations_path = audit_root.join("operations.jsonl");
    if audit_root.exists()
        && operations_path.exists()
        && fs::metadata(&operations_path)
            .map(|meta| meta.len() > 0)
            .unwrap_or(false)
    {
        return Vec::new();
    }
    vec![json!({"kind": "audit_trail_gap", "path": "outputs/audit/operations.jsonl"})]
}

pub fn duplicate_alias_set_issues(records: &[crate::common::MarkdownRecord]) -> Vec<Value> {
    let mut alias_map: BTreeMap<Vec<String>, Vec<String>> = BTreeMap::new();
    for record in live_records(records) {
        if !matches!(record.kind.as_str(), "topic" | "concept" | "entity") {
            continue;
        }
        let mut aliases = list_field(&record.frontmatter, "aliases")
            .into_iter()
            .map(|alias| slugify_identity(&alias))
            .filter(|alias| !alias.is_empty())
            .collect::<Vec<_>>();
        aliases.sort();
        aliases.dedup();
        if aliases.len() < 2 {
            continue;
        }
        alias_map
            .entry(aliases)
            .or_default()
            .push(record.path.clone());
    }
    alias_map
        .into_iter()
        .filter(|(_, paths)| paths.len() >= 2)
        .map(|(aliases, paths)| json!({"kind": "duplicate_alias_set", "aliases": aliases, "paths": paths}))
        .collect()
}

pub fn volatile_page_stale_issues(records: &[crate::common::MarkdownRecord]) -> Vec<Value> {
    let thresholds: BTreeMap<&str, i64> = crate::config::VOLATILITY_THRESHOLDS
        .iter()
        .copied()
        .collect();
    let now = chrono::Utc::now();
    let mut issues = Vec::new();
    for record in live_records(records) {
        if !matches!(
            record.kind.as_str(),
            "summary" | "topic" | "concept" | "entity" | "procedure"
        ) {
            continue;
        }
        let volatility = record
            .frontmatter
            .get("domain_volatility")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .trim()
            .to_lowercase();
        let Some(threshold) = thresholds.get(volatility.as_str()) else {
            continue;
        };
        let Some(last_reviewed) = parse_datetime(record.frontmatter.get("last_reviewed_at"))
            .or_else(|| parse_datetime(record.frontmatter.get("updated_at")))
            .or_else(|| parse_datetime(record.frontmatter.get("approved_at")))
        else {
            continue;
        };
        let age_days = (now - last_reviewed).num_seconds() / 86_400;
        if age_days > *threshold {
            issues.push(json!({
                "kind": "volatile_page_stale",
                "path": record.path,
                "domain_volatility": volatility,
                "age_days": age_days,
                "threshold_days": threshold,
            }));
        }
    }
    issues
}

pub fn audit_vault_mechanics(vault_root: &Path) -> Result<Value> {
    let records = collect_markdown_records(vault_root)?;
    let mut issues = Vec::new();
    issues.extend(alias_wikilink_table_issues(&records));
    issues.extend(duplicate_identity_issues(&records));
    issues.extend(missing_confidence_metadata_issues(&records));
    issues.extend(confidence_decay_due_issues(&records));
    issues.extend(supersession_gap_issues(&records));
    issues.extend(stale_qa_issues(&records));
    issues.extend(writeback_backlog_issues(&records));
    issues.extend(episodic_backlog_issues(&records));
    issues.extend(broken_wikilink_issues(&records));
    issues.extend(orphan_page_issues(&records));
    issues.extend(duplicate_alias_set_issues(&records));
    issues.extend(volatile_page_stale_issues(&records));
    issues.extend(memory_knowledge_mix_issues(&records));
    issues.extend(graph_gap_issues(&records));
    issues.extend(creator_guidance_issues(vault_root, &records)?);
    issues.extend(reuse_gap_issues(&records));
    issues.extend(underused_source_issues(&records));
    issues.extend(unapproved_live_issues(&records));
    issues.extend(weak_live_sources_issues(&records));
    issues.extend(approved_conflict_issues(&records));
    issues.extend(review_backlog_issues(vault_root)?);
    issues.extend(private_scope_leak_issues(&records));
    issues.extend(sensitivity_metadata_gap_issues(&records));
    issues.extend(procedural_promotion_gap_issues(vault_root)?);
    issues.extend(audit_trail_gap_issues(vault_root));
    issues.extend(briefing_staleness_issue_values(vault_root, &records)?);

    issues.sort_by_key(|issue| {
        (
            issue
                .get("kind")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
            issue
                .get("path")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
            issue
                .get("line")
                .and_then(Value::as_i64)
                .unwrap_or_default(),
        )
    });
    let mut issue_counts = BTreeMap::new();
    for issue in &issues {
        let kind = issue
            .get("kind")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        *issue_counts.entry(kind).or_insert(0_i64) += 1;
    }
    Ok(json!({
        "vault_root": crate::common::normalize_path_string(vault_root.to_string_lossy().as_ref()),
        "layout_family": crate::layout::detect_layout_family(vault_root),
        "issue_counts": issue_counts,
        "issues": issues,
    }))
}
