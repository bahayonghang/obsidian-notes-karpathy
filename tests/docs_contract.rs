mod common;

use common::{entry_eval_root, entry_skill_root, read_text, repo_root};
use onkb::dev::doc_fragments::{
    build_installation_boundary_fragment, build_skill_inventory_fragment,
};
use serde_json::Value;

#[test]
fn bundle_docs_and_registry_stay_consistent() {
    let repo_root = repo_root();
    let entry_root = entry_skill_root();
    let readme = read_text(&repo_root.join("README.md"));
    let readme_cn = read_text(&repo_root.join("README_CN.md"));
    let claude = read_text(&repo_root.join("CLAUDE.md"));
    let install_doc = read_text(&repo_root.join("docs").join("guide").join("installation.md"));
    let install_doc_zh = read_text(
        &repo_root
            .join("docs")
            .join("zh")
            .join("guide")
            .join("installation.md"),
    );
    let docs_overview = read_text(&repo_root.join("docs").join("skills").join("overview.md"));
    let docs_overview_zh = read_text(
        &repo_root
            .join("docs")
            .join("zh")
            .join("skills")
            .join("overview.md"),
    );
    let workflow_overview = read_text(&repo_root.join("docs").join("workflow").join("overview.md"));
    let workflow_overview_zh = read_text(
        &repo_root
            .join("docs")
            .join("zh")
            .join("workflow")
            .join("overview.md"),
    );
    let registry_text = read_text(
        &entry_root
            .join("scripts")
            .join("skill-contract-registry.json"),
    );
    let skill_inventory = build_skill_inventory_fragment(&repo_root).expect("skill inventory");
    let install_boundary =
        build_installation_boundary_fragment(&repo_root).expect("installation boundary");

    for needle in [
        "chinese-llm-wiki-compat.md",
        "query-writeback-lifecycle.md",
        "memory-lifecycle.md",
        "graph-contract.md",
        "source-manifest-contract.md",
        "profile-contract.md",
        "automation-hooks.md",
        "compile-method.md",
        "archive-model.md",
        "outputs/web/",
        "\"path\":",
        "\"install_scope\":",
        "\"doc_targets\":",
        "\"eval_targets\":",
        "\"companion_refs\":",
    ] {
        assert!(registry_text.contains(needle), "{needle}");
    }

    for needle in [
        "Chinese-LLM-Wiki",
        "Companion skill matrix",
        "MEMORY.md",
        "outputs/episodes/",
        "outputs/web/",
        "wiki/live/procedures/",
    ] {
        assert!(readme.contains(needle), "{needle}");
    }
    for needle in [
        "Chinese-LLM-Wiki",
        "搭配技能矩阵",
        "outputs/episodes/",
        "outputs/web/",
    ] {
        assert!(readme_cn.contains(needle), "{needle}");
    }
    assert!(claude.contains("web-access"));
    assert!(claude.contains("publish"));
    assert!(claude.contains("wiki/live/procedures/"));
    assert!(claude.contains("evals/skills/obsidian-notes-karpathy/fixtures/"));
    assert!(install_doc.contains("evals/skills/obsidian-notes-karpathy/"));
    assert!(!install_doc.contains("obsidian-notes-karpathy/evals/"));
    assert!(install_doc_zh.contains("evals/skills/obsidian-notes-karpathy/"));
    assert!(!install_doc_zh.contains("obsidian-notes-karpathy/evals/"));
    assert!(docs_overview.contains("one package entry skill and six operational skills"));
    assert!(docs_overview_zh.contains("1 个入口技能和 6 个操作技能"));
    assert!(skill_inventory.contains("`kb-query`"));
    assert!(
        install_boundary.contains("Repo-only dev assets stay outside the installed runtime bundle")
    );
    assert!(workflow_overview.contains("<WorkflowLifecycleDiagram"));
    assert!(workflow_overview_zh.contains("<WorkflowLifecycleDiagram"));
    assert!(!workflow_overview.contains("```mermaid"));
    assert!(!workflow_overview_zh.contains("```mermaid"));
    assert!(
        repo_root
            .join("docs")
            .join(".vitepress")
            .join("theme")
            .join("components")
            .join("WorkflowLifecycleDiagram.vue")
            .exists()
    );
}

