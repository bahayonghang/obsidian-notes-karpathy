mod common;

use std::fs;

use common::{copy_dir, fixtures_root, read_text, run_json, tree_snapshot};
use tempfile::tempdir;

const ARCHIVE_REL: &str = "outputs/qa/2026-04-08-writeback-gap.md";
const DRAFT_REL: &str = "wiki/drafts/summaries/writeback/2026-04-08-writeback-gap.md";
const PACKAGE_REL: &str = "wiki/drafts/indices/packages/writeback/2026-04-08-writeback-gap.md";

#[test]
fn compile_writeback_dry_run_reports_eligible_without_writing() {
    let fixture = fixtures_root().join("writeback-backlog");
    let scan = run_json(&["--json", "compile", "writeback", fixture.to_str().unwrap()]);

    assert_eq!(scan["action"].as_str(), Some("compile_writeback"));
    assert_eq!(scan["write"].as_bool(), Some(false));
    assert_eq!(scan["counts"]["eligible"].as_i64(), Some(1));
    assert_eq!(scan["counts"]["skipped"].as_i64(), Some(0));
    let item = &scan["items"][0];
    assert_eq!(item["path"].as_str(), Some(ARCHIVE_REL));
    assert_eq!(item["status"].as_str(), Some("eligible"));
    assert_eq!(item["draft_path"].as_str(), Some(DRAFT_REL));
    assert_eq!(item["package_path"].as_str(), Some(PACKAGE_REL));
    assert_eq!(
        item["grounding"][0].as_str(),
        Some("[[wiki/live/concepts/review-gate]]")
    );
    assert_eq!(scan["written_paths"].as_array().map(Vec::len), Some(0));
    assert!(!fixture.join(DRAFT_REL).exists());
}

#[test]
fn compile_writeback_write_scaffolds_draft_advances_status_and_reruns_clean() {
    let temp = tempdir().expect("tempdir");
    let fixture_copy = temp.path().join("writeback-backlog");
    copy_dir(&fixtures_root().join("writeback-backlog"), &fixture_copy);
    let vault = fixture_copy.to_str().unwrap();
    let original_archive = read_text(&fixture_copy.join(ARCHIVE_REL));

    let write = run_json(&["--json", "compile", "writeback", vault, "--write"]);
    assert_eq!(write["write"].as_bool(), Some(true));
    assert_eq!(
        write["written_paths"]
            .as_array()
            .expect("written")
            .iter()
            .filter_map(|item| item.as_str())
            .collect::<Vec<_>>(),
        vec![PACKAGE_REL, DRAFT_REL]
    );
    assert_eq!(
        write["advanced_sources"][0]["path"].as_str(),
        Some(ARCHIVE_REL)
    );
    assert_eq!(
        write["advanced_sources"][0]["from"].as_str(),
        Some("pending")
    );
    assert_eq!(write["advanced_sources"][0]["to"].as_str(), Some("drafted"));

    let draft_text = read_text(&fixture_copy.join(DRAFT_REL));
    for required in [
        "draft_id: \"writeback--2026-04-08-writeback-gap\"",
        "compiled_from:\n  - \"[[wiki/live/concepts/review-gate]]\"",
        "capture_sources:\n  - \"[[wiki/live/concepts/review-gate]]\"",
        "writeback_source: \"[[outputs/qa/2026-04-08-writeback-gap]]\"",
        "review_state: \"pending\"",
        "review_score: \"0.70\"",
        "blocking_flags: []",
        "evidence_coverage: \"0.50\"",
        "uncertainty_level: \"medium\"",
        "promotion_target: \"semantic\"",
        "review_package_meta: \"[[wiki/drafts/indices/packages/writeback/2026-04-08-writeback-gap]]\"",
        "- [[wiki/live/concepts/writeback-loop]]",
    ] {
        assert!(
            draft_text.contains(required),
            "draft scaffold missing `{required}`"
        );
    }
    assert!(!draft_text.contains("compiled_from:\n  - \"[[outputs/"));

    let package_text = read_text(&fixture_copy.join(PACKAGE_REL));
    assert!(package_text.contains("source_id: \"writeback--2026-04-08-writeback-gap\""));
    assert!(package_text.contains(
        "summary_path: \"[[wiki/drafts/summaries/writeback/2026-04-08-writeback-gap]]\""
    ));

    // 归档产物只推进状态行并插入草稿指针，其余逐字节保留。
    let advanced_archive = read_text(&fixture_copy.join(ARCHIVE_REL));
    let expected_archive = original_archive.replace(
        "writeback_status: pending\n",
        concat!(
            "writeback_status: \"drafted\"\n",
            "writeback_draft: \"[[wiki/drafts/summaries/writeback/2026-04-08-writeback-gap]]\"\n",
        ),
    );
    assert_eq!(advanced_archive, expected_archive);

    let audit_text = read_text(&fixture_copy.join("outputs/audit/operations.jsonl"));
    assert!(audit_text.contains("\"action\":\"compile_writeback\""));
    assert!(audit_text.contains("\"eligible_count\":1"));

    let queue = run_json(&["--json", "review", "queue", vault]);
    assert!(
        queue["items"]
            .as_array()
            .expect("queue items")
            .iter()
            .any(|item| item["path"].as_str() == Some(DRAFT_REL)),
        "scaffold draft should enter the review queue"
    );

    let before = tree_snapshot(&fixture_copy);
    let rerun = run_json(&["--json", "compile", "writeback", vault, "--write"]);
    assert_eq!(rerun["counts"]["eligible"].as_i64(), Some(0));
    assert_eq!(rerun["counts"]["skipped"].as_i64(), Some(1));
    assert_eq!(
        rerun["items"][0]["reason"].as_str(),
        Some("writeback_status_drafted")
    );
    assert_eq!(rerun["written_paths"].as_array().map(Vec::len), Some(0));
    let after = tree_snapshot(&fixture_copy);
    assert_eq!(before, after, "rerun --write must not change any file");
}

