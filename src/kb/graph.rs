use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::Result;
use serde_json::{Value, json};

use crate::audit_log;
use crate::common::{list_field, now_iso};
use crate::layout::collect_markdown_records;
use crate::payload::{GraphEdge, GraphNode, GraphSnapshot};
use crate::query::live_records;

pub fn build_graph_snapshot(vault_root: &Path) -> Result<Value> {
    let records = live_records(&collect_markdown_records(vault_root)?)
        .into_iter()
        .filter(|record| {
            matches!(
                record.kind.as_str(),
                "summary" | "concept" | "entity" | "procedure" | "topic"
            )
        })
        .collect::<Vec<_>>();
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut edge_type_counts: BTreeMap<String, i64> = BTreeMap::new();
    for record in &records {
        let related = list_field(&record.frontmatter, "related");
        let question_links = list_field(&record.frontmatter, "question_links");
        let relationship_notes = list_field(&record.frontmatter, "relationship_notes");
        let sources = list_field(&record.frontmatter, "sources");
        let visibility_scope = record
            .frontmatter
            .get("visibility_scope")
            .and_then(Value::as_str)
            .unwrap_or("shared")
            .to_string();
        nodes.push(GraphNode {
            id: record.path_no_ext(),
            path: record.path.clone(),
            kind: record.kind.clone(),
            title: record
                .frontmatter
                .get("title")
                .and_then(Value::as_str)
                .unwrap_or(&record.basename())
                .to_string(),
            visibility_scope: visibility_scope.clone(),
            topic_hub: record
                .frontmatter
                .get("topic_hub")
                .cloned()
                .unwrap_or(Value::Null),
            source_count: sources.len(),
            related_count: related.len(),
            question_link_count: question_links.len(),
            has_relationship_notes: !relationship_notes.is_empty(),
            graph_required: matches!(
                record
                    .frontmatter
                    .get("graph_required")
                    .and_then(Value::as_str)
                    .map(|value| value.trim().to_lowercase())
                    .as_deref(),
                Some("true" | "yes" | "1")
            ),
            confidence_score: record
                .frontmatter
                .get("confidence_score")
                .cloned()
                .unwrap_or(Value::Null),
            confidence_band: record
                .frontmatter
                .get("confidence_band")
                .cloned()
                .unwrap_or(Value::Null),
            approved_at: record
                .frontmatter
                .get("approved_at")
                .cloned()
                .unwrap_or(Value::Null),
            last_confirmed_at: record
                .frontmatter
                .get("last_confirmed_at")
                .cloned()
                .unwrap_or(Value::Null),
        });
        for relation in ["related", "supersedes", "superseded_by"] {
            let targets = list_field(&record.frontmatter, relation);
            *edge_type_counts
                .entry(relation.to_string())
                .or_insert(0_i64) += targets.len() as i64;
            for target in targets {
                edges.push(GraphEdge {
                    source: record.path_no_ext(),
                    relation: relation.to_string(),
                    target,
                    source_kind: record.kind.clone(),
                    source_visibility_scope: visibility_scope.clone(),
                });
            }
        }
    }
    let snapshot = GraphSnapshot {
        vault_root: crate::common::normalize_path_string(vault_root.to_string_lossy().as_ref()),
        generated_at: now_iso(),
        candidate_policy: "candidate-only".to_string(),
        node_count: nodes.len(),
        edge_count: edges.len(),
        edge_type_counts,
        nodes,
        edges,
    };
    Ok(serde_json::to_value(&snapshot)?)
}

pub fn write_graph_snapshot(vault_root: &Path, payload: &Value) -> Result<String> {
    let output_path = vault_root
        .join("outputs")
        .join("health")
        .join("graph-snapshot.json");
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output_path, serde_json::to_string_pretty(payload)?)?;
    let rel = crate::common::relative_posix(&output_path, vault_root);
    audit_log::append_event(
        vault_root,
        "build_graph_snapshot",
        &json!({
            "output_path": rel,
            "node_count": payload.get("node_count").cloned().unwrap_or(Value::Null),
            "edge_count": payload.get("edge_count").cloned().unwrap_or(Value::Null),
        }),
    )?;
    Ok(rel)
}
