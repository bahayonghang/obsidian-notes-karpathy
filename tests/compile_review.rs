mod common;

use std::fs;

use common::{copy_dir, fixtures_root, run_json, run_json_with_env};
use tempfile::tempdir;

#[test]
fn ingest_scan_reports_manifest_drift() {
    let payload = run_json(&[
        "--json",
        "ingest",
        "scan",
        fixtures_root().join("needs-ingest").to_str().unwrap(),
    ]);
    assert_eq!(payload["manifest_present"].as_bool(), Some(true));
    assert_eq!(payload["manifest_status"].as_str(), Some("stale"));
    assert_eq!(payload["needs_ingest"].as_bool(), Some(true));
    assert_eq!(payload["counts"]["new"].as_i64(), Some(1));
    assert_eq!(payload["counts"]["removed"].as_i64(), Some(1));
}

#[test]
fn compile_scan_reports_review_gated_capture_trust() {
    let payload = run_json(&[
        "--json",
        "compile",
        "scan",
        fixtures_root().join("needs-review").to_str().unwrap(),
    ]);
    assert_eq!(payload["layout_family"].as_str(), Some("review-gated"));
    assert_eq!(payload["counts"]["new"].as_i64(), Some(0));
    assert_eq!(payload["counts"]["changed"].as_i64(), Some(0));
    assert_eq!(payload["counts"]["unchanged"].as_i64(), Some(2));

    let items = payload["items"].as_array().expect("items");
    let human_item = items
        .iter()
        .find(|item| {
            item["path"].as_str() == Some("raw/human/articles/2026-04-01-compiler-safety.md")
        })
        .expect("human item");
    let agent_item = items
        .iter()
        .find(|item| {
            item["path"].as_str() == Some("raw/agents/researcher/2026-04-02-review-gates.md")
        })
        .expect("agent item");

    assert_eq!(human_item["capture_source"].as_str(), Some("human"));
    assert_eq!(human_item["capture_trust"].as_str(), Some("curated"));
    assert_eq!(
        human_item["summary_path"].as_str(),
        Some("wiki/drafts/summaries/human/articles/2026-04-01-compiler-safety.md")
    );
    assert_eq!(agent_item["capture_source"].as_str(), Some("agent"));
    assert_eq!(agent_item["capture_trust"].as_str(), Some("untrusted"));
    assert_eq!(agent_item["agent_role"].as_str(), Some("researcher"));
}

#[test]
fn compile_scan_reports_skipped_pdf_without_companion_skill() {
    let payload = run_json_with_env(
        &[
            "--json",
            "compile",
            "scan",
            fixtures_root()
                .join("paper-intake-with-handle")
                .to_str()
                .unwrap(),
        ],
        &common::repo_root(),
        &[("KB_COMPANION_SKILL_PATHS", "")],
    );
    assert_eq!(payload["counts"]["new"].as_i64(), Some(1));
    assert_eq!(payload["counts"]["skipped"].as_i64(), Some(1));
    assert_eq!(payload["items"][0]["ingest_plan"].as_str(), Some("skip"));
    assert_eq!(
        payload["items"][0]["ingest_reason"].as_str(),
        Some("paper_workbench_required_for_raw_papers")
    );
}

#[test]
fn review_queue_applies_mixed_gate() {
    let payload = run_json(&[
        "--json",
        "review",
        "queue",
        fixtures_root()
            .join("review-conflict-mixed-gate")
            .to_str()
            .unwrap(),
    ]);
    assert_eq!(payload["counts"]["approve"].as_i64(), Some(1));
    assert_eq!(payload["counts"]["reject"].as_i64(), Some(1));
    assert_eq!(payload["counts"]["needs-human"].as_i64(), Some(1));
}

#[test]
fn compile_build_writes_topics_and_review_meta() {
    let temp = tempdir().expect("tempdir");
    let fixture_copy = temp.path().join("needs-compilation");
    copy_dir(&fixtures_root().join("needs-compilation"), &fixture_copy);

    run_json(&[
        "--json",
        "ingest",
        "sync",
        fixture_copy.to_str().unwrap(),
        "--write",
    ]);
    let payload = run_json(&[
        "--json",
        "compile",
        "build",
        fixture_copy.to_str().unwrap(),
        "--write",
    ]);

    assert!(payload["package_count"].as_i64().unwrap_or_default() >= 1);
    assert!(fixture_copy
        .join("wiki")
        .join("drafts")
        .join("topics")
        .exists());
    assert!(fixture_copy
        .join("wiki")
        .join("drafts")
        .join("indices")
        .join("packages")
        .exists());
    let written_paths = payload["written_paths"].as_array().expect("written_paths");
    assert!(written_paths.iter().any(|item| item
        .as_str()
        .unwrap_or_default()
        .starts_with("wiki/drafts/topics/")));
    assert!(written_paths.iter().any(|item| item
        .as_str()
        .unwrap_or_default()
        .starts_with("wiki/drafts/indices/packages/")));
}

