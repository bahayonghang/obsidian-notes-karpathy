use std::path::{Path, PathBuf};

use anyhow::Result;
use serde_json::{Value, json};

use crate::audit_log;
use crate::common::{list_field, now_iso, write_markdown};
use crate::layout::collect_markdown_records;

fn episode_path(vault_root: &Path, rel_output_path: &str) -> PathBuf {
    let name = Path::new(rel_output_path)
        .with_extension("md")
        .file_name()
        .map(PathBuf::from)
        .unwrap_or_default();
    vault_root.join("outputs").join("episodes").join(name)
}

pub fn build_memory_episodes(vault_root: &Path) -> Result<Value> {
    let records = collect_markdown_records(vault_root)?;
    let proposals = records
        .into_iter()
        .filter(|record| matches!(record.kind.as_str(), "qa" | "content_output" | "report_output" | "slide_output" | "chart_output"))
        .map(|record| {
            let target_path = episode_path(vault_root, &record.path);
            json!({
                "source_path": record.path,
                "episode_path": crate::common::relative_posix(&target_path, vault_root),
                "source_live_pages": list_field(&record.frontmatter, "source_live_pages"),
                "open_questions_touched": list_field(&record.frontmatter, "open_questions_touched"),
                "writeback_candidates": list_field(&record.frontmatter, "writeback_candidates"),
                "followup_route": record.frontmatter.get("followup_route").and_then(Value::as_str).map(|value| value.trim().to_lowercase()).filter(|value| !value.is_empty()).unwrap_or_else(|| "none".to_string()),
                "visibility_scope": record.frontmatter.get("visibility_scope").and_then(Value::as_str).unwrap_or("shared"),
            })
        })
        .collect::<Vec<_>>();
    Ok(json!({
        "vault_root": crate::common::normalize_path_string(vault_root.to_string_lossy().as_ref()),
        "proposal_count": proposals.len(),
        "proposals": proposals,
    }))
}

pub fn write_memory_episodes(vault_root: &Path, payload: &Value) -> Result<Vec<String>> {
    let mut written = Vec::new();
    for proposal in payload
        .get("proposals")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
    {
        let episode_path = vault_root.join(
            proposal
                .get("episode_path")
                .and_then(Value::as_str)
                .unwrap_or_default(),
        );
        let source_path = proposal
            .get("source_path")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let basename = Path::new(source_path)
            .file_stem()
            .map(|value| value.to_string_lossy().into_owned())
            .unwrap_or_default();
        let mut lines = vec![
            "---".to_string(),
            format!(r#"title: "Episode: {basename}""#),
            format!(r#"episode_id: "{basename}""#),
            "memory_tier: episodic".to_string(),
            format!(r#"captured_at: "{}""#, now_iso()),
            format!(
                r#"episode_scope: "{}""#,
                Path::new(source_path)
                    .components()
                    .nth(1)
                    .map(|value| value.as_os_str().to_string_lossy().into_owned())
                    .unwrap_or_default()
            ),
            "source_artifacts:".to_string(),
            format!(r#"  - "[[{}]]""#, source_path.trim_end_matches(".md")),
        ];
        for key in [
            "source_live_pages",
            "open_questions_touched",
            "writeback_candidates",
        ] {
            lines.push(format!("{key}:"));
            let values = proposal
                .get(key)
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();
            if values.is_empty() {
                lines.push(r#"  - """#.to_string());
            } else {
                for value in values {
                    lines.push(format!(r#"  - "{}""#, value.as_str().unwrap_or_default()));
                }
            }
        }
        lines.extend([
            "consolidation_status: pending".to_string(),
            format!(
                "followup_route: {}",
                proposal
                    .get("followup_route")
                    .and_then(Value::as_str)
                    .unwrap_or("none")
            ),
            format!(
                r#"visibility_scope: "{}""#,
                proposal
                    .get("visibility_scope")
                    .and_then(Value::as_str)
                    .unwrap_or("shared")
            ),
            "---".to_string(),
            String::new(),
            format!("# Episode: {basename}"),
            String::new(),
            "## Source Artifact".to_string(),
            String::new(),
            format!("- [[{}]]", source_path.trim_end_matches(".md")),
            String::new(),
            "## Durable Signals".to_string(),
            String::new(),
            "- Promote reusable knowledge through draft -> review -> live.".to_string(),
            "- Keep this page as episodic memory, not approved topic truth.".to_string(),
        ]);
        write_markdown(&episode_path, &lines)?;
        written.push(crate::common::relative_posix(&episode_path, vault_root));
    }
    if !written.is_empty() {
        audit_log::append_event(
            vault_root,
            "build_memory_episodes",
            &json!({"written_paths": written}),
        )?;
    }
    Ok(written)
}
