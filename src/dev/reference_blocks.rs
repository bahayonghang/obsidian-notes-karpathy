use anyhow::{Result, anyhow};

use super::Registry;

pub fn bullet_prefix(skill_name: &str) -> &'static str {
    if skill_name == "obsidian-notes-karpathy" {
        "./"
    } else {
        "../obsidian-notes-karpathy/"
    }
}

pub fn build_shared_reference_bullets(
    skill_name: &str,
    registry: &Registry,
) -> Result<Vec<String>> {
    let Some(skill_entry) = registry.skills.get(skill_name) else {
        return Err(anyhow!("Unknown skill: {skill_name}"));
    };

    let prefix = bullet_prefix(skill_name);
    let mut bullets = vec![format!("- `{prefix}scripts/skill-contract-registry.json`")];
    bullets.extend(
        skill_entry
            .reads
            .iter()
            .map(|reference| format!("- `{prefix}references/{reference}`")),
    );
    Ok(bullets)
}

pub fn render_shared_reference_block(skill_name: &str, registry: &Registry) -> Result<String> {
    Ok(build_shared_reference_bullets(skill_name, registry)?.join("\n"))
}

pub fn extract_reference_bullets(skill_text: &str) -> Vec<String> {
    let lines = skill_text.lines().collect::<Vec<_>>();
    let mut section_lines = Vec::new();
    let mut in_section = false;
    let mut scope_ready = false;

    for line in lines {
        if line.starts_with("## ") {
            if in_section {
                break;
            }
            in_section = line.starts_with("## Read before");
            scope_ready = line == "## Scope";
            continue;
        }

        if scope_ready && line.trim() == "Before checking the vault, read these files first:" {
            in_section = true;
            scope_ready = false;
            continue;
        }

        if in_section {
            section_lines.push(line);
        }
    }

    if section_lines.is_empty() && skill_text.contains("skill-contract-registry.json") {
        section_lines = skill_text.lines().collect();
    }

    section_lines
        .into_iter()
        .filter_map(|line| {
            let trimmed = line.trim();
            trimmed
                .strip_prefix("- `")
                .and_then(|value| value.strip_suffix('`'))
                .map(|value| value.to_string())
        })
        .collect()
}
