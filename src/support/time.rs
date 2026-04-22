use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::{DateTime, Timelike, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::Value;

static DATE_ONLY_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\d{4}-\d{2}-\d{2}$").expect("valid date regex"));

pub fn parse_datetime(value: Option<&Value>) -> Option<DateTime<Utc>> {
    let candidate = match value {
        Some(Value::String(text)) if !text.trim().is_empty() => text.trim().to_string(),
        _ => return None,
    };
    let normalized = if candidate.ends_with('Z') {
        format!("{}+00:00", &candidate[..candidate.len() - 1])
    } else if DATE_ONLY_RE.is_match(&candidate) {
        format!("{candidate}T00:00:00+00:00")
    } else {
        candidate
    };
    chrono::DateTime::parse_from_rfc3339(&normalized)
        .ok()
        .map(|value| value.with_timezone(&Utc))
}

pub fn now_iso() -> String {
    let now = Utc::now();
    now.with_nanosecond(0)
        .unwrap_or(now)
        .to_rfc3339()
        .replace("+00:00", "Z")
}

pub fn write_markdown(path: &Path, lines: &[String]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    fs::write(path, format!("{}\n", lines.join("\n")))
        .with_context(|| format!("write {}", path.display()))?;
    Ok(())
}
