use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use regex::Regex;
use serde::Deserialize;

use super::repo::read_utf8;

#[derive(Clone, Debug, Deserialize)]
pub struct Registry {
    pub contract_family: String,
    #[serde(default = "default_eval_root")]
    pub eval_root: String,
    #[serde(default)]
    pub shared_references: Vec<String>,
    pub skills: BTreeMap<String, SkillEntry>,
}

fn default_eval_root() -> String {
    "evals/skills/obsidian-notes-karpathy".to_string()
}

fn default_install_scope() -> String {
    "runtime".to_string()
}

#[derive(Clone, Debug, Deserialize)]
pub struct SkillEntry {
    pub role: String,
    pub path: String,
    #[serde(default = "default_install_scope")]
    pub install_scope: String,
    #[serde(default)]
    pub doc_targets: Vec<String>,
    #[serde(default)]
    pub eval_targets: EvalTargets,
    #[serde(default)]
    pub companion_refs: Vec<String>,
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

#[derive(Clone, Debug, Default, Deserialize)]
pub struct EvalTargets {
    #[serde(default)]
    pub trigger_manifest: String,
    #[serde(default)]
    pub runtime_manifest: String,
    #[serde(default)]
    pub writable_runtime_manifest: String,
}

pub fn entry_skill_root(repo_root: &Path) -> PathBuf {
    repo_root.join("skills").join("obsidian-notes-karpathy")
}

pub fn registry_path(repo_root: &Path) -> PathBuf {
    entry_skill_root(repo_root)
        .join("scripts")
        .join("skill-contract-registry.json")
}

pub fn eval_root(repo_root: &Path) -> Result<PathBuf> {
    let registry = load_registry(repo_root)?;
    Ok(repo_root.join(registry.eval_root))
}

pub fn runtime_evals_path(repo_root: &Path) -> PathBuf {
    eval_root(repo_root)
        .unwrap_or_else(|_| repo_root.join(default_eval_root()))
        .join("runtime-evals.json")
}

pub fn writable_runtime_evals_path(repo_root: &Path) -> PathBuf {
    eval_root(repo_root)
        .unwrap_or_else(|_| repo_root.join(default_eval_root()))
        .join("runtime-evals-writable.json")
}

pub fn trigger_evals_path(repo_root: &Path) -> PathBuf {
    eval_root(repo_root)
        .unwrap_or_else(|_| repo_root.join(default_eval_root()))
        .join("trigger-evals.json")
}

pub fn skill_paths(repo_root: &Path) -> Result<BTreeMap<String, PathBuf>> {
    let registry = load_registry(repo_root)?;
    Ok(registry
        .skills
        .iter()
        .map(|(skill_name, entry)| (skill_name.clone(), repo_root.join(&entry.path)))
        .collect())
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
