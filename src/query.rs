use std::collections::BTreeSet;
use std::path::Path;

use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::Value;

use crate::common::{list_field, load_markdown, MarkdownRecord};
use crate::layout::{collect_markdown_records, resolve_vault_profile};
use crate::payload::{QueryRanked, QueryScope, RankedEntry, ScopeLeak, SensitivityCandidate};

static TOKEN_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[A-Za-z0-9_-]+").expect("valid token regex"));

pub fn query_scope(vault_root: &Path) -> Result<Value> {
    let layout_family = crate::layout::detect_layout_family(vault_root);
    let profile = resolve_vault_profile(vault_root);
    let (included_roots, candidate_roots, excluded_roots): (Vec<&str>, Vec<&str>, Vec<&str>) =
        if layout_family == "review-gated" {
            if profile == "fast-personal" {
                (
                    vec!["wiki/live", "outputs/qa"],
                    vec!["outputs/episodes"],
                    vec!["raw", "wiki/drafts", "outputs/reviews"],
                )
            } else {
                (
                    vec!["wiki/live", "wiki/briefings", "outputs/qa"],
                    vec!["outputs/episodes"],
                    vec!["raw", "wiki/drafts", "outputs/reviews"],
                )
            }
        } else {
            (vec!["wiki", "outputs/qa"], Vec::new(), vec!["raw"])
        };

    let mut included = Vec::new();
    for rel_root in &included_roots {
        let root = vault_root.join(rel_root);
        if !root.exists() {
            continue;
        }
        for entry in walkdir::WalkDir::new(&root).sort_by_file_name() {
            let entry = entry?;
            if entry.file_type().is_file()
                && entry.path().extension().and_then(|value| value.to_str()) == Some("md")
            {
                included.push(crate::common::relative_posix(entry.path(), vault_root));
            }
        }
    }

    let mut excluded = Vec::new();
    for rel_root in &excluded_roots {
        let root = vault_root.join(rel_root);
        if !root.exists() {
            continue;
        }
        for entry in walkdir::WalkDir::new(&root).sort_by_file_name() {
            let entry = entry?;
            if entry.file_type().is_file()
                && entry.path().extension().and_then(|value| value.to_str()) == Some("md")
            {
                excluded.push(crate::common::relative_posix(entry.path(), vault_root));
            }
        }
    }

    let mut candidate_paths = Vec::new();
    for rel_root in &candidate_roots {
        let root = vault_root.join(rel_root);
        if !root.exists() {
            continue;
        }
        for entry in walkdir::WalkDir::new(&root).sort_by_file_name() {
            let entry = entry?;
            if entry.file_type().is_file()
                && entry.path().extension().and_then(|value| value.to_str()) == Some("md")
            {
                candidate_paths.push(crate::common::relative_posix(entry.path(), vault_root));
            }
        }
    }

    let graph_snapshot = vault_root
        .join("outputs")
        .join("health")
        .join("graph-snapshot.json");
    if graph_snapshot.exists() {
        candidate_paths.push(crate::common::relative_posix(&graph_snapshot, vault_root));
    }
    let memory_path = vault_root.join("MEMORY.md");
    if memory_path.exists() {
        excluded.push(crate::common::relative_posix(&memory_path, vault_root));
        excluded.sort();
        excluded.dedup();
    }

    let mut scope_leaks = Vec::new();
    let mut sensitivity_candidates = Vec::new();
    let mut scanned_paths = included.clone();
    scanned_paths.extend(candidate_paths.clone());
    for rel_path in scanned_paths {
        let abs_path = vault_root.join(&rel_path);
        if !abs_path.exists() || abs_path.extension().and_then(|value| value.to_str()) != Some("md")
        {
            continue;
        }
        let record = load_markdown(&abs_path, Some(vault_root))?;
        let visibility_scope = record
            .frontmatter
            .get("visibility_scope")
            .and_then(Value::as_str)
            .unwrap_or("shared")
            .trim()
            .to_lowercase();
        if visibility_scope == "private" {
            scope_leaks.push(ScopeLeak {
                path: rel_path.clone(),
                visibility_scope: "private".to_string(),
                reason: "private_surface_in_default_query_scope".to_string(),
            });
        }
        let sensitivity_level = record
            .frontmatter
            .get("sensitivity_level")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .trim()
            .to_lowercase();
        if matches!(
            sensitivity_level.as_str(),
            "internal" | "restricted" | "secret"
        ) {
            sensitivity_candidates.push(SensitivityCandidate {
                path: rel_path.clone(),
                sensitivity_level,
                candidate_kind: if included.contains(&rel_path) {
                    "included".to_string()
                } else {
                    "candidate".to_string()
                },
            });
        }
    }

    candidate_paths.sort();
    candidate_paths.dedup();
    let scope = QueryScope {
        vault_root: crate::common::normalize_path_string(vault_root.to_string_lossy().as_ref()),
        layout_family,
        profile,
        included_paths: included,
        candidate_paths,
        candidate_policy: "candidate-only".to_string(),
        excluded_paths: excluded,
        excluded_prefixes: excluded_roots.iter().map(|s| (*s).to_string()).collect(),
        scope_leaks,
        sensitivity_candidates,
    };
    Ok(serde_json::to_value(&scope)?)
}

