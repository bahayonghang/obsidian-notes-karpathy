use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use anyhow::{Context, Result};
use serde_json::{json, Value};

use super::{
    description_re, load_registry, name_re, read_utf8, runtime_evals_path, skill_paths,
    trigger_evals_path, writable_runtime_evals_path, Registry,
};

const USE_TRIGGER_PATTERNS: [&str; 2] = ["Use this skill when", "Use this skill whenever"];
const BOUNDARY_PATTERNS: [&str; 4] = ["Do not use", "Prefer ", "only route", "not "];
const OUTPUT_SECTION_PATTERNS: [&str; 3] = ["## Output", "## Outputs", "## Report output"];
const COMPATIBILITY_REFERENCE: &str = "chinese-llm-wiki-compat.md";
const COMPATIBILITY_TRIGGER_TOKENS: [&str; 9] = [
    "来源页",
    "主题页",
    "实体页",
    "综合页",
    "output/analyses",
    "output/reports",
    "中文优先",
    "原文证据摘录",
    "先读 wiki/index.md",
];
const COMPATIBILITY_REFERENCE_SKILLS: [&str; 4] = [
    "obsidian-notes-karpathy",
    "kb-init",
    "kb-query",
    "kb-review",
];
const COMPATIBILITY_ROUTE_SKILLS: [&str; 6] = [
    "obsidian-notes-karpathy",
    "kb-init",
    "kb-compile",
    "kb-review",
    "kb-query",
    "kb-render",
];

pub fn load_skill_catalog(repo_root: &Path) -> Result<BTreeMap<String, String>> {
    let mut catalog = BTreeMap::new();
    for (skill_name, skill_path) in skill_paths(repo_root) {
        let text = read_utf8(&skill_path)?;
        let description = parse_frontmatter_field(&text, &description_re()).with_context(|| {
            format!(
                "Missing description frontmatter in {}",
                skill_path.display()
            )
        })?;
        catalog.insert(skill_name, description);
    }
    Ok(catalog)
}

pub fn parse_frontmatter_field(text: &str, pattern: &regex::Regex) -> Option<String> {
    pattern
        .captures(text)
        .and_then(|captures| captures.get(1))
        .map(|value| value.as_str().trim().to_string())
}

struct EvalSkillSets {
    trigger_data: Vec<Value>,
    trigger_skills: BTreeSet<String>,
    runtime_skills: BTreeSet<String>,
    writable_skills: BTreeSet<String>,
}

fn load_eval_skill_sets(repo_root: &Path) -> Result<EvalSkillSets> {
    let trigger_data =
        serde_json::from_str::<Vec<Value>>(&read_utf8(&trigger_evals_path(repo_root))?)
            .context("parse trigger-evals.json")?;
    let runtime_data = serde_json::from_str::<Value>(&read_utf8(&runtime_evals_path(repo_root))?)
        .context("parse runtime-evals.json")?;
    let writable_data =
        serde_json::from_str::<Value>(&read_utf8(&writable_runtime_evals_path(repo_root))?)
            .context("parse runtime-evals-writable.json")?;

    let trigger_skills = trigger_data
        .iter()
        .filter_map(|item| item.get("expected_skill").and_then(Value::as_str))
        .map(ToString::to_string)
        .collect::<BTreeSet<_>>();
    let runtime_skills = runtime_data
        .get("evals")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|item| item.get("skill").and_then(Value::as_str))
        .map(ToString::to_string)
        .collect::<BTreeSet<_>>();
    let writable_skills = writable_data
        .get("evals")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|item| item.get("skill").and_then(Value::as_str))
        .map(ToString::to_string)
        .collect::<BTreeSet<_>>();

    Ok(EvalSkillSets {
        trigger_data,
        trigger_skills,
        runtime_skills,
        writable_skills,
    })
}

pub fn compatibility_trigger_report(trigger_data: &[Value]) -> Value {
    let serialized = serde_json::to_string(trigger_data).unwrap_or_default();
    let missing_tokens = COMPATIBILITY_TRIGGER_TOKENS
        .iter()
        .filter(|token| !serialized.contains(**token))
        .map(|token| Value::String((*token).to_string()))
        .collect::<Vec<_>>();

    let mut route_hits = COMPATIBILITY_ROUTE_SKILLS
        .iter()
        .map(|skill_name| ((*skill_name).to_string(), 0_i64))
        .collect::<BTreeMap<_, _>>();

    for item in trigger_data {
        let Some(query) = item.get("query").and_then(Value::as_str) else {
            continue;
        };
        if !COMPATIBILITY_TRIGGER_TOKENS
            .iter()
            .any(|token| query.contains(token))
        {
            continue;
        }
        let Some(expected_skill) = item.get("expected_skill").and_then(Value::as_str) else {
            continue;
        };
        if let Some(count) = route_hits.get_mut(expected_skill) {
            *count += 1;
        }
    }

    let missing_routes = route_hits
        .iter()
        .filter(|(_, count)| **count == 0)
        .map(|(skill_name, _)| Value::String(skill_name.clone()))
        .collect::<Vec<_>>();

    json!({
        "covered": missing_tokens.is_empty() && missing_routes.is_empty(),
        "missing_tokens": missing_tokens,
        "missing_routes": missing_routes,
        "route_hits": route_hits,
    })
}

