mod common;

use std::fs;

use common::{copy_dir, fixtures_root, read_text, run_json, tree_snapshot};
use tempfile::tempdir;

const CONCEPTS_REL: &str = "wiki/live/indices/CONCEPTS.md";
const EDITORIAL_REL: &str = "wiki/live/indices/EDITORIAL-PRIORITIES.md";
const RECENT_REL: &str = "wiki/live/indices/RECENT.md";

#[test]
fn review_indices_write_repairs_drift_and_reruns_clean() {
    let temp = tempdir().expect("tempdir");
    let fixture_copy = temp.path().join("ready-for-query");
    copy_dir(&fixtures_root().join("ready-for-query"), &fixture_copy);
    let vault = fixture_copy.to_str().unwrap();

    let scan = run_json(&["--json", "review", "indices", vault]);
    assert_eq!(scan["write"].as_bool(), Some(false));
    assert_eq!(scan["drift_count"].as_i64(), Some(5));
    assert_eq!(
        scan["files"][CONCEPTS_REL]["status"].as_str(),
        Some("missing")
    );
    assert_eq!(
        scan["files"][EDITORIAL_REL]["status"].as_str(),
        Some("excluded")
    );
    assert_eq!(scan["written_paths"].as_array().map(Vec::len), Some(0));

    let write = run_json(&["--json", "review", "indices", vault, "--write"]);
    assert_eq!(write["write"].as_bool(), Some(true));
    assert_eq!(write["written_paths"].as_array().map(Vec::len), Some(5));
    let concepts_text = read_text(&fixture_copy.join(CONCEPTS_REL));
    assert!(concepts_text.contains("- [[wiki/live/concepts/review-gate]] | Review Gate"));
    let index_text = read_text(&fixture_copy.join("wiki/live/indices/INDEX.md"));
    assert!(index_text.contains("- concepts: 1"));
    assert!(index_text.contains("- summaries: 1"));
    assert!(!fixture_copy.join(EDITORIAL_REL).exists());
    let audit_text = read_text(&fixture_copy.join("outputs/audit/operations.jsonl"));
    assert!(audit_text.contains("\"action\":\"rebuild_navigation_indices\""));

    let concepts_path = fixture_copy.join(CONCEPTS_REL);
    let mut dirty = read_text(&concepts_path);
    dirty.push_str("stale hand-written line\n");
    fs::write(&concepts_path, dirty).expect("dirty concepts");

    let drifted = run_json(&["--json", "review", "indices", vault]);
    assert_eq!(drifted["drift_count"].as_i64(), Some(1));
    assert_eq!(
        drifted["files"][CONCEPTS_REL]["status"].as_str(),
        Some("drifted")
    );

    let repair = run_json(&["--json", "review", "indices", vault, "--write"]);
    assert_eq!(
        repair["written_paths"]
            .as_array()
            .expect("written")
            .iter()
            .filter_map(|item| item.as_str())
            .collect::<Vec<_>>(),
        vec![CONCEPTS_REL]
    );

    let before = tree_snapshot(&fixture_copy.join("wiki/live/indices"));
    let rerun = run_json(&["--json", "review", "indices", vault, "--write"]);
    assert_eq!(rerun["drift_count"].as_i64(), Some(0));
    assert_eq!(rerun["written_paths"].as_array().map(Vec::len), Some(0));
    let after = tree_snapshot(&fixture_copy.join("wiki/live/indices"));
    assert_eq!(before, after);
}

#[test]
fn review_indices_reports_unmanaged_index_home_and_never_writes_it() {
    let temp = tempdir().expect("tempdir");
    let fixture_copy = temp.path().join("ready-for-query");
    copy_dir(&fixtures_root().join("ready-for-query"), &fixture_copy);
    let vault = fixture_copy.to_str().unwrap();
    let home_path = fixture_copy.join("wiki/index.md");
    let original_home = read_text(&home_path);

    let scan = run_json(&["--json", "review", "indices", vault]);
    assert_eq!(
        scan["files"]["wiki/index.md"]["status"].as_str(),
        Some("unmanaged")
    );

    let write = run_json(&["--json", "review", "indices", vault, "--write"]);
    assert!(
        !write["written_paths"]
            .as_array()
            .expect("written")
            .iter()
            .any(|item| item.as_str() == Some("wiki/index.md"))
    );
    assert_eq!(read_text(&home_path), original_home);
}

#[test]
fn review_indices_splices_only_managed_region_of_index_home() {
    let temp = tempdir().expect("tempdir");
    let fixture_copy = temp.path().join("ready-for-query");
    copy_dir(&fixtures_root().join("ready-for-query"), &fixture_copy);
    let vault = fixture_copy.to_str().unwrap();
    let home_path = fixture_copy.join("wiki/index.md");
    fs::write(
        &home_path,
        concat!(
            "# Test Vault\n\n",
            "Custom prose before.\n\n",
            "<!-- onkb:indices:begin -->\n",
            "stale hand-written block\n",
            "<!-- onkb:indices:end -->\n\n",
            "Custom prose after.\n",
        ),
    )
    .expect("seed managed home");

    let scan = run_json(&["--json", "review", "indices", vault]);
    assert_eq!(
        scan["files"]["wiki/index.md"]["status"].as_str(),
        Some("drifted")
    );

    let write = run_json(&["--json", "review", "indices", vault, "--write"]);
    assert!(
        write["written_paths"]
            .as_array()
            .expect("written")
            .iter()
            .any(|item| item.as_str() == Some("wiki/index.md"))
    );
    let spliced = read_text(&home_path);
    assert!(spliced.contains("Custom prose before."));
    assert!(spliced.contains("Custom prose after."));
    assert!(spliced.contains("## Approved Live Indices"));
    assert!(spliced.contains("Approved live pages: 2"));
    assert!(!spliced.contains("stale hand-written block"));

    let rerun = run_json(&["--json", "review", "indices", vault]);
    assert_eq!(
        rerun["files"]["wiki/index.md"]["status"].as_str(),
        Some("in_sync")
    );
}

