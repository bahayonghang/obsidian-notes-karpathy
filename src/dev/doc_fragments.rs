use std::path::Path;

use anyhow::Result;

use super::load_registry;

pub fn build_skill_inventory_fragment(repo_root: &Path) -> Result<String> {
    let registry = load_registry(repo_root)?;
    let mut lines = vec![
        "| Skill | Role | Runtime install | Docs | Baseline |".to_string(),
        "| --- | --- | --- | --- | --- |".to_string(),
    ];

    for (skill_name, entry) in registry.skills {
        let docs = if entry.doc_targets.is_empty() {
            "-".to_string()
        } else {
            entry.doc_targets.join("<br>")
        };
        lines.push(format!(
            "| `{skill_name}` | {} | {} | {} | `{}` |",
            entry.role, entry.install_scope, docs, entry.baseline_command
        ));
    }

    Ok(lines.join("\n"))
}

pub fn build_installation_boundary_fragment(repo_root: &Path) -> Result<String> {
    let registry = load_registry(repo_root)?;
    let runtime_skill_dirs = registry
        .skills
        .into_iter()
        .filter(|(_, entry)| entry.install_scope == "runtime")
        .filter_map(|(skill_name, entry)| {
            std::path::Path::new(&entry.path).parent().map(|path| {
                format!(
                    "- `{skill_name}` -> `{}`",
                    path.to_string_lossy().replace('\\', "/")
                )
            })
        })
        .collect::<Vec<_>>();

    let mut lines = vec![
        "Runtime bundle installs these skill directories:".to_string(),
        runtime_skill_dirs.join("\n"),
        String::new(),
        "Repo-only dev assets stay outside the installed runtime bundle:".to_string(),
        format!("- `{}`", registry.eval_root),
        "- `target/`, `.runtime-evals/`, and other generated workspaces".to_string(),
    ];
    lines.retain(|line| !line.is_empty());
    Ok(lines.join("\n"))
}
