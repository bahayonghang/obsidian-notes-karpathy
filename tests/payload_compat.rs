//! 阶段 2 的向后兼容护栏。
//!
//! 为每个主 CLI 命令 × 代表性 fixture 打一个 JSON snapshot。
//! 首次运行用 `INSTA_UPDATE=auto cargo test --test payload_compat` 生成并接受 baseline。
//! 之后任何改动如果让字段名/层级/值漂移，测试就会失败。
//!
//! 路径/时间戳等不稳定字段用 insta redaction 屏蔽。

use std::path::PathBuf;

use tempfile::tempdir;

mod common;

use common::{copy_dir, fixtures_root, run_json};

const VAULT_REDACT: &str = "[VAULT]";
const TS_REDACT: &str = "[TIMESTAMP]";
const PATH_REDACT: &str = "[PATH]";

fn fixture(name: &str) -> PathBuf {
    fixtures_root().join(name)
}

fn fixture_str(name: &str) -> String {
    fixture(name).to_string_lossy().into_owned()
}

#[test]
fn doctor_payload_snapshot() {
    let payload = run_json(&["--json", "doctor"]);
    insta::assert_json_snapshot!("doctor", payload, {
        ".binary_path" => PATH_REDACT,
        ".version" => "[VERSION]",
    });
}

#[test]
fn status_ready_for_query_snapshot() {
    let payload = run_json(&["--json", "status", &fixture_str("ready-for-query")]);
    insta::assert_json_snapshot!("status_ready_for_query", payload, {
        ".vault_root" => VAULT_REDACT,
        ".summary" => "[SUMMARY]",
    });
}

#[test]
fn ingest_scan_needs_ingest_snapshot() {
    let payload = run_json(&["--json", "ingest", "scan", &fixture_str("needs-ingest")]);
    insta::assert_json_snapshot!("ingest_scan_needs_ingest", payload, {
        ".vault_root" => VAULT_REDACT,
        ".items[].raw_mtime" => TS_REDACT,
        ".items[].last_verified_at" => TS_REDACT,
        ".items[].first_seen_at" => TS_REDACT,
        ".items[].last_seen_at" => TS_REDACT,
    });
}

#[test]
fn compile_scan_needs_compilation_snapshot() {
    let payload = run_json(&[
        "--json",
        "compile",
        "scan",
        &fixture_str("needs-compilation"),
    ]);
    insta::assert_json_snapshot!("compile_scan_needs_compilation", payload, {
        ".vault_root" => VAULT_REDACT,
        ".items[].raw_mtime" => TS_REDACT,
        ".items[].last_verified_at" => TS_REDACT,
    });
}

#[test]
fn review_queue_mixed_gate_snapshot() {
    let payload = run_json(&[
        "--json",
        "review",
        "queue",
        &fixture_str("review-conflict-mixed-gate"),
    ]);
    insta::assert_json_snapshot!("review_queue_mixed_gate", payload, {
        ".vault_root" => VAULT_REDACT,
    });
}

#[test]
fn review_lint_ready_for_query_snapshot() {
    let payload = run_json(&[
        "--json",
        "review",
        "lint",
        &fixture_str("ready-for-query"),
    ]);
    insta::assert_json_snapshot!("review_lint_ready_for_query", payload, {
        ".vault_root" => VAULT_REDACT,
    });
}

#[test]
fn review_governance_ready_for_query_snapshot() {
    let payload = run_json(&[
        "--json",
        "review",
        "governance",
        &fixture_str("ready-for-query"),
    ]);
    insta::assert_json_snapshot!("review_governance_ready_for_query", payload, {
        ".vault_root" => VAULT_REDACT,
    });
}

#[test]
fn review_graph_ready_for_query_snapshot() {
    let payload = run_json(&[
        "--json",
        "review",
        "graph",
        &fixture_str("ready-for-query"),
    ]);
    insta::assert_json_snapshot!("review_graph_ready_for_query", payload, {
        ".vault_root" => VAULT_REDACT,
        ".generated_at" => TS_REDACT,
    });
}

#[test]
fn query_scope_ready_for_query_snapshot() {
    let payload = run_json(&[
        "--json",
        "query",
        "scope",
        &fixture_str("ready-for-query"),
    ]);
    insta::assert_json_snapshot!("query_scope_ready_for_query", payload, {
        ".vault_root" => VAULT_REDACT,
    });
}

#[test]
fn query_rank_ready_for_query_snapshot() {
    let payload = run_json(&[
        "--json",
        "query",
        "rank",
        &fixture_str("ready-for-query"),
        "review gate",
    ]);
    insta::assert_json_snapshot!("query_rank_ready_for_query", payload, {
        ".vault_root" => VAULT_REDACT,
    });
}

#[test]
fn init_needs_setup_snapshot() {
    let temp = tempdir().expect("tempdir");
    let target = temp.path().join("needs-setup");
    copy_dir(&fixtures_root().join("needs-setup"), &target);
    let payload = run_json(&[
        "--json",
        "init",
        target.to_str().unwrap(),
        "--include-governance",
    ]);
    insta::assert_json_snapshot!("init_needs_setup", payload, {
        ".vault_root" => VAULT_REDACT,
    });
}

#[test]
fn migrate_needs_migration_snapshot() {
    let temp = tempdir().expect("tempdir");
    let target = temp.path().join("needs-migration");
    copy_dir(&fixtures_root().join("needs-migration"), &target);
    let payload = run_json(&["--json", "migrate", target.to_str().unwrap()]);
    insta::assert_json_snapshot!("migrate_needs_migration", payload, {
        ".vault_root" => VAULT_REDACT,
        ".scaffold.vault_root" => VAULT_REDACT,
        ".written_manifest" => PATH_REDACT,
        ".migrated_raw[].source" => PATH_REDACT,
        ".migrated_raw[].target" => PATH_REDACT,
        ".migrated_live[].source" => PATH_REDACT,
        ".migrated_live[].target" => PATH_REDACT,
    });
}