#[test]
fn trigger_and_runtime_eval_manifests_cover_core_routes() {
    let trigger_evals: Vec<Value> =
        serde_json::from_str(&read_text(&entry_eval_root().join("trigger-evals.json")))
            .expect("trigger evals");
    assert!(trigger_evals.len() >= 20);
    let serialized = serde_json::to_string(&trigger_evals).expect("serialize trigger evals");
    for token in [
        "来源页",
        "主题页",
        "实体页",
        "综合页",
        "output/analyses",
        "output/reports",
        "中文优先",
        "原文证据摘录",
        "先读 wiki/index.md",
    ] {
        assert!(serialized.contains(token), "{token}");
    }

    let runtime_evals: Value =
        serde_json::from_str(&read_text(&entry_eval_root().join("runtime-evals.json")))
            .expect("runtime evals");
    let writable_evals: Value = serde_json::from_str(&read_text(
        &entry_eval_root().join("runtime-evals-writable.json"),
    ))
    .expect("writable runtime evals");

    for skill_name in [
        "kb-init",
        "kb-ingest",
        "kb-compile",
        "kb-review",
        "kb-query",
        "kb-render",
    ] {
        let runtime_count = runtime_evals["evals"]
            .as_array()
            .expect("runtime evals")
            .iter()
            .filter(|item| item["skill"].as_str() == Some(skill_name))
            .count();
        let writable_count = writable_evals["evals"]
            .as_array()
            .expect("writable evals")
            .iter()
            .filter(|item| item["skill"].as_str() == Some(skill_name))
            .count();
        assert!(runtime_count >= 2, "{skill_name} runtime coverage");
        assert!(writable_count >= 1, "{skill_name} writable coverage");
    }
}

#[test]
fn references_include_creator_memory_and_archive_contract_fields() {
    let entry_root = entry_skill_root();
    let summary_template = read_text(&entry_root.join("references").join("summary-template.md"));
    let review_template = read_text(&entry_root.join("references").join("review-template.md"));
    let qa_template = read_text(&entry_root.join("references").join("qa-template.md"));
    let content_template = read_text(
        &entry_root
            .join("references")
            .join("content-output-template.md"),
    );
    let file_model = read_text(&entry_root.join("references").join("file-model.md"));
    let archive_model = read_text(&entry_root.join("references").join("archive-model.md"));
    let manifest_contract = read_text(
        &entry_root
            .join("references")
            .join("source-manifest-contract.md"),
    );
    let health_rubric = read_text(&entry_root.join("references").join("health-rubric.md"));

    for needle in [
        "evidence_coverage",
        "uncertainty_level",
        "promotion_target",
        "boundary_conditions",
        "assumption_flags",
        "transfer_targets",
    ] {
        assert!(summary_template.contains(needle), "{needle}");
    }
    assert!(review_template.contains("promotion_reason"));
    assert!(review_template.contains("fact_inference_separation"));
    assert!(qa_template.contains("writeback_candidates"));
    assert!(qa_template.contains("followup_route"));
    assert!(content_template.contains("writeback_candidates"));
    assert!(file_model.contains("MEMORY.md"));
    assert!(file_model.contains("outputs/episodes/"));
    assert!(file_model.contains("wiki/live/procedures/"));
    assert!(archive_model.contains("source retention archive"));
    assert!(archive_model.contains("artifact archive"));
    assert!(manifest_contract.contains("capture_method"));
    assert!(manifest_contract.contains("linked_assets"));
    assert!(manifest_contract.contains("source_profile"));
    assert!(health_rubric.contains("Creator Consistency"));
    assert!(health_rubric.contains("Reuse Signals"));
}
