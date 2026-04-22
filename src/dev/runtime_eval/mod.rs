use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Utc;
use serde_json::{Value, json};

use crate::dev::{Registry, read_utf8};

pub mod grader;
pub mod paths;
pub mod runner;

pub use grader::detect_root_leakage;
pub use runner::{classify_failure, fallback_runner_for};

pub struct RuntimeEvalOptions<'a> {
    pub manifest_override: Option<&'a Path>,
    pub runner: Option<&'a str>,
    pub workspace: Option<&'a Path>,
    pub reuse_baseline_from: Option<&'a Path>,
    pub skills: &'a [String],
    pub eval_ids: &'a [String],
    pub dry_run: bool,
    pub limit: Option<usize>,
    pub timeout_sec: u64,
    pub writable: bool,
}

pub fn utc_stamp() -> String {
    Utc::now().format("%Y%m%dT%H%M%SZ").to_string()
}

pub fn load_manifest(path: &Path) -> Result<Value> {
    serde_json::from_str(&read_utf8(path)?).with_context(|| format!("parse {}", path.display()))
}

pub fn validate_manifest(manifest: &Value, registry: &Registry) -> Vec<String> {
    let mut errors = Vec::new();
    let Some(evals) = manifest.get("evals").and_then(Value::as_array) else {
        return vec!["Manifest must contain an 'evals' list.".to_string()];
    };

    for item in evals {
        let eval_id = item
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("<missing-id>");
        for field in ["id", "skill", "prompt", "files", "vault_root"] {
            if item.get(field).is_none() {
                errors.push(format!("{eval_id}: missing required field '{field}'."));
            }
        }
        let skill = item
            .get("skill")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if !registry.skills.contains_key(skill) {
            errors.push(format!("{eval_id}: unsupported skill '{skill}'."));
        }

        let Some(files) = item.get("files").and_then(Value::as_array) else {
            errors.push(format!("{eval_id}: files must be a non-empty list."));
            continue;
        };
        if files.is_empty() {
            errors.push(format!("{eval_id}: files must be a non-empty list."));
            continue;
        }

        let mode = item
            .get("mode")
            .and_then(Value::as_str)
            .unwrap_or("read-only");
        if !paths::SUPPORTED_MODES.contains(&mode) {
            errors.push(format!("{eval_id}: unsupported mode '{mode}'."));
        }

        let vault_root = item.get("vault_root").and_then(Value::as_str);
        let vault_fixture_root = vault_root.and_then(paths::fixture_root_for_relpath);
        if vault_fixture_root.is_none() {
            errors.push(format!(
                "{eval_id}: vault_root must point at a fixture root under evals/fixtures/."
            ));
        }

        let mut fixture_roots = files
            .iter()
            .filter_map(Value::as_str)
            .map(paths::fixture_root_for_relpath)
            .collect::<Vec<_>>();
        if fixture_roots.iter().any(|value| value.is_none()) {
            errors.push(format!(
                "{eval_id}: all files must live under evals/fixtures/."
            ));
        } else {
            fixture_roots.sort();
            fixture_roots.dedup();
            if fixture_roots.len() != 1 {
                errors.push(format!(
                    "{eval_id}: files must belong to exactly one fixture root."
                ));
            } else if vault_fixture_root.is_some()
                && fixture_roots.first().cloned().flatten() != vault_fixture_root
            {
                errors.push(format!(
                    "{eval_id}: files do not match the declared vault_root."
                ));
            }
        }

        if let Some(checks) = item.get("checks")
            && !checks.is_array()
        {
            errors.push(format!("{eval_id}: checks must be a list when provided."));
        }
    }

    errors
}