#[test]
fn compile_writeback_skips_held_routes_and_ungrounded_candidates() {
    let temp = tempdir().expect("tempdir");
    let fixture_copy = temp.path().join("writeback-backlog");
    copy_dir(&fixtures_root().join("writeback-backlog"), &fixture_copy);
    let vault = fixture_copy.to_str().unwrap();
    let qa_dir = fixture_copy.join("outputs/qa");

    let variants: [(&str, &str, &str); 5] = [
        (
            "2026-04-09-triaged.md",
            "writeback_status: triaged\nfollowup_route: draft\n",
            "writeback_status_triaged",
        ),
        (
            "2026-04-09-rejected.md",
            "writeback_status: rejected\nfollowup_route: draft\n",
            "writeback_status_rejected",
        ),
        (
            "2026-04-09-review-route.md",
            "writeback_status: pending\nfollowup_route: review\n",
            "followup_route_review",
        ),
        (
            "2026-04-09-route-missing.md",
            "writeback_status: pending\n",
            "followup_route_missing",
        ),
        (
            "2026-04-09-no-grounding.md",
            "writeback_status: pending\nfollowup_route: draft\n",
            "missing_live_grounding",
        ),
    ];
    for (name, status_lines, _) in &variants {
        let grounding = if *name == "2026-04-09-no-grounding.md" {
            ""
        } else {
            "source_live_pages:\n  - \"[[wiki/live/concepts/review-gate]]\"\n"
        };
        fs::write(
            qa_dir.join(name),
            format!(
                "---\nquestion: \"variant\"\nwriteback_candidates:\n  - \"[[wiki/live/concepts/review-gate]]\"\n{status_lines}{grounding}---\n\n# Variant\n"
            ),
        )
        .expect("seed variant");
    }
    fs::write(
        qa_dir.join("2026-04-09-no-candidates.md"),
        concat!(
            "---\n",
            "question: \"variant\"\n",
            "writeback_status: pending\n",
            "followup_route: draft\n",
            "source_live_pages:\n",
            "  - \"[[wiki/live/concepts/review-gate]]\"\n",
            "---\n\n# Variant\n",
        ),
    )
    .expect("seed no-candidates variant");
    // sources 回退：无 source_live_pages 时取 sources 中的 live/raw 引用。
    fs::create_dir_all(fixture_copy.join("outputs/content/articles")).expect("content dir");
    fs::write(
        fixture_copy.join("outputs/content/articles/2026-04-09-sources-fallback.md"),
        concat!(
            "---\n",
            "title: \"Fallback Piece\"\n",
            "writeback_candidates:\n",
            "  - \"strengthen [[wiki/live/concepts/review-gate]] links\"\n",
            "writeback_status: pending\n",
            "followup_route: draft\n",
            "sources:\n",
            "  - \"[[wiki/live/concepts/review-gate]]\"\n",
            "  - \"[[outputs/qa/2026-04-08-writeback-gap]]\"\n",
            "---\n\n# Fallback Piece\n",
        ),
    )
    .expect("seed sources fallback variant");

    let write = run_json(&["--json", "compile", "writeback", vault, "--write"]);
    assert_eq!(write["counts"]["eligible"].as_i64(), Some(2));
    assert_eq!(write["counts"]["skipped"].as_i64(), Some(6));
    let items = write["items"].as_array().expect("items");
    let reason_for = |path: &str| -> Option<&str> {
        items
            .iter()
            .find(|item| item["path"].as_str() == Some(path))
            .and_then(|item| item["reason"].as_str())
    };
    for (name, _, expected_reason) in &variants {
        assert_eq!(
            reason_for(&format!("outputs/qa/{name}")),
            Some(*expected_reason),
            "variant {name}"
        );
    }
    assert_eq!(
        reason_for("outputs/qa/2026-04-09-no-candidates.md"),
        Some("no_writeback_candidates")
    );

    let fallback = items
        .iter()
        .find(|item| {
            item["path"].as_str() == Some("outputs/content/articles/2026-04-09-sources-fallback.md")
        })
        .expect("fallback item");
    assert_eq!(fallback["status"].as_str(), Some("eligible"));
    assert_eq!(
        fallback["grounding"]
            .as_array()
            .expect("grounding")
            .iter()
            .filter_map(|value| value.as_str())
            .collect::<Vec<_>>(),
        vec!["[[wiki/live/concepts/review-gate]]"],
        "sources fallback must keep live refs and drop archive refs"
    );

    // 变体文件不被触碰：跳过项状态行保持原样。
    let triaged_text = read_text(&qa_dir.join("2026-04-09-triaged.md"));
    assert!(triaged_text.contains("writeback_status: triaged"));
    assert!(
        !fixture_copy
            .join("wiki/drafts/summaries/writeback/2026-04-09-triaged.md")
            .exists()
    );
}

