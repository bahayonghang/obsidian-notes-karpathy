use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde_json::{Map, Value, json};

use crate::common::{list_field, load_markdown, now_iso, parse_scalar, relative_posix};
use crate::layout::{
    accepted_raw_sources, compute_hash, detect_companion_skills, manifest_path, pdf_ingest_plan,
    raw_source_metadata, resolve_vault_profile, sidecar_for_pdf, source_class_for_raw,
    summary_for_raw,
};

pub const MANIFEST_VERSION: i64 = 1;

const MANIFEST_FIELDS: [&str; 10] = [
    "source_id",
    "path",
    "source_type",
    "capture_origin",
    "source_url_or_handle",
    "content_hash",
    "first_seen_at",
    "last_seen_at",
    "ingest_status",
    "normalized_outputs",
];
const MANIFEST_OPTIONAL_SCALAR_FIELDS: [&str; 4] = [
    "deferred_to",
    "metadata_path",
    "capture_method",
    "source_profile",
];
const MANIFEST_OPTIONAL_LIST_FIELDS: [&str; 1] = ["linked_assets"];

fn yaml_scalar(value: Option<&Value>) -> String {
    match value {
        None | Some(Value::Null) => r#""""#.to_string(),
        Some(Value::Bool(flag)) => flag.to_string(),
        Some(Value::Number(number)) => number.to_string(),
        Some(Value::String(text)) => {
            serde_json::to_string(text).unwrap_or_else(|_| r#""""#.to_string())
        }
        Some(Value::Array(_)) | Some(Value::Object(_)) => r#""""#.to_string(),
    }
}

fn parse_manifest_scalar(raw_value: &str) -> Value {
    let value = raw_value.trim();
    if value.is_empty() {
        return Value::String(String::new());
    }
    if value == "true" {
        return Value::Bool(true);
    }
    if value == "false" {
        return Value::Bool(false);
    }
    if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
        return serde_json::from_str(value)
            .unwrap_or_else(|_| Value::String(value.trim_matches('"').to_string()));
    }
    if value.starts_with('\'') && value.ends_with('\'') && value.len() >= 2 {
        return Value::String(value[1..value.len() - 1].to_string());
    }
    if value.chars().all(|ch| ch.is_ascii_digit()) {
        return value
            .parse::<i64>()
            .map(|parsed| Value::Number(parsed.into()))
            .unwrap_or_else(|_| Value::String(value.to_string()));
    }
    parse_scalar(value)
}

fn source_record(
    vault_root: &Path,
    raw_path: &Path,
    source_class: &str,
) -> Result<Option<crate::common::MarkdownRecord>> {
    if source_class == "markdown" {
        return Ok(Some((*load_markdown(raw_path, Some(vault_root))?).clone()));
    }
    let sidecar_path = sidecar_for_pdf(raw_path);
    if sidecar_path.exists() {
        return Ok(Some(
            (*load_markdown(&sidecar_path, Some(vault_root))?).clone(),
        ));
    }
    Ok(None)
}

fn metadata_list(record: Option<&crate::common::MarkdownRecord>, keys: &[&str]) -> Vec<String> {
    let Some(record) = record else {
        return Vec::new();
    };
    let mut values = Vec::new();
    for key in keys {
        values.extend(list_field(&record.frontmatter, key));
    }
    values.sort();
    values.dedup();
    values
}

