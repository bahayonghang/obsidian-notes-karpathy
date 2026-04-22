use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Result;

use crate::common::MarkdownRecord;
use crate::layout::{collect_markdown_records, detect_layout_family};

pub struct AuditContext {
    vault_root: PathBuf,
    layout_family: String,
    records: Vec<Arc<MarkdownRecord>>,
}

impl AuditContext {
    pub fn load(vault_root: &Path) -> Result<Self> {
        Ok(Self {
            vault_root: vault_root.to_path_buf(),
            layout_family: detect_layout_family(vault_root).to_string(),
            records: collect_markdown_records(vault_root)?,
        })
    }

    pub fn vault_root(&self) -> &Path {
        &self.vault_root
    }

    pub fn layout_family(&self) -> &str {
        &self.layout_family
    }

    pub fn records(&self) -> &[Arc<MarkdownRecord>] {
        &self.records
    }
}
