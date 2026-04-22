mod common;

use onkb::common::parse_simple_yaml;
use onkb::dev::{
    current_repo_root, load_registry,
    runtime_eval::{
        build_prompt, classify_failure, fallback_runner_for, try_reuse_baseline_artifacts,
        validate_manifest,
    },
    skill_audit::load_skill_catalog,
    trigger_eval::{build_trigger_prompt, parse_selected_skill, summarize_trigger_results},
};

use common::{repo_root, run_json, run_stdout};
use serde_json::json;
use tempfile::tempdir;

#[test]
fn runtime_eval_supports_dry_run_without_execution() {
    let repo_root = repo_root();
    let payload = run_json(&[
        "--json",
        "dev",
        "eval-runtime",
        "--dry-run",
        "--runner",
        "codex",
        "--workspace",
        repo_root.join(".runtime-eval-test").to_str().unwrap(),
    ]);
    assert!(matches!(
        payload["status"].as_str(),
        Some("planned") | Some("skipped")
    ));
    assert!(payload["eval_count"].as_i64().unwrap_or_default() >= 10);
}

#[test]
fn runtime_eval_supports_skill_and_eval_id_filters() {
    let repo_root = repo_root();
    let payload = run_json(&[
        "--json",
        "dev",
        "eval-runtime",
        "--dry-run",
        "--runner",
        "claude",
        "--workspace",
        repo_root
            .join(".runtime-eval-filter-test")
            .to_str()
            .unwrap(),
        "--skill",
        "kb-query",
        "--eval-id",
        "kb-query-web-export",
    ]);
    assert!(matches!(
        payload["status"].as_str(),
        Some("planned") | Some("skipped")
    ));
    assert_eq!(payload["eval_count"].as_i64(), Some(1));
    assert_eq!(
        payload["skills"]
            .as_array()
            .and_then(|items| items.first())
            .and_then(|item| item.as_str()),
        Some("kb-query")
    );
    assert_eq!(
        payload["eval_ids"]
            .as_array()
            .and_then(|items| items.first())
            .and_then(|item| item.as_str()),
        Some("kb-query-web-export")
    );
}

#[test]
fn runtime_eval_can_reuse_previous_read_only_baseline() {
    let repo_root = repo_root();
    let temp = tempdir().expect("tempdir");
    let previous_workspace = temp.path().join("previous");
    let baseline_dir = previous_workspace
        .join("kb-query")
        .join("kb-query-web-export")
        .join("baseline");
    std::fs::create_dir_all(&baseline_dir).expect("create baseline dir");
    std::fs::write(
        previous_workspace
            .join("kb-query")
            .join("kb-query-web-export")
            .join("metadata.json"),
        serde_json::to_string_pretty(&json!({
            "id": "kb-query-web-export",
            "skill": "kb-query",
            "files": ["evals/fixtures/ready-for-query/wiki/live/concepts/review-gate.md"],
            "prompt": "Explain web export grounding.",
            "mode": "read-only",
            "source_vault_root": "skills/obsidian-notes-karpathy/evals/fixtures/ready-for-query",
            "checks": [],
        }))
        .expect("metadata json"),
    )
    .expect("write metadata");
    std::fs::write(baseline_dir.join("last_message.txt"), "cached baseline").expect("message");
    std::fs::write(
        baseline_dir.join("result.json"),
        serde_json::to_string_pretty(&json!({
            "runner": "claude",
            "returncode": 0,
            "duration_ms": 42,
            "output_path": "last_message.txt",
            "output_captured": true,
            "failure_kind": "ok",
        }))
        .expect("result json"),
    )
    .expect("write result");

    let target_dir = temp.path().join("current").join("baseline");
    let reused = try_reuse_baseline_artifacts(
        &repo_root,
        Some(&previous_workspace),
        "kb-query",
        "kb-query-web-export",
        "read-only",
        &json!({
            "id": "kb-query-web-export",
            "skill": "kb-query",
            "files": ["evals/fixtures/ready-for-query/wiki/live/concepts/review-gate.md"],
            "prompt": "Explain web export grounding.",
            "mode": "read-only",
            "source_vault_root": "skills/obsidian-notes-karpathy/evals/fixtures/ready-for-query",
            "checks": [],
        }),
        &target_dir,
    )
    .expect("reuse result")
    .expect("reused baseline");

    assert_eq!(reused["reused"].as_bool(), Some(true));
    assert_eq!(
        std::fs::read_to_string(target_dir.join("last_message.txt")).expect("copied message"),
        "cached baseline"
    );
}

