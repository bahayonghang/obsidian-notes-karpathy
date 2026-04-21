use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use anyhow::Result;
use serde_json::{json, Value};

use crate::common::{list_field, now_iso, slugify_identity};
use crate::health::audit_vault_mechanics;
use crate::layout::collect_markdown_records;
use crate::query::live_records;

const GAP_KINDS: [&str; 18] = [
    "broken_wikilink",
    "orphan_page",
    "stale_qa",
    "writeback_backlog",
    "weak_live_sources",
    "duplicate_concept",
    "duplicate_entity",
    "duplicate_alias_set",
    "volatile_page_stale",
    "stale_briefing",
    "weakly_connected_live_page",
    "hub_candidate_backlog",
    "missing_confidence_metadata",
    "confidence_decay_due",
    "supersession_gap",
    "episodic_backlog",
    "procedural_promotion_gap",
    "graph_gap",
];

fn append_audit_event(vault_root: &Path, action: &str, payload: &Value) -> Result<()> {
    let audit_path = vault_root
        .join("outputs")
        .join("audit")
        .join("operations.jsonl");
    if let Some(parent) = audit_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut existing = if audit_path.exists() {
        fs::read_to_string(&audit_path)?
    } else {
        String::new()
    };
    existing.push_str(&serde_json::to_string(&json!({
        "timestamp": now_iso(),
        "action": action,
        "payload": payload,
    }))?);
    existing.push('\n');
    fs::write(audit_path, existing)?;
    Ok(())
}

fn body_open_questions(record: &crate::common::MarkdownRecord) -> Vec<String> {
    let mut questions = Vec::new();
    let mut in_open_questions = false;
    for line in record.body.lines() {
        let stripped = line.trim();
        if stripped.starts_with("## ") {
            in_open_questions = stripped == "## Open Questions";
            continue;
        }
        if !in_open_questions {
            continue;
        }
        if stripped.starts_with("- [ ] ")
            || stripped.starts_with("- [x] ")
            || stripped.starts_with("- [X] ")
        {
            let question = stripped[6..].trim();
            if !question.is_empty() {
                questions.push(question.to_string());
            }
        } else if let Some(question) = stripped.strip_prefix("- ") {
            let question = question.trim();
            if !question.is_empty() {
                questions.push(question.to_string());
            }
        } else if !stripped.is_empty() {
            in_open_questions = false;
        }
    }
    questions
}

