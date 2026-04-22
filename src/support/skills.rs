use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use include_dir::{Dir, include_dir};
use serde::Deserialize;
use serde_json::Value;

use crate::payload::{SkillInstallPayload, SkillInstallTarget};

static SKILLS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/skills");
const EMBEDDED_REGISTRY_PATH: &str = "obsidian-notes-karpathy/scripts/skill-contract-registry.json";

#[derive(Clone, Debug, Deserialize)]
struct EmbeddedRegistry {
    skills: std::collections::BTreeMap<String, EmbeddedSkillEntry>,
}

#[derive(Clone, Debug, Deserialize)]
struct EmbeddedSkillEntry {
    path: String,
    #[serde(default = "default_install_scope")]
    install_scope: String,
}

fn default_install_scope() -> String {
    "runtime".to_string()
}

fn embedded_registry() -> Result<EmbeddedRegistry> {
    let registry_text = SKILLS_DIR
        .get_file(EMBEDDED_REGISTRY_PATH)
        .and_then(|file| file.contents_utf8())
        .ok_or_else(|| anyhow::anyhow!("Embedded registry missing at {EMBEDDED_REGISTRY_PATH}"))?;
    serde_json::from_str(registry_text).context("parse embedded skill registry")
}

fn embedded_runtime_skill_dirs() -> Result<Vec<(String, String)>> {
    let registry = embedded_registry()?;
    let mut dirs = registry
        .skills
        .into_iter()
        .filter(|(_, entry)| entry.install_scope == "runtime")
        .filter_map(|(skill_name, entry)| {
            let path = entry.path.replace('\\', "/");
            let relative = path.strip_prefix("skills/").unwrap_or(&path).to_string();
            let dir = std::path::Path::new(&relative)
                .parent()
                .map(|value| value.to_string_lossy().replace('\\', "/"))?;
            Some((skill_name, dir))
        })
        .collect::<Vec<_>>();
    dirs.sort_by(|left, right| left.0.cmp(&right.0));
    dirs.dedup_by(|left, right| left.0 == right.0 && left.1 == right.1);
    Ok(dirs)
}

pub fn install_skills(
    workspace: &Path,
    install_claude: bool,
    install_codex: bool,
    overwrite: bool,
) -> Result<Value> {
    let install_both = !install_claude && !install_codex;
    let mut payload = SkillInstallPayload::default();

    if install_both || install_claude {
        let dir = workspace.join(".claude").join("skills");
        let (installed, skipped) = install_to(&dir, overwrite)?;
        payload.claude = Some(SkillInstallTarget {
            target_dir: crate::common::normalize_path_string(dir.to_string_lossy().as_ref()),
            installed,
            skipped,
        });
    }
    if install_both || install_codex {
        let dir = workspace.join(".agents").join("skills");
        let (installed, skipped) = install_to(&dir, overwrite)?;
        payload.codex = Some(SkillInstallTarget {
            target_dir: crate::common::normalize_path_string(dir.to_string_lossy().as_ref()),
            installed,
            skipped,
        });
    }

    Ok(serde_json::to_value(&payload)?)
}

pub fn list_skills() -> Vec<String> {
    if let Ok(runtime_dirs) = embedded_runtime_skill_dirs() {
        return runtime_dirs
            .into_iter()
            .map(|(skill_name, _)| skill_name)
            .collect();
    }

    let mut fallback = SKILLS_DIR
        .dirs()
        .map(|dir| {
            dir.path()
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|name| !name.is_empty())
        .collect::<Vec<_>>();
    fallback.sort();
    fallback
}

pub fn show_skill(name: &str) -> Result<String> {
    let runtime_dirs = embedded_runtime_skill_dirs()?;
    let (_, skill_dir) = runtime_dirs
        .into_iter()
        .find(|(skill_name, _)| skill_name == name)
        .ok_or_else(|| anyhow::anyhow!("Skill \"{name}\" not found."))?;
    let path = format!("{skill_dir}/SKILL.md");
    SKILLS_DIR
        .get_file(path)
        .map(|file| file.contents_utf8().unwrap_or_default().to_string())
        .ok_or_else(|| anyhow::anyhow!("Skill \"{name}\" not found."))
}

pub fn bundle_available() -> bool {
    !list_skills().is_empty()
}

fn install_to(target_dir: &Path, overwrite: bool) -> Result<(Vec<String>, Vec<String>)> {
    fs::create_dir_all(target_dir).with_context(|| format!("create {}", target_dir.display()))?;
    let mut installed = Vec::new();
    let mut skipped = Vec::new();
    for (name, relative_dir) in embedded_runtime_skill_dirs()? {
        let Some(dir) = SKILLS_DIR.get_dir(&relative_dir) else {
            return Err(anyhow::anyhow!(
                "Embedded skill directory missing for {name}: {relative_dir}"
            ));
        };
        if name.is_empty() {
            continue;
        }
        let target_skill_dir = target_dir.join(&name);
        if target_skill_dir.exists() && !overwrite {
            skipped.push(name.clone());
            continue;
        }
        copy_embedded_dir(dir, &target_skill_dir)?;
        installed.push(name);
    }
    installed.sort();
    skipped.sort();
    Ok((installed, skipped))
}

fn copy_embedded_dir(dir: &Dir<'_>, target_dir: &Path) -> Result<()> {
    fs::create_dir_all(target_dir)?;
    for subdir in dir.dirs() {
        let rel = subdir
            .path()
            .strip_prefix(dir.path())
            .unwrap_or(subdir.path());
        let target = target_dir.join(rel);
        copy_embedded_dir(subdir, &target)?;
    }
    for file in dir.files() {
        let rel = file.path().strip_prefix(dir.path()).unwrap_or(file.path());
        let target = target_dir.join(rel);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&target, file.contents())
            .with_context(|| format!("write {}", target.display()))?;
    }
    Ok(())
}
