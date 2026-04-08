# pyright: reportMissingImports=false
import unittest

from _bundle_test_support import FIXTURES_DIR, run_json_script


class QueryHealthTests(unittest.TestCase):
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
