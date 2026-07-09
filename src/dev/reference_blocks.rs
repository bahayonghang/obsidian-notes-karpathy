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

pub fn build_on_demand_bullets(skill_name: &str, registry: &Registry) -> Result<Vec<String>> {
    let Some(skill_entry) = registry.skills.get(skill_name) else {
        return Err(anyhow!("Unknown skill: {skill_name}"));
    };

    let prefix = bullet_prefix(skill_name);
    Ok(skill_entry
        .reads_on_demand
        .iter()
        .map(|entry| format!("- `{prefix}references/{}` — {}", entry.file, entry.when))
        .collect())
}

pub fn render_shared_reference_block(skill_name: &str, registry: &Registry) -> Result<String> {
    let mut block = build_shared_reference_bullets(skill_name, registry)?.join("\n");
    let on_demand = build_on_demand_bullets(skill_name, registry)?;
    if !on_demand.is_empty() {
        block.push_str("\n\n## Load on demand\n\nLoad these only when the trigger applies:\n\n");
        block.push_str(&on_demand.join("\n"));
    }
    Ok(block)
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

/// 提取 "## Load on demand" 小节里的完整 bullet 行（含触发条件），用于与
/// `build_on_demand_bullets` 的期望逐条精确比对。
pub fn extract_on_demand_bullets(skill_text: &str) -> Vec<String> {
    let mut bullets = Vec::new();
    let mut in_section = false;
    for line in skill_text.lines() {
        if line.starts_with("## ") {
            if in_section {
                break;
            }
            in_section = line.trim() == "## Load on demand";
            continue;
        }
        if in_section {
            let trimmed = line.trim();
            if trimmed.starts_with("- `") {
                bullets.push(trimmed.to_string());
            }
        }
    }
    bullets
}