fn has_any(text: &str, tokens: &[&str]) -> bool {
    tokens.iter().any(|token| text.contains(token))
}

fn audit_skill(
    skill_name: &str,
    skill_path: &Path,
    registry: &Registry,
    trigger_skills: &BTreeSet<String>,
    runtime_skills: &BTreeSet<String>,
    writable_skills: &BTreeSet<String>,
    repo_root: &Path,
) -> Result<Value> {
    let text = read_utf8(skill_path)?;
    let description = parse_frontmatter_field(&text, &description_re());
    let frontmatter_name = parse_frontmatter_field(&text, &name_re());
    let registry_entry = registry.skills.get(skill_name);
    let role = registry_entry
        .map(|entry| entry.role.clone())
        .unwrap_or_else(|| "unknown".to_string());
    let compatibility_reference_expected = COMPATIBILITY_REFERENCE_SKILLS.contains(&skill_name);

    let mut checks = BTreeMap::new();
    checks.insert(
        "frontmatter_name_matches".to_string(),
        Value::Bool(frontmatter_name.as_deref() == Some(skill_name)),
    );
    checks.insert(
        "description_present".to_string(),
        Value::Bool(description.is_some()),
    );
    checks.insert(
        "description_has_trigger_phrase".to_string(),
        Value::Bool(
            description
                .as_deref()
                .map(|value| has_any(value, &USE_TRIGGER_PATTERNS))
                .unwrap_or(false),
        ),
    );
    checks.insert(
        "description_has_boundary_language".to_string(),
        Value::Bool(
            description
                .as_deref()
                .map(|value| has_any(value, &BOUNDARY_PATTERNS))
                .unwrap_or(false),
        ),
    );
    checks.insert(
        "description_has_multilingual_trigger".to_string(),
        Value::Bool(
            description
                .as_deref()
                .map(|value| value.chars().any(|ch| ch as u32 > 127))
                .unwrap_or(false),
        ),
    );
    checks.insert(
        "has_read_before_section".to_string(),
        Value::Bool(text.contains("## Read before") || text.contains("## Scope")),
    );
    checks.insert(
        "has_output_section".to_string(),
        Value::Bool(has_any(&text, &OUTPUT_SECTION_PATTERNS)),
    );
    checks.insert(
        "mentions_registry".to_string(),
        Value::Bool(text.contains("skill-contract-registry.json")),
    );
    checks.insert(
        "mentions_baseline_command".to_string(),
        Value::Bool(
            registry_entry
                .map(|entry| {
                    !entry.baseline_command.is_empty() && text.contains(&entry.baseline_command)
                })
                .unwrap_or(false),
        ),
    );
    checks.insert(
        "mentions_cli_install_fallback".to_string(),
        Value::Bool(
            text.contains("If `onkb` is missing")
                || text.contains("If the shell reports `onkb` is not installed"),
        ),
    );
    checks.insert(
        "trigger_eval_covered".to_string(),
        Value::Bool(trigger_skills.contains(skill_name)),
    );
    checks.insert(
        "runtime_eval_covered".to_string(),
        Value::Bool(runtime_skills.contains(skill_name)),
    );
    checks.insert(
        "writable_runtime_covered".to_string(),
        Value::Bool(
            role != "operation"
                || registry_entry
                    .map(|entry| entry.writes.is_empty() || writable_skills.contains(skill_name))
                    .unwrap_or(true),
        ),
    );
    checks.insert(
        "compatibility_reference_covered".to_string(),
        Value::Bool(!compatibility_reference_expected || text.contains(COMPATIBILITY_REFERENCE)),
    );

    let blocking_keys = [
        "frontmatter_name_matches",
        "description_present",
        "description_has_trigger_phrase",
        "description_has_boundary_language",
        "has_read_before_section",
        "has_output_section",
        "mentions_registry",
        "mentions_baseline_command",
        "mentions_cli_install_fallback",
        "trigger_eval_covered",
        "runtime_eval_covered",
        "writable_runtime_covered",
        "compatibility_reference_covered",
    ];
    let blocking_issues = blocking_keys
        .iter()
        .filter(|key| checks.get(**key).and_then(Value::as_bool) != Some(true))
        .map(|key| Value::String((*key).to_string()))
        .collect::<Vec<_>>();

    let mut warnings = Vec::new();
    if checks
        .get("description_has_multilingual_trigger")
        .and_then(Value::as_bool)
        != Some(true)
    {
        warnings.push(Value::String(
            "description_has_multilingual_trigger".to_string(),
        ));
    }
    if text.lines().count() > 180 {
        warnings.push(Value::String("body_is_long".to_string()));
    }

    let passed_checks = checks
        .values()
        .filter(|value| value.as_bool() == Some(true))
        .count();
    let score = ((passed_checks as f64) / (checks.len() as f64) * 1000.0).round() / 1000.0;

    Ok(json!({
        "name": skill_name,
        "path": super::relative_to_repo(repo_root, skill_path),
        "role": role,
        "line_count": text.lines().count(),
        "description": description,
        "checks": checks,
        "blocking_issues": blocking_issues,
        "warnings": warnings,
        "score": score,
    }))
}

