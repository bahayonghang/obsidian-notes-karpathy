# pyright: reportMissingImports=false
import shutil
import tempfile
import unittest
from pathlib import Path

from _bundle_test_support import FIXTURES_DIR, run_json_script


class QueryHealthTests(unittest.TestCase):
    def test_governance_index_builder_can_write_indices(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            fixture_copy = Path(tmp) / "ready-for-query"
            shutil.copytree(FIXTURES_DIR / "ready-for-query", fixture_copy)
            payload = run_json_script("build_governance_indices.py", str(fixture_copy), "--write")

            indices_root = fixture_copy / "wiki" / "live" / "indices"
            self.assertTrue((indices_root / "QUESTIONS.md").exists())
            self.assertTrue((indices_root / "GAPS.md").exists())
            self.assertTrue((indices_root / "ALIASES.md").exists())
            self.assertIn("Why does a review gate help multi-agent knowledge systems?", payload["questions"])

    def test_query_scope_ignores_raw_and_drafts_for_review_gated_layout(self) -> None:
        scope = run_json_script("scan_query_scope.py", str(FIXTURES_DIR / "ready-for-query"))

        self.assertEqual(scope["layout_family"], "review-gated")
        self.assertIn("wiki/live/summaries/human/articles/2026-04-05-approved-summary.md", scope["included_paths"])
        self.assertIn("wiki/briefings/researcher.md", scope["included_paths"])
        self.assertIn("outputs/qa/2026-04-06-review-gate-benefits.md", scope["included_paths"])
        self.assertIn("raw/human/articles/2026-04-05-approved-summary.md", scope["excluded_paths"])
        self.assertIn("wiki/drafts/summaries/human/articles/2026-04-05-approved-summary.md", scope["excluded_paths"])

    def test_query_scope_excludes_memory_from_topic_retrieval(self) -> None:
        scope = run_json_script("scan_query_scope.py", str(FIXTURES_DIR / "memory-boundary"))

        self.assertEqual(scope["layout_family"], "review-gated")
        self.assertIn("wiki/live/concepts/editorial-boundary.md", scope["included_paths"])
        self.assertIn("MEMORY.md", scope["excluded_paths"])

    def test_lint_obsidian_mechanics_flags_review_gated_staleness_and_backlog(self) -> None:
        review_ready = run_json_script("lint_obsidian_mechanics.py", str(FIXTURES_DIR / "needs-review"))
        briefing_refresh = run_json_script("lint_obsidian_mechanics.py", str(FIXTURES_DIR / "needs-briefing-refresh"))

        review_issue_kinds = {issue["kind"] for issue in review_ready["issues"]}
        stale_issue_kinds = {issue["kind"] for issue in briefing_refresh["issues"]}

        self.assertIn("review_backlog", review_issue_kinds)
        self.assertIn("stale_briefing", stale_issue_kinds)

    def test_governance_index_builder_surfaces_questions_gaps_and_aliases(self) -> None:
        query_payload = run_json_script("build_governance_indices.py", str(FIXTURES_DIR / "ready-for-query"))
        maintenance_payload = run_json_script("build_governance_indices.py", str(FIXTURES_DIR / "needs-maintenance"))
        refresh_payload = run_json_script("build_governance_indices.py", str(FIXTURES_DIR / "needs-governance-refresh"))

        self.assertIn("Why does a review gate help multi-agent knowledge systems?", query_payload["questions"])
        self.assertIn("When should governance indices be refreshed?", query_payload["questions"])
        self.assertIn("review-gate", {row["canonical_slug"] for row in query_payload["alias_rows"]})
        self.assertIn("approval-gate", set(query_payload["alias_rows"][0]["aliases"]))
        self.assertIn("stale_qa", maintenance_payload["gap_issue_kinds"])
        self.assertIn("broken_wikilink", maintenance_payload["gap_issue_kinds"])
        self.assertIn("Which approval signals should trigger a governance refresh?", refresh_payload["questions"])
        self.assertIn("How should archived Q&A feed governance refreshes?", refresh_payload["questions"])

    def test_governance_index_builder_bootstrap_fixture_can_write_indices(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            fixture_copy = Path(tmp) / "governance-enabled-bootstrap"
            shutil.copytree(FIXTURES_DIR / "governance-enabled-bootstrap", fixture_copy)
            payload = run_json_script("build_governance_indices.py", str(fixture_copy), "--write")

            indices_root = fixture_copy / "wiki" / "live" / "indices"
            self.assertTrue((indices_root / "QUESTIONS.md").exists())
            self.assertTrue((indices_root / "GAPS.md").exists())
            self.assertTrue((indices_root / "ALIASES.md").exists())
            self.assertIn("What governance scaffolding should kb-init generate?", payload["questions"])

    def test_detect_lifecycle_and_health_flag_memory_and_writeback_issues(self) -> None:
        writeback = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "writeback-backlog"))
        memory = run_json_script("lint_obsidian_mechanics.py", str(FIXTURES_DIR / "memory-boundary"))
        writeback_lint = run_json_script("lint_obsidian_mechanics.py", str(FIXTURES_DIR / "writeback-backlog"))

        self.assertEqual(writeback["state"], "needs-maintenance")
        self.assertEqual(writeback["route"], "kb-health")
        self.assertIn("writeback_backlog", writeback["health_flags"])

        memory_issue_kinds = {issue["kind"] for issue in memory["issues"]}
        writeback_issue_kinds = {issue["kind"] for issue in writeback_lint["issues"]}
        self.assertIn("memory_knowledge_mix", memory_issue_kinds)
        self.assertIn("writeback_backlog", writeback_issue_kinds)


if __name__ == "__main__":
    unittest.main()
