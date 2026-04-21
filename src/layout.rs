use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use chrono::{DateTime, Timelike, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};

use crate::common::{
    iter_markdown_records, load_markdown, normalize_path_string, relative_posix, MarkdownRecord,
};

pub static ARXIV_ID_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\d{4}\.\d{4,5}(?:v\d+)?)").expect("valid arxiv id regex"));
pub static ARXIV_URL_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"https?://(?:www\.)?(?:arxiv\.org/(?:abs|pdf)|alphaxiv\.org/(?:overview|abs))/([^?#/]+)",
    )
    .expect("valid arxiv url regex")
});

pub const PDF_SIDECAR_SUFFIX: &str = ".source.md";
pub const DEFAULT_KB_PROFILE: &str = "governed-team";
pub const PDF_COMPANION_SKILLS: [&str; 2] = ["paper-workbench", "pdf"];
const IMAGE_ASSET_SUFFIXES: [&str; 6] = [".png", ".jpg", ".jpeg", ".gif", ".webp", ".svg"];
const DATA_ASSET_SUFFIXES: [&str; 5] = [".csv", ".tsv", ".json", ".jsonl", ".ndjson"];
const LEGACY_RAW_DIRS: [&str; 4] = ["articles", "papers", "podcasts", "repos"];

pub fn is_pdf_sidecar(path: &Path) -> bool {
    path.file_name()
        .map(|value| value.to_string_lossy().ends_with(PDF_SIDECAR_SUFFIX))
        .unwrap_or(false)
}

pub fn sidecar_for_pdf(raw_path: &Path) -> PathBuf {
    let stem = raw_path
        .file_stem()
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_default();
    raw_path.with_file_name(format!("{stem}{PDF_SIDECAR_SUFFIX}"))
}

pub fn detect_layout_family(vault_root: &Path) -> String {
    let review_gated = [
        "wiki/drafts",
        "wiki/live",
        "wiki/briefings",
        "outputs/reviews",
        "raw/human",
        "raw/agents",
    ]
    .iter()
    .any(|rel| vault_root.join(rel).exists());
    if review_gated {
        return "review-gated".to_string();
    }

    let legacy = [
        "wiki/summaries",
        "wiki/concepts",
        "wiki/entities",
        "raw/articles",
        "raw/papers",
        "raw/podcasts",
        "raw/repos",
    ]
    .iter()
    .any(|rel| vault_root.join(rel).exists());
    if legacy {
        return "legacy-layout".to_string();
    }
    "uninitialized".to_string()
}

pub fn raw_source_metadata(vault_root: &Path, raw_path: &Path) -> Map<String, Value> {
    let mut metadata = Map::new();
    metadata.insert("capture_source".to_string(), json!("legacy"));
    metadata.insert("capture_trust".to_string(), json!("legacy"));
    metadata.insert("agent_role".to_string(), Value::Null);
    metadata.insert("legacy_layout".to_string(), Value::Bool(true));

    let rel_parts = relative_posix(raw_path, vault_root)
        .split('/')
        .map(|part| part.to_string())
        .collect::<Vec<_>>();
    if rel_parts.len() >= 3 && rel_parts.get(1).map(String::as_str) == Some("human") {
        metadata.insert("capture_source".to_string(), json!("human"));
        metadata.insert("capture_trust".to_string(), json!("curated"));
        metadata.insert("legacy_layout".to_string(), Value::Bool(false));
    } else if rel_parts.len() >= 4 && rel_parts.get(1).map(String::as_str) == Some("agents") {
        metadata.insert("capture_source".to_string(), json!("agent"));
        metadata.insert("capture_trust".to_string(), json!("untrusted"));
        metadata.insert("agent_role".to_string(), json!(rel_parts[2].clone()));
        metadata.insert("legacy_layout".to_string(), Value::Bool(false));
    }
    metadata
}

