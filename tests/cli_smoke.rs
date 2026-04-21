use std::fs;
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use serde_json::Value;
use tempfile::tempdir;
use walkdir::WalkDir;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn fixtures_root() -> PathBuf {
    repo_root()
        .join("skills")
        .join("obsidian-notes-karpathy")
        .join("evals")
        .join("fixtures")
}

fn run_json(args: &[&str]) -> Value {
    let output = Command::cargo_bin("onkb")
        .expect("binary")
        .args(args)
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    serde_json::from_slice(&output).expect("valid json")
}

fn copy_dir(source: &Path, target: &Path) {
    for entry in WalkDir::new(source) {
        let entry = entry.expect("walkdir");
        let rel = entry.path().strip_prefix(source).expect("relative");
        let dest = target.join(rel);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&dest).expect("create dir");
        } else {
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent).expect("create parent");
            }
            fs::copy(entry.path(), &dest).expect("copy file");
        }
    }
}

#[test]
fn doctor_reports_embedded_bundle() {
    let payload = run_json(&["--json", "doctor"]);
    assert_eq!(payload["embedded_assets_detected"], Value::Bool(true));
    assert_eq!(payload["skill_bundle_available"], Value::Bool(true));
}

#[test]
fn status_routes_needs_review_fixture() {
    let fixture = fixtures_root().join("needs-review");
    let payload = run_json(&["--json", "status", fixture.to_str().unwrap()]);
    assert_eq!(payload["state"], Value::String("needs-review".to_string()));
    assert_eq!(payload["route"], Value::String("kb-review".to_string()));
}

#[test]
fn ingest_scan_reports_manifest_drift() {
    let fixture = fixtures_root().join("needs-ingest");
    let payload = run_json(&["--json", "ingest", "scan", fixture.to_str().unwrap()]);
    assert_eq!(payload["needs_ingest"], Value::Bool(true));
    assert_eq!(payload["counts"]["new"], Value::Number(1.into()));
    assert_eq!(payload["counts"]["removed"], Value::Number(1.into()));
}

#[test]
fn review_queue_applies_mixed_gate() {
    let fixture = fixtures_root().join("review-conflict-mixed-gate");
    let payload = run_json(&["--json", "review", "queue", fixture.to_str().unwrap()]);
    assert_eq!(payload["counts"]["approve"], Value::Number(1.into()));
    assert_eq!(payload["counts"]["reject"], Value::Number(1.into()));
    assert_eq!(payload["counts"]["needs-human"], Value::Number(1.into()));
}

#[test]
fn query_scope_keeps_live_boundary() {
    let fixture = fixtures_root().join("ready-for-query");
    let payload = run_json(&["--json", "query", "scope", fixture.to_str().unwrap()]);
    let included = payload["included_paths"]
        .as_array()
        .expect("included_paths");
    let excluded = payload["excluded_paths"]
        .as_array()
        .expect("excluded_paths");
    assert!(included
        .iter()
        .any(|item| item.as_str() == Some("wiki/live/concepts/review-gate.md")));
    assert!(excluded
        .iter()
        .any(|item| item.as_str() == Some("raw/human/articles/2026-04-05-approved-summary.md")));
}

#[test]
fn init_scaffolds_governance_assets() {
    let temp = tempdir().expect("tempdir");
    let target = temp.path().join("needs-setup");
    copy_dir(&fixtures_root().join("needs-setup"), &target);
    let payload = run_json(&[
        "--json",
        "init",
        target.to_str().unwrap(),
        "--include-governance",
    ]);
    assert!(target.join("AGENTS.md").exists());
    assert!(target
        .join("wiki")
        .join("live")
        .join("indices")
        .join("QUESTIONS.md")
        .exists());
    assert!(payload["created_dirs"]
        .as_array()
        .expect("created_dirs")
        .iter()
        .any(|item| item.as_str() == Some("raw/human/articles")));
}

#[test]
fn migrate_preserves_and_rehomes_legacy_content() {
    let temp = tempdir().expect("tempdir");
    let target = temp.path().join("needs-migration");
    copy_dir(&fixtures_root().join("needs-migration"), &target);
    let payload = run_json(&["--json", "migrate", target.to_str().unwrap()]);
    assert!(target
        .join("raw")
        .join("articles")
        .join("2026-03-20-old-pattern.md")
        .exists());
    assert!(target
        .join("raw")
        .join("human")
        .join("articles")
        .join("2026-03-20-old-pattern.md")
        .exists());
    assert!(target
        .join("wiki")
        .join("live")
        .join("summaries")
        .join("2026-03-20-old-pattern.md")
        .exists());
    assert!(target
        .join(payload["migration_report"].as_str().unwrap())
        .exists());
}

#[test]
fn render_web_writes_static_package() {
    let temp = tempdir().expect("tempdir");
    let target = temp.path().join("ready-for-query");
    copy_dir(&fixtures_root().join("ready-for-query"), &target);
    let payload = run_json(&[
        "--json",
        "render",
        target.to_str().unwrap(),
        "--mode",
        "web",
        "--source",
        "wiki/live/concepts/review-gate.md",
        "--write",
    ]);
    let package_root = target.join(payload["package_root"].as_str().unwrap());
    assert!(package_root.join("manifest.json").exists());
    assert!(package_root.join("app.js").exists());
}