#[test]
fn trigger_eval_supports_dry_run_without_execution() {
    let payload = run_json(&["--json", "dev", "eval-trigger", "--dry-run", "--limit", "2"]);
    assert!(matches!(
        payload["status"].as_str(),
        Some("planned") | Some("skipped")
    ));
    assert_eq!(payload["eval_count"].as_i64(), Some(2));
}

#[test]
fn trigger_eval_supports_skill_filtering() {
    let payload = run_json(&[
        "--json",
        "dev",
        "eval-trigger",
        "--dry-run",
        "--skill",
        "kb-render",
    ]);
    assert!(matches!(
        payload["status"].as_str(),
        Some("planned") | Some("skipped")
    ));
    assert!(payload["eval_count"].as_i64().unwrap_or_default() >= 1);
    assert_eq!(
        payload["selected_skills"]
            .as_array()
            .and_then(|items| items.first())
            .and_then(|item| item.as_str()),
        Some("kb-render")
    );
}

#[test]
fn runtime_eval_classifies_infra_failures() {
    assert_eq!(
        classify_failure(1, "", "ERROR: stream disconnected before completion"),
        "infra_failure"
    );
    assert_eq!(
        classify_failure(124, "", "Runtime eval attempt timed out after 60 seconds."),
        "infra_failure"
    );
    assert_eq!(
        classify_failure(1, "", "profile.ps1: Cannot dot-source this command"),
        "infra_failure"
    );
    assert_eq!(
        classify_failure(1, "", "ordinary tool failure"),
        "runner_failure"
    );
    assert_eq!(classify_failure(0, "ok", ""), "ok");
}

#[test]
fn runtime_eval_prompt_includes_windows_safety_guidance() {
    let prompt = build_prompt(
        true,
        "kb-init",
        "Explain the next step.",
        &["evals/fixtures/needs-setup/README.md".to_string()],
        "evals/fixtures/needs-setup",
        "read-only",
        &repo_root(),
    )
    .expect("prompt");
    assert!(prompt.contains("On Windows, avoid shell globs"));
    assert!(prompt.contains("Read and follow the skill"));
    assert!(prompt.contains("only target vault root"));
    assert!(prompt.contains("Do not quote or restate the task instructions verbatim"));
    assert!(prompt.contains("Do not echo full absolute repository paths"));
    assert!(!prompt.contains("//?/"));
}

#[test]
fn runtime_eval_prompt_respects_writable_copy_mode() {
    let prompt = build_prompt(
        false,
        "kb-init",
        "Repair the copied vault.",
        &[".runtime-evals/kb-init/README.md".to_string()],
        ".runtime-evals/kb-init/target",
        "writable-copy",
        &repo_root(),
    )
    .expect("prompt");
    assert!(prompt.contains("modify files only under the declared vault root"));
    assert!(!prompt.contains("Work in read-only mode"));
}

#[test]
fn runtime_eval_prefers_claude_fallback_for_codex() {
    let fallback = fallback_runner_for("codex");
    assert!(fallback.is_none() || fallback.as_deref() == Some("claude"));
    assert_eq!(fallback_runner_for("claude"), None);
}

#[test]
fn render_reference_block_uses_registry_driven_shared_refs() {
    let stdout = run_stdout(&["--json", "dev", "render-reference-block", "kb-init"]);
    assert!(stdout.contains("../obsidian-notes-karpathy/scripts/skill-contract-registry.json"));
    assert!(stdout.contains("../obsidian-notes-karpathy/references/source-manifest-contract.md"));
}

