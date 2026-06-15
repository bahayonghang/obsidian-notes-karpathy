use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use include_dir::{Dir, include_dir};
use serde::Deserialize;
use serde_json::Value;

use crate::cli::args::SkillInstallArgs;
use crate::payload::{SkillInstallPayload, SkillInstallTarget, SkillInstallTargetResult};

static SKILLS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/skills");
const EMBEDDED_REGISTRY_PATH: &str = "obsidian-notes-karpathy/scripts/skill-contract-registry.json";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InstallScope {
    Local,
    Global,
}

impl InstallScope {
    fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Global => "global",
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct InstallTargetDef {
    key: &'static str,
    label: &'static str,
    scope: InstallScope,
    relative_path: &'static str,
    legacy_field: Option<LegacyInstallField>,
}

#[derive(Clone, Copy, Debug)]
enum LegacyInstallField {
    Claude,
    Codex,
}

const LOCAL_INSTALL_TARGETS: &[InstallTargetDef] = &[
    InstallTargetDef {
        key: "claude",
        label: "Claude Code",
        scope: InstallScope::Local,
        relative_path: ".claude/skills",
        legacy_field: Some(LegacyInstallField::Claude),
    },
    InstallTargetDef {
        key: "codex",
        label: "Codex / generic AGENTS",
        scope: InstallScope::Local,
        relative_path: ".agents/skills",
        legacy_field: Some(LegacyInstallField::Codex),
    },
    InstallTargetDef {
        key: "cursor",
        label: "Cursor",
        scope: InstallScope::Local,
        relative_path: ".cursor/skills",
        legacy_field: None,
    },
    InstallTargetDef {
        key: "windsurf",
        label: "Windsurf",
        scope: InstallScope::Local,
        relative_path: ".windsurf/skills",
        legacy_field: None,
    },
    InstallTargetDef {
        key: "kiro",
        label: "Kiro",
        scope: InstallScope::Local,
        relative_path: ".kiro/skills",
        legacy_field: None,
    },
    InstallTargetDef {
        key: "pi",
        label: "Pi",
        scope: InstallScope::Local,
        relative_path: ".pi/skills",
        legacy_field: None,
    },
];

const GLOBAL_INSTALL_TARGETS: &[InstallTargetDef] = &[
    InstallTargetDef {
        key: "claude",
        label: "Claude Code",
        scope: InstallScope::Global,
        relative_path: ".claude/skills",
        legacy_field: None,
    },
    InstallTargetDef {
        key: "codex",
        label: "Codex",
        scope: InstallScope::Global,
        relative_path: ".codex/skills",
        legacy_field: None,
    },
    InstallTargetDef {
        key: "gemini",
        label: "Gemini CLI",
        scope: InstallScope::Global,
        relative_path: ".gemini/skills",
        legacy_field: None,
    },
    InstallTargetDef {
        key: "agents",
        label: "AGENTS-aware tools",
        scope: InstallScope::Global,
        relative_path: ".agents/skills",
        legacy_field: None,
    },
];

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

pub fn install_skills(args: &SkillInstallArgs) -> Result<Value> {
    let targets = selected_targets(args)?;
    let mut payload = SkillInstallPayload::default();

    for (target, dir) in targets {
        let (installed, skipped) = install_to(&dir, args.overwrite)?;
        let normalized_dir = crate::common::normalize_path_string(dir.to_string_lossy().as_ref());
        let legacy = SkillInstallTarget {
            target_dir: normalized_dir.clone(),
            installed: installed.clone(),
            skipped: skipped.clone(),
        };
        if let Some(legacy_field) = target.legacy_field {
            match legacy_field {
                LegacyInstallField::Claude => payload.claude = Some(legacy),
                LegacyInstallField::Codex => payload.codex = Some(legacy),
            }
        }
        payload.targets.push(SkillInstallTargetResult {
            key: target.key.to_string(),
            scope: target.scope.as_str().to_string(),
            label: target.label.to_string(),
            target_dir: normalized_dir,
            installed,
            skipped,
        });
    }

    Ok(serde_json::to_value(&payload)?)
}

fn selected_targets(args: &SkillInstallArgs) -> Result<Vec<(InstallTargetDef, PathBuf)>> {
    if args.global {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not resolve home dir"))?;
        return selected_targets_with_home(args, &home);
    }
    selected_targets_with_home(args, Path::new(""))
}

#[cfg(test)]
fn selected_skill_install_targets(
    args: &SkillInstallArgs,
    home: &Path,
) -> Result<Vec<SkillInstallTargetResult>> {
    selected_targets_with_home(args, home).map(|targets| {
        targets
            .into_iter()
            .map(|(target, dir)| SkillInstallTargetResult {
                key: target.key.to_string(),
                scope: target.scope.as_str().to_string(),
                label: target.label.to_string(),
                target_dir: crate::common::normalize_path_string(dir.to_string_lossy().as_ref()),
                installed: Vec::new(),
                skipped: Vec::new(),
            })
            .collect()
    })
}

fn selected_targets_with_home(
    args: &SkillInstallArgs,
    home: &Path,
) -> Result<Vec<(InstallTargetDef, PathBuf)>> {
    if args.global {
        if args.all_local || args.cursor || args.windsurf || args.kiro || args.pi {
            bail!("--global supports only --claude, --codex, --gemini, and --agents target flags");
        }
        if args.dir.is_some() {
            bail!("--dir cannot be combined with --global");
        }
        let selected = selected_target_defs(args, GLOBAL_INSTALL_TARGETS, true);
        return Ok(selected
            .into_iter()
            .map(|target| (target, home.join(target.relative_path)))
            .collect());
    }

    if args.agents || args.gemini {
        bail!(
            "local install supports --claude, --codex, --cursor, --windsurf, --kiro, --pi, and --all-local"
        );
    }
    let workspace = args
        .dir
        .clone()
        .unwrap_or(std::env::current_dir().context("resolve current directory")?);
    let selected = selected_target_defs(args, LOCAL_INSTALL_TARGETS, false);
    Ok(selected
        .into_iter()
        .map(|target| (target, workspace.join(target.relative_path)))
        .collect())
}

fn selected_target_defs(
    args: &SkillInstallArgs,
    candidates: &[InstallTargetDef],
    global: bool,
) -> Vec<InstallTargetDef> {
    let any_target_flag = args.claude
        || args.codex
        || args.cursor
        || args.windsurf
        || args.kiro
        || args.pi
        || args.agents
        || args.gemini;

    if global {
        if !any_target_flag {
            return candidates.to_vec();
        }
        return candidates
            .iter()
            .copied()
            .filter(|target| target_flag_selected(args, target.key))
            .collect();
    }

    if args.all_local {
        return candidates.to_vec();
    }
    if !any_target_flag {
        return candidates
            .iter()
            .copied()
            .filter(|target| matches!(target.key, "claude" | "codex"))
            .collect();
    }
    candidates
        .iter()
        .copied()
        .filter(|target| target_flag_selected(args, target.key))
        .collect()
}

fn target_flag_selected(args: &SkillInstallArgs, key: &str) -> bool {
    match key {
        "claude" => args.claude,
        "codex" => args.codex,
        "cursor" => args.cursor,
        "windsurf" => args.windsurf,
        "kiro" => args.kiro,
        "pi" => args.pi,
        "agents" => args.agents,
        "gemini" => args.gemini,
        _ => false,
    }
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
        if target_skill_dir.exists() {
            remove_existing_target(&target_skill_dir)?;
        }
        copy_embedded_dir(dir, &target_skill_dir)?;
        installed.push(name);
    }
    installed.sort();
    skipped.sort();
    Ok((installed, skipped))
}

