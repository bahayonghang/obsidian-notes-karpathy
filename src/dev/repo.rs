use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};

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

pub fn read_utf8(path: &Path) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("read {}", path.display()))
}

pub fn relative_to_repo(repo_root: &Path, path: &Path) -> String {
    let repo_candidates = [
        repo_root.to_path_buf(),
        repo_root
            .canonicalize()
            .unwrap_or_else(|_| repo_root.to_path_buf()),
    ];
    let path_candidates = [
        path.to_path_buf(),
        path.canonicalize().unwrap_or_else(|_| path.to_path_buf()),
    ];

    for candidate in &path_candidates {
        for root in &repo_candidates {
            if let Ok(relative) = candidate.strip_prefix(root) {
                return relative.to_string_lossy().replace('\\', "/");
            }
        }
    }

    path.to_string_lossy().replace('\\', "/")
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