#[test]
fn review_indices_always_excludes_editorial_priorities() {
    let temp = tempdir().expect("tempdir");
    let fixture_copy = temp.path().join("ready-for-query");
    copy_dir(&fixtures_root().join("ready-for-query"), &fixture_copy);
    let vault = fixture_copy.to_str().unwrap();
    let editorial_path = fixture_copy.join(EDITORIAL_REL);
    fs::create_dir_all(editorial_path.parent().unwrap()).expect("indices dir");
    fs::write(
        &editorial_path,
        "# Editorial Priorities\n\n- Hand-curated focus.\n",
    )
    .expect("seed editorial page");

    let write = run_json(&["--json", "review", "indices", vault, "--write"]);
    assert_eq!(
        write["files"][EDITORIAL_REL]["status"].as_str(),
        Some("excluded")
    );
    assert!(
        !write["written_paths"]
            .as_array()
            .expect("written")
            .iter()
            .any(|item| item.as_str() == Some(EDITORIAL_REL))
    );
    assert_eq!(
        read_text(&editorial_path),
        "# Editorial Priorities\n\n- Hand-curated focus.\n"
    );
}

#[test]
fn review_indices_recent_orders_by_approval_and_counts_unlisted() {
    let temp = tempdir().expect("tempdir");
    let fixture_copy = temp.path().join("ready-for-query");
    copy_dir(&fixtures_root().join("ready-for-query"), &fixture_copy);
    let vault = fixture_copy.to_str().unwrap();
    fs::write(
        fixture_copy.join("wiki/live/concepts/fresh-reviewed.md"),
        concat!(
            "---\n",
            "title: \"Fresh Reviewed\"\n",
            "last_reviewed_at: \"2026-05-01T09:00:00Z\"\n",
            "---\n\n",
            "# Fresh Reviewed\n",
        ),
    )
    .expect("seed reviewed page");
    fs::create_dir_all(fixture_copy.join("wiki/live/topics")).expect("topics dir");
    fs::write(
        fixture_copy.join("wiki/live/topics/no-dates.md"),
        "---\ntitle: \"No Dates Topic\"\n---\n\n# No Dates Topic\n",
    )
    .expect("seed undated page");

    let scan = run_json(&["--json", "review", "indices", vault]);
    assert_eq!(
        scan["files"][RECENT_REL]["unlisted_count"].as_i64(),
        Some(1)
    );

    run_json(&["--json", "review", "indices", vault, "--write"]);
    let recent_text = read_text(&fixture_copy.join(RECENT_REL));
    let reviewed_pos = recent_text
        .find("- [[wiki/live/concepts/fresh-reviewed]] | Fresh Reviewed | reviewed: 2026-05-01T09:00:00Z")
        .expect("reviewed row");
    let concept_pos = recent_text
        .find("- [[wiki/live/concepts/review-gate]] | Review Gate | approved: 2026-04-06T08:05:00Z")
        .expect("concept row");
    let summary_pos = recent_text
        .find("approved: 2026-04-06T08:00:00Z")
        .expect("summary row");
    assert!(reviewed_pos < concept_pos);
    assert!(concept_pos < summary_pos);
    assert!(!recent_text.contains("no-dates"));

    let topics_text = read_text(&fixture_copy.join("wiki/live/indices/TOPICS.md"));
    assert!(topics_text.contains("- [[wiki/live/topics/no-dates]] | No Dates Topic"));
}

#[test]
fn review_automation_scheduled_health_includes_navigation_indices() {
    let payload = run_json(&[
        "--json",
        "review",
        "automation",
        fixtures_root().join("ready-for-query").to_str().unwrap(),
        "--mode",
        "scheduled-health",
    ]);
    assert_eq!(
        payload["navigation_indices"]["action"].as_str(),
        Some("review_indices")
    );
    assert_eq!(
        payload["navigation_indices"]["drift_count"].as_i64(),
        Some(5)
    );

    let temp = tempdir().expect("tempdir");
    let fixture_copy = temp.path().join("ready-for-query");
    copy_dir(&fixtures_root().join("ready-for-query"), &fixture_copy);
    run_json(&[
        "--json",
        "review",
        "automation",
        fixture_copy.to_str().unwrap(),
        "--mode",
        "scheduled-health",
        "--write",
    ]);
    assert!(fixture_copy.join(RECENT_REL).exists());
    assert!(fixture_copy.join("wiki/live/indices/INDEX.md").exists());
}