pub fn manifest_optional_metadata_for_source(
    vault_root: &Path,
    raw_path: &Path,
    source_class: &str,
    plan: Option<&Map<String, Value>>,
) -> Result<Map<String, Value>> {
    let record = source_record(vault_root, raw_path, source_class)?;
    let intake_meta = raw_source_metadata(vault_root, raw_path);

    let mut capture_method = record
        .as_ref()
        .and_then(|value| value.frontmatter.get("capture_method"))
        .and_then(Value::as_str)
        .map(|value| value.trim().to_string())
        .unwrap_or_default();
    if capture_method.is_empty() {
        let capture_source = intake_meta
            .get("capture_source")
            .and_then(Value::as_str)
            .unwrap_or_default();
        capture_method = if capture_source == "agent" {
            "agent-capture".to_string()
        } else if matches!(source_class, "image_asset" | "data_asset" | "paper_pdf") {
            "file-drop".to_string()
        } else if record
            .as_ref()
            .and_then(|value| value.frontmatter.get("clipped_at"))
            .is_some()
        {
            "web-clipper".to_string()
        } else {
            "manual-markdown".to_string()
        };
    }

    let mut source_profile = String::new();
    if let Some(record) = record.as_ref() {
        for key in ["source_profile", "account_id", "creator_profile", "channel"] {
            if let Some(Value::String(value)) = record.frontmatter.get(key)
                && !value.trim().is_empty()
            {
                source_profile = value.trim().to_string();
                break;
            }
        }
    }

    let linked_assets = metadata_list(
        record.as_ref(),
        &["linked_assets", "attachments", "images", "assets"],
    );

    let mut payload = Map::new();
    payload.insert("capture_method".to_string(), json!(capture_method));
    payload.insert("linked_assets".to_string(), json!(linked_assets));
    if !source_profile.is_empty() {
        payload.insert("source_profile".to_string(), json!(source_profile));
    }
    if let Some(plan) = plan
        && let Some(Value::String(metadata_path)) = plan.get("metadata_path")
        && !metadata_path.trim().is_empty()
    {
        payload.insert("metadata_path".to_string(), json!(metadata_path.trim()));
    }
    Ok(payload)
}