pub fn build_prompt(
    use_skill: bool,
    skill: &str,
    prompt: &str,
    files: &[String],
    vault_root: &str,
    mode: &str,
    repo_root: &Path,
) -> Result<String> {
    let file_block = files
        .iter()
        .map(|file_path| format!("- {file_path}"))
        .collect::<Vec<_>>()
        .join("\n");
    let root_instruction = format!(
        "Treat `{vault_root}` as the only target vault root for this task.\nThe listed files are evidence from that vault, not alternate roots.\nDo not reason about the repository root as if it were the vault.\n"
    );
    let common = if mode == "writable-copy" {
        "You may modify files only under the declared vault root.\nDo not modify repository-tracked files outside that target vault copy.\nUse only repository-local evidence from the declared vault root and the listed files.\nPrefer the listed files over open-ended repo exploration.\nIf search is required, prefer rg over grep for repository-local searches.\nOn Windows, avoid shell globs inside literal paths. Prefer reading the exact listed files, and if search is required use PowerShell-native commands instead of rg against wildcarded absolute paths.\nDo not quote or restate the task instructions verbatim.\nDo not echo full absolute repository paths back to the user; prefer repo-relative or vault-relative paths.\nReturn a concise answer with:\n1. the conclusion\n2. the files used\n3. any assumptions\n"
    } else {
        "Work in read-only mode. Do not modify any files.\nUse only repository-local evidence from the declared vault root and the listed files.\nPrefer the listed files over open-ended repo exploration.\nIf search is required, prefer rg over grep for repository-local searches.\nOn Windows, avoid shell globs inside literal paths. Prefer reading the exact listed files, and if search is required use PowerShell-native commands instead of rg against wildcarded absolute paths.\nDo not quote or restate the task instructions verbatim.\nDo not echo full absolute repository paths back to the user; prefer repo-relative or vault-relative paths.\nReturn a concise answer with:\n1. the conclusion\n2. the files used\n3. any assumptions\n"
    };

    if use_skill {
        let skill_path = paths::skill_doc_path(repo_root, skill)?;
        return Ok(format!(
            "Read and follow the skill at `{}` for this task.\n{root_instruction}{common}\nTask:\n{prompt}\n\nRelevant files:\n{file_block}\n",
            paths::repo_relative(repo_root, &skill_path)
        ));
    }

    Ok(format!(
        "Do not use any external skill instructions beyond the repository itself.\n{root_instruction}{common}\nTask:\n{prompt}\n\nRelevant files:\n{file_block}\n"
    ))
}

pub fn build_plan_payload(
    manifest: &Value,
    workspace_root: &Path,
    runner: Option<&str>,
    status: &str,
    reason: Option<&str>,
) -> Value {
    let evals = manifest
        .get("evals")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let skills = evals
        .iter()
        .filter_map(|item| item.get("skill").and_then(Value::as_str))
        .collect::<std::collections::BTreeSet<_>>();
    let eval_ids = evals
        .iter()
        .filter_map(|item| item.get("id").and_then(Value::as_str))
        .collect::<std::collections::BTreeSet<_>>();
    let modes = evals
        .iter()
        .map(|item| {
            item.get("mode")
                .and_then(Value::as_str)
                .unwrap_or("read-only")
        })
        .collect::<std::collections::BTreeSet<_>>();
    json!({
        "status": status,
        "runner": runner,
        "workspace": workspace_root.to_string_lossy(),
        "eval_count": evals.len(),
        "reason": reason,
        "skills": skills,
        "eval_ids": eval_ids,
        "modes": modes,
    })
}

/*
 * ========================================================================
 * 步骤1：按 skill 和 eval id 缩小运行时评估集合
 * ========================================================================
 * 目标：
 * 1) 让 Darwin 式迭代可以只跑目标 skill 或目标 case
 * 2) 保持现有 manifest 结构和默认全量行为不变
 */
fn filter_manifest_evals(manifest: &mut Value, skills: &[String], eval_ids: &[String]) {
    let Some(evals) = manifest.get_mut("evals").and_then(Value::as_array_mut) else {
        return;
    };
    if skills.is_empty() && eval_ids.is_empty() {
        return;
    }

    let allowed_skills = skills
        .iter()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    let allowed_eval_ids = eval_ids
        .iter()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();

    evals.retain(|item| {
        let skill_ok = allowed_skills.is_empty()
            || item
                .get("skill")
                .and_then(Value::as_str)
                .map(|skill| allowed_skills.contains(skill))
                .unwrap_or(false);
        let eval_id_ok = allowed_eval_ids.is_empty()
            || item
                .get("id")
                .and_then(Value::as_str)
                .map(|eval_id| allowed_eval_ids.contains(eval_id))
                .unwrap_or(false);
        skill_ok && eval_id_ok
    });
}

/*
 * ========================================================================
 * 步骤3：复用上一轮成功的 baseline 产物
 * ========================================================================
 * 目标：
 * 1) 在 read-only 评估里只重跑 with-skill
 * 2) 用上一轮稳定 baseline 缩短迭代回路
 */
fn copy_run_tree(source: &Path, target: &Path) -> Result<()> {
    for entry in walkdir::WalkDir::new(source) {
        let entry = match entry {
            Ok(value) => value,
            Err(_) => continue,
        };
        let relative = entry.path().strip_prefix(source).unwrap_or(entry.path());
        let destination = target.join(relative);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&destination)
                .with_context(|| format!("create {}", destination.display()))?;
        } else {
            if let Some(parent) = destination.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("create {}", parent.display()))?;
            }
            fs::copy(entry.path(), &destination)
                .with_context(|| format!("copy {}", entry.path().display()))?;
        }
    }
    Ok(())
}