#[test]
fn skill_audit_requires_cli_install_fallback_language() {
    let payload = run_json(&["--json", "dev", "audit-skills"]);
    let skills = payload["skills"].as_array().expect("skills");
    assert!(!skills.is_empty());
    assert!(
        skills.iter().all(|item| {
            item["checks"]["mentions_cli_install_fallback"].as_bool() == Some(true)
        })
    );
}

#[test]
fn runtime_eval_manifest_validation_rejects_mixed_fixture_roots() {
    let registry = load_registry(&current_repo_root().expect("repo root")).expect("registry");
    let manifest = json!({
        "evals": [{
            "id": "mixed-roots",
            "skill": "kb-init",
            "prompt": "Inspect the fixture vault.",
            "vault_root": "evals/fixtures/needs-setup",
            "files": [
                "evals/fixtures/needs-setup/README.md",
                "evals/fixtures/needs-review/AGENTS.md"
            ]
        }]
    });
    let errors = validate_manifest(&manifest, &registry);
    assert!(!errors.is_empty());
    assert!(
        errors
            .iter()
            .any(|error| error.contains("files must belong to exactly one fixture root"))
    );
}

#[test]
fn trigger_eval_loads_catalog_and_parses_json_output() {
    let catalog = load_skill_catalog(&repo_root()).expect("catalog");
    assert!(catalog.contains_key("kb-init"));
    let prompt = build_trigger_prompt("先读 wiki/index.md，再判断来源页还是综合页。", &catalog);
    assert!(prompt.contains("The user request may be written in English or Chinese."));
    assert!(prompt.contains("Treat Chinese routing phrases such as 来源页"));
    assert_eq!(
        parse_selected_skill(r#"{"selected_skill":"none","reason":"outside scope"}"#),
        None
    );
    assert_eq!(
        parse_selected_skill(r#"{"selected_skill":"kb-query","reason":"live-grounded request"}"#),
        Some("kb-query".to_string())
    );
}

#[test]
fn trigger_eval_summary_reports_precision_and_recall() {
    let summary = summarize_trigger_results(
        &[
            json!({"expected_skill": "kb-init", "actual_skill": "kb-init", "matched": true}),
            json!({"expected_skill": "kb-init", "actual_skill": "kb-query", "matched": false}),
            json!({"expected_skill": null, "actual_skill": "kb-query", "matched": false}),
        ],
        &["kb-init".to_string(), "kb-query".to_string()],
    );
    assert_eq!(summary["matched"].as_i64(), Some(1));
    assert_eq!(
        summary["per_skill"]["kb-init"]["true_positive"].as_i64(),
        Some(1)
    );
    assert_eq!(
        summary["per_skill"]["kb-query"]["false_positive"].as_i64(),
        Some(2)
    );
}

#[test]
fn parse_simple_yaml_handles_empty_values_and_lists() {
    let parsed = parse_simple_yaml(
        "title: Test Note\naliases:\n  - alpha\n  - beta\nempty_list: []\nowner:\n",
    );
    assert_eq!(parsed["title"], "Test Note");
    assert_eq!(parsed["aliases"], json!(["alpha", "beta"]));
    assert_eq!(parsed["empty_list"], json!([]));
    assert_eq!(parsed["owner"], json!([]));
}

#[test]
fn parse_simple_yaml_preserves_colons_and_special_characters() {
    let parsed = parse_simple_yaml(
        "note: keep:the:rest\nurl: \"https://example.com/a:b\"\nlabel: 'RAG vs. wiki: tradeoffs'\nitems: [one, two, three]\n",
    );
    assert_eq!(parsed["note"], "keep:the:rest");
    assert_eq!(parsed["url"], "https://example.com/a:b");
    assert_eq!(parsed["label"], "RAG vs. wiki: tradeoffs");
    assert_eq!(parsed["items"], json!(["one", "two", "three"]));
}
