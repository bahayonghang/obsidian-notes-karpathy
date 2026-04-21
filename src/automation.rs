use std::path::Path;

use anyhow::Result;
use serde_json::{json, Value};

use crate::compile::scan_compile_delta;
use crate::episodes::{build_memory_episodes, write_memory_episodes};
use crate::governance::{build_governance_indices, write_governance_indices};
use crate::graph::{build_graph_snapshot, write_graph_snapshot};
use crate::health::audit_vault_mechanics;
use crate::ingest::{scan_ingest_delta, sync_source_manifest};

pub fn run_automation(vault_root: &Path, mode: &str, write: bool) -> Result<Value> {
    let mut payload = json!({
        "vault_root": crate::common::normalize_path_string(vault_root.to_string_lossy().as_ref()),
        "mode": mode,
        "write": write,
    });
    match mode {
        "scheduled-health" => {
            let lint = audit_vault_mechanics(vault_root)?;
            let governance = build_governance_indices(vault_root)?;
            let graph = build_graph_snapshot(vault_root)?;
            payload["lint"] = lint;
            payload["governance"] = governance.clone();
            payload["graph"] = graph.clone();
            if write {
                write_governance_indices(vault_root, &governance)?;
                payload["graph_output_path"] = json!(write_graph_snapshot(vault_root, &graph)?);
            }
        }
        "session-end" => {
            let episodes = build_memory_episodes(vault_root)?;
            if write {
                payload["written_paths"] = json!(write_memory_episodes(vault_root, &episodes)?);
            }
            payload["episodes"] = episodes;
        }
        "query-archive" => {
            let governance = build_governance_indices(vault_root)?;
            let episodes = build_memory_episodes(vault_root)?;
            if write {
                write_governance_indices(vault_root, &governance)?;
                payload["written_paths"] = json!(write_memory_episodes(vault_root, &episodes)?);
            }
            payload["governance"] = governance;
            payload["episodes"] = episodes;
        }
        "new-source" => {
            let ingest = scan_ingest_delta(vault_root)?;
            let compile_delta = scan_compile_delta(vault_root)?;
            let graph = build_graph_snapshot(vault_root)?;
            payload["ingest"] = if write {
                sync_source_manifest(vault_root)?
            } else {
                ingest
            };
            payload["compile_delta"] = compile_delta;
            payload["graph"] = graph.clone();
            if write {
                payload["graph_output_path"] = json!(write_graph_snapshot(vault_root, &graph)?);
            }
        }
        _ => anyhow::bail!("Unsupported mode: {mode}"),
    }
    Ok(payload)
}
