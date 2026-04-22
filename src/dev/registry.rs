use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use regex::Regex;
use serde::Deserialize;

use super::repo::read_utf8;

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

pub fn load_registry(repo_root: &Path) -> Result<Registry> {
    let payload = read_utf8(&registry_path(repo_root))?;
    serde_json::from_str(&payload).context("parse skill contract registry")
}

pub fn description_re() -> Regex {
    Regex::new(r"(?m)^description:\s*(.+)$").expect("valid description regex")
}

pub fn name_re() -> Regex {
    Regex::new(r"(?m)^name:\s*(.+)$").expect("valid name regex")
}