pub fn source_class_for_raw(raw_path: &Path) -> String {
    let suffix = raw_path
        .extension()
        .map(|value| format!(".{}", value.to_string_lossy().to_lowercase()))
        .unwrap_or_default();
    let parts = raw_path
        .components()
        .map(|part| part.as_os_str().to_string_lossy().to_lowercase())
        .collect::<Vec<_>>();
    if suffix == ".pdf" {
        return "paper_pdf".to_string();
    }
    if parts.iter().any(|part| part == "assets") && IMAGE_ASSET_SUFFIXES.contains(&suffix.as_str())
    {
        return "image_asset".to_string();
    }
    if parts.iter().any(|part| part == "data") && DATA_ASSET_SUFFIXES.contains(&suffix.as_str()) {
        return "data_asset".to_string();
    }
    "markdown".to_string()
}

pub fn manifest_path(vault_root: &Path) -> PathBuf {
    vault_root.join("raw").join("_manifest.yaml")
}

pub fn is_manifest_path(vault_root: &Path, raw_path: &Path) -> bool {
    match (
        raw_path.canonicalize(),
        manifest_path(vault_root).canonicalize(),
    ) {
        (Ok(lhs), Ok(rhs)) => lhs == rhs,
        _ => raw_path == manifest_path(vault_root),
    }
}

pub fn load_manifest_profile(vault_root: &Path) -> Option<String> {
    let path = manifest_path(vault_root);
    let text = fs::read_to_string(path).ok()?;
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') || !line.starts_with("profile:") {
            continue;
        }
        let value = line
            .split_once(':')?
            .1
            .trim()
            .trim_matches('"')
            .trim_matches('\'');
        if value.is_empty() {
            return None;
        }
        return Some(value.to_string());
    }
    None
}

pub fn resolve_vault_profile(vault_root: &Path) -> String {
    if let Some(profile) = load_manifest_profile(vault_root) {
        return profile;
    }
    let index_path = vault_root.join("wiki").join("index.md");
    if index_path.exists() {
        if let Ok(record) = load_markdown(&index_path, Some(vault_root)) {
            if let Some(Value::String(profile)) = record.frontmatter.get("kb_profile") {
                if !profile.trim().is_empty() {
                    return profile.trim().to_string();
                }
            }
        }
    }
    DEFAULT_KB_PROFILE.to_string()
}

pub fn configured_skill_roots() -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if let Ok(override_paths) = env::var("KB_COMPANION_SKILL_PATHS") {
        for part in override_paths.split(if cfg!(windows) { ';' } else { ':' }) {
            if !part.trim().is_empty() {
                candidates.push(PathBuf::from(part.trim()));
            }
        }
    } else {
        if let Ok(codex_home) = env::var("CODEX_HOME") {
            candidates.push(PathBuf::from(codex_home).join("skills"));
        }
        if let Some(home) = dirs::home_dir() {
            candidates.push(home.join(".codex").join("skills"));
            candidates.push(home.join(".claude").join("skills"));
            candidates.push(home.join(".agents").join("skills"));
        }
    }

    let mut deduped = Vec::new();
    let mut seen = BTreeMap::new();
    for candidate in candidates {
        let key = normalize_path_string(candidate.to_string_lossy().as_ref());
        if seen.insert(key, true).is_none() {
            deduped.push(candidate);
        }
    }
    deduped
}

pub fn detect_companion_skills(skill_roots: Option<&[PathBuf]>) -> Value {
    let roots = skill_roots
        .map(|value| value.to_vec())
        .unwrap_or_else(configured_skill_roots);
    let mut skills = serde_json::Map::new();
    for skill_name in PDF_COMPANION_SKILLS {
        let available = roots
            .iter()
            .any(|root| root.join(skill_name).join("SKILL.md").exists());
        skills.insert(skill_name.to_string(), Value::Bool(available));
    }
    json!({
        "search_roots": roots.iter().map(|root| normalize_path_string(root.to_string_lossy().as_ref())).collect::<Vec<_>>(),
        "skills": skills,
    })
}