pub fn try_reuse_baseline_artifacts(
    repo_root: &Path,
    previous_workspace: Option<&Path>,
    skill: &str,
    eval_id: &str,
    mode: &str,
    current_metadata: &Value,
    baseline_dir: &Path,
) -> Result<Option<Value>> {
    if mode != "read-only" {
        return Ok(None);
    }
    let Some(previous_workspace) = previous_workspace else {
        return Ok(None);
    };

    let previous_eval_dir = previous_workspace.join(skill).join(eval_id);
    let previous_metadata_path = previous_eval_dir.join("metadata.json");
    let previous_baseline_dir = previous_eval_dir.join("baseline");
    let previous_result_path = previous_baseline_dir.join("result.json");
    if !previous_metadata_path.exists() || !previous_result_path.exists() {
        return Ok(None);
    }

    let previous_metadata = serde_json::from_str::<Value>(&read_utf8(&previous_metadata_path)?)
        .with_context(|| format!("parse {}", previous_metadata_path.display()))?;
    if previous_metadata != *current_metadata {
        return Ok(None);
    }

    let mut previous_result = serde_json::from_str::<Value>(&read_utf8(&previous_result_path)?)
        .with_context(|| format!("parse {}", previous_result_path.display()))?;
    if previous_result.get("failure_kind").and_then(Value::as_str) != Some("ok") {
        return Ok(None);
    }
    if previous_result
        .get("output_captured")
        .and_then(Value::as_bool)
        != Some(true)
    {
        return Ok(None);
    }

    if baseline_dir.exists() {
        fs::remove_dir_all(baseline_dir)
            .with_context(|| format!("remove {}", baseline_dir.display()))?;
    }
    copy_run_tree(&previous_baseline_dir, baseline_dir)?;
    previous_result["reused"] = Value::Bool(true);
    previous_result["reused_from"] =
        Value::String(paths::repo_relative(repo_root, &previous_baseline_dir));
    Ok(Some(previous_result))
}

