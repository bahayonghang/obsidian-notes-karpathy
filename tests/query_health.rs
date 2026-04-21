mod common;

use std::fs;

use common::{copy_dir, fixtures_root, run_json};
use tempfile::tempdir;

#[test]
fn query_scope_ignores_raw_and_drafts_for_review_gated_layout() {
    let payload = run_json(&[
        "--json",
        "query",
        "scope",
        fixtures_root().join("ready-for-query").to_str().unwrap(),
    ]);
    assert_eq!(payload["layout_family"].as_str(), Some("review-gated"));
    assert!(payload["included_paths"]
        .as_array()
        .expect("included")
        .iter()
        .any(|item| item.as_str()
            == Some("wiki/live/summaries/human/articles/2026-04-05-approved-summary.md")));
    assert!(payload["included_paths"]
        .as_array()
        .expect("included")
        .iter()
        .any(|item| item.as_str() == Some("wiki/briefings/researcher.md")));
    assert!(payload["excluded_paths"]
        .as_array()
        .expect("excluded")
        .iter()
        .any(|item| item.as_str() == Some("raw/human/articles/2026-04-05-approved-summary.md")));
    assert!(payload["excluded_paths"]
        .as_array()
        .expect("excluded")
        .iter()
        .any(|item| item.as_str()
            == Some("wiki/drafts/summaries/human/articles/2026-04-05-approved-summary.md")));
}

#[test]
fn query_scope_excludes_memory_from_topic_retrieval() {
    let payload = run_json(&[
        "--json",
        "query",
        "scope",
        fixtures_root().join("memory-boundary").to_str().unwrap(),
    ]);
    assert_eq!(payload["layout_family"].as_str(), Some("review-gated"));
    assert!(payload["included_paths"]
        .as_array()
        .expect("included")
        .iter()
        .any(|item| item.as_str() == Some("wiki/live/concepts/editorial-boundary.md")));
    assert!(payload["excluded_paths"]
        .as_array()
        .expect("excluded")
        .iter()
        .any(|item| item.as_str() == Some("MEMORY.md")));
}

#[test]
fn fast_personal_profile_skips_briefing_refresh_route() {
    let temp = tempdir().expect("tempdir");
    let fixture_copy = temp.path().join("needs-briefing-refresh");
    copy_dir(
        &fixtures_root().join("needs-briefing-refresh"),
        &fixture_copy,
    );
    run_json(&[
        "--json",
        "ingest",
        "sync",
        fixture_copy.to_str().unwrap(),
        "--write",
    ]);
    let manifest_path = fixture_copy.join("raw").join("_manifest.yaml");
    let manifest_text = fs::read_to_string(&manifest_path).expect("manifest");
    fs::write(
        &manifest_path,
        manifest_text.replace("profile: \"governed-team\"", "profile: \"fast-personal\""),
    )
    .expect("write manifest");

    let payload = run_json(&["--json", "status", fixture_copy.to_str().unwrap()]);
    assert_eq!(payload["profile"].as_str(), Some("fast-personal"));
    assert_ne!(payload["state"].as_str(), Some("needs-briefing-refresh"));
}

#[test]
fn review_governance_can_write_indices() {
    let temp = tempdir().expect("tempdir");
    let fixture_copy = temp.path().join("ready-for-query");
    copy_dir(&fixtures_root().join("ready-for-query"), &fixture_copy);
    let payload = run_json(&[
        "--json",
        "review",
        "governance",
        fixture_copy.to_str().unwrap(),
        "--write",
    ]);

    let indices_root = fixture_copy.join("wiki").join("live").join("indices");
    assert!(indices_root.join("QUESTIONS.md").exists());
    assert!(indices_root.join("GAPS.md").exists());
    assert!(indices_root.join("ALIASES.md").exists());
    assert!(indices_root.join("ENTITIES.md").exists());
    assert!(indices_root.join("RELATIONSHIPS.md").exists());
    assert!(payload["questions"]
        .as_array()
        .expect("questions")
        .iter()
        .any(|item| item.as_str() == Some("When should governance indices be refreshed?")));
}

