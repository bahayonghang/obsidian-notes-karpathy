use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::dev::{entry_skill_root, relative_to_repo, skill_paths};

pub const SUPPORTED_MODES: [&str; 2] = ["read-only", "writable-copy"];
pub const INFRA_FAILURE_PATTERNS: [&str; 7] = [
    "stream disconnected before completion",
    "reconnecting...",
    "timed out",
    "profile.ps1",
    "microsoft.powershell_profile.ps1",
    "cannot dot-source this command",
    "os error 123",
];

pub fn default_workspace_root(repo_root: &Path) -> PathBuf {
    repo_root.join(".runtime-evals")
}

pub fn entry_relative(repo_root: &Path, path_str: &str) -> PathBuf {
    normalize_path(&entry_skill_root(repo_root).join(path_str))
}

pub fn fixture_root_for_relpath(path_str: &str) -> Option<String> {
    let rel_path = PathBuf::from(path_str);
    let parts = rel_path
        .components()
        .map(|part| part.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    if parts.len() < 3 || parts[0] != "evals" || parts[1] != "fixtures" {
        return None;
    }
    Some(
        PathBuf::from(&parts[0])
            .join(&parts[1])
            .join(&parts[2])
            .to_string_lossy()
            .replace('\\', "/"),
    )
}

pub fn repo_relative(repo_root: &Path, path: &Path) -> String {
    relative_to_repo(repo_root, path)
}

pub fn normalize_path(path: &Path) -> PathBuf {
    if path.exists() {
        path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
    } else {
        path.to_path_buf()
    }
}

pub fn skill_doc_path(repo_root: &Path, skill: &str) -> Result<PathBuf> {
    let paths = skill_paths(repo_root);
    paths
        .get(skill)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("Unsupported skill: {skill}"))
}