#[test]
fn compile_writeback_drafted_status_leaves_backlog_but_triaged_stays() {
    let temp = tempdir().expect("tempdir");
    let fixture_copy = temp.path().join("writeback-backlog");
    copy_dir(&fixtures_root().join("writeback-backlog"), &fixture_copy);
    let vault = fixture_copy.to_str().unwrap();
    fs::write(
        fixture_copy.join("outputs/qa/2026-04-09-still-triaged.md"),
        concat!(
            "---\n",
            "question: \"triaged variant\"\n",
            "writeback_candidates:\n",
            "  - \"[[wiki/live/concepts/review-gate]]\"\n",
            "writeback_status: triaged\n",
            "followup_route: draft\n",
            "---\n\n# Triaged Variant\n",
        ),
    )
    .expect("seed triaged variant");

    run_json(&["--json", "compile", "writeback", vault, "--write"]);
    let lint = run_json(&["--json", "review", "lint", vault]);
    let backlog: Vec<&str> = lint["issues"]
        .as_array()
        .expect("issues")
        .iter()
        .filter(|issue| issue["kind"].as_str() == Some("writeback_backlog"))
        .filter_map(|issue| issue["path"].as_str())
        .collect();
    assert_eq!(
        backlog,
        vec!["outputs/qa/2026-04-09-still-triaged.md"],
        "drafted artifact leaves the backlog; triaged stays visible"
    );
}
