# pyright: reportMissingImports=false
import json
import re
import unittest
from pathlib import Path

from _bundle_test_support import (
    ENTRY_SKILL_ROOT,
    REGISTRY_PATH,
    REPO_ROOT,
    RUNTIME_EVALS_PATH,
    SKILL_PATHS,
    WRITABLE_RUNTIME_EVALS_PATH,
)


class DocsAndEvalsTests(unittest.TestCase):
    def test_reference_templates_include_memory_writeback_and_provenance_fields(self) -> None:
        summary_template = (ENTRY_SKILL_ROOT / "references" / "summary-template.md").read_text(encoding="utf-8")
        review_template = (ENTRY_SKILL_ROOT / "references" / "review-template.md").read_text(encoding="utf-8")
        qa_template = (ENTRY_SKILL_ROOT / "references" / "qa-template.md").read_text(encoding="utf-8")
        content_template = (ENTRY_SKILL_ROOT / "references" / "content-output-template.md").read_text(encoding="utf-8")
        file_model = (ENTRY_SKILL_ROOT / "references" / "file-model.md").read_text(encoding="utf-8")
        procedure_template = (ENTRY_SKILL_ROOT / "references" / "procedure-template.md").read_text(encoding="utf-8")
        episode_template = (ENTRY_SKILL_ROOT / "references" / "episode-template.md").read_text(encoding="utf-8")
        compile_method = (ENTRY_SKILL_ROOT / "references" / "compile-method.md").read_text(encoding="utf-8")
        archive_model = (ENTRY_SKILL_ROOT / "references" / "archive-model.md").read_text(encoding="utf-8")
        manifest_contract = (ENTRY_SKILL_ROOT / "references" / "source-manifest-contract.md").read_text(encoding="utf-8")
        health_rubric = (ENTRY_SKILL_ROOT / "references" / "health-rubric.md").read_text(encoding="utf-8")

        self.assertIn("evidence_coverage", summary_template)
        self.assertIn("uncertainty_level", summary_template)
        self.assertIn("promotion_target", summary_template)
        self.assertIn("candidate_relationships", summary_template)
        self.assertIn("topic_candidates", summary_template)
        self.assertIn("review_package_meta", summary_template)
        self.assertIn("boundary_conditions", summary_template)
        self.assertIn("assumption_flags", summary_template)
        self.assertIn("transfer_targets", summary_template)
        self.assertIn("promotion_reason", review_template)
        self.assertIn("fact_inference_separation", review_template)
        self.assertIn("supersession_decision", review_template)
        self.assertIn("writeback_candidates", qa_template)
        self.assertIn("writeback_status", qa_template)
        self.assertIn("followup_route", qa_template)
        self.assertIn("source_live_pages", qa_template)
        self.assertIn("confidence_posture", qa_template)
        self.assertIn("crystallized_from_episode", qa_template)
        self.assertIn("writeback_candidates", content_template)
        self.assertIn("followup_route", content_template)
        self.assertIn("source_live_pages", content_template)
        self.assertIn("crystallized_from_episode", content_template)
        self.assertIn("MEMORY.md", file_model)
        self.assertIn("web-access", file_model)
        self.assertIn("浓缩 -> 质疑 -> 对标", file_model)
        self.assertIn("followup_route", file_model)
        self.assertIn("outputs/episodes/", file_model)
        self.assertIn("wiki/live/procedures/", file_model)
        self.assertIn("outputs/audit/operations.jsonl", file_model)
        self.assertIn("Creator workflow mapping", file_model)
        self.assertIn("source retention archive", archive_model)
        self.assertIn("artifact archive", archive_model)
        self.assertIn("raw/09-archive/", archive_model)
        self.assertIn("Prior coverage reused", content_template)
        self.assertIn("procedure_id", procedure_template)
        self.assertIn("memory_tier: episodic", episode_template)
        self.assertIn("capture_method", manifest_contract)
        self.assertIn("linked_assets", manifest_contract)
        self.assertIn("source_profile", manifest_contract)
        self.assertIn("Creator Consistency", health_rubric)
        self.assertIn("Reuse Signals", health_rubric)
        self.assertIn("浓缩 -> 质疑 -> 对标", compile_method)

    def test_bundle_docs_and_trigger_evals_stay_consistent(self) -> None:
        readme = (REPO_ROOT / "README.md").read_text(encoding="utf-8")
        readme_cn = (REPO_ROOT / "README_CN.md").read_text(encoding="utf-8")
        docs_readme = (REPO_ROOT / "docs" / "README.md").read_text(encoding="utf-8")
        docs_config = (REPO_ROOT / "docs" / ".vitepress" / "config.ts").read_text(encoding="utf-8")
        entry_skill = (ENTRY_SKILL_ROOT / "SKILL.md").read_text(encoding="utf-8")
        registry_text = REGISTRY_PATH.read_text(encoding="utf-8")
        trigger_eval_path = ENTRY_SKILL_ROOT / "evals" / "trigger-evals.json"

        self.assertIn("provenance-and-alias-policy.md", registry_text)
        self.assertIn("questions-and-reflection-policy.md", registry_text)
        self.assertIn("query-writeback-lifecycle.md", registry_text)
        self.assertIn("taxonomy-and-hubs.md", registry_text)
        self.assertIn("paper-ingestion-lifecycle.md", registry_text)
        self.assertIn("memory-lifecycle.md", registry_text)
        self.assertIn("graph-contract.md", registry_text)
        self.assertIn("source-manifest-contract.md", registry_text)
        self.assertIn("topic-template.md", registry_text)
        self.assertIn("render-template.md", registry_text)
        self.assertIn("profile-contract.md", registry_text)
        self.assertIn("automation-hooks.md", registry_text)
        self.assertIn("compile-method.md", registry_text)
        self.assertIn("archive-model.md", registry_text)
        self.assertIn("outputs/web/", registry_text)

        self.assertIn("kb-review", readme)
        self.assertIn("kb-ingest", readme)
        self.assertIn("kb-render", readme)
        self.assertIn("Companion skill matrix", readme)
        self.assertIn("bootstrap_review_gated_vault.py", readme)
        self.assertIn("migrate_legacy_vault.py", readme)
        self.assertIn("vault_status.py", readme)
        self.assertIn("web-access", readme)
        self.assertIn("publish` mode", readme)
        self.assertIn("Archive Surfaces", readme)
        self.assertIn("kb-review", readme_cn)
        self.assertIn("kb-ingest", readme_cn)
        self.assertIn("kb-render", readme_cn)
        self.assertIn("搭配技能矩阵", readme_cn)
        self.assertIn("bootstrap_review_gated_vault.py", readme_cn)
        self.assertIn("migrate_legacy_vault.py", readme_cn)
        self.assertIn("vault_status.py", readme_cn)
        self.assertIn("web-access", readme_cn)
        self.assertIn("publish` mode", readme_cn)
        self.assertIn("归档面", readme_cn)
        self.assertIn("kb-review", entry_skill)
        self.assertIn("creator knowledge compiler", entry_skill)
        self.assertIn("MEMORY.md", readme)
        self.assertIn("outputs/episodes/", readme)
        self.assertIn("outputs/web/", readme)
        self.assertIn("wiki/live/procedures/", readme)
        self.assertIn("outputs/episodes/", readme_cn)
        self.assertIn("outputs/web/", readme_cn)
        self.assertIn("web-access", (REPO_ROOT / "CLAUDE.md").read_text(encoding="utf-8"))
        self.assertIn("publish", (REPO_ROOT / "CLAUDE.md").read_text(encoding="utf-8"))
        self.assertIn("wiki/live/procedures/", (REPO_ROOT / "CLAUDE.md").read_text(encoding="utf-8"))
        self.assertIn("source library / clipped research", readme)
        self.assertIn("素材库 / 网页摘录", readme_cn)
        self.assertIn("architecture/", docs_readme)
        self.assertIn("/architecture/overview", docs_config)
        self.assertIn("/architecture/archive-model", docs_config)
        self.assertIn("/zh/architecture/archive-model", docs_config)
        self.assertTrue((REPO_ROOT / "docs" / "architecture" / "overview.md").exists())
        self.assertTrue((REPO_ROOT / "docs" / "architecture" / "archive-model.md").exists())
        self.assertTrue((REPO_ROOT / "docs" / "zh" / "architecture" / "overview.md").exists())
        self.assertTrue((REPO_ROOT / "docs" / "zh" / "architecture" / "archive-model.md").exists())
        self.assertTrue(trigger_eval_path.exists())
        trigger_evals = json.loads(trigger_eval_path.read_text(encoding="utf-8"))
        self.assertGreaterEqual(len(trigger_evals), 20)

    def test_skill_facing_docs_use_latest_lifecycle_wording_instead_of_generation_labels(self) -> None:
        explicit_generation_label = re.compile(r"\bv2\b", re.IGNORECASE)
        skill_facing_paths = [
            *SKILL_PATHS.values(),
            *sorted((ENTRY_SKILL_ROOT / "references").glob("*.md")),
        ]

        offenders: list[str] = []
        for path in skill_facing_paths:
            text = path.read_text(encoding="utf-8")
            if explicit_generation_label.search(text):
                offenders.append(path.relative_to(REPO_ROOT).as_posix())

        self.assertEqual(offenders, [])

        lifecycle_matrix = (ENTRY_SKILL_ROOT / "references" / "lifecycle-matrix.md").read_text(encoding="utf-8")
        memory_lifecycle = (ENTRY_SKILL_ROOT / "references" / "memory-lifecycle.md").read_text(encoding="utf-8")
        self.assertIn("backward compatibility rule", lifecycle_matrix)
        self.assertIn("Old vaults can adopt this gradually", memory_lifecycle)

    def test_docs_use_native_workflow_diagram_and_retire_old_chinese_labels(self) -> None:
        banned_phrases = ("按症状进入", "运行模型", "审校门")
        core_docs = [
            REPO_ROOT / "README_CN.md",
            REPO_ROOT / "docs" / ".vitepress" / "config.ts",
            REPO_ROOT / "docs" / "zh" / "index.md",
            REPO_ROOT / "docs" / "zh" / "guide" / "introduction.md",
            REPO_ROOT / "docs" / "zh" / "skills" / "overview.md",
            REPO_ROOT / "docs" / "zh" / "workflow" / "overview.md",
        ]

        for path in core_docs:
            text = path.read_text(encoding="utf-8")
            for phrase in banned_phrases:
                self.assertNotIn(phrase, text, msg=f"{path.relative_to(REPO_ROOT)} still contains retired phrase {phrase!r}")

        workflow_pages = [
            REPO_ROOT / "docs" / "workflow" / "overview.md",
            REPO_ROOT / "docs" / "zh" / "workflow" / "overview.md",
        ]
        for path in workflow_pages:
            text = path.read_text(encoding="utf-8")
            self.assertNotIn("```mermaid", text, msg=f"{path.relative_to(REPO_ROOT)} should not use Mermaid fences anymore.")
            self.assertIn("<WorkflowLifecycleDiagram", text, msg=f"{path.relative_to(REPO_ROOT)} should use the native workflow diagram component.")

        component_path = REPO_ROOT / "docs" / ".vitepress" / "theme" / "components" / "WorkflowLifecycleDiagram.vue"
        self.assertTrue(component_path.exists())

    def test_trigger_evals_cover_all_skill_routes_and_negative_controls(self) -> None:
        trigger_evals = json.loads((ENTRY_SKILL_ROOT / "evals" / "trigger-evals.json").read_text(encoding="utf-8"))
        expected_skill_counts: dict[str, int] = {}

        for item in trigger_evals:
            expected_skill = item["expected_skill"] if item["expected_skill"] is not None else "none"
            expected_skill_counts[expected_skill] = expected_skill_counts.get(expected_skill, 0) + 1

        for skill_name in [*SKILL_PATHS.keys(), "none"]:
            self.assertIn(skill_name, expected_skill_counts)

        self.assertGreaterEqual(expected_skill_counts["kb-review"], 2)
        self.assertGreaterEqual(expected_skill_counts["kb-ingest"], 2)
        self.assertGreaterEqual(expected_skill_counts["kb-query"], 4)
        self.assertGreaterEqual(expected_skill_counts["kb-render"], 1)
        self.assertTrue(any(item["expected_skill"] == "kb-query" and "archive this answer" in item["query"].lower() for item in trigger_evals))
        self.assertTrue(any(item["expected_skill"] == "kb-review" and "archive backlog" in item["query"].lower() for item in trigger_evals))
        self.assertTrue(any(item["expected_skill"] == "obsidian-notes-karpathy" and "raw/09-archive" in item["query"] for item in trigger_evals))

    def test_evals_cover_review_gated_routes_and_legacy_migration_cases(self) -> None:
        evals = json.loads((ENTRY_SKILL_ROOT / "evals" / "evals.json").read_text(encoding="utf-8"))["evals"]
        fixture_names = {
            Path(file_path).parts[2]
            for item in evals
            for file_path in item.get("files", [])
            if file_path.startswith("evals/fixtures/")
        }

        for fixture_name in {
            "missing-contract-guidance",
            "bootstrap-raw-intake",
            "needs-migration",
            "needs-briefing-refresh",
            "needs-compilation",
            "memory-boundary",
            "needs-maintenance",
            "needs-ingest",
            "ready-for-query",
            "needs-review",
            "writeback-backlog",
            "needs-governance-refresh",
            "governance-enabled-bootstrap",
            "confidence-decay",
            "supersession-chain",
            "episodic-consolidation",
            "procedural-promotion",
            "graph-relationship-gap",
            "private-shared-boundary",
            "hybrid-candidate-routing",
            "audit-trail",
            "creator-consistency",
        }:
            self.assertIn(fixture_name, fixture_names)

        for legacy_fixture in {"legacy-render-breakage", "legacy-compiled-layout", "legacy-answer-drift"}:
            self.assertNotIn(legacy_fixture, fixture_names)

    def test_runtime_eval_manifest_covers_all_operational_skills(self) -> None:
        manifest = json.loads(RUNTIME_EVALS_PATH.read_text(encoding="utf-8"))
        counts: dict[str, int] = {}
        for item in manifest["evals"]:
            counts[item["skill"]] = counts.get(item["skill"], 0) + 1
            self.assertIn("vault_root", item)
            self.assertEqual(item.get("mode", "read-only"), "read-only")

        for skill_name in ("kb-init", "kb-ingest", "kb-compile", "kb-review", "kb-query", "kb-render"):
            self.assertGreaterEqual(counts.get(skill_name, 0), 2)

    def test_writable_runtime_eval_manifest_covers_mutating_operational_skills(self) -> None:
        manifest = json.loads(WRITABLE_RUNTIME_EVALS_PATH.read_text(encoding="utf-8"))
        counts: dict[str, int] = {}
        for item in manifest["evals"]:
            counts[item["skill"]] = counts.get(item["skill"], 0) + 1
            self.assertEqual(item.get("mode"), "writable-copy")
            self.assertIn("vault_root", item)
            self.assertTrue(item.get("checks"))

        for skill_name in ("kb-init", "kb-ingest", "kb-compile", "kb-review", "kb-query", "kb-render"):
            self.assertGreaterEqual(counts.get(skill_name, 0), 1)


if __name__ == "__main__":
    unittest.main()
