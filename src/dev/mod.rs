use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use regex::Regex;
use serde::Deserialize;

pub mod contract;
pub mod reference_blocks;
pub mod runtime_eval;
pub mod skill_audit;
pub mod trigger_eval;

#[derive(Clone, Debug, Deserialize)]
pub struct Registry {
    pub contract_family: String,
    #[serde(default)]
    pub shared_references: Vec<String>,
    pub skills: BTreeMap<String, SkillEntry>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SkillEntry {
    pub role: String,
    #[serde(default)]
    pub reads: Vec<String>,
    #[serde(default)]
    pub baseline_command: String,
    #[serde(default)]
    pub writes: Vec<String>,
    #[serde(default)]
    pub outputs: Vec<String>,
    #[serde(default)]
    pub writes_mechanical_fix_only: Vec<String>,
    #[serde(default)]
    pub routes: Vec<String>,
}

pub fn current_repo_root() -> Result<PathBuf> {
    let current_dir = env::current_dir().context("read current directory")?;
    if let Some(found) = find_repo_root(&current_dir) {
        return Ok(found);
    }

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if let Some(found) = find_repo_root(&manifest_dir) {
        return Ok(found);
    }

    Err(anyhow!("Could not locate repo root for onkb dev commands."))
}

fn find_repo_root(start: &Path) -> Option<PathBuf> {
    for ancestor in start.ancestors() {
        let registry_path = ancestor
            .join("skills")
            .join("obsidian-notes-karpathy")
            .join("scripts")
            .join("skill-contract-registry.json");
        if ancestor.join("Cargo.toml").exists() && registry_path.exists() {
            return Some(ancestor.to_path_buf());
        }
    }
    None
}

pub fn entry_skill_root(repo_root: &Path) -> PathBuf {
    repo_root.join("skills").join("obsidian-notes-karpathy")
}

pub fn registry_path(repo_root: &Path) -> PathBuf {
    entry_skill_root(repo_root)
        .join("scripts")
        .join("skill-contract-registry.json")
}

pub fn runtime_evals_path(repo_root: &Path) -> PathBuf {
    entry_skill_root(repo_root)
        .join("evals")
        .join("runtime-evals.json")
}

pub fn writable_runtime_evals_path(repo_root: &Path) -> PathBuf {
    entry_skill_root(repo_root)
        .join("evals")
        .join("runtime-evals-writable.json")
}

pub fn trigger_evals_path(repo_root: &Path) -> PathBuf {
    entry_skill_root(repo_root)
        .join("evals")
        .join("trigger-evals.json")
}

pub fn skill_paths(repo_root: &Path) -> BTreeMap<String, PathBuf> {
    let entry_root = entry_skill_root(repo_root);
    BTreeMap::from([
        (
            "obsidian-notes-karpathy".to_string(),
            entry_root.join("SKILL.md"),
        ),
        (
            "kb-init".to_string(),
            repo_root.join("skills").join("kb-init").join("SKILL.md"),
        ),
        (
            "kb-ingest".to_string(),
            repo_root.join("skills").join("kb-ingest").join("SKILL.md"),
        ),
        (
            "kb-compile".to_string(),
            repo_root.join("skills").join("kb-compile").join("SKILL.md"),
        ),
        (
            "kb-review".to_string(),
            repo_root.join("skills").join("kb-review").join("SKILL.md"),
        ),
        (
            "kb-query".to_string(),
            repo_root.join("skills").join("kb-query").join("SKILL.md"),
        ),
        (
            "kb-render".to_string(),
            repo_root.join("skills").join("kb-render").join("SKILL.md"),
        ),
    ])
}

pub fn read_utf8(path: &Path) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("read {}", path.display()))
}

pub fn load_registry(repo_root: &Path) -> Result<Registry> {
    let payload = read_utf8(&registry_path(repo_root))?;
    serde_json::from_str(&payload).context("parse skill contract registry")
}

pub fn relative_to_repo(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .map(|value| value.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| path.to_string_lossy().replace('\\', "/"))
}

pub fn list_repo_files(repo_root: &Path, relative_roots: &[&str]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for relative_root in relative_roots {
        let absolute_root = repo_root.join(relative_root);
        if absolute_root.is_file() {
            files.push(absolute_root);
            continue;
        }
        if !absolute_root.exists() {
            continue;
        }
        for entry in walkdir::WalkDir::new(&absolute_root) {
            let entry = match entry {
                Ok(value) => value,
                Err(_) => continue,
            };
            if entry.file_type().is_file() {
                files.push(entry.path().to_path_buf());
            }
        }
    }
    files.sort();
    Ok(files)
}

pub fn description_re() -> Regex {
    Regex::new(r"(?m)^description:\s*(.+)$").expect("valid description regex")
}

pub fn name_re() -> Regex {
    Regex::new(r"(?m)^name:\s*(.+)$").expect("valid name regex")
}