pub fn load_source_manifest(vault_root: &Path) -> Result<Value> {
    let path = manifest_path(vault_root);
    if !path.exists() {
        return Ok(json!({
            "version": MANIFEST_VERSION,
            "generated_at": Value::Null,
            "profile": resolve_vault_profile(vault_root),
            "sources": [],
        }));
    }

    let text = fs::read_to_string(&path)?;
    let mut data = Map::new();
    data.insert("version".to_string(), json!(MANIFEST_VERSION));
    data.insert("generated_at".to_string(), Value::Null);
    data.insert("profile".to_string(), Value::Null);
    data.insert("sources".to_string(), Value::Array(Vec::new()));

    let mut current: Option<Map<String, Value>> = None;
    let mut active_list_field: Option<String> = None;

    for raw_line in text.lines() {
        if raw_line.trim().is_empty() || raw_line.trim_start().starts_with('#') {
            continue;
        }
        if raw_line.starts_with("version:") {
            data.insert(
                "version".to_string(),
                parse_manifest_scalar(raw_line.split_once(':').unwrap().1),
            );
            continue;
        }
        if raw_line.starts_with("generated_at:") {
            data.insert(
                "generated_at".to_string(),
                parse_manifest_scalar(raw_line.split_once(':').unwrap().1),
            );
            continue;
        }
        if raw_line.starts_with("profile:") {
            data.insert(
                "profile".to_string(),
                parse_manifest_scalar(raw_line.split_once(':').unwrap().1),
            );
            continue;
        }
        if raw_line.starts_with("sources:") {
            continue;
        }
        if let Some(remainder) = raw_line.strip_prefix("  - ") {
            if let Some(existing) = current.take() {
                data.get_mut("sources")
                    .and_then(Value::as_array_mut)
                    .expect("sources array")
                    .push(Value::Object(existing));
            }
            let mut map = Map::new();
            active_list_field = None;
            if let Some((key, value)) = remainder.split_once(':') {
                map.insert(key.trim().to_string(), parse_manifest_scalar(value));
            }
            current = Some(map);
            continue;
        }
        let Some(current_map) = current.as_mut() else {
            continue;
        };
        if raw_line.starts_with("    ") && raw_line.trim_end().ends_with(':') {
            let list_key = raw_line.trim().trim_end_matches(':').to_string();
            if list_key == "normalized_outputs"
                || MANIFEST_OPTIONAL_LIST_FIELDS.contains(&list_key.as_str())
            {
                current_map.insert(list_key.clone(), Value::Array(Vec::new()));
                active_list_field = Some(list_key);
                continue;
            }
        }
        if let Some(field) = active_list_field.as_ref()
            && let Some(stripped) = raw_line.strip_prefix("      - ")
        {
            current_map
                .entry(field.clone())
                .or_insert_with(|| Value::Array(Vec::new()))
                .as_array_mut()
                .expect("manifest list")
                .push(parse_manifest_scalar(stripped));
            continue;
        }
        active_list_field = None;
        if raw_line.starts_with("    ")
            && let Some((key, value)) = raw_line.trim().split_once(':')
        {
            current_map.insert(key.trim().to_string(), parse_manifest_scalar(value));
        }
    }

    if let Some(existing) = current.take() {
        data.get_mut("sources")
            .and_then(Value::as_array_mut)
            .expect("sources array")
            .push(Value::Object(existing));
    }

    if data
        .get("profile")
        .map(|value| value.is_null())
        .unwrap_or(true)
    {
        data.insert(
            "profile".to_string(),
            json!(resolve_vault_profile(vault_root)),
        );
    }

    if let Some(items) = data.get_mut("sources").and_then(Value::as_array_mut) {
        for item in items {
            if let Some(map) = item.as_object_mut() {
                let normalized_outputs = map
                    .get("normalized_outputs")
                    .and_then(Value::as_array)
                    .map(|values| {
                        values
                            .iter()
                            .filter_map(|value| value.as_str().map(ToString::to_string))
                            .filter(|v| !v.is_empty())
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                map.insert("normalized_outputs".to_string(), json!(normalized_outputs));
                let linked_assets = map
                    .get("linked_assets")
                    .and_then(Value::as_array)
                    .map(|values| {
                        values
                            .iter()
                            .filter_map(|value| value.as_str().map(ToString::to_string))
                            .filter(|v| !v.is_empty())
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                map.insert("linked_assets".to_string(), json!(linked_assets));
            }
        }
    }

    Ok(Value::Object(data))
}

pub fn write_source_manifest(vault_root: &Path, payload: &Value) -> Result<PathBuf> {
    let path = manifest_path(vault_root);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut lines = vec![
        format!(
            "version: {}",
            payload
                .get("version")
                .and_then(Value::as_i64)
                .unwrap_or(MANIFEST_VERSION)
        ),
        format!(
            "generated_at: {}",
            yaml_scalar(
                payload
                    .get("generated_at")
                    .filter(|value| !value.is_null())
                    .or(Some(&json!(now_iso())))
            )
        ),
        format!(
            "profile: {}",
            yaml_scalar(
                payload
                    .get("profile")
                    .filter(|value| !value.is_null())
                    .or(Some(&json!(resolve_vault_profile(vault_root))))
            )
        ),
        "sources:".to_string(),
    ];
    for item in payload
        .get("sources")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
    {
        let map = item.as_object().cloned().unwrap_or_default();
        lines.push(format!(
            "  - source_id: {}",
            yaml_scalar(map.get("source_id"))
        ));
        for field in MANIFEST_FIELDS.iter().skip(1) {
            if *field == "normalized_outputs" {
                lines.push("    normalized_outputs:".to_string());
                let values = map
                    .get(*field)
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default();
                if values.is_empty() {
                    lines.push(r#"      - ""#.to_string());
                } else {
                    for entry in values {
                        lines.push(format!("      - {}", yaml_scalar(Some(&entry))));
                    }
                }
                continue;
            }
            lines.push(format!("    {field}: {}", yaml_scalar(map.get(*field))));
        }
        for field in MANIFEST_OPTIONAL_SCALAR_FIELDS {
            if map.get(field).is_some()
                && !map.get(field).unwrap().is_null()
                && map.get(field).unwrap() != &Value::String(String::new())
            {
                lines.push(format!("    {field}: {}", yaml_scalar(map.get(field))));
            }
        }
        for field in MANIFEST_OPTIONAL_LIST_FIELDS {
            if !map.contains_key(field) {
                continue;
            }
            lines.push(format!("    {field}:"));
            let values = map
                .get(field)
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();
            if values.is_empty() {
                lines.push(r#"      - ""#.to_string());
            } else {
                for entry in values {
                    lines.push(format!("      - {}", yaml_scalar(Some(&entry))));
                }
            }
        }
    }
    fs::write(&path, format!("{}\n", lines.join("\n")))?;
    Ok(path)
}

fn stable_source_id(rel_path: &str) -> String {
    rel_path
        .replace('\\', "/")
        .replace('/', "--")
        .trim_end_matches(".md")
        .to_string()
}

fn source_url_or_handle(
    vault_root: &Path,
    raw_path: &Path,
    source_class: &str,
    plan: Option<&Map<String, Value>>,
) -> Result<String> {
    if let Some(plan) = plan {
        for key in ["source_url", "paper_handle"] {
            if let Some(Value::String(value)) = plan.get(key)
                && !value.trim().is_empty()
            {
                return Ok(value.trim().to_string());
            }
        }
    }
    if source_class == "markdown" {
        let record = load_markdown(raw_path, Some(vault_root))?;
        for key in ["source", "paper_id"] {
            if let Some(Value::String(value)) = record.frontmatter.get(key)
                && !value.trim().is_empty()
            {
                return Ok(value.trim().to_string());
            }
        }
    }
    let sidecar_path = sidecar_for_pdf(raw_path);
    if sidecar_path.exists() {
        let record = load_markdown(&sidecar_path, Some(vault_root))?;
        for key in ["source", "paper_id"] {
            if let Some(Value::String(value)) = record.frontmatter.get(key)
                && !value.trim().is_empty()
            {
                return Ok(value.trim().to_string());
            }
        }
    }
    Ok(String::new())
}

fn manifest_entry_for_source(
    vault_root: &Path,
    raw_path: &Path,
    existing: Option<&Map<String, Value>>,
    companion_status: &Map<String, Value>,
    now_iso_value: &str,
) -> Result<Map<String, Value>> {
    let rel_path = relative_posix(raw_path, vault_root);
    let source_class = source_class_for_raw(Path::new(&rel_path));
    let summary_path = relative_posix(&summary_for_raw(vault_root, raw_path), vault_root);
    let source_meta = raw_source_metadata(vault_root, raw_path);
    let plan = if source_class == "paper_pdf" {
        Some(pdf_ingest_plan(vault_root, raw_path, companion_status))
    } else {
        None
    };
    let optional_meta =
        manifest_optional_metadata_for_source(vault_root, raw_path, &source_class, plan.as_ref())?;

    let ingest_status = if source_class == "paper_pdf" {
        if plan
            .as_ref()
            .and_then(|value| value.get("ingest_plan"))
            .and_then(Value::as_str)
            == Some("paper-workbench")
        {
            "deferred"
        } else {
            "deferred-missing-skill"
        }
    } else {
        "ready-for-compile"
    };

    let mut entry = Map::new();
    entry.insert(
        "source_id".to_string(),
        existing
            .and_then(|value| value.get("source_id"))
            .cloned()
            .unwrap_or_else(|| json!(stable_source_id(&rel_path))),
    );
    entry.insert("path".to_string(), json!(rel_path));
    entry.insert("source_type".to_string(), json!(source_class));
    entry.insert(
        "capture_origin".to_string(),
        source_meta
            .get("capture_source")
            .cloned()
            .unwrap_or_else(|| json!("legacy")),
    );
    entry.insert(
        "source_url_or_handle".to_string(),
        json!(source_url_or_handle(
            vault_root,
            raw_path,
            &source_class,
            plan.as_ref()
        )?),
    );
    entry.insert("content_hash".to_string(), json!(compute_hash(raw_path)?));
    entry.insert(
        "first_seen_at".to_string(),
        existing
            .and_then(|value| value.get("first_seen_at"))
            .cloned()
            .unwrap_or_else(|| json!(now_iso_value)),
    );
    entry.insert("last_seen_at".to_string(), json!(now_iso_value));
    entry.insert("ingest_status".to_string(), json!(ingest_status));
    entry.insert("normalized_outputs".to_string(), json!([summary_path]));
    for (key, value) in optional_meta {
        entry.insert(key, value);
    }
    if let Some(plan) = plan
        && let Some(Value::String(ingest_plan)) = plan.get("ingest_plan")
        && (ingest_plan == "paper-workbench" || ingest_plan == "skip")
    {
        entry.insert("deferred_to".to_string(), json!("paper-workbench"));
    }
    Ok(entry)
}

pub fn scan_ingest_delta(vault_root: &Path) -> Result<Value> {
    let manifest = load_source_manifest(vault_root)?;
    let companion_status = detect_companion_skills(None)
        .get("skills")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let existing_by_path = manifest
        .get("sources")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|item| item.as_object().cloned())
        .filter_map(|item| {
            item.get("path")
                .and_then(Value::as_str)
                .map(|path| (path.to_string(), item.clone()))
        })
        .collect::<BTreeMap<_, _>>();

    let mut counts = BTreeMap::from([
        ("new".to_string(), 0_i64),
        ("changed".to_string(), 0_i64),
        ("unchanged".to_string(), 0_i64),
        ("removed".to_string(), 0_i64),
    ]);
    let mut items = Vec::new();
    let now = now_iso();
    let mut seen_paths = Vec::new();

    for raw_path in accepted_raw_sources(vault_root) {
        let rel_path = relative_posix(&raw_path, vault_root);
        seen_paths.push(rel_path.clone());
        let existing = existing_by_path.get(&rel_path);
        let candidate =
            manifest_entry_for_source(vault_root, &raw_path, existing, &companion_status, &now)?;
        let status = if let Some(existing) = existing {
            let changed = [
                "content_hash",
                "ingest_status",
                "source_url_or_handle",
                "capture_method",
                "source_profile",
            ]
            .iter()
            .any(|field| existing.get(*field) != candidate.get(*field))
                || existing.get("normalized_outputs") != candidate.get("normalized_outputs")
                || existing.get("linked_assets") != candidate.get("linked_assets");
            if changed {
                *counts.get_mut("changed").unwrap() += 1;
                "changed"
            } else {
                *counts.get_mut("unchanged").unwrap() += 1;
                "unchanged"
            }
        } else {
            *counts.get_mut("new").unwrap() += 1;
            "new"
        };
        let mut item = candidate.clone();
        item.insert("status".to_string(), json!(status));
        items.push(Value::Object(item));
    }

    for (rel_path, existing) in existing_by_path {
        if seen_paths.contains(&rel_path) {
            continue;
        }
        *counts.get_mut("removed").unwrap() += 1;
        let mut removed = existing.clone();
        removed.insert("status".to_string(), json!("removed"));
        items.push(Value::Object(removed));
    }

    let manifest_present = manifest_path(vault_root).exists();
    let needs_ingest = manifest_present
        && ["new", "changed", "removed"]
            .iter()
            .any(|key| counts.get(*key).copied().unwrap_or_default() > 0);
    let manifest_status = if manifest_present && !needs_ingest {
        "current"
    } else if manifest_present {
        "stale"
    } else {
        "missing"
    };
    let delta = crate::payload::IngestDelta {
        vault_root: crate::common::normalize_path_string(vault_root.to_string_lossy().as_ref()),
        profile: manifest
            .get("profile")
            .and_then(Value::as_str)
            .map(ToString::to_string)
            .unwrap_or_else(|| resolve_vault_profile(vault_root)),
        manifest_path: relative_posix(&manifest_path(vault_root), vault_root),
        manifest_present,
        manifest_status: manifest_status.to_string(),
        bootstrap_manifest_required: !manifest_present && !items.is_empty(),
        needs_ingest,
        counts,
        items,
        written_manifest: None,
    };
    Ok(serde_json::to_value(&delta)?)
}

pub fn sync_source_manifest(vault_root: &Path) -> Result<Value> {
    let delta_value = scan_ingest_delta(vault_root)?;
    let mut delta: crate::payload::IngestDelta = serde_json::from_value(delta_value)?;
    let manifest = load_source_manifest(vault_root)?;
    let current_entries = delta
        .items
        .iter()
        .filter_map(|item| item.as_object().cloned())
        .filter(|item| item.get("status").and_then(Value::as_str) != Some("removed"))
        .collect::<Vec<_>>();

    let mut sources = Vec::new();
    for entry in current_entries {
        let mut source = Map::new();
        for field in MANIFEST_FIELDS {
            source.insert(
                field.to_string(),
                entry.get(field).cloned().unwrap_or_else(|| {
                    if field == "normalized_outputs" {
                        Value::Array(Vec::new())
                    } else {
                        Value::String(String::new())
                    }
                }),
            );
        }
        for field in MANIFEST_OPTIONAL_SCALAR_FIELDS {
            if let Some(value) = entry.get(field) {
                source.insert(field.to_string(), value.clone());
            }
        }
        for field in MANIFEST_OPTIONAL_LIST_FIELDS {
            if let Some(value) = entry.get(field) {
                source.insert(field.to_string(), value.clone());
            }
        }
        sources.push(Value::Object(source));
    }
    sources.sort_by_key(|value| {
        value
            .get("path")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string()
    });

    let payload = json!({
        "version": MANIFEST_VERSION,
        "generated_at": now_iso(),
        "profile": manifest.get("profile").cloned().unwrap_or_else(|| json!(resolve_vault_profile(vault_root))),
        "sources": sources,
    });
    let output_path = write_source_manifest(vault_root, &payload)?;
    delta.written_manifest = Some(relative_posix(&output_path, vault_root));
    delta.manifest_present = true;
    delta.manifest_status = "current".to_string();
    delta.needs_ingest = false;
    delta.bootstrap_manifest_required = false;
    Ok(serde_json::to_value(&delta)?)
}