#[test]
fn review_lint_flags_backlog_and_boundary_issues() {
    let writeback = run_json(&[
        "--json",
        "status",
        fixtures_root().join("writeback-backlog").to_str().unwrap(),
    ]);
    let memory = run_json(&[
        "--json",
        "review",
        "lint",
        fixtures_root().join("memory-boundary").to_str().unwrap(),
    ]);
    let writeback_lint = run_json(&[
        "--json",
        "review",
        "lint",
        fixtures_root().join("writeback-backlog").to_str().unwrap(),
    ]);

    assert_eq!(writeback["state"].as_str(), Some("needs-maintenance"));
    assert_eq!(writeback["route"].as_str(), Some("kb-review"));

    let memory_issue_kinds = memory["issues"]
        .as_array()
        .expect("issues")
        .iter()
        .filter_map(|item| item["kind"].as_str())
        .collect::<Vec<_>>();
    let writeback_issue_kinds = writeback_lint["issues"]
        .as_array()
        .expect("issues")
        .iter()
        .filter_map(|item| item["kind"].as_str())
        .collect::<Vec<_>>();
    assert!(memory_issue_kinds.contains(&"memory_knowledge_mix"));
    assert!(writeback_issue_kinds.contains(&"writeback_backlog"));
}

#[test]
fn review_lint_covers_latest_health_flags() {
    let confidence = run_json(&[
        "--json",
        "review",
        "lint",
        fixtures_root().join("confidence-decay").to_str().unwrap(),
    ]);
    let supersession = run_json(&[
        "--json",
        "review",
        "lint",
        fixtures_root().join("supersession-chain").to_str().unwrap(),
    ]);
    let episodes = run_json(&[
        "--json",
        "review",
        "lint",
        fixtures_root()
            .join("episodic-consolidation")
            .to_str()
            .unwrap(),
    ]);

    let confidence_kinds = confidence["issues"]
        .as_array()
        .expect("issues")
        .iter()
        .filter_map(|item| item["kind"].as_str())
        .collect::<Vec<_>>();
    let supersession_kinds = supersession["issues"]
        .as_array()
        .expect("issues")
        .iter()
        .filter_map(|item| item["kind"].as_str())
        .collect::<Vec<_>>();
    let episode_kinds = episodes["issues"]
        .as_array()
        .expect("issues")
        .iter()
        .filter_map(|item| item["kind"].as_str())
        .collect::<Vec<_>>();

    assert!(confidence_kinds.contains(&"missing_confidence_metadata"));
    assert!(confidence_kinds.contains(&"confidence_decay_due"));
    assert!(supersession_kinds.contains(&"supersession_gap"));
    assert!(episode_kinds.contains(&"episodic_backlog"));
}

#[test]
fn render_writes_web_and_slide_outputs_with_grounding_metadata() {
    let temp = tempdir().expect("tempdir");
    let fixture_copy = temp.path().join("ready-for-query");
    copy_dir(&fixtures_root().join("ready-for-query"), &fixture_copy);

    let web_payload = run_json(&[
        "--json",
        "render",
        fixture_copy.to_str().unwrap(),
        "--mode",
        "web",
        "--source",
        "wiki/live/concepts/review-gate.md",
        "--write",
    ]);
    let slides_payload = run_json(&[
        "--json",
        "render",
        fixture_copy.to_str().unwrap(),
        "--mode",
        "slides",
        "--source",
        "wiki/live/concepts/review-gate.md",
        "--write",
        "--output",
        "outputs/slides/2026-04-10-review-gate.md",
    ]);

    let package_root = fixture_copy.join(web_payload["package_root"].as_str().unwrap());
    assert!(package_root.join("index.html").exists());
    assert!(package_root.join("manifest.json").exists());
    let slide_text =
        fs::read_to_string(fixture_copy.join(slides_payload["output_path"].as_str().unwrap()))
            .expect("slides");
    assert!(slide_text.contains("source_live_pages"));
    assert!(slide_text.contains("followup_route"));
}
