//! 统一的审计事件追加通道。
//!
//! 所有写到 `outputs/audit/operations.jsonl` 的调用必须走这里。
//! 使用 append-mode 打开文件，避免"读整个文件 -> 追加 -> 写回"的 O(N^2) 行为。

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use serde_json::{Value, json};

use crate::common::now_iso;

/// 以单条 JSON 行格式追加一个审计事件。
///
/// 事件结构：
/// ```json
/// {"timestamp": "...Z", "action": "<action>", "payload": <payload>}
/// ```
///
/// 若 `outputs/audit/` 目录不存在则会被创建。
pub fn append_event(vault_root: &Path, action: &str, payload: &Value) -> Result<()> {
    let audit_path = vault_root
        .join("outputs")
        .join("audit")
        .join("operations.jsonl");
    if let Some(parent) = audit_path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }

    let line = serde_json::to_string(&json!({
        "timestamp": now_iso(),
        "action": action,
        "payload": payload,
    }))?;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&audit_path)
        .with_context(|| format!("open {}", audit_path.display()))?;
    writeln!(file, "{line}").with_context(|| format!("write {}", audit_path.display()))?;
    Ok(())
}
