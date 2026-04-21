use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use anyhow::Result;
use serde_json::{json, Map, Value};

use crate::common::{
    list_field, load_markdown, normalize_identity, parse_datetime, relative_posix,
    slugify_identity, write_markdown,
};
use crate::ingest::{load_source_manifest, manifest_optional_metadata_for_source};
use crate::layout::{
    accepted_raw_sources, compute_hash, detect_companion_skills, detect_layout_family,
    iso_from_timestamp, manifest_path, pdf_ingest_plan, raw_source_metadata, source_class_for_raw,
    summary_for_raw,
};

fn staleness_hint(
    raw_mtime_dt: Option<chrono::DateTime<chrono::Utc>>,
    now_dt: Option<chrono::DateTime<chrono::Utc>>,
) -> (bool, Option<i64>) {
    match (raw_mtime_dt, now_dt) {
        (Some(raw_mtime_dt), Some(now_dt)) => {
            let age_days = ((now_dt - raw_mtime_dt).num_seconds() / 86_400).max(0);
            (
                age_days > crate::config::STALENESS_HINT_DAYS,
                Some(age_days),
            )
        }
        _ => (false, None),
    }
}

fn identity_candidates(raw_path: &Path, metadata: &Map<String, Value>) -> BTreeSet<String> {
    let mut identities = BTreeSet::from([
        raw_path
            .file_stem()
            .map(|value| value.to_string_lossy().into_owned())
            .unwrap_or_default(),
        raw_path
            .file_stem()
            .map(|value| value.to_string_lossy().replace('-', " "))
            .unwrap_or_default(),
    ]);
    for key in ["title", "author"] {
        if let Some(Value::String(value)) = metadata.get(key) {
            if !value.trim().is_empty() {
                identities.insert(value.trim().to_string());
            }
        }
    }
    identities
        .into_iter()
        .filter(|identity| !normalize_identity(identity).is_empty())
        .collect()
}

fn candidate_record(
    vault_root: &Path,
    raw_path: &Path,
) -> Result<Option<crate::common::MarkdownRecord>> {
    if raw_path.extension().and_then(|value| value.to_str()) == Some("md") {
        return Ok(Some(load_markdown(raw_path, Some(vault_root))?));
    }
    let sidecar_path = raw_path.with_file_name(format!(
        "{}.source.md",
        raw_path
            .file_stem()
            .map(|value| value.to_string_lossy())
            .unwrap_or_default()
    ));
    if sidecar_path.exists() {
        return Ok(Some(load_markdown(&sidecar_path, Some(vault_root))?));
    }
    Ok(None)
}

fn clean_candidate_slug(value: &str) -> Option<String> {
    let slug = slugify_identity(value);
    if slug.is_empty()
        || [
            "article", "articles", "note", "notes", "raw", "human", "agent", "agents",
        ]
        .contains(&slug.as_str())
    {
        return None;
    }
    Some(slug)
}

