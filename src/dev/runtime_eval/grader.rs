use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde_json::{Value, json};

use crate::common::MARKDOWN_LINK_RE;

use super::paths::{entry_relative, normalize_path, repo_relative};

pub fn resolve_source_vault_root(repo_root: &Path, item: &Value) -> Result<PathBuf> {
    let vault_root = item
        .get("vault_root")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow::anyhow!("Missing vault_root"))?;
    Ok(entry_relative(repo_root, vault_root))
}

fn copy_tree(source: &Path, target: &Path) -> Result<()> {
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

pub fn prepare_target_vault(
    source_vault_root: &Path,
    eval_dir: &Path,
    run_label: &str,
    mode: &str,
) -> Result<PathBuf> {
    if mode == "read-only" {
        return Ok(source_vault_root.to_path_buf());
    }

    let target_vault_root = eval_dir
        .join("targets")
        .join(run_label)
        .join(source_vault_root.file_name().unwrap_or_default())
        .to_path_buf();
    if target_vault_root.exists() {
        fs::remove_dir_all(&target_vault_root)
            .with_context(|| format!("remove {}", target_vault_root.display()))?;
    }
    if let Some(parent) = target_vault_root.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    copy_tree(source_vault_root, &target_vault_root)?;
    Ok(target_vault_root)
}

pub fn materialize_files(
    repo_root: &Path,
    source_vault_root: &Path,
    target_vault_root: &Path,
    file_paths: &[String],
) -> Result<Vec<PathBuf>> {
    let mut resolved = Vec::new();
    for file_path in file_paths {
        let source_file = entry_relative(repo_root, file_path);
        if target_vault_root == source_vault_root {
            resolved.push(source_file);
            continue;
        }

        let relative_inside_vault =
            source_file
                .strip_prefix(source_vault_root)
                .with_context(|| {
                    format!(
                        "strip prefix {} from {}",
                        source_vault_root.display(),
                        source_file.display()
                    )
                })?;
        resolved.push(normalize_path(
            &target_vault_root.join(relative_inside_vault),
        ));
    }
    Ok(resolved)
}

pub fn detect_root_leakage(text: &str, vault_root: &Path, repo_root: &Path) -> Value {
    let scrubbed = MARKDOWN_LINK_RE.replace_all(text, "$1");
    let normalized = scrubbed.replace('\\', "/").to_ascii_lowercase();
    let repo_root = normalize_path(repo_root)
        .to_string_lossy()
        .replace('\\', "/")
        .to_ascii_lowercase();
    let declared_root = normalize_path(vault_root)
        .to_string_lossy()
        .replace('\\', "/")
        .to_ascii_lowercase();
    let mut reasons = Vec::new();

    if declared_root != repo_root && normalized.contains(&repo_root) {
        reasons.push("repo_root_path_mentioned".to_string());
    }
    if declared_root != repo_root && normalized.contains("current workspace root") {
        reasons.push("current_workspace_root_referenced".to_string());
    }

    json!({
        "detected": !reasons.is_empty(),
        "reasons": reasons,
    })
}

fn file_digest(path: &Path) -> Result<String> {
    use sha2::{Digest, Sha256};

    let bytes = fs::read(path).with_context(|| format!("read {}", path.display()))?;
    let mut digest = Sha256::new();
    digest.update(bytes);
    Ok(digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

fn tree_snapshot(root: &Path) -> Result<BTreeMap<String, String>> {
    let mut snapshot = BTreeMap::new();
    if !root.exists() {
        return Ok(snapshot);
    }
    for entry in walkdir::WalkDir::new(root) {
        let entry = match entry {
            Ok(value) => value,
            Err(_) => continue,
        };
        if !entry.file_type().is_file() {
            continue;
        }
        snapshot.insert(
            entry
                .path()
                .strip_prefix(root)
                .unwrap_or(entry.path())
                .to_string_lossy()
                .replace('\\', "/"),
            file_digest(entry.path())?,
        );
    }
    Ok(snapshot)
}

pub fn run_checks(
    repo_root: &Path,
    target_vault_root: &Path,
    source_vault_root: &Path,
    checks: &[Value],
) -> Result<Value> {
    fn matches_glob_pattern(pattern: &str, candidate: &str) -> bool {
        let mut regex_pattern = regex::escape(pattern);
        regex_pattern = regex_pattern.replace(r"\*\*", ".*");
        regex_pattern = regex_pattern.replace(r"\*", "[^/]*");
        regex::Regex::new(&format!("^{regex_pattern}$"))
            .expect("valid generated glob regex")
            .is_match(candidate)
    }

    let mut results = Vec::new();
    for check in checks {
        let kind = check
            .get("kind")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let name = check
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let (passed, evidence) = match kind {
            "exists" => {
                let target = target_vault_root.join(
                    check
                        .get("path")
                        .and_then(Value::as_str)
                        .unwrap_or_default(),
                );
                (
                    target.exists(),
                    if target.exists() {
                        repo_relative(repo_root, &target)
                    } else {
                        format!(
                            "missing:{}",
                            check
                                .get("path")
                                .and_then(Value::as_str)
                                .unwrap_or_default()
                        )
                    },
                )
            }
            "dir_non_empty" => {
                let target = target_vault_root.join(
                    check
                        .get("path")
                        .and_then(Value::as_str)
                        .unwrap_or_default(),
                );
                let passed = target.exists()
                    && target
                        .read_dir()
                        .map(|mut it| it.next().is_some())
                        .unwrap_or(false);
                (
                    passed,
                    if passed {
                        repo_relative(repo_root, &target)
                    } else {
                        format!(
                            "empty:{}",
                            check
                                .get("path")
                                .and_then(Value::as_str)
                                .unwrap_or_default()
                        )
                    },
                )
            }
            "same_tree" => {
                let relative = check
                    .get("path")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                let source = source_vault_root.join(relative);
                let target = target_vault_root.join(relative);
                let passed = tree_snapshot(&source)? == tree_snapshot(&target)?;
                (
                    passed,
                    if passed {
                        relative.to_string()
                    } else {
                        format!("tree_mismatch:{relative}")
                    },
                )
            }
            "glob_count_gte" => {
                let pattern = check
                    .get("pattern")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                let minimum = check
                    .get("min_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default() as usize;
                let match_count = walkdir::WalkDir::new(target_vault_root)
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter(|entry| entry.file_type().is_file())
                    .filter(|entry| {
                        let relative = entry
                            .path()
                            .strip_prefix(target_vault_root)
                            .unwrap_or(entry.path())
                            .to_string_lossy()
                            .replace('\\', "/");
                        matches_glob_pattern(pattern, &relative)
                    })
                    .count();
                (
                    match_count >= minimum,
                    format!("matches={match_count} pattern={pattern}"),
                )
            }
            "file_contains" => {
                let relative = check
                    .get("path")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                let target = target_vault_root.join(relative);
                let needle = check
                    .get("needle")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                let passed = target.exists()
                    && fs::read_to_string(&target)
                        .unwrap_or_default()
                        .contains(needle);
                (
                    passed,
                    if passed {
                        repo_relative(repo_root, &target)
                    } else {
                        format!("missing_needle:{relative}")
                    },
                )
            }
            _ => (false, format!("unsupported_check_kind:{kind}")),
        };
        results.push(json!({
            "name": name,
            "kind": kind,
            "passed": passed,
            "evidence": evidence,
        }));
    }

    Ok(json!({
        "total": results.len(),
        "passed": results.iter().filter(|result| result.get("passed").and_then(Value::as_bool) == Some(true)).count(),
        "results": results,
    }))
}