pub fn build_governance_indices(vault_root: &Path) -> Result<Value> {
    let records = collect_markdown_records(vault_root)?;
    let live = live_records(&records);
    let audit = audit_vault_mechanics(vault_root)?;

    let mut questions = Vec::new();
    let mut seen_questions = BTreeSet::new();
    let mut alias_rows = Vec::new();
    let mut entity_rows = Vec::new();
    let mut relationship_rows = Vec::new();
    let mut writeback_backlog = Vec::new();
    let mut followup_routes = Vec::new();
    let mut route_counts = BTreeMap::new();
    let mut hub_candidates = Vec::new();

    for record in &live {
        for question in body_open_questions(record) {
            if seen_questions.insert(question.clone()) {
                questions.push(question);
            }
        }
        if let Some(Value::String(question)) = record.frontmatter.get("question") {
            if !question.trim().is_empty() && seen_questions.insert(question.trim().to_string()) {
                questions.push(question.trim().to_string());
            }
        }

        let canonical = record
            .frontmatter
            .get("canonical_name")
            .and_then(Value::as_str)
            .or_else(|| record.frontmatter.get("concept_id").and_then(Value::as_str))
            .or_else(|| record.frontmatter.get("entity_id").and_then(Value::as_str))
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| record.basename());
        let mut aliases = list_field(&record.frontmatter, "aliases");
        aliases.sort();
        aliases.dedup();
        alias_rows.push(json!({
            "path": record.path,
            "canonical_name": canonical,
            "canonical_slug": slugify_identity(&canonical),
            "aliases": aliases,
        }));
        if record.kind == "entity" {
            entity_rows.push(json!({
                "path": record.path,
                "entity_id": record.frontmatter.get("entity_id").and_then(Value::as_str).map(ToString::to_string).unwrap_or_else(|| record.basename()),
                "entity_type": record.frontmatter.get("entity_type").and_then(Value::as_str).unwrap_or("other"),
                "visibility_scope": record.frontmatter.get("visibility_scope").and_then(Value::as_str).unwrap_or("shared"),
                "aliases": aliases,
            }));
        }
        if matches!(
            record.kind.as_str(),
            "summary" | "concept" | "entity" | "procedure"
        ) {
            for target in list_field(&record.frontmatter, "related") {
                relationship_rows
                    .push(json!({"source": record.path, "relation": "related", "target": target}));
            }
            for target in list_field(&record.frontmatter, "supersedes") {
                relationship_rows.push(
                    json!({"source": record.path, "relation": "supersedes", "target": target}),
                );
            }
            for target in list_field(&record.frontmatter, "superseded_by") {
                relationship_rows.push(
                    json!({"source": record.path, "relation": "superseded_by", "target": target}),
                );
            }
        }
    }

    for record in &records {
        let is_archived_output =
            record.path.starts_with("outputs/qa/") || record.path.starts_with("outputs/content/");
        if !is_archived_output {
            continue;
        }
        let candidates = list_field(&record.frontmatter, "writeback_candidates");
        if !candidates.is_empty() {
            let status = record
                .frontmatter
                .get("writeback_status")
                .and_then(Value::as_str)
                .map(|value| value.trim().to_lowercase());
            if matches!(status.as_deref(), None | Some("pending")) {
                writeback_backlog.push(json!({
                    "path": record.path,
                    "candidate_count": candidates.len(),
                    "writeback_status": status,
                    "followup_route": record.frontmatter.get("followup_route").and_then(Value::as_str).map(|value| value.trim().to_lowercase()),
                }));
            }
        }
        let followup_route = record
            .frontmatter
            .get("followup_route")
            .and_then(Value::as_str)
            .map(|value| value.trim().to_lowercase())
            .filter(|value| !value.is_empty());
        if let Some(followup_route) = followup_route {
            followup_routes
                .push(json!({"path": record.path, "followup_route": followup_route.clone()}));
            *route_counts.entry(followup_route).or_insert(0_i64) += 1;
        }
    }

    for record in &live {
        let related = list_field(&record.frontmatter, "related");
        let topic_hub = record
            .frontmatter
            .get("topic_hub")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let question_links = list_field(&record.frontmatter, "question_links");
        let open_questions = list_field(&record.frontmatter, "open_questions");
        if related.len() <= 1
            && topic_hub.is_empty()
            && question_links.is_empty()
            && open_questions.is_empty()
        {
            hub_candidates
                .push(json!({"path": record.path, "reason": "weakly connected live page"}));
        }
    }

    let gap_issues = audit
        .get("issues")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter(|issue| {
            issue
                .get("kind")
                .and_then(Value::as_str)
                .map(|kind| {
                    GAP_KINDS.contains(&kind)
                        || matches!(
                            kind,
                            "editorial_drift"
                                | "profile_conflict"
                                | "reuse_gap"
                                | "underused_sources"
                        )
                })
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();

    let confidence_maintenance = gap_issues
        .iter()
        .filter(|issue| matches!(issue.get("kind").and_then(Value::as_str), Some("missing_confidence_metadata" | "confidence_decay_due")))
        .map(|issue| {
            json!({
                "path": issue.get("path").cloned().unwrap_or(Value::Null),
                "issue_kind": issue.get("kind").cloned().unwrap_or(Value::Null),
                "recommended_action": issue.get("recommended_action").cloned().unwrap_or(Value::Null),
                "missing_keys": issue.get("missing_keys").cloned().unwrap_or_else(|| json!([])),
                "next_review_due_at": issue.get("next_review_due_at").cloned().unwrap_or(Value::Null),
                "overdue_days": issue.get("overdue_days").cloned().unwrap_or(Value::Null),
                "last_confirmed_at": issue.get("last_confirmed_at").cloned().unwrap_or(Value::Null),
            })
        })
        .collect::<Vec<_>>();
    let closure_signals = gap_issues
        .iter()
        .filter(|issue| matches!(issue.get("kind").and_then(Value::as_str), Some("supersession_gap" | "episodic_backlog" | "procedural_promotion_gap")))
        .map(|issue| {
            json!({
                "path": issue.get("path").cloned().unwrap_or(Value::Null),
                "issue_kind": issue.get("kind").cloned().unwrap_or(Value::Null),
                "recommended_action": issue.get("recommended_action").cloned().unwrap_or(Value::Null),
                "followup_route": issue.get("followup_route").cloned().unwrap_or(Value::Null),
                "reasons": issue.get("reasons").cloned().unwrap_or_else(|| json!([])),
                "consolidation_status": issue.get("consolidation_status").cloned().unwrap_or(Value::Null),
                "expected_procedure_path": issue.get("expected_procedure_path").cloned().unwrap_or(Value::Null),
            })
        })
        .collect::<Vec<_>>();

    let mut questions_md = "# Open Questions\n\n".to_string();
    if questions.is_empty() {
        questions_md.push_str("- [ ] No open questions captured yet.\n");
    } else {
        for question in &questions {
            questions_md.push_str(&format!("- [ ] {question}\n"));
        }
    }

    let mut gaps_md = "# Gap Report\n\n".to_string();
    if gap_issues.is_empty() {
        gaps_md.push_str("- No governance gaps detected.\n");
    } else {
        for issue in &gap_issues {
            gaps_md.push_str(&format!(
                "- {}: {}\n",
                issue
                    .get("kind")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
                issue
                    .get("path")
                    .and_then(Value::as_str)
                    .unwrap_or("(no-path)"),
            ));
        }
    }

    if !writeback_backlog.is_empty() {
        gaps_md.push_str("\n## Writeback Backlog Signals\n\n");
        for item in &writeback_backlog {
            gaps_md.push_str(&format!(
                "- {} | candidates: {} | status: {} | route: {}\n",
                item.get("path").and_then(Value::as_str).unwrap_or_default(),
                item.get("candidate_count")
                    .and_then(Value::as_i64)
                    .unwrap_or_default(),
                item.get("writeback_status")
                    .and_then(Value::as_str)
                    .unwrap_or("open"),
                item.get("followup_route")
                    .and_then(Value::as_str)
                    .unwrap_or("unspecified"),
            ));
        }
    }

    if !confidence_maintenance.is_empty() {
        gaps_md.push_str("\n## Confidence Maintenance Signals\n\n");
        for item in &confidence_maintenance {
            if item.get("issue_kind").and_then(Value::as_str) == Some("missing_confidence_metadata")
            {
                let missing = item
                    .get("missing_keys")
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|value| value.as_str().map(ToString::to_string))
                    .collect::<Vec<_>>()
                    .join(", ");
                gaps_md.push_str(&format!(
                    "- {} | issue: missing_confidence_metadata | missing: {} | action: {}\n",
                    item.get("path").and_then(Value::as_str).unwrap_or_default(),
                    if missing.is_empty() {
                        "(unspecified)"
                    } else {
                        &missing
                    },
                    item.get("recommended_action")
                        .and_then(Value::as_str)
                        .unwrap_or("review_confidence_metadata"),
                ));
            } else {
                gaps_md.push_str(&format!(
                    "- {} | issue: confidence_decay_due | due: {} | overdue_days: {} | action: {}\n",
                    item.get("path").and_then(Value::as_str).unwrap_or_default(),
                    item.get("next_review_due_at").and_then(Value::as_str).unwrap_or("unknown"),
                    item.get("overdue_days").and_then(Value::as_i64).unwrap_or_default(),
                    item.get("recommended_action").and_then(Value::as_str).unwrap_or("review_confidence_metadata"),
                ));
            }
        }
    }

    if !closure_signals.is_empty() {
        gaps_md.push_str("\n## Closure Signals\n\n");
        for item in &closure_signals {
            let kind = item
                .get("issue_kind")
                .and_then(Value::as_str)
                .unwrap_or_default();
            if kind == "supersession_gap" {
                let reasons = item
                    .get("reasons")
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|value| value.as_str().map(ToString::to_string))
                    .collect::<Vec<_>>()
                    .join(", ");
                gaps_md.push_str(&format!(
                    "- {} | issue: supersession_gap | route: {} | reasons: {} | action: {}\n",
                    item.get("path").and_then(Value::as_str).unwrap_or_default(),
                    item.get("followup_route")
                        .and_then(Value::as_str)
                        .unwrap_or("unspecified"),
                    if reasons.is_empty() {
                        "(unspecified)"
                    } else {
                        &reasons
                    },
                    item.get("recommended_action")
                        .and_then(Value::as_str)
                        .unwrap_or("review_followup"),
                ));
            } else if kind == "episodic_backlog" {
                gaps_md.push_str(&format!(
                    "- {} | issue: episodic_backlog | status: {} | route: {} | action: {}\n",
                    item.get("path").and_then(Value::as_str).unwrap_or_default(),
                    item.get("consolidation_status")
                        .and_then(Value::as_str)
                        .unwrap_or("open"),
                    item.get("followup_route")
                        .and_then(Value::as_str)
                        .unwrap_or("unspecified"),
                    item.get("recommended_action")
                        .and_then(Value::as_str)
                        .unwrap_or("review_followup"),
                ));
            } else {
                gaps_md.push_str(&format!(
                    "- {} | issue: procedural_promotion_gap | expected: {} | route: {} | action: {}\n",
                    item.get("path").and_then(Value::as_str).unwrap_or_default(),
                    item.get("expected_procedure_path").and_then(Value::as_str).unwrap_or("(unspecified)"),
                    item.get("followup_route").and_then(Value::as_str).unwrap_or("unspecified"),
                    item.get("recommended_action").and_then(Value::as_str).unwrap_or("review_followup"),
                ));
            }
        }
    }

    let creator_issues = gap_issues
        .iter()
        .filter(|issue| {
            matches!(
                issue.get("kind").and_then(Value::as_str),
                Some("editorial_drift" | "profile_conflict")
            )
        })
        .cloned()
        .collect::<Vec<_>>();
    if !creator_issues.is_empty() {
        gaps_md.push_str("\n## Creator Consistency Signals\n\n");
        for issue in creator_issues {
            gaps_md.push_str(&format!(
                "- {} | issue: {} | account: {} | detail: {}\n",
                issue
                    .get("path")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
                issue
                    .get("kind")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
                issue
                    .get("account_key")
                    .and_then(Value::as_str)
                    .unwrap_or("n/a"),
                issue
                    .get("reason")
                    .and_then(Value::as_str)
                    .or_else(|| issue.get("field").and_then(Value::as_str))
                    .unwrap_or("unspecified"),
            ));
        }
    }
    let reuse_issues = gap_issues
        .iter()
        .filter(|issue| {
            matches!(
                issue.get("kind").and_then(Value::as_str),
                Some("reuse_gap" | "underused_sources")
            )
        })
        .cloned()
        .collect::<Vec<_>>();
    if !reuse_issues.is_empty() {
        gaps_md.push_str("\n## Reuse Signals\n\n");
        for issue in reuse_issues {
            gaps_md.push_str(&format!(
                "- {} | issue: {} | action: {}\n",
                issue
                    .get("path")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
                issue
                    .get("kind")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
                issue
                    .get("recommended_action")
                    .and_then(Value::as_str)
                    .unwrap_or("review_reuse_signal"),
            ));
        }
    }
    if !followup_routes.is_empty() {
        gaps_md.push_str("\n## Follow-up Routes Seen In Archived Outputs\n\n");
        for item in &followup_routes {
            gaps_md.push_str(&format!(
                "- {}: {}\n",
                item.get("followup_route")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
                item.get("path").and_then(Value::as_str).unwrap_or_default(),
            ));
        }
    }
    if !route_counts.is_empty() {
        gaps_md.push_str("\n## Follow-up Route Clusters\n\n");
        for (route, count) in &route_counts {
            gaps_md.push_str(&format!("- {route}: {count} archived outputs\n"));
        }
    }
    if !hub_candidates.is_empty() {
        gaps_md.push_str("\n## Hub And Relationship Hints\n\n");
        for item in &hub_candidates {
            gaps_md.push_str(&format!(
                "- {}: {}\n",
                item.get("reason")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
                item.get("path").and_then(Value::as_str).unwrap_or_default(),
            ));
        }
    }

    let mut aliases_md = "# Alias Registry\n\n".to_string();
    if alias_rows.is_empty() {
        aliases_md.push_str("- No alias-bearing live notes found.\n");
    } else {
        for row in &alias_rows {
            let alias_list = row
                .get("aliases")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .filter_map(|value| value.as_str().map(ToString::to_string))
                .collect::<Vec<_>>()
                .join(", ");
            aliases_md.push_str(&format!(
                "- `{}` -> {} | aliases: {} | path: {}\n",
                row.get("canonical_slug")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
                row.get("canonical_name")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
                if alias_list.is_empty() {
                    "(none)"
                } else {
                    &alias_list
                },
                row.get("path").and_then(Value::as_str).unwrap_or_default(),
            ));
        }
    }

    let mut entities_md = "# Entity Registry\n\n".to_string();
    if entity_rows.is_empty() {
        entities_md.push_str("- No entity pages found.\n");
    } else {
        for row in &entity_rows {
            let alias_list = row
                .get("aliases")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .filter_map(|value| value.as_str().map(ToString::to_string))
                .collect::<Vec<_>>()
                .join(", ");
            entities_md.push_str(&format!(
                "- `{}` | type: {} | scope: {} | aliases: {} | path: {}\n",
                row.get("entity_id")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
                row.get("entity_type")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
                row.get("visibility_scope")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
                if alias_list.is_empty() {
                    "(none)"
                } else {
                    &alias_list
                },
                row.get("path").and_then(Value::as_str).unwrap_or_default(),
            ));
        }
    }

    let mut relationships_md = "# Relationship Registry\n\n".to_string();
    if relationship_rows.is_empty() {
        relationships_md.push_str("- No explicit relationship edges found.\n");
    } else {
        for row in &relationship_rows {
            relationships_md.push_str(&format!(
                "- {} --{}--> {}\n",
                row.get("source")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
                row.get("relation")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
                row.get("target")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
            ));
        }
    }

    Ok(json!({
        "vault_root": crate::common::normalize_path_string(vault_root.to_string_lossy().as_ref()),
        "questions": questions,
        "gap_issue_kinds": gap_issues
            .iter()
            .filter_map(|issue| issue.get("kind").and_then(Value::as_str).map(ToString::to_string))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>(),
        "alias_rows": alias_rows,
        "entity_rows": entity_rows,
        "relationship_rows": relationship_rows,
        "writeback_backlog": writeback_backlog,
        "confidence_maintenance": confidence_maintenance,
        "closure_signals": closure_signals,
        "followup_routes": followup_routes,
        "route_counts": route_counts,
        "hub_candidates": hub_candidates,
        "files": {
            "QUESTIONS.md": questions_md,
            "GAPS.md": gaps_md,
            "ALIASES.md": aliases_md,
            "ENTITIES.md": entities_md,
            "RELATIONSHIPS.md": relationships_md,
        },
    }))
}

pub fn write_governance_indices(vault_root: &Path, payload: &Value) -> Result<Vec<String>> {
    let indices_root = vault_root.join("wiki").join("live").join("indices");
    fs::create_dir_all(&indices_root)?;
    let mut written_files = Vec::new();
    let files = payload
        .get("files")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    for (name, content) in files {
        let path = indices_root.join(&name);
        fs::write(&path, content.as_str().unwrap_or_default())?;
        written_files.push(crate::common::relative_posix(&path, vault_root));
    }
    append_audit_event(
        vault_root,
        "build_governance_indices",
        &json!({
            "written_paths": written_files,
            "question_count": payload.get("questions").and_then(Value::as_array).map(Vec::len).unwrap_or_default(),
            "gap_issue_kinds": payload.get("gap_issue_kinds").cloned().unwrap_or_else(|| json!([])),
        }),
    )?;
    Ok(written_files)
}