#[test]
fn compile_build_emits_three_step_fields() {
    let temp = tempdir().expect("tempdir");
    let fixture_copy = temp.path().join("needs-compilation");
    copy_dir(&fixtures_root().join("needs-compilation"), &fixture_copy);
    let raw_path = fixture_copy
        .join("raw")
        .join("human")
        .join("articles")
        .join("2026-04-12-three-step-source.md");
    fs::write(
        &raw_path,
        "---\ntitle: \"Three Step Source\"\ncore_conclusions:\n  - \"Compile should preserve signal instead of writing only a soft summary.\"\nkey_evidence:\n  - \"Founders need boundary conditions before they reuse a claim.\"\nassumption_flags:\n  - \"Assumes the source still matches the current market context.\"\nboundary_conditions:\n  - \"Applies to creator-led distribution, not generic SaaS.\"\ntransfer_targets:\n  - \"creator-growth-playbooks\"\npromotion_target: procedural\n---\n\n# Three Step Source\n",
    )
    .expect("write raw");

    run_json(&[
        "--json",
        "ingest",
        "sync",
        fixture_copy.to_str().unwrap(),
        "--write",
    ]);
    run_json(&[
        "--json",
        "compile",
        "build",
        fixture_copy.to_str().unwrap(),
        "--write",
    ]);

    let summary_text = fs::read_to_string(
        fixture_copy
            .join("wiki")
            .join("drafts")
            .join("summaries")
            .join("human")
            .join("articles")
            .join("2026-04-12-three-step-source.md"),
    )
    .expect("summary");
    assert!(summary_text.contains("boundary_conditions:"));
    assert!(summary_text.contains("assumption_flags:"));
    assert!(summary_text.contains("transfer_targets:"));
    assert!(summary_text.contains("promotion_target: \"procedural\""));
    assert!(summary_text.contains("## Compression"));
    assert!(summary_text.contains("## Assumption Checks"));
    assert!(summary_text.contains("## Transfer Targets"));
}

#[test]
fn ingest_sync_tracks_asset_data_and_profile_metadata() {
    let temp = tempdir().expect("tempdir");
    let fixture_copy = temp.path().join("ready-for-query");
    copy_dir(&fixtures_root().join("ready-for-query"), &fixture_copy);
    let asset_path = fixture_copy
        .join("raw")
        .join("human")
        .join("assets")
        .join("diagram.png");
    let data_path = fixture_copy
        .join("raw")
        .join("human")
        .join("data")
        .join("stats.json");
    fs::create_dir_all(asset_path.parent().unwrap()).expect("assets dir");
    fs::create_dir_all(data_path.parent().unwrap()).expect("data dir");
    fs::write(&asset_path, b"png-bytes").expect("asset");
    fs::write(&data_path, "{\"count\": 3}").expect("data");

    let manifest_payload = run_json(&[
        "--json",
        "ingest",
        "sync",
        fixture_copy.to_str().unwrap(),
        "--write",
    ]);
    let compile_payload = run_json(&["--json", "compile", "scan", fixture_copy.to_str().unwrap()]);

    let manifest_text = fs::read_to_string(
        fixture_copy.join(manifest_payload["written_manifest"].as_str().unwrap()),
    )
    .expect("manifest");
    assert!(manifest_text.contains("raw/human/assets/diagram.png"));
    assert!(manifest_text.contains("raw/human/data/stats.json"));
    let items = compile_payload["items"].as_array().expect("items");
    let asset_item = items
        .iter()
        .find(|item| item["path"].as_str() == Some("raw/human/assets/diagram.png"))
        .expect("asset item");
    let data_item = items
        .iter()
        .find(|item| item["path"].as_str() == Some("raw/human/data/stats.json"))
        .expect("data item");
    assert_eq!(asset_item["source_class"].as_str(), Some("image_asset"));
    assert_eq!(data_item["source_class"].as_str(), Some("data_asset"));
}