pub fn extract_paper_handle(candidate: &str) -> Option<String> {
    fn bounded_arxiv_id(text: &str) -> Option<String> {
        for captures in ARXIV_ID_RE.captures_iter(text) {
            let matched = captures.get(1)?;
            let start = matched.start();
            let end = matched.end();
            let left_ok = text[..start]
                .chars()
                .next_back()
                .map(|value| !value.is_ascii_digit())
                .unwrap_or(true);
            let right_ok = text[end..]
                .chars()
                .next()
                .map(|value| !value.is_ascii_digit())
                .unwrap_or(true);
            if left_ok && right_ok {
                return Some(matched.as_str().to_string());
            }
        }
        None
    }

    let text = candidate.trim();
    if text.is_empty() {
        return None;
    }
    if let Some(captures) = ARXIV_URL_RE.captures(text) {
        let url_tail = captures
            .get(1)?
            .as_str()
            .strip_suffix(".pdf")
            .unwrap_or(captures.get(1)?.as_str());
        if let Some(handle) = bounded_arxiv_id(url_tail) {
            return Some(handle);
        }
    }
    bounded_arxiv_id(text)
}

pub fn pdf_ingest_plan(
    vault_root: &Path,
    raw_path: &Path,
    companion_status: &Map<String, Value>,
) -> Map<String, Value> {
    let mut plan = Map::new();
    plan.insert("source_class".to_string(), json!("paper_pdf"));
    plan.insert("paper_handle".to_string(), Value::Null);
    plan.insert("paper_handle_source".to_string(), Value::Null);
    plan.insert("ingest_plan".to_string(), Value::Null);
    plan.insert("ingest_reason".to_string(), Value::Null);
    plan.insert(
        "companion_status".to_string(),
        Value::Object(companion_status.clone()),
    );

    let sidecar_path = sidecar_for_pdf(raw_path);
    if sidecar_path.exists() {
        if let Ok(sidecar_record) = load_markdown(&sidecar_path, Some(vault_root)) {
            plan.insert(
                "metadata_path".to_string(),
                json!(sidecar_record.path.clone()),
            );
            if let Some(Value::String(title)) = sidecar_record.frontmatter.get("title") {
                if !title.trim().is_empty() {
                    plan.insert("paper_title".to_string(), json!(title.trim()));
                }
            }
            if let Some(Value::String(source_url)) = sidecar_record.frontmatter.get("source") {
                if !source_url.trim().is_empty() {
                    plan.insert("source_url".to_string(), json!(source_url.trim()));
                }
            }
            for (source_name, candidate) in [
                ("paper_id", sidecar_record.frontmatter.get("paper_id")),
                ("source", sidecar_record.frontmatter.get("source")),
            ] {
                if let Some(Value::String(text)) = candidate {
                    if let Some(handle) = extract_paper_handle(text) {
                        plan.insert("paper_handle".to_string(), json!(handle));
                        plan.insert("paper_handle_source".to_string(), json!(source_name));
                        break;
                    }
                }
            }
        }
    }

    if plan.get("paper_handle").unwrap_or(&Value::Null).is_null() {
        let stem = raw_path
            .file_stem()
            .map(|value| value.to_string_lossy().into_owned())
            .unwrap_or_default();
        if let Some(handle) = extract_paper_handle(&stem) {
            plan.insert("paper_handle".to_string(), json!(handle));
            plan.insert("paper_handle_source".to_string(), json!("filename"));
        }
    }

    let paper_workbench = companion_status
        .get("paper-workbench")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    if paper_workbench {
        plan.insert("ingest_plan".to_string(), json!("paper-workbench"));
        plan.insert(
            "ingest_reason".to_string(),
            json!("paper_workbench_directory_policy"),
        );
        return plan;
    }

    plan.insert("ingest_plan".to_string(), json!("skip"));
    plan.insert(
        "ingest_reason".to_string(),
        json!("paper_workbench_required_for_raw_papers"),
    );
    plan
}