pub fn build_payload(repo_root: &Path, skill_filters: &[String]) -> Result<Value> {
    let eval_sets = load_eval_skill_sets(repo_root)?;
    let registry = load_registry(repo_root)?;
    let compatibility_report = compatibility_trigger_report(&eval_sets.trigger_data);

    let audits = skill_paths(repo_root)
        .iter()
        .filter(|(skill_name, _)| {
            skill_filters.is_empty() || skill_filters.iter().any(|item| item == *skill_name)
        })
        .map(|(skill_name, skill_path)| {
            audit_skill(
                skill_name,
                skill_path,
                &registry,
                &eval_sets.trigger_skills,
                &eval_sets.runtime_skills,
                &eval_sets.writable_skills,
                repo_root,
            )
        })
        .collect::<Result<Vec<_>>>()?;

    let blocking_issue_count = audits
        .iter()
        .map(|item| {
            item.get("blocking_issues")
                .and_then(Value::as_array)
                .map(Vec::len)
                .unwrap_or_default()
        })
        .sum::<usize>();
    let warning_count = audits
        .iter()
        .map(|item| {
            item.get("warnings")
                .and_then(Value::as_array)
                .map(Vec::len)
                .unwrap_or_default()
        })
        .sum::<usize>();
    let compatibility_reference_covered = audits
        .iter()
        .filter(|item| {
            item.get("name")
                .and_then(Value::as_str)
                .map(|name| COMPATIBILITY_REFERENCE_SKILLS.contains(&name))
                .unwrap_or(false)
        })
        .filter(|item| {
            item.get("checks")
                .and_then(|checks| checks.get("compatibility_reference_covered"))
                .and_then(Value::as_bool)
                == Some(true)
        })
        .count();
    let average_score = if audits.is_empty() {
        0.0
    } else {
        let total = audits
            .iter()
            .filter_map(|item| item.get("score").and_then(Value::as_f64))
            .sum::<f64>();
        ((total / audits.len() as f64) * 1000.0).round() / 1000.0
    };

    Ok(json!({
        "status": if blocking_issue_count == 0 { "ok" } else { "error" },
        "summary": {
            "skill_count": audits.len(),
            "selected_skills": skill_filters,
            "trigger_eval_covered": audits.iter().filter(|item| item.get("checks").and_then(|checks| checks.get("trigger_eval_covered")).and_then(Value::as_bool) == Some(true)).count(),
            "runtime_eval_covered": audits.iter().filter(|item| item.get("checks").and_then(|checks| checks.get("runtime_eval_covered")).and_then(Value::as_bool) == Some(true)).count(),
            "writable_runtime_covered": audits.iter().filter(|item| item.get("checks").and_then(|checks| checks.get("writable_runtime_covered")).and_then(Value::as_bool) == Some(true)).count(),
            "compatibility_reference_covered": compatibility_reference_covered,
            "compatibility_reference_expected": COMPATIBILITY_REFERENCE_SKILLS.len(),
            "chinese_llm_wiki_trigger_covered": compatibility_report.get("covered").and_then(Value::as_bool).unwrap_or(false),
            "blocking_issue_count": blocking_issue_count,
            "warning_count": warning_count,
            "average_score": average_score,
        },
        "compatibility": compatibility_report,
        "skills": audits,
    }))
}