pub fn live_records(records: &[MarkdownRecord]) -> Vec<MarkdownRecord> {
    records
        .iter()
        .filter(|record| record.path.starts_with("wiki/live/"))
        .cloned()
        .collect()
}

fn tokenize(text: &str) -> BTreeSet<String> {
    TOKEN_RE
        .find_iter(text)
        .map(|value| value.as_str().to_lowercase())
        .collect()
}

pub fn rank_query_candidates(vault_root: &Path, query: &str) -> Result<Value> {
    let scope = query_scope(vault_root)?;
    let records = collect_markdown_records(vault_root)?
        .into_iter()
        .map(|record| (record.path.clone(), record))
        .collect::<std::collections::HashMap<_, _>>();
    let query_tokens = tokenize(query);
    let included = scope
        .get("included_paths")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|value| value.as_str().map(ToString::to_string))
        .collect::<Vec<_>>();
    let candidates = scope
        .get("candidate_paths")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|value| value.as_str().map(ToString::to_string))
        .collect::<Vec<_>>();
    let candidate_policy = scope
        .get("candidate_policy")
        .and_then(Value::as_str)
        .unwrap_or("candidate-only")
        .to_string();
    let mut ranked: Vec<RankedEntry> = Vec::new();

    for path in included.iter().chain(candidates.iter()) {
        let candidate_kind = if included.contains(path) {
            "included".to_string()
        } else {
            "candidate".to_string()
        };
        let truth_boundary = if candidate_kind == "included" {
            "approved-live".to_string()
        } else {
            candidate_policy.clone()
        };

        if path.ends_with("graph-snapshot.json") {
            let graph_path = vault_root.join(path);
            let title = graph_path
                .file_name()
                .map(|value| value.to_string_lossy().into_owned())
                .unwrap_or_default();
            let graph_text = std::fs::read_to_string(&graph_path).unwrap_or_default();
            let lexical_overlap = query_tokens
                .intersection(&tokenize(&format!("{title} {graph_text}")))
                .count() as i64;
            let metadata_score = 1_i64;
            let graph_score = 2_i64;
            let score = lexical_overlap * 10 + metadata_score * 2 + graph_score;
            ranked.push(RankedEntry {
                path: path.clone(),
                kind: "graph_snapshot".to_string(),
                candidate_kind,
                truth_boundary,
                score,
                lexical_overlap,
                metadata_score,
                graph_score,
                title,
            });
            continue;
        }

        let Some(record) = records.get(path) else {
            continue;
        };
        let title = record
            .frontmatter
            .get("title")
            .and_then(Value::as_str)
            .unwrap_or(&record.basename())
            .to_string();
        let mut text_tokens = tokenize(&title);
        text_tokens.extend(tokenize(&record.body));
        let lexical_overlap = query_tokens.intersection(&text_tokens).count() as i64;
        let mut metadata_score = 0_i64;
        if record.frontmatter.contains_key("topic_hub") {
            metadata_score += 1;
        }
        if record
            .frontmatter
            .get("visibility_scope")
            .and_then(Value::as_str)
            == Some("shared")
        {
            metadata_score += 1;
        }
        if matches!(
            record.kind.as_str(),
            "topic" | "concept" | "entity" | "procedure" | "briefing" | "qa"
        ) {
            metadata_score += 1;
        }
        if matches!(record.kind.as_str(), "briefing" | "qa") {
            metadata_score += 1;
        }

        let mut graph_score = 0_i64;
        graph_score += list_field(&record.frontmatter, "related").len() as i64;
        if record.frontmatter.contains_key("topic_hub") {
            graph_score += 1;
        }
        if record.frontmatter.contains_key("question_links") {
            graph_score += 1;
        }
        if record.frontmatter.contains_key("crystallized_from_episode") {
            graph_score += 1;
        }
        if record.kind == "topic" {
            graph_score += 2;
        }
        let score = lexical_overlap * 10 + metadata_score * 2 + graph_score;
        ranked.push(RankedEntry {
            path: path.clone(),
            kind: record.kind.clone(),
            candidate_kind,
            truth_boundary,
            score,
            lexical_overlap,
            metadata_score,
            graph_score,
            title,
        });
    }

    ranked.sort_by_key(|entry| {
        (
            -(entry.score as i128),
            entry.candidate_kind != "included",
            entry.path.clone(),
        )
    });

    let payload = QueryRanked {
        vault_root: crate::common::normalize_path_string(vault_root.to_string_lossy().as_ref()),
        query: query.to_string(),
        candidate_policy,
        included_count: included.len(),
        candidate_count: candidates.len(),
        ranked_paths: ranked,
    };
    Ok(serde_json::to_value(&payload)?)
}