pub fn accepted_raw_sources(vault_root: &Path) -> Vec<PathBuf> {
    let raw_root = vault_root.join("raw");
    if !raw_root.exists() {
        return Vec::new();
    }
    let layout_family = detect_layout_family(vault_root);
    let mut sources = Vec::new();
    for entry in walkdir::WalkDir::new(&raw_root).sort_by_file_name() {
        let entry = match entry {
            Ok(value) => value,
            Err(_) => continue,
        };
        let path = entry.path();
        if !entry.file_type().is_file() {
            continue;
        }
        let rel = relative_posix(path, vault_root);
        if rel.split('/').any(|part| part.starts_with('.')) {
            continue;
        }
        if is_manifest_path(vault_root, path)
            || path
                .file_name()
                .and_then(|v| v.to_str())
                .map(|v| v.starts_with('_'))
                .unwrap_or(false)
        {
            continue;
        }
        if is_pdf_sidecar(path) {
            continue;
        }
        if layout_family == "review-gated"
            && rel.starts_with("raw/")
            && LEGACY_RAW_DIRS
                .iter()
                .any(|dir| rel.split('/').nth(1) == Some(*dir))
        {
            continue;
        }

        let source_class = source_class_for_raw(Path::new(&rel));
        if matches!(source_class.as_str(), "image_asset" | "data_asset") {
            sources.push(path.to_path_buf());
            continue;
        }

        let suffix = path
            .extension()
            .map(|value| format!(".{}", value.to_string_lossy().to_lowercase()))
            .unwrap_or_default();
        if !matches!(suffix.as_str(), ".md" | ".pdf") {
            continue;
        }
        if suffix == ".pdf" && !rel.split('/').any(|part| part == "papers") {
            continue;
        }
        sources.push(path.to_path_buf());
    }
    sources
}

pub fn draft_summary_for_raw(vault_root: &Path, raw_path: &Path) -> PathBuf {
    let rel = raw_path
        .strip_prefix(vault_root.join("raw"))
        .unwrap_or(raw_path);
    vault_root
        .join("wiki")
        .join("drafts")
        .join("summaries")
        .join(rel)
        .with_extension("md")
}

pub fn legacy_summary_for_raw(vault_root: &Path, raw_path: &Path) -> PathBuf {
    let summary_name = if raw_path.extension().and_then(|value| value.to_str()) == Some("md") {
        raw_path.file_name().map(PathBuf::from).unwrap_or_default()
    } else {
        PathBuf::from(format!(
            "{}.md",
            raw_path
                .file_stem()
                .map(|value| value.to_string_lossy())
                .unwrap_or_default()
        ))
    };
    vault_root.join("wiki").join("summaries").join(summary_name)
}

pub fn summary_for_raw(vault_root: &Path, raw_path: &Path) -> PathBuf {
    if detect_layout_family(vault_root) == "legacy-layout" {
        legacy_summary_for_raw(vault_root, raw_path)
    } else {
        draft_summary_for_raw(vault_root, raw_path)
    }
}

pub fn iso_from_timestamp(timestamp: std::time::SystemTime) -> String {
    let date: DateTime<Utc> = timestamp.into();
    date.with_nanosecond(0)
        .unwrap_or(date)
        .to_rfc3339()
        .replace("+00:00", "Z")
}

pub fn compute_hash(path: &Path) -> Result<String> {
    let bytes = fs::read(path)?;
    let mut digest = Sha256::new();
    digest.update(bytes);
    Ok(format!("{:x}", digest.finalize()))
}

pub fn collect_markdown_records(vault_root: &Path) -> Result<Vec<MarkdownRecord>> {
    let layout_family = detect_layout_family(vault_root);
    let mut records = if layout_family == "review-gated" {
        iter_markdown_records(
            vault_root,
            &[
                "raw",
                "wiki/live",
                "wiki/briefings",
                "outputs/qa",
                "outputs/content",
                "outputs/reports",
                "outputs/slides",
                "outputs/charts",
                "outputs/episodes",
                "outputs/reviews",
            ],
        )?
    } else {
        iter_markdown_records(vault_root, &["raw", "wiki", "outputs/qa"])?
    };
    let memory_path = vault_root.join("MEMORY.md");
    if layout_family == "review-gated" && memory_path.exists() {
        records.push(load_markdown(&memory_path, Some(vault_root))?);
    }
    Ok(records)
}
