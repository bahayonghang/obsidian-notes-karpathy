# pyright: reportMissingImports=false
import json
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

        self.assertIn("evidence_coverage", summary_template)
        self.assertIn("uncertainty_level", summary_template)
        self.assertIn("promotion_reason", review_template)
        self.assertIn("fact_inference_separation", review_template)
        self.assertIn("writeback_candidates", qa_template)
        self.assertIn("writeback_status", qa_template)
        self.assertIn("writeback_candidates", content_template)
        self.assertIn("MEMORY.md", file_model)

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

        self.assertIn("kb-review", readme)
        self.assertIn("kb-review", readme_cn)
        self.assertIn("kb-review", entry_skill)
        self.assertIn("MEMORY.md", readme)
        self.assertIn("architecture/", docs_readme)
        self.assertIn("/architecture/overview", docs_config)
        self.assertTrue((REPO_ROOT / "docs" / "architecture" / "overview.md").exists())
        self.assertTrue((REPO_ROOT / "docs" / "zh" / "architecture" / "overview.md").exists())
        self.assertTrue(trigger_eval_path.exists())
        trigger_evals = json.loads(trigger_eval_path.read_text(encoding="utf-8"))
        self.assertGreaterEqual(len(trigger_evals), 20)

    def test_trigger_evals_cover_all_skill_routes_and_negative_controls(self) -> None:
        trigger_evals = json.loads((ENTRY_SKILL_ROOT / "evals" / "trigger-evals.json").read_text(encoding="utf-8"))
        expected_skill_counts: dict[str, int] = {}

        for item in trigger_evals:
            expected_skill = item["expected_skill"] if item["expected_skill"] is not None else "none"
            expected_skill_counts[expected_skill] = expected_skill_counts.get(expected_skill, 0) + 1

        for skill_name in [*SKILL_PATHS.keys(), "none"]:
            self.assertIn(skill_name, expected_skill_counts)

        self.assertGreaterEqual(expected_skill_counts["kb-review"], 2)

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
            "ready-for-query",
            "needs-review",
            "writeback-backlog",
            "needs-governance-refresh",
            "governance-enabled-bootstrap",
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

        for skill_name in ("kb-init", "kb-compile", "kb-review", "kb-query", "kb-health"):
            self.assertGreaterEqual(counts.get(skill_name, 0), 2)

    def test_writable_runtime_eval_manifest_covers_mutating_operational_skills(self) -> None:
        manifest = json.loads(WRITABLE_RUNTIME_EVALS_PATH.read_text(encoding="utf-8"))
        counts: dict[str, int] = {}
        for item in manifest["evals"]:
            counts[item["skill"]] = counts.get(item["skill"], 0) + 1
            self.assertEqual(item.get("mode"), "writable-copy")
            self.assertIn("vault_root", item)
            self.assertTrue(item.get("checks"))

        for skill_name in ("kb-init", "kb-compile", "kb-review"):
            self.assertGreaterEqual(counts.get(skill_name, 0), 1)


if __name__ == "__main__":
    unittest.main()