pub fn run_runtime_eval(
    repo_root: &Path,
    registry: &Registry,
    options: RuntimeEvalOptions<'_>,
) -> Result<Value> {
    let manifest_path = options
        .manifest_override
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            if options.writable {
                super::writable_runtime_evals_path(repo_root)
            } else {
                super::runtime_evals_path(repo_root)
            }
        });
    let mut manifest = load_manifest(&manifest_path)?;
    filter_manifest_evals(&mut manifest, options.skills, options.eval_ids);
    if let Some(limit) = options.limit {
        let trimmed = manifest
            .get("evals")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .take(limit)
            .collect::<Vec<_>>();
        manifest["evals"] = Value::Array(trimmed);
    }

    let validation_errors = validate_manifest(&manifest, registry);
    let workspace_root = options
        .workspace
        .map(PathBuf::from)
        .unwrap_or_else(|| paths::default_workspace_root(repo_root).join(utc_stamp()))
        .to_path_buf();
    if !validation_errors.is_empty() {
        return Ok(json!({
            "status": "error",
            "workspace": workspace_root.to_string_lossy(),
            "errors": validation_errors,
        }));
    }

    let runner = runner::detect_runner(options.runner);
    if runner.is_none() {
        return Ok(build_plan_payload(
            &manifest,
            &workspace_root,
            None,
            "skipped",
            Some("No supported runner found."),
        ));
    }
    let runner = runner.unwrap();

    if options.dry_run {
        return Ok(build_plan_payload(
            &manifest,
            &workspace_root,
            Some(&runner),
            "planned",
            None,
        ));
    }

    fs::create_dir_all(&workspace_root)
        .with_context(|| format!("create {}", workspace_root.display()))?;
    let evals = manifest
        .get("evals")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut summary = Vec::new();
    for item in evals {
        let eval_id = item
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("missing-id");
        let skill = item
            .get("skill")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let mode = item
            .get("mode")
            .and_then(Value::as_str)
            .unwrap_or("read-only");
        let eval_dir = workspace_root.join(skill).join(eval_id);
        let with_skill_dir = eval_dir.join("with_skill");
        let baseline_dir = eval_dir.join("baseline");
        let source_vault_root = grader::resolve_source_vault_root(repo_root, &item)?;
        let files = item
            .get("files")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter_map(|value| value.as_str().map(ToString::to_string))
            .collect::<Vec<_>>();
        let metadata = json!({
            "id": eval_id,
            "skill": skill,
            "files": files,
            "prompt": item.get("prompt").cloned().unwrap_or(Value::Null),
            "mode": mode,
            "source_vault_root": paths::repo_relative(repo_root, &source_vault_root),
            "checks": item.get("checks").cloned().unwrap_or_else(|| json!([])),
        });
        fs::create_dir_all(&eval_dir).with_context(|| format!("create {}", eval_dir.display()))?;
        fs::write(
            eval_dir.join("metadata.json"),
            serde_json::to_string_pretty(&metadata)?,
        )
        .with_context(|| format!("write {}", eval_dir.join("metadata.json").display()))?;

        let with_skill_target_root =
            grader::prepare_target_vault(&source_vault_root, &eval_dir, "with_skill", mode)?;
        let baseline_target_root =
            grader::prepare_target_vault(&source_vault_root, &eval_dir, "baseline", mode)?;
        let with_skill_files = grader::materialize_files(
            repo_root,
            &source_vault_root,
            &with_skill_target_root,
            &files,
        )?;
        let baseline_files = grader::materialize_files(
            repo_root,
            &source_vault_root,
            &baseline_target_root,
            &files,
        )?;
        let sandbox_mode = if mode == "read-only" {
            "read-only"
        } else {
            "workspace-write"
        };

        let with_skill_prompt = build_prompt(
            true,
            skill,
            item.get("prompt")
                .and_then(Value::as_str)
                .unwrap_or_default(),
            &with_skill_files
                .iter()
                .map(|path| paths::repo_relative(repo_root, path))
                .collect::<Vec<_>>(),
            &paths::repo_relative(repo_root, &with_skill_target_root),
            mode,
            repo_root,
        )?;
        let baseline_prompt = build_prompt(
            false,
            skill,
            item.get("prompt")
                .and_then(Value::as_str)
                .unwrap_or_default(),
            &baseline_files
                .iter()
                .map(|path| paths::repo_relative(repo_root, path))
                .collect::<Vec<_>>(),
            &paths::repo_relative(repo_root, &baseline_target_root),
            mode,
            repo_root,
        )?;

        let mut with_skill_result = runner::execute_run(
            &runner,
            &with_skill_prompt,
            &with_skill_dir,
            options.timeout_sec,
            sandbox_mode,
            repo_root,
        )?;
        let mut baseline_result = if let Some(reused) = try_reuse_baseline_artifacts(
            repo_root,
            options.reuse_baseline_from,
            skill,
            eval_id,
            mode,
            &metadata,
            &baseline_dir,
        )? {
            reused
        } else {
            runner::execute_run(
                &runner,
                &baseline_prompt,
                &baseline_dir,
                options.timeout_sec,
                sandbox_mode,
                repo_root,
            )?
        };

        for (run_dir, result_payload, target_vault_root) in [
            (
                &with_skill_dir,
                &mut with_skill_result,
                &with_skill_target_root,
            ),
            (&baseline_dir, &mut baseline_result, &baseline_target_root),
        ] {
            let last_message =
                fs::read_to_string(run_dir.join("last_message.txt")).unwrap_or_default();
            let leakage = grader::detect_root_leakage(&last_message, target_vault_root, repo_root);
            fs::write(
                run_dir.join("root_leakage.json"),
                serde_json::to_string_pretty(&leakage)?,
            )
            .with_context(|| format!("write {}", run_dir.join("root_leakage.json").display()))?;
            result_payload["root_leakage"] = leakage;

            let checks_payload = grader::run_checks(
                repo_root,
                target_vault_root,
                &source_vault_root,
                item.get("checks")
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default()
                    .as_slice(),
            )?;
            fs::write(
                run_dir.join("checks.json"),
                serde_json::to_string_pretty(&checks_payload)?,
            )
            .with_context(|| format!("write {}", run_dir.join("checks.json").display()))?;
            result_payload["checks"] = checks_payload;
            result_payload["target_vault_root"] =
                Value::String(paths::repo_relative(repo_root, target_vault_root));
            result_payload["mode"] = Value::String(mode.to_string());
            fs::write(
                run_dir.join("result.json"),
                serde_json::to_string_pretty(result_payload)?,
            )
            .with_context(|| format!("write {}", run_dir.join("result.json").display()))?;
        }

        summary.push(json!({
            "id": eval_id,
            "skill": skill,
            "mode": mode,
            "source_vault_root": paths::repo_relative(repo_root, &source_vault_root),
            "with_skill": with_skill_result,
            "baseline": baseline_result,
        }));
    }

    let mut payload =
        build_plan_payload(&manifest, &workspace_root, Some(&runner), "completed", None);
    payload["runs"] = Value::Array(summary);
    Ok(payload)
}
