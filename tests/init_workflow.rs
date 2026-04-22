mod common;

use std::fs;

use common::{copy_dir, fixtures_root, run_json, run_json_with_env};
use tempfile::tempdir;

#[test]
fn init_creates_canonical_starter_assets() {
    let temp = tempdir().expect("tempdir");
    let fixture_copy = temp.path().join("needs-setup");
    copy_dir(&fixtures_root().join("needs-setup"), &fixture_copy);

    let payload = run_json(&[
        "--json",
        "init",
        fixture_copy.to_str().unwrap(),
        "--include-governance",
    ]);

    assert!(
        payload["created_dirs"]
            .as_array()
            .expect("created_dirs")
            .iter()
            .any(|item| item.as_str() == Some("raw/human/articles"))
    );
    assert!(fixture_copy.join("AGENTS.md").exists());
    assert!(fixture_copy.join("CLAUDE.md").exists());
    assert!(fixture_copy.join("MEMORY.md").exists());
    assert!(fixture_copy.join("raw").join("_manifest.yaml").exists());
    assert!(fixture_copy.join("wiki").join("index.md").exists());
    assert!(fixture_copy.join("wiki").join("log.md").exists());
    assert!(
        fixture_copy
            .join("wiki")
            .join("briefings")
            .join("researcher.md")
            .exists()
    );
    assert!(
        fixture_copy
            .join("wiki")
            .join("live")
            .join("indices")
            .join("QUESTIONS.md")
            .exists()
    );
    let manifest_text =
        fs::read_to_string(fixture_copy.join("raw").join("_manifest.yaml")).expect("manifest");
    assert!(manifest_text.contains("profile: \"governed-team\""));
}

#[test]
fn init_preserves_existing_support_files_while_repairing_structure() {
    let temp = tempdir().expect("tempdir");
    let fixture_copy = temp.path().join("needs-repair-live-only");
    copy_dir(
        &fixtures_root().join("needs-repair-live-only"),
        &fixture_copy,
    );

    let live_path = fixture_copy
        .join("wiki")
        .join("live")
        .join("summaries")
        .join("human")
        .join("articles")
        .join("2026-04-07-incomplete-live.md");
    let original_live_text = fs::read_to_string(&live_path).expect("live text");
    let payload = run_json(&["--json", "init", fixture_copy.to_str().unwrap()]);

    assert!(
        payload["preserved_files"]
            .as_array()
            .expect("preserved_files")
            .iter()
            .any(|item| item.as_str() == Some("wiki/index.md"))
    );
    assert!(fixture_copy.join("wiki").join("drafts").exists());
    assert!(fixture_copy.join("wiki").join("briefings").exists());
    assert!(fixture_copy.join("outputs").join("reviews").exists());
    let repaired_live_text = fs::read_to_string(&live_path).expect("repaired live");
    assert_eq!(original_live_text, repaired_live_text);
}

#[test]
fn init_surfaces_profile_choice_in_scaffolded_files() {
    let temp = tempdir().expect("tempdir");
    let fixture_copy = temp.path().join("needs-setup");
    copy_dir(&fixtures_root().join("needs-setup"), &fixture_copy);

    run_json(&[
        "--json",
        "init",
        fixture_copy.to_str().unwrap(),
        "--profile",
        "fast-personal",
        "--include-governance",
    ]);

    let manifest_text =
        fs::read_to_string(fixture_copy.join("raw").join("_manifest.yaml")).expect("manifest");
    let index_text = fs::read_to_string(fixture_copy.join("wiki").join("index.md")).expect("index");
    let briefing_text = fs::read_to_string(
        fixture_copy
            .join("wiki")
            .join("briefings")
            .join("researcher.md"),
    )
    .expect("briefing");

    assert!(manifest_text.contains("profile: \"fast-personal\""));
    assert!(index_text.contains("kb_profile: \"fast-personal\""));
    assert!(briefing_text.contains("kb_profile: \"fast-personal\""));
}

#[test]
fn migrate_preserves_originals_and_syncs_manifest() {
    let temp = tempdir().expect("tempdir");
    let fixture_copy = temp.path().join("needs-migration");
    copy_dir(&fixtures_root().join("needs-migration"), &fixture_copy);

    let payload = run_json(&["--json", "migrate", fixture_copy.to_str().unwrap()]);

    assert!(
        fixture_copy
            .join("raw")
            .join("articles")
            .join("2026-03-20-old-pattern.md")
            .exists()
    );
    assert!(
        fixture_copy
            .join("raw")
            .join("human")
            .join("articles")
            .join("2026-03-20-old-pattern.md")
            .exists()
    );
    assert!(
        fixture_copy
            .join("wiki")
            .join("summaries")
            .join("2026-03-20-old-pattern.md")
            .exists()
    );
    assert!(
        fixture_copy
            .join("wiki")
            .join("live")
            .join("summaries")
            .join("2026-03-20-old-pattern.md")
            .exists()
    );
    assert!(
        fixture_copy
            .join(payload["migration_report"].as_str().unwrap())
            .exists()
    );
    let manifest_text =
        fs::read_to_string(fixture_copy.join(payload["written_manifest"].as_str().unwrap()))
            .expect("manifest");
    assert!(manifest_text.contains("raw/human/articles/2026-03-20-old-pattern.md"));
}

#[test]
fn status_reports_stage_route_and_counts_for_key_states() {
    let expectations = [
        ("needs-setup", "needs-setup"),
        ("needs-ingest", "needs-ingest"),
        ("needs-review", "needs-review"),
        ("needs-maintenance", "needs-maintenance"),
        ("ready-for-query", "needs-maintenance"),
    ];

    for (fixture_name, expected_state) in expectations {
        let payload = run_json(&[
            "--json",
            "status",
            fixtures_root().join(fixture_name).to_str().unwrap(),
        ]);
        assert_eq!(payload["state"].as_str(), Some(expected_state));
        assert!(payload["counts"].get("raw_sources").is_some());
        assert!(payload["counts"].get("live_pages").is_some());
        assert!(
            payload["summary"]
                .as_str()
                .unwrap_or_default()
                .contains("Next step:")
        );
    }
}

#[test]
fn compile_scan_respects_companion_skill_path_override() {
    let fixture = fixtures_root().join("paper-intake-with-handle");
    let skill_home_root = fixtures_root().join("companion-skill-homes");
    let payload = run_json_with_env(
        &["--json", "compile", "scan", fixture.to_str().unwrap()],
        &common::repo_root(),
        &[(
            "KB_COMPANION_SKILL_PATHS",
            skill_home_root.join("both").to_str().unwrap(),
        )],
    );
    assert_eq!(payload["layout_family"].as_str(), Some("legacy-layout"));
    assert_eq!(
        payload["items"][0]["ingest_plan"].as_str(),
        Some("paper-workbench")
    );
    assert_eq!(
        payload["items"][0]["paper_handle"].as_str(),
        Some("1706.03762")
    );
}
