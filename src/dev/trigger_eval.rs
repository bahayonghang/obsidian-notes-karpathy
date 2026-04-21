use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde_json::{json, Value};

use super::runtime_eval::runner;
use super::skill_audit::load_skill_catalog;
use super::{read_utf8, trigger_evals_path};

pub fn build_trigger_prompt(
    query: &str,
    catalog: &std::collections::BTreeMap<String, String>,
) -> String {
    let skill_lines = catalog
        .iter()
        .map(|(name, description)| format!("- {name}: {description}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "You are running an offline trigger-classification benchmark.\nDo not answer the user request. Do not give policy advice. Do not explain how skills usually work.\nClassify which single skill should be consulted first for the request below.\nThe user request may be written in English or Chinese.\nTreat Chinese routing phrases such as 来源页, 主题页, 实体页, 综合页, output/analyses, output/reports, 中文优先, 原文证据摘录, and 先读 wiki/index.md as meaningful routing signals rather than defaulting to `none`.\nAvailable skills:\n{skill_lines}\n\nChoose exactly one skill from the list above, or `none` if none should be consulted first.\nPrefer operation-specific skills over the router when the operation is already clear.\nUse `none` only when the request clearly sits outside this Obsidian review-gated vault workflow.\nDo not invent companion skills that are not in the list.\nReturn exactly one minified JSON object and nothing else.\nRequired schema: {{\"selected_skill\":\"<skill-name-or-none>\",\"reason\":\"<short reason>\"}}\nIf you output anything before or after the JSON object, the benchmark run is invalid.\n\nUser request:\n{query}\n"
    )
}

pub fn parse_selected_skill(text: &str) -> Option<String> {
    let stripped = text.trim();
    if stripped.is_empty() {
        return None;
    }
    if let Ok(payload) = serde_json::from_str::<Value>(stripped) {
        if let Some(selected) = payload.get("selected_skill").and_then(Value::as_str) {
            if selected == "none" {
                return None;
            }
            return Some(selected.to_string());
        }
    }
    let pattern = regex::Regex::new(
        r#""selected_skill"\s*:\s*"(obsidian-notes-karpathy|kb-init|kb-ingest|kb-compile|kb-review|kb-query|kb-render|none)""#,
    )
    .expect("valid trigger selection regex");
    pattern.captures(stripped).and_then(|captures| {
        captures.get(1).and_then(|value| {
            if value.as_str() == "none" {
                None
            } else {
                Some(value.as_str().to_string())
            }
        })
    })
}

pub fn summarize_trigger_results(results: &[Value], skill_names: &[String]) -> Value {
    let total = results.len() as i64;
    let matched = results
        .iter()
        .filter(|item| item.get("matched").and_then(Value::as_bool) == Some(true))
        .count() as i64;

    let mut per_skill = serde_json::Map::new();
    for skill_name in skill_names
        .iter()
        .cloned()
        .chain(std::iter::once("none".to_string()))
    {
        let true_positive = results
            .iter()
            .filter(|item| {
                item.get("expected_skill")
                    .and_then(Value::as_str)
                    .unwrap_or("none")
                    == skill_name
                    && item
                        .get("actual_skill")
                        .and_then(Value::as_str)
                        .unwrap_or("none")
                        == skill_name
            })
            .count() as i64;
        let false_positive = results
            .iter()
            .filter(|item| {
                item.get("expected_skill")
                    .and_then(Value::as_str)
                    .unwrap_or("none")
                    != skill_name
                    && item
                        .get("actual_skill")
                        .and_then(Value::as_str)
                        .unwrap_or("none")
                        == skill_name
            })
            .count() as i64;
        let false_negative = results
            .iter()
            .filter(|item| {
                item.get("expected_skill")
                    .and_then(Value::as_str)
                    .unwrap_or("none")
                    == skill_name
                    && item
                        .get("actual_skill")
                        .and_then(Value::as_str)
                        .unwrap_or("none")
                        != skill_name
            })
            .count() as i64;
        let precision = if true_positive + false_positive > 0 {
            Some(true_positive as f64 / (true_positive + false_positive) as f64)
        } else {
            None
        };
        let recall = if true_positive + false_negative > 0 {
            Some(true_positive as f64 / (true_positive + false_negative) as f64)
        } else {
            None
        };
        per_skill.insert(
            skill_name.clone(),
            json!({
                "expected": results.iter().filter(|item| item.get("expected_skill").and_then(Value::as_str).unwrap_or("none") == skill_name).count(),
                "actual": results.iter().filter(|item| item.get("actual_skill").and_then(Value::as_str).unwrap_or("none") == skill_name).count(),
                "true_positive": true_positive,
                "false_positive": false_positive,
                "false_negative": false_negative,
                "precision": precision,
                "recall": recall,
            }),
        );
    }

    json!({
        "total": total,
        "matched": matched,
        "accuracy": if total > 0 { Some(matched as f64 / total as f64) } else { None::<f64> },
        "per_skill": per_skill,
        "false_positive_count": results.iter().filter(|item| {
            item.get("expected_skill").and_then(Value::as_str) != item.get("actual_skill").and_then(Value::as_str)
                && item.get("actual_skill").is_some()
                && !item.get("actual_skill").unwrap_or(&Value::Null).is_null()
        }).count(),
        "false_negative_count": results.iter().filter(|item| {
            !item.get("expected_skill").unwrap_or(&Value::Null).is_null()
                && item.get("expected_skill").and_then(Value::as_str) != item.get("actual_skill").and_then(Value::as_str)
        }).count(),
    })
}

fn load_eval_set(path: &Path) -> Result<Vec<Value>> {
    serde_json::from_str(&read_utf8(path)?).with_context(|| format!("parse {}", path.display()))
}

fn default_workspace_root(repo_root: &Path) -> PathBuf {
    repo_root.join(".trigger-evals")
}

pub fn run_trigger_eval(
    repo_root: &Path,
    eval_set: Option<&Path>,
    runner: Option<&str>,
    workspace: Option<&Path>,
    dry_run: bool,
    limit: Option<usize>,
    timeout_sec: u64,
) -> Result<Value> {
    let eval_set_path = eval_set
        .map(PathBuf::from)
        .unwrap_or_else(|| trigger_evals_path(repo_root));
    let mut evals = load_eval_set(&eval_set_path)?;
    if let Some(limit) = limit {
        evals.truncate(limit);
    }
    let runner = runner::detect_runner(runner);
    let workspace_root = workspace
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            default_workspace_root(repo_root).join(crate::dev::runtime_eval::utc_stamp())
        })
        .to_path_buf();
    let catalog = load_skill_catalog(repo_root)?;

    if runner.is_none() {
        return Ok(json!({
            "status": "skipped",
            "workspace": workspace_root.to_string_lossy(),
            "reason": "No supported runner found.",
            "eval_count": evals.len(),
        }));
    }
    let runner = runner.unwrap();

    if dry_run {
        return Ok(json!({
            "status": "planned",
            "workspace": workspace_root.to_string_lossy(),
            "runner": runner,
            "eval_count": evals.len(),
            "skills": catalog.keys().cloned().collect::<Vec<_>>(),
        }));
    }

    fs::create_dir_all(&workspace_root)
        .with_context(|| format!("create {}", workspace_root.display()))?;
    let mut results = Vec::new();
    for (index, item) in evals.iter().enumerate() {
        let run_dir = workspace_root.join(format!("{:02}", index + 1));
        let query = item
            .get("query")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let prompt = build_trigger_prompt(query, &catalog);
        let result_payload = runner::execute_run(
            &runner,
            &prompt,
            &run_dir,
            timeout_sec,
            "read-only",
            repo_root,
        )?;
        let output_text = fs::read_to_string(run_dir.join("last_message.txt")).unwrap_or_default();
        let actual_skill = parse_selected_skill(&output_text);
        let expected_skill = item
            .get("expected_skill")
            .and_then(Value::as_str)
            .map(ToString::to_string);
        let run_result = json!({
            "query": query,
            "expected_skill": expected_skill,
            "actual_skill": actual_skill,
            "matched": expected_skill == actual_skill,
            "runner": result_payload.get("runner").cloned().unwrap_or(Value::Null),
            "reason": item.get("reason").cloned().unwrap_or(Value::Null),
        });
        fs::write(
            run_dir.join("selection.json"),
            serde_json::to_string_pretty(&run_result)?,
        )
        .with_context(|| format!("write {}", run_dir.join("selection.json").display()))?;
        results.push(run_result);
    }

    let summary = summarize_trigger_results(&results, &catalog.keys().cloned().collect::<Vec<_>>());
    let payload = json!({
        "status": "completed",
        "workspace": workspace_root.to_string_lossy(),
        "runner": runner,
        "eval_count": evals.len(),
        "results": results,
        "summary": summary,
    });
    fs::write(
        workspace_root.join("summary.json"),
        serde_json::to_string_pretty(&payload)?,
    )
    .with_context(|| format!("write {}", workspace_root.join("summary.json").display()))?;
    Ok(payload)
}
