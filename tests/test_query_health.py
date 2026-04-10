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
            self.assertIn("When should governance indices be refreshed?", payload["questions"])
            self.assertNotIn("## Follow-up Routes Seen In Archived Outputs", (indices_root / "GAPS.md").read_text(encoding="utf-8"))

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
        writeback_payload = run_json_script("build_governance_indices.py", str(FIXTURES_DIR / "writeback-backlog"))

        self.assertIn("When should governance indices be refreshed?", query_payload["questions"])
        self.assertIn("review-gate", {row["canonical_slug"] for row in query_payload["alias_rows"]})
        self.assertIn("approval-gate", set(query_payload["alias_rows"][0]["aliases"]))
        self.assertIn("stale_qa", maintenance_payload["gap_issue_kinds"])
        self.assertIn("broken_wikilink", maintenance_payload["gap_issue_kinds"])
        self.assertIn("Which approval signals should trigger a governance refresh?", refresh_payload["questions"])
        self.assertNotIn("How should archived Q&A feed governance refreshes?", refresh_payload["questions"])
        self.assertTrue(any(item["writeback_status"] == "pending" for item in writeback_payload["writeback_backlog"]))
        self.assertTrue(any(item["followup_route"] == "health" for item in refresh_payload["followup_routes"]))

    def test_governance_index_builder_filters_completed_writeback_and_non_archived_routes(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            fixture_copy = Path(tmp) / "ready-for-query"
            shutil.copytree(FIXTURES_DIR / "ready-for-query", fixture_copy)

            qa_path = fixture_copy / "outputs" / "qa" / "2026-04-09-compiled-writeback.md"
            qa_path.write_text(
                """---
question: \"Should completed writeback still appear in backlog?\"
writeback_candidates:
  - \"[[wiki/live/concepts/review-gate]]\"
writeback_status: compiled
followup_route: draft
---

# Compiled Writeback
""",
                encoding="utf-8",
            )

            briefing_path = fixture_copy / "wiki" / "briefings" / "researcher.md"
            briefing_text = briefing_path.read_text(encoding="utf-8")
            briefing_text = briefing_text.replace(
                'source_live_pages:\n  - "[[wiki/live/summaries/human/articles/2026-04-05-approved-summary]]"\n  - "[[wiki/live/concepts/review-gate]]"\n',
                'source_live_pages:\n  - "[[wiki/live/summaries/human/articles/2026-04-05-approved-summary]]"\n  - "[[wiki/live/concepts/review-gate]]"\nfollowup_route: health\n',
            )
            briefing_path.write_text(briefing_text, encoding="utf-8")

            payload = run_json_script("build_governance_indices.py", str(fixture_copy))

            backlog_paths = {item["path"] for item in payload["writeback_backlog"]}
            route_paths = {item["path"] for item in payload["followup_routes"]}

            self.assertNotIn("outputs/qa/2026-04-09-compiled-writeback.md", backlog_paths)
            self.assertNotIn("wiki/briefings/researcher.md", route_paths)

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
