use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use include_dir::{include_dir, Dir};
use serde_json::Value;

use crate::payload::{SkillInstallPayload, SkillInstallTarget};

static SKILLS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/skills");

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
    let mut names = SKILLS_DIR
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
    names.sort();
    names
}

pub fn show_skill(name: &str) -> Result<String> {
    let path = format!("{name}/SKILL.md");
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
    for dir in SKILLS_DIR.dirs() {
        let Some(name) = dir
            .path()
            .file_name()
            .map(|value| value.to_string_lossy().into_owned())
        else {
            continue;
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