fn topic_candidates(
    vault_root: &Path,
    raw_path: &Path,
    record: Option<&crate::common::MarkdownRecord>,
) -> Vec<String> {
    let rel = raw_path
        .strip_prefix(vault_root.join("raw"))
        .unwrap_or(raw_path)
        .components()
        .map(|part| part.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    let mut candidates = BTreeSet::new();
    for part in rel.iter().take(rel.len().saturating_sub(1)) {
        if let Some(slug) = clean_candidate_slug(part) {
            candidates.insert(slug);
        }
    }
    if let Some(record) = record {
        for tag in list_field(&record.frontmatter, "tags") {
            if let Some(slug) = clean_candidate_slug(&tag) {
                candidates.insert(slug);
            }
        }
    }
    let source_class = source_class_for_raw(raw_path);
    if source_class == "image_asset" {
        candidates.insert("assets".to_string());
    }
    if source_class == "data_asset" {
        candidates.insert("data".to_string());
    }
    if candidates.is_empty() {
        if let Some(slug) = raw_path
            .file_stem()
            .and_then(|value| clean_candidate_slug(&value.to_string_lossy()))
        {
            candidates.insert(slug);
        }
    }
    candidates.into_iter().collect()
}

fn concept_candidates(
    raw_path: &Path,
    record: Option<&crate::common::MarkdownRecord>,
) -> Vec<String> {
    let mut candidates = BTreeSet::new();
    if let Some(record) = record {
        for key in ["concepts", "tags"] {
            for value in list_field(&record.frontmatter, key) {
                if let Some(slug) = clean_candidate_slug(&value) {
                    candidates.insert(slug);
                }
            }
        }
        if let Some(Value::String(title)) = record.frontmatter.get("title") {
            if let Some(slug) = clean_candidate_slug(title) {
                candidates.insert(slug);
            }
        }
    }
    if candidates.is_empty() {
        if let Some(slug) = raw_path
            .file_stem()
            .and_then(|value| clean_candidate_slug(&value.to_string_lossy()))
        {
            candidates.insert(slug);
        }
    }
    candidates.into_iter().collect()
}

fn entity_candidates(
    raw_path: &Path,
    record: Option<&crate::common::MarkdownRecord>,
) -> Vec<String> {
    let mut candidates = BTreeSet::new();
    if let Some(record) = record {
        for key in ["author", "project", "tool", "library", "dataset", "entity"] {
            if let Some(Value::String(value)) = record.frontmatter.get(key) {
                if let Some(slug) = clean_candidate_slug(value) {
                    candidates.insert(slug);
                }
            }
        }
        for value in list_field(&record.frontmatter, "authors") {
            if let Some(slug) = clean_candidate_slug(&value) {
                candidates.insert(slug);
            }
        }
    }
    if raw_path
        .components()
        .any(|part| part.as_os_str().to_string_lossy() == "repos")
    {
        if let Some(slug) = raw_path
            .file_stem()
            .and_then(|value| clean_candidate_slug(&value.to_string_lossy()))
        {
            candidates.insert(slug);
        }
    }
    candidates.into_iter().collect()
}

fn relationship_candidates(
    topic_candidates: &[String],
    concept_candidates: &[String],
    entity_candidates: &[String],
) -> Vec<String> {
    let mut relationships = Vec::new();
    relationships.extend(
        topic_candidates
            .iter()
            .map(|slug| format!("belongs_to:[[wiki/live/topics/{slug}]]")),
    );
    relationships.extend(
        concept_candidates
            .iter()
            .map(|slug| format!("supports:[[wiki/live/concepts/{slug}]]")),
    );
    relationships.extend(
        entity_candidates
            .iter()
            .map(|slug| format!("mentions:[[wiki/live/entities/{slug}]]")),
    );
    relationships
}

fn review_package_meta_path(vault_root: &Path, raw_path: &Path) -> String {
    let rel = raw_path
        .strip_prefix(vault_root.join("raw"))
        .unwrap_or(raw_path);
    relative_posix(
        &vault_root
            .join("wiki")
            .join("drafts")
            .join("indices")
            .join("packages")
            .join(rel)
            .with_extension("md"),
        vault_root,
    )
}

fn source_package(vault_root: &Path, raw_path: &Path) -> Result<Map<String, Value>> {
    let record = candidate_record(vault_root, raw_path)?;
    let topic_candidates = topic_candidates(vault_root, raw_path, record.as_ref());
    let concept_candidates = concept_candidates(raw_path, record.as_ref());
    let entity_candidates = entity_candidates(raw_path, record.as_ref());
    let relationship_candidates =
        relationship_candidates(&topic_candidates, &concept_candidates, &entity_candidates);
    let review_meta_path = review_package_meta_path(vault_root, raw_path);
    let mut payload = Map::new();
    payload.insert(
        "summary".to_string(),
        json!(relative_posix(
            &summary_for_raw(vault_root, raw_path),
            vault_root
        )),
    );
    payload.insert("concept_candidates".to_string(), json!(concept_candidates));
    payload.insert("entity_candidates".to_string(), json!(entity_candidates));
    payload.insert(
        "relationship_candidates".to_string(),
        json!(relationship_candidates),
    );
    payload.insert("topic_candidates".to_string(), json!(topic_candidates));
    payload.insert("review_package_meta".to_string(), json!(review_meta_path));
    Ok(payload)
}

fn record_list(record: Option<&crate::common::MarkdownRecord>, keys: &[&str]) -> Vec<String> {
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

fn compile_method_payload(
    record: Option<&crate::common::MarkdownRecord>,
    topic_candidates: &[String],
    concept_candidates: &[String],
) -> Map<String, Value> {
    let boundary_conditions =
        record_list(record, &["boundary_conditions", "limits", "boundary_notes"]);
    let assumption_flags = record_list(record, &["assumption_flags", "assumptions"]);
    let mut transfer_targets = record_list(
        record,
        &[
            "transfer_targets",
            "cross_domain_targets",
            "transferable_to",
        ],
    );
    let mut core_conclusions = record_list(record, &["core_conclusions"]);
    let mut key_evidence = record_list(record, &["key_evidence"]);

    if core_conclusions.is_empty() {
        if let Some(record) = record {
            if let Some(Value::String(title)) = record.frontmatter.get("title") {
                if !title.trim().is_empty() {
                    core_conclusions.push(format!(
                        "{} introduces a durable candidate for the review gate.",
                        title.trim()
                    ));
                }
            }
        }
    }
    if key_evidence.is_empty() {
        if let Some(record) = record {
            if let Some(Value::String(source)) = record.frontmatter.get("source") {
                if !source.trim().is_empty() {
                    key_evidence.push(source.trim().to_string());
                }
            }
        }
    }
    if transfer_targets.is_empty() {
        if let Some(topic) = topic_candidates.first() {
            transfer_targets.push(format!("hub:{topic}"));
        } else if let Some(concept) = concept_candidates.first() {
            transfer_targets.push(format!("concept:{concept}"));
        }
    }

    let promotion_target = record
        .and_then(|record| record.frontmatter.get("promotion_target"))
        .and_then(Value::as_str)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "semantic".to_string());

    let mut payload = Map::new();
    payload.insert(
        "boundary_conditions".to_string(),
        json!(boundary_conditions),
    );
    payload.insert("assumption_flags".to_string(), json!(assumption_flags));
    payload.insert("transfer_targets".to_string(), json!(transfer_targets));
    payload.insert("core_conclusions".to_string(), json!(core_conclusions));
    payload.insert("key_evidence".to_string(), json!(key_evidence));
    payload.insert("promotion_target".to_string(), json!(promotion_target));
    payload
}

pub fn scan_compile_delta(vault_root: &Path) -> Result<Value> {
    let mut items = Vec::new();
    let mut counts = BTreeMap::from([
        ("new".to_string(), 0_i64),
        ("changed".to_string(), 0_i64),
        ("unchanged".to_string(), 0_i64),
        ("skipped".to_string(), 0_i64),
    ]);
    let mut ingest_counts: BTreeMap<String, i64> = BTreeMap::new();
    let companion_skills = detect_companion_skills(None);
    let companion_status = companion_skills
        .get("skills")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let layout_family = detect_layout_family(vault_root);
    let manifest = load_source_manifest(vault_root)?;
    let manifest_by_path = manifest
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
    let now_dt = parse_datetime(Some(&json!(crate::common::now_iso())));

    let mut identity_registry: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for rel_root in [
        "wiki/drafts/concepts",
        "wiki/live/concepts",
        "wiki/drafts/entities",
        "wiki/live/entities",
    ] {
        let root = vault_root.join(rel_root);
        if !root.exists() {
            continue;
        }
        for entry in walkdir::WalkDir::new(root).sort_by_file_name() {
            let entry = match entry {
                Ok(value) => value,
                Err(_) => continue,
            };
            if !entry.file_type().is_file()
                || entry.path().extension().and_then(|value| value.to_str()) != Some("md")
            {
                continue;
            }
            let record = load_markdown(entry.path(), Some(vault_root))?;
            let basename = entry
                .path()
                .file_stem()
                .map(|value| value.to_string_lossy().into_owned())
                .unwrap_or_default();
            identity_registry
                .entry(slugify_identity(&basename))
                .or_default()
                .insert(record.path.clone());
            for key in ["title", "canonical_name"] {
                if let Some(Value::String(value)) = record.frontmatter.get(key) {
                    if !value.trim().is_empty() {
                        identity_registry
                            .entry(slugify_identity(value))
                            .or_default()
                            .insert(record.path.clone());
                    }
                }
            }
            for alias in list_field(&record.frontmatter, "aliases") {
                identity_registry
                    .entry(slugify_identity(&alias))
                    .or_default()
                    .insert(record.path.clone());
            }
        }
    }

    for raw_path in accepted_raw_sources(vault_root) {
        let rel_path = relative_posix(&raw_path, vault_root);
        let summary_path = summary_for_raw(vault_root, &raw_path);
        let raw_hash = compute_hash(&raw_path)?;
        let raw_mtime = iso_from_timestamp(fs::metadata(&raw_path)?.modified()?);
        let mut item = Map::new();
        item.insert("path".to_string(), json!(rel_path.clone()));
        item.insert(
            "summary_path".to_string(),
            json!(relative_posix(&summary_path, vault_root)),
        );
        item.insert("raw_hash".to_string(), json!(raw_hash.clone()));
        item.insert("raw_mtime".to_string(), json!(raw_mtime.clone()));
        item.insert(
            "source_class".to_string(),
            json!(source_class_for_raw(Path::new(&rel_path))),
        );
        item.insert("layout_family".to_string(), json!(layout_family.clone()));
        item.insert(
            "manifest_path".to_string(),
            json!(relative_posix(&manifest_path(vault_root), vault_root)),
        );
        for (key, value) in raw_source_metadata(vault_root, &raw_path) {
            item.insert(key, value);
        }
        if let Some(entry) = manifest_by_path.get(&rel_path) {
            item.insert("manifest_status".to_string(), json!("tracked"));
            if let Some(value) = entry.get("source_id") {
                item.insert("manifest_source_id".to_string(), value.clone());
            }
            item.insert(
                "source_url_or_handle".to_string(),
                entry
                    .get("source_url_or_handle")
                    .cloned()
                    .unwrap_or_else(|| json!("")),
            );
            item.insert(
                "capture_method".to_string(),
                entry
                    .get("capture_method")
                    .cloned()
                    .unwrap_or_else(|| json!("")),
            );
            item.insert(
                "linked_assets".to_string(),
                entry
                    .get("linked_assets")
                    .cloned()
                    .unwrap_or_else(|| json!([])),
            );
            item.insert(
                "source_profile".to_string(),
                entry
                    .get("source_profile")
                    .cloned()
                    .unwrap_or_else(|| json!("")),
            );
        } else {
            item.insert("manifest_status".to_string(), json!("untracked"));
        }
        let raw_mtime_dt = parse_datetime(Some(&json!(raw_mtime.clone())));
        let (stale_hint, age_days) = staleness_hint(raw_mtime_dt, now_dt);
        item.insert("last_verified_at".to_string(), json!(raw_mtime));
        item.insert("possibly_outdated".to_string(), Value::Bool(stale_hint));
        if let Some(age_days) = age_days {
            item.insert("source_age_days".to_string(), json!(age_days));
        }

        let alias_candidates = identity_candidates(&raw_path, &item)
            .into_iter()
            .map(|identity| slugify_identity(&identity))
            .filter(|identity| !identity.is_empty())
            .collect::<BTreeSet<_>>();
        let duplicate_paths = alias_candidates
            .iter()
            .flat_map(|candidate| {
                identity_registry
                    .get(candidate)
                    .cloned()
                    .unwrap_or_default()
            })
            .collect::<BTreeSet<_>>();
        item.insert(
            "alias_candidates".to_string(),
            json!(alias_candidates.into_iter().collect::<Vec<_>>()),
        );
        item.insert(
            "duplicate_candidates".to_string(),
            json!(duplicate_paths.into_iter().collect::<Vec<_>>()),
        );

        let source_class = item
            .get("source_class")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        if source_class == "paper_pdf" {
            for (key, value) in pdf_ingest_plan(vault_root, &raw_path, &companion_status) {
                item.insert(key, value);
            }
        } else {
            item.insert("ingest_plan".to_string(), json!("markdown"));
            item.insert("ingest_reason".to_string(), json!("markdown_source"));
        }
        if !manifest_by_path.contains_key(&rel_path) {
            let optional_meta =
                manifest_optional_metadata_for_source(vault_root, &raw_path, &source_class, None)?;
            for (key, value) in optional_meta {
                item.entry(key).or_insert(value);
            }
        }

        let mut source_package = source_package(vault_root, &raw_path)?;
        source_package.insert(
            "capture_method".to_string(),
            item.get("capture_method")
                .cloned()
                .unwrap_or_else(|| json!("")),
        );
        source_package.insert(
            "linked_assets".to_string(),
            item.get("linked_assets")
                .cloned()
                .unwrap_or_else(|| json!([])),
        );
        if let Some(source_profile) = item.get("source_profile") {
            if !source_profile.is_null() {
                source_package.insert("source_profile".to_string(), source_profile.clone());
            }
        }
        item.insert("source_package".to_string(), Value::Object(source_package));

        let ingest_plan = item
            .get("ingest_plan")
            .and_then(Value::as_str)
            .unwrap_or("markdown")
            .to_string();
        *ingest_counts.entry(ingest_plan.clone()).or_insert(0) += 1;
        if ingest_plan == "skip" {
            *counts.get_mut("skipped").unwrap() += 1;
        }

        if !summary_path.exists() {
            item.insert("status".to_string(), json!("new"));
            item.insert("reason".to_string(), json!("missing_summary"));
            *counts.get_mut("new").unwrap() += 1;
            items.push(Value::Object(item));
            continue;
        }

        let summary_record = load_markdown(&summary_path, Some(vault_root))?;
        let summary_hash = summary_record
            .frontmatter
            .get("source_hash")
            .and_then(Value::as_str)
            .map(ToString::to_string);
        let summary_mtime = parse_datetime(summary_record.frontmatter.get("source_mtime"));
        let raw_mtime_dt = parse_datetime(item.get("raw_mtime"));

        if let Some(summary_hash) = summary_hash {
            item.insert(
                "recorded_source_hash".to_string(),
                json!(summary_hash.clone()),
            );
            if summary_hash == raw_hash {
                item.insert("status".to_string(), json!("unchanged"));
                item.insert("reason".to_string(), json!("source_hash_match"));
                *counts.get_mut("unchanged").unwrap() += 1;
            } else {
                item.insert("status".to_string(), json!("changed"));
                item.insert("reason".to_string(), json!("source_hash_mismatch"));
                *counts.get_mut("changed").unwrap() += 1;
            }
            items.push(Value::Object(item));
            continue;
        }

        let recorded_source_mtime = summary_record
            .frontmatter
            .get("source_mtime")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        if let (Some(summary_mtime), Some(raw_mtime_dt)) = (summary_mtime, raw_mtime_dt) {
            if raw_mtime_dt > summary_mtime {
                item.insert("status".to_string(), json!("changed"));
                item.insert("reason".to_string(), json!("source_mtime_outdated"));
                item.insert(
                    "recorded_source_mtime".to_string(),
                    json!(recorded_source_mtime),
                );
                *counts.get_mut("changed").unwrap() += 1;
            } else {
                item.insert("status".to_string(), json!("unchanged"));
                item.insert("reason".to_string(), json!("source_mtime_current"));
                item.insert(
                    "recorded_source_mtime".to_string(),
                    json!(recorded_source_mtime),
                );
                *counts.get_mut("unchanged").unwrap() += 1;
            }
        } else {
            item.insert("status".to_string(), json!("unchanged"));
            item.insert("reason".to_string(), json!("source_mtime_current"));
            item.insert(
                "recorded_source_mtime".to_string(),
                json!(recorded_source_mtime),
            );
            *counts.get_mut("unchanged").unwrap() += 1;
        }

        items.push(Value::Object(item));
    }

    let delta = crate::payload::CompileDelta {
        vault_root: crate::common::normalize_path_string(vault_root.to_string_lossy().as_ref()),
        layout_family: layout_family.to_string(),
        counts,
        ingest_counts,
        companion_skills: detect_companion_skills(None),
        items,
    };
    Ok(serde_json::to_value(&delta)?)
}

fn title_for_source(raw_path: &Path, record: Option<&crate::common::MarkdownRecord>) -> String {
    if let Some(record) = record {
        if let Some(Value::String(title)) = record.frontmatter.get("title") {
            if !title.trim().is_empty() {
                return title.trim().to_string();
            }
        }
    }
    raw_path
        .file_stem()
        .map(|value| value.to_string_lossy().replace('-', " "))
        .unwrap_or_default()
}

fn merge_topic_source_refs(path: &Path, source_ref: &str) -> Result<Vec<String>> {
    let mut merged = BTreeSet::new();
    if path.exists() {
        let record = load_markdown(path, None)?;
        for source in list_field(&record.frontmatter, "source_refs") {
            merged.insert(source);
        }
    }
    merged.insert(source_ref.to_string());
    Ok(merged.into_iter().collect())
}

pub fn build_draft_packages(vault_root: &Path, write: bool) -> Result<Value> {
    let delta = scan_compile_delta(vault_root)?;
    let items = delta
        .get("items")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut packages = Vec::new();
    let mut written_paths = Vec::new();

    for item in items {
        let map = item.as_object().cloned().unwrap_or_default();
        let status = map
            .get("status")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if !matches!(status, "new" | "changed") {
            continue;
        }
        if map.get("ingest_plan").and_then(Value::as_str) == Some("skip") {
            continue;
        }

        let raw_path = vault_root.join(map.get("path").and_then(Value::as_str).unwrap_or_default());
        let record = candidate_record(vault_root, &raw_path)?;
        let source_package = map
            .get("source_package")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default();
        let topic_candidates = source_package
            .get("topic_candidates")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter_map(|value| value.as_str().map(ToString::to_string))
            .collect::<Vec<_>>();
        let concept_candidates = source_package
            .get("concept_candidates")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter_map(|value| value.as_str().map(ToString::to_string))
            .collect::<Vec<_>>();
        let entity_candidates = source_package
            .get("entity_candidates")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter_map(|value| value.as_str().map(ToString::to_string))
            .collect::<Vec<_>>();
        let relationship_candidates = source_package
            .get("relationship_candidates")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter_map(|value| value.as_str().map(ToString::to_string))
            .collect::<Vec<_>>();
        let compile_method =
            compile_method_payload(record.as_ref(), &topic_candidates, &concept_candidates);
        let package = json!({
            "source_id": map.get("manifest_source_id").cloned().unwrap_or_else(|| json!(map.get("path").and_then(Value::as_str).unwrap_or_default().replace("/", "--").trim_end_matches(".md"))),
            "source_path": map.get("path").cloned().unwrap_or_else(|| json!("")),
            "title": title_for_source(&raw_path, record.as_ref()),
            "source_package": source_package,
            "capture_source": map.get("capture_source").cloned().unwrap_or_else(|| json!("legacy")),
            "source_class": map.get("source_class").cloned().unwrap_or_else(|| json!("markdown")),
            "capture_method": map.get("capture_method").cloned().unwrap_or_else(|| json!("")),
            "linked_assets": map.get("linked_assets").cloned().unwrap_or_else(|| json!([])),
            "source_profile": map.get("source_profile").cloned().unwrap_or_else(|| json!("")),
            "compile_method": compile_method,
        });
        packages.push(package.clone());

        if !write {
            continue;
        }

        let summary_path = vault_root.join(
            source_package
                .get("summary")
                .and_then(Value::as_str)
                .unwrap_or_default(),
        );
        let review_meta_path = vault_root.join(
            source_package
                .get("review_package_meta")
                .and_then(Value::as_str)
                .unwrap_or_default(),
        );
        let source_path = map.get("path").and_then(Value::as_str).unwrap_or_default();
        let source_ref = if source_path.ends_with(".md") {
            format!("[[{}]]", source_path.trim_end_matches(".md"))
        } else {
            source_path.to_string()
        };
        let compile_method = package
            .get("compile_method")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default();
        let title = package
            .get("title")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let promotion_target = compile_method
            .get("promotion_target")
            .and_then(Value::as_str)
            .unwrap_or("semantic");

        let mut summary_lines = vec![
            "---".to_string(),
            format!("title: {}", crate::common::json_string(title)),
            format!("source_file: {}", crate::common::json_string(&source_ref)),
            format!(
                "source_hash: {}",
                crate::common::json_string(
                    map.get("raw_hash")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                )
            ),
            format!(
                "source_mtime: {}",
                crate::common::json_string(
                    map.get("raw_mtime")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                )
            ),
            format!(
                "draft_id: {}",
                crate::common::json_string(
                    package
                        .get("source_id")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                )
            ),
            format!("compiled_from: {}", crate::common::json_string(&source_ref)),
            "capture_sources:".to_string(),
            format!("  - {}", crate::common::json_string(&source_ref)),
            r#"review_state: "pending""#.to_string(),
            r#"review_score: "0.75""#.to_string(),
            "blocking_flags: []".to_string(),
            format!(
                "promotion_target: {}",
                crate::common::json_string(promotion_target)
            ),
        ];
        for (field, values) in [
            ("topic_candidates", topic_candidates.clone()),
            ("concept_candidates", concept_candidates.clone()),
            ("entity_candidates", entity_candidates.clone()),
            ("relationship_candidates", relationship_candidates.clone()),
            (
                "boundary_conditions",
                compile_method
                    .get("boundary_conditions")
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|value| value.as_str().map(ToString::to_string))
                    .collect(),
            ),
            (
                "assumption_flags",
                compile_method
                    .get("assumption_flags")
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|value| value.as_str().map(ToString::to_string))
                    .collect(),
            ),
            (
                "transfer_targets",
                compile_method
                    .get("transfer_targets")
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|value| value.as_str().map(ToString::to_string))
                    .collect(),
            ),
        ] {
            summary_lines.push(format!("{field}:"));
            if values.is_empty() {
                summary_lines.push(r#"  - """#.to_string());
            } else {
                for value in values {
                    summary_lines.push(format!("  - {}", crate::common::json_string(&value)));
                }
            }
        }
        summary_lines.push(format!(
            "review_package_meta: {}",
            crate::common::json_string(&format!(
                "[[{}]]",
                source_package
                    .get("review_package_meta")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .trim_end_matches(".md")
            ))
        ));
        summary_lines.push("---".to_string());
        summary_lines.push(String::new());
        summary_lines.push(format!("# {title}"));
        summary_lines.push(String::new());
        summary_lines.push("## Source Package".to_string());
        summary_lines.push(String::new());
        summary_lines.push(format!("- source_path: `{source_path}`"));
        summary_lines.push(format!(
            "- source_class: `{}`",
            map.get("source_class")
                .and_then(Value::as_str)
                .unwrap_or_default()
        ));
        summary_lines.push(format!(
            "- capture_source: `{}`",
            map.get("capture_source")
                .and_then(Value::as_str)
                .unwrap_or_default()
        ));
        summary_lines.push(format!(
            "- capture_method: `{}`",
            map.get("capture_method")
                .and_then(Value::as_str)
                .unwrap_or("unknown")
        ));
        summary_lines.push(format!(
            "- source_profile: `{}`",
            map.get("source_profile")
                .and_then(Value::as_str)
                .unwrap_or("none")
        ));
        summary_lines.push(String::new());
        summary_lines.push("## Compression".to_string());
        summary_lines.push(String::new());
        summary_lines.push("### Core Conclusions".to_string());
        summary_lines.push(String::new());
        let core_conclusions = compile_method
            .get("core_conclusions")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        if core_conclusions.is_empty() {
            summary_lines.push("- No core conclusions extracted yet.".to_string());
        } else {
            for value in core_conclusions {
                if let Some(text) = value.as_str() {
                    summary_lines.push(format!("- {text}"));
                }
            }
        }
        summary_lines.push(String::new());
        summary_lines.push("### Key Evidence".to_string());
        summary_lines.push(String::new());
        let key_evidence = compile_method
            .get("key_evidence")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        if key_evidence.is_empty() {
            summary_lines.push("- No key evidence extracted yet.".to_string());
        } else {
            for value in key_evidence {
                if let Some(text) = value.as_str() {
                    summary_lines.push(format!("- {text}"));
                }
            }
        }
        summary_lines.push(String::new());
        summary_lines.push("## Assumption Checks".to_string());
        summary_lines.push(String::new());
        summary_lines.push("### Assumption Flags".to_string());
        summary_lines.push(String::new());
        let assumption_flags = compile_method
            .get("assumption_flags")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        if assumption_flags.is_empty() {
            summary_lines.push("- None surfaced yet.".to_string());
        } else {
            for value in assumption_flags {
                if let Some(text) = value.as_str() {
                    summary_lines.push(format!("- {text}"));
                }
            }
        }
        summary_lines.push(String::new());
        summary_lines.push("### Boundary Conditions".to_string());
        summary_lines.push(String::new());
        let boundary_conditions = compile_method
            .get("boundary_conditions")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        if boundary_conditions.is_empty() {
            summary_lines.push("- None surfaced yet.".to_string());
        } else {
            for value in boundary_conditions {
                if let Some(text) = value.as_str() {
                    summary_lines.push(format!("- {text}"));
                }
            }
        }
        summary_lines.push(String::new());
        summary_lines.push("## Transfer Targets".to_string());
        summary_lines.push(String::new());
        let transfer_targets = compile_method
            .get("transfer_targets")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        if transfer_targets.is_empty() {
            summary_lines.push("- None surfaced yet.".to_string());
        } else {
            for value in transfer_targets {
                if let Some(text) = value.as_str() {
                    summary_lines.push(format!("- {text}"));
                }
            }
        }
        let linked_assets = map
            .get("linked_assets")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter_map(|value| value.as_str().map(ToString::to_string))
            .collect::<Vec<_>>();
        if !linked_assets.is_empty() {
            summary_lines.push(String::new());
            summary_lines.push("## Linked Assets".to_string());
            summary_lines.push(String::new());
            for value in linked_assets {
                summary_lines.push(format!("- `{value}`"));
            }
        }
        for (heading, values) in [
            ("Candidate Topics", topic_candidates.clone()),
            ("Candidate Concepts", concept_candidates.clone()),
            ("Candidate Entities", entity_candidates.clone()),
        ] {
            summary_lines.push(String::new());
            summary_lines.push(format!("## {heading}"));
            summary_lines.push(String::new());
            if values.is_empty() {
                summary_lines.push("- None".to_string());
            } else {
                for value in values {
                    if heading == "Candidate Topics" {
                        summary_lines.push(format!("- [[wiki/drafts/topics/{value}]]"));
                    } else {
                        summary_lines.push(format!("- `{value}`"));
                    }
                }
            }
        }
        summary_lines.push(String::new());
        summary_lines.push("## Relationship Candidates".to_string());
        summary_lines.push(String::new());
        if relationship_candidates.is_empty() {
            summary_lines.push("- None".to_string());
        } else {
            for value in &relationship_candidates {
                summary_lines.push(format!("- `{value}`"));
            }
        }
        write_markdown(&summary_path, &summary_lines)?;
        written_paths.push(relative_posix(&summary_path, vault_root));

        for slug in &topic_candidates {
            let topic_path = vault_root
                .join("wiki")
                .join("drafts")
                .join("topics")
                .join(format!("{slug}.md"));
            let source_refs = merge_topic_source_refs(&topic_path, &source_ref)?;
            let mut topic_lines = vec![
                "---".to_string(),
                format!(
                    "title: {}",
                    crate::common::json_string(&slug.replace('-', " "))
                ),
                format!(
                    "draft_id: {}",
                    crate::common::json_string(&format!("topic-{slug}"))
                ),
                r#"review_state: "pending""#.to_string(),
                r#"review_score: "0.70""#.to_string(),
                "source_refs:".to_string(),
            ];
            for value in &source_refs {
                topic_lines.push(format!("  - {}", crate::common::json_string(value)));
            }
            topic_lines.push("---".to_string());
            topic_lines.push(String::new());
            topic_lines.push(format!("# {}", slug.replace('-', " ")));
            topic_lines.push(String::new());
            topic_lines.push("## Why this topic exists".to_string());
            topic_lines.push(String::new());
            topic_lines.push(
                "- Candidate browse-layer topic compiled from recent source packages.".to_string(),
            );
            topic_lines.push(String::new());
            topic_lines.push("## Source Refs".to_string());
            topic_lines.push(String::new());
            for value in source_refs {
                topic_lines.push(format!("- {value}"));
            }
            write_markdown(&topic_path, &topic_lines)?;
            let topic_rel = relative_posix(&topic_path, vault_root);
            if !written_paths.contains(&topic_rel) {
                written_paths.push(topic_rel);
            }
        }

        let mut package_lines = vec![
            "---".to_string(),
            format!(
                "title: {}",
                crate::common::json_string(&format!(
                    "Review Package: {}",
                    package
                        .get("source_id")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                ))
            ),
            format!(
                "source_id: {}",
                crate::common::json_string(
                    package
                        .get("source_id")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                )
            ),
            r#"review_state: "pending""#.to_string(),
            format!(
                "summary_path: {}",
                crate::common::json_string(&format!(
                    "[[{}]]",
                    source_package
                        .get("summary")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .trim_end_matches(".md")
                ))
            ),
            format!(
                "promotion_target: {}",
                crate::common::json_string(promotion_target)
            ),
            "topic_candidates:".to_string(),
        ];
        if topic_candidates.is_empty() {
            package_lines.push(r#"  - ""#.to_string());
        } else {
            for value in &topic_candidates {
                package_lines.push(format!(
                    "  - {}",
                    crate::common::json_string(&format!("[[wiki/drafts/topics/{value}]]"))
                ));
            }
        }
        package_lines.push("relationship_candidates:".to_string());
        if relationship_candidates.is_empty() {
            package_lines.push(r#"  - ""#.to_string());
        } else {
            for value in &relationship_candidates {
                package_lines.push(format!("  - {}", crate::common::json_string(value)));
            }
        }
        package_lines.push("---".to_string());
        package_lines.push(String::new());
        package_lines.push(format!(
            "# Review Package: {}",
            package
                .get("source_id")
                .and_then(Value::as_str)
                .unwrap_or_default()
        ));
        package_lines.push(String::new());
        package_lines.push(format!(
            "- summary: [[{}]]",
            source_package
                .get("summary")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .trim_end_matches(".md")
        ));
        package_lines.push(format!("- source: `{source_path}`"));
        package_lines.push(format!(
            "- capture_method: `{}`",
            map.get("capture_method")
                .and_then(Value::as_str)
                .unwrap_or("unknown")
        ));
        write_markdown(&review_meta_path, &package_lines)?;
        written_paths.push(relative_posix(&review_meta_path, vault_root));
    }

    let result = crate::payload::CompileBuildResult {
        vault_root: crate::common::normalize_path_string(vault_root.to_string_lossy().as_ref()),
        package_count: packages.len(),
        packages,
        written_paths,
    };
    Ok(serde_json::to_value(&result)?)
}