fn remove_existing_target(target: &Path) -> Result<()> {
    let metadata = fs::symlink_metadata(target)
        .with_context(|| format!("inspect existing {}", target.display()))?;
    if metadata.is_dir() && !metadata.file_type().is_symlink() {
        fs::remove_dir_all(target).with_context(|| format!("remove {}", target.display()))?;
    } else {
        fs::remove_file(target).with_context(|| format!("remove {}", target.display()))?;
    }
    Ok(())
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn args() -> SkillInstallArgs {
        SkillInstallArgs {
            claude: false,
            codex: false,
            cursor: false,
            windsurf: false,
            kiro: false,
            pi: false,
            agents: false,
            gemini: false,
            all_local: false,
            global: false,
            dir: None,
            overwrite: false,
        }
    }

    #[test]
    fn global_target_resolution_uses_provided_home() {
        let mut args = args();
        args.global = true;
        args.claude = true;
        args.codex = true;
        args.gemini = true;
        args.agents = true;
        let home = PathBuf::from("/isolated-home");

        let targets = selected_skill_install_targets(&args, &home).expect("targets");

        let paths = targets
            .iter()
            .map(|target| {
                (
                    target.key.as_str(),
                    target.scope.as_str(),
                    target.target_dir.as_str(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            paths,
            vec![
                ("claude", "global", "/isolated-home/.claude/skills"),
                ("codex", "global", "/isolated-home/.codex/skills"),
                ("gemini", "global", "/isolated-home/.gemini/skills"),
                ("agents", "global", "/isolated-home/.agents/skills"),
            ]
        );
    }
}
