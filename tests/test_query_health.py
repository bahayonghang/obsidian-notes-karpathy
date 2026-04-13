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
            self.assertTrue((indices_root / "ENTITIES.md").exists())
            self.assertTrue((indices_root / "RELATIONSHIPS.md").exists())
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
        self.assertEqual(scope["candidate_paths"], [])

    def test_query_scope_excludes_memory_from_topic_retrieval(self) -> None:
        scope = run_json_script("scan_query_scope.py", str(FIXTURES_DIR / "memory-boundary"))

        self.assertEqual(scope["layout_family"], "review-gated")
        self.assertIn("wiki/live/concepts/editorial-boundary.md", scope["included_paths"])
        self.assertIn("MEMORY.md", scope["excluded_paths"])

    def test_query_scope_fast_personal_profile_skips_briefings(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            fixture_copy = Path(tmp) / "ready-for-query"
            shutil.copytree(FIXTURES_DIR / "ready-for-query", fixture_copy)
            sync_payload = run_json_script("sync_source_manifest.py", str(fixture_copy))
            manifest_path = fixture_copy / sync_payload["written_manifest"]
            manifest_text = manifest_path.read_text(encoding="utf-8").replace('profile: "governed-team"', 'profile: "fast-personal"')
            manifest_path.write_text(manifest_text, encoding="utf-8")

            scope = run_json_script("scan_query_scope.py", str(fixture_copy))
            self.assertEqual(scope["profile"], "fast-personal")
            self.assertNotIn("wiki/briefings/researcher.md", scope["included_paths"])

    def test_fast_personal_profile_delays_briefing_refresh_route(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            fixture_copy = Path(tmp) / "needs-briefing-refresh"
            shutil.copytree(FIXTURES_DIR / "needs-briefing-refresh", fixture_copy)
            sync_payload = run_json_script("sync_source_manifest.py", str(fixture_copy))
            manifest_path = fixture_copy / sync_payload["written_manifest"]
            manifest_text = manifest_path.read_text(encoding="utf-8").replace('profile: "governed-team"', 'profile: "fast-personal"')
            manifest_path.write_text(manifest_text, encoding="utf-8")

            payload = run_json_script("detect_lifecycle.py", str(fixture_copy))
            self.assertEqual(payload["profile"], "fast-personal")
            self.assertNotEqual(payload["state"], "needs-briefing-refresh")

    def test_query_scope_surfaces_candidate_paths_without_widening_truth(self) -> None:
        scope = run_json_script("scan_query_scope.py", str(FIXTURES_DIR / "hybrid-candidate-routing"))

        self.assertEqual(scope["layout_family"], "review-gated")
        self.assertIn("wiki/live/concepts/hybrid-routing.md", scope["included_paths"])
        self.assertIn("outputs/episodes/2026-04-10-hybrid-routing.md", scope["candidate_paths"])
        self.assertIn("outputs/health/graph-snapshot.json", scope["candidate_paths"])
        self.assertNotIn("outputs/episodes/2026-04-10-hybrid-routing.md", scope["included_paths"])
        self.assertEqual(scope["candidate_policy"], "candidate-only")

    def test_query_scope_flags_private_and_sensitive_surfaces_without_widening_truth(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            fixture_copy = Path(tmp) / "private-shared-boundary"
            shutil.copytree(FIXTURES_DIR / "ready-for-query", fixture_copy)

            private_qa = fixture_copy / "outputs" / "qa" / "2026-04-10-private-answer.md"
            private_qa.write_text(
                """---
question: \"What should stay private?\"
visibility_scope: private
sensitivity_level: restricted
source_live_pages:
  - \"[[wiki/live/concepts/review-gate]]\"
---

# Private Answer
""",
                encoding="utf-8",
            )

            scope = run_json_script("scan_query_scope.py", str(fixture_copy))
            self.assertTrue(any(item["path"] == "outputs/qa/2026-04-10-private-answer.md" for item in scope["scope_leaks"]))
            self.assertTrue(any(item["path"] == "outputs/qa/2026-04-10-private-answer.md" for item in scope["sensitivity_candidates"]))
            self.assertIn("outputs/qa/2026-04-10-private-answer.md", scope["included_paths"])


    def test_lint_obsidian_mechanics_flags_review_gated_staleness_and_backlog(self) -> None:
        review_ready = run_json_script("lint_obsidian_mechanics.py", str(FIXTURES_DIR / "needs-review"))
        briefing_refresh = run_json_script("lint_obsidian_mechanics.py", str(FIXTURES_DIR / "needs-briefing-refresh"))

        review_issue_kinds = {issue["kind"] for issue in review_ready["issues"]}
        stale_issue_kinds = {issue["kind"] for issue in briefing_refresh["issues"]}

        self.assertIn("review_backlog", review_issue_kinds)
        self.assertIn("stale_briefing", stale_issue_kinds)

    def test_governance_index_builder_surfaces_questions_gaps_aliases_and_confidence_maintenance(self) -> None:
        query_payload = run_json_script("build_governance_indices.py", str(FIXTURES_DIR / "ready-for-query"))
        maintenance_payload = run_json_script("build_governance_indices.py", str(FIXTURES_DIR / "needs-maintenance"))
        refresh_payload = run_json_script("build_governance_indices.py", str(FIXTURES_DIR / "needs-governance-refresh"))
        writeback_payload = run_json_script("build_governance_indices.py", str(FIXTURES_DIR / "writeback-backlog"))
        confidence_payload = run_json_script("build_governance_indices.py", str(FIXTURES_DIR / "confidence-decay"))

        self.assertIn("When should governance indices be refreshed?", query_payload["questions"])
        self.assertIn("review-gate", {row["canonical_slug"] for row in query_payload["alias_rows"]})
        self.assertIn("approval-gate", set(query_payload["alias_rows"][0]["aliases"]))
        self.assertIn("stale_qa", maintenance_payload["gap_issue_kinds"])
        self.assertIn("broken_wikilink", maintenance_payload["gap_issue_kinds"])
        self.assertIn("Which approval signals should trigger a governance refresh?", refresh_payload["questions"])
        self.assertNotIn("How should archived Q&A feed governance refreshes?", refresh_payload["questions"])
        self.assertTrue(any(item["writeback_status"] == "pending" for item in writeback_payload["writeback_backlog"]))
        self.assertTrue(any(item["followup_route"] == "review" for item in refresh_payload["followup_routes"]))
        self.assertEqual({item["issue_kind"] for item in confidence_payload["confidence_maintenance"]}, {"missing_confidence_metadata", "confidence_decay_due"})
        self.assertTrue(any(item["recommended_action"] == "fill_core_confidence_metadata" for item in confidence_payload["confidence_maintenance"]))
        self.assertTrue(any(item["recommended_action"] == "refresh_confidence_review" for item in confidence_payload["confidence_maintenance"]))
        self.assertIn("## Confidence Maintenance Signals", confidence_payload["files"]["GAPS.md"])

        supersession_payload = run_json_script("build_governance_indices.py", str(FIXTURES_DIR / "supersession-chain"))
        episode_payload = run_json_script("build_governance_indices.py", str(FIXTURES_DIR / "episodic-consolidation"))
        procedure_payload = run_json_script("build_governance_indices.py", str(FIXTURES_DIR / "procedural-promotion"))
        self.assertTrue(any(item["issue_kind"] == "supersession_gap" and item["recommended_action"] == "reconcile_supersession_chain" for item in supersession_payload["closure_signals"]))
        self.assertTrue(any(item["issue_kind"] == "episodic_backlog" and item["recommended_action"] == "consolidate_episode_followup" for item in episode_payload["closure_signals"]))
        self.assertTrue(any(item["issue_kind"] == "procedural_promotion_gap" and item["recommended_action"] == "draft_procedure_candidate" for item in procedure_payload["closure_signals"]))
        self.assertIn("## Closure Signals", supersession_payload["files"]["GAPS.md"])

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
                'source_live_pages:\n  - "[[wiki/live/summaries/human/articles/2026-04-05-approved-summary]]"\n  - "[[wiki/live/concepts/review-gate]]"\nfollowup_route: review\n',
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

    def test_detect_lifecycle_and_health_flag_memory_writeback_and_boundary_issues(self) -> None:
        writeback = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "writeback-backlog"))
        memory = run_json_script("lint_obsidian_mechanics.py", str(FIXTURES_DIR / "memory-boundary"))
        writeback_lint = run_json_script("lint_obsidian_mechanics.py", str(FIXTURES_DIR / "writeback-backlog"))

        with tempfile.TemporaryDirectory() as tmp:
            fixture_copy = Path(tmp) / "private-shared-boundary"
            shutil.copytree(FIXTURES_DIR / "ready-for-query", fixture_copy)
            private_qa = fixture_copy / "outputs" / "qa" / "2026-04-10-private-answer.md"
            private_qa.write_text(
                """---
question: \"What should stay private?\"
visibility_scope: private
source_live_pages:
  - \"[[wiki/live/concepts/review-gate]]\"
---

# Private Answer
""",
                encoding="utf-8",
            )
            private_lint = run_json_script("lint_obsidian_mechanics.py", str(fixture_copy))

        self.assertEqual(writeback["state"], "needs-maintenance")
        self.assertEqual(writeback["route"], "kb-review")
        self.assertEqual(writeback["route_mode"], "maintenance")
        self.assertIn("writeback_backlog", writeback["health_flags"])

        memory_issue_kinds = {issue["kind"] for issue in memory["issues"]}
        writeback_issue_kinds = {issue["kind"] for issue in writeback_lint["issues"]}
        private_issue_kinds = {issue["kind"] for issue in private_lint["issues"]}
        self.assertIn("memory_knowledge_mix", memory_issue_kinds)
        self.assertIn("writeback_backlog", writeback_issue_kinds)
        self.assertIn("private_scope_leak", private_issue_kinds)
        self.assertIn("sensitivity_metadata_gap", private_issue_kinds)

    def test_latest_health_flags_cover_confidence_supersession_episode_procedure_graph_and_audit(self) -> None:
        confidence = run_json_script("lint_obsidian_mechanics.py", str(FIXTURES_DIR / "confidence-decay"))
        supersession = run_json_script("lint_obsidian_mechanics.py", str(FIXTURES_DIR / "supersession-chain"))
        episodes = run_json_script("lint_obsidian_mechanics.py", str(FIXTURES_DIR / "episodic-consolidation"))
        procedures = run_json_script("lint_obsidian_mechanics.py", str(FIXTURES_DIR / "procedural-promotion"))
        graph_gap = run_json_script("lint_obsidian_mechanics.py", str(FIXTURES_DIR / "graph-relationship-gap"))
        audit_gap = run_json_script("lint_obsidian_mechanics.py", str(FIXTURES_DIR / "audit-trail"))

        confidence_issues = {issue["kind"]: issue for issue in confidence["issues"]}
        supersession_issues = {issue["kind"]: issue for issue in supersession["issues"]}
        episodic_issues = {issue["kind"]: issue for issue in episodes["issues"]}
        procedural_issues = {issue["kind"]: issue for issue in procedures["issues"]}
        self.assertIn("missing_confidence_metadata", confidence_issues)
        self.assertIn("confidence_decay_due", confidence_issues)
        self.assertEqual(confidence_issues["missing_confidence_metadata"]["recommended_action"], "fill_core_confidence_metadata")
        self.assertEqual(confidence_issues["confidence_decay_due"]["recommended_action"], "refresh_confidence_review")
        self.assertGreaterEqual(confidence_issues["confidence_decay_due"]["overdue_days"], 1)
        self.assertIn("supersession_gap", supersession_issues)
        self.assertEqual(supersession_issues["supersession_gap"]["recommended_action"], "reconcile_supersession_chain")
        self.assertEqual(supersession_issues["supersession_gap"]["followup_route"], "review")
        self.assertIn("episodic_backlog", episodic_issues)
        self.assertEqual(episodic_issues["episodic_backlog"]["recommended_action"], "consolidate_episode_followup")
        self.assertEqual(episodic_issues["episodic_backlog"]["followup_route"], "draft")
        self.assertIn("procedural_promotion_gap", procedural_issues)
        self.assertEqual(procedural_issues["procedural_promotion_gap"]["recommended_action"], "draft_procedure_candidate")
        self.assertEqual(procedural_issues["procedural_promotion_gap"]["followup_route"], "draft")
        self.assertIn("graph_gap", {issue["kind"] for issue in graph_gap["issues"]})
        self.assertIn("audit_trail_gap", {issue["kind"] for issue in audit_gap["issues"]})

    def test_rank_query_candidates_prefers_approved_live_surfaces_without_widening_truth(self) -> None:
        payload = run_json_script(
            "rank_query_candidates.py",
            str(FIXTURES_DIR / "hybrid-candidate-routing"),
            "hybrid routing truth boundary",
        )

        self.assertEqual(payload["candidate_policy"], "candidate-only")
        self.assertGreaterEqual(payload["included_count"], 1)
        self.assertGreaterEqual(payload["candidate_count"], 1)
        self.assertGreaterEqual(len(payload["ranked_paths"]), 3)

        top = payload["ranked_paths"][0]
        self.assertEqual(top["path"], "wiki/live/concepts/hybrid-routing.md")
        self.assertEqual(top["candidate_kind"], "included")
        self.assertEqual(top["truth_boundary"], "approved-live")

        candidate_items = [item for item in payload["ranked_paths"] if item["candidate_kind"] == "candidate"]
        self.assertTrue(any(item["path"] == "outputs/episodes/2026-04-10-hybrid-routing.md" for item in candidate_items))
        self.assertTrue(any(item["path"] == "outputs/qa/2026-04-10-hybrid-answer.md" for item in payload["ranked_paths"]))
        self.assertTrue(any(item["path"] == "outputs/health/graph-snapshot.json" for item in candidate_items))
        self.assertTrue(all(item["truth_boundary"] == "candidate-only" for item in candidate_items))

    def test_rank_query_candidates_can_prioritize_topics(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            fixture_copy = Path(tmp) / "ready-for-query"
            shutil.copytree(FIXTURES_DIR / "ready-for-query", fixture_copy)
            topic_path = fixture_copy / "wiki" / "live" / "topics" / "review-gates.md"
            topic_path.parent.mkdir(parents=True, exist_ok=True)
            topic_path.write_text(
                """---
title: "Review Gates"
trust_level: approved
related:
  - "[[wiki/live/concepts/review-gate]]"
---

# Review Gates
""",
                encoding="utf-8",
            )
            payload = run_json_script("rank_query_candidates.py", str(fixture_copy), "review gates")
            self.assertEqual(payload["ranked_paths"][0]["kind"], "topic")

    def test_render_live_artifact_writes_supported_modes(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            fixture_copy = Path(tmp) / "ready-for-query"
            shutil.copytree(FIXTURES_DIR / "ready-for-query", fixture_copy)
            source_path = "wiki/live/concepts/review-gate.md"

            slides = run_json_script(
                "render_live_artifact.py",
                str(fixture_copy),
                "--mode",
                "slides",
                "--source",
                source_path,
                "--write",
            )
            report = run_json_script(
                "render_live_artifact.py",
                str(fixture_copy),
                "--mode",
                "report",
                "--source",
                source_path,
                "--write",
            )
            canvas = run_json_script(
                "render_live_artifact.py",
                str(fixture_copy),
                "--mode",
                "canvas",
                "--source",
                source_path,
                "--write",
            )
            web = run_json_script(
                "render_live_artifact.py",
                str(fixture_copy),
                "--mode",
                "web",
                "--source",
                source_path,
                "--write",
            )

            self.assertTrue((fixture_copy / slides["output_path"]).exists())
            self.assertTrue((fixture_copy / report["output_path"]).exists())
            self.assertTrue((fixture_copy / canvas["output_path"]).exists())
            self.assertTrue((fixture_copy / web["output_path"]).exists())
            self.assertTrue((fixture_copy / web["package_root"] / "manifest.json").exists())
            self.assertTrue((fixture_copy / web["package_root"] / "app.js").exists())
            self.assertIn("source_live_pages", (fixture_copy / slides["output_path"]).read_text(encoding="utf-8"))
            self.assertIn("followup_route", (fixture_copy / report["output_path"]).read_text(encoding="utf-8"))
            self.assertIn("source_live_pages", (fixture_copy / web["package_root"] / "manifest.json").read_text(encoding="utf-8"))

    def test_render_live_artifact_web_mode_rejects_unapproved_sources(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            fixture_copy = Path(tmp) / "ready-for-query"
            shutil.copytree(FIXTURES_DIR / "ready-for-query", fixture_copy)
            payload = run_json_script(
                "render_live_artifact.py",
                str(fixture_copy),
                "--mode",
                "web",
                "--source",
                "raw/human/articles/2026-04-05-approved-summary.md",
                "--write",
            )

            self.assertIn("raw/human/articles/2026-04-05-approved-summary.md", payload["rejected_source_paths"])
            self.assertEqual(payload["source_paths"], [])
            self.assertTrue((fixture_copy / payload["package_root"] / "manifest.json").exists())

    def test_automation_runner_writes_outputs_and_audit(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            health_copy = Path(tmp) / "graph-relationship-gap"
            shutil.copytree(FIXTURES_DIR / "graph-relationship-gap", health_copy)
            payload = run_json_script("run_automation_hooks.py", str(health_copy), "--mode", "scheduled-health", "--write")

            self.assertEqual(payload["mode"], "scheduled-health")
            self.assertIn("governance", payload)
            self.assertIn("graph", payload)
            self.assertTrue((health_copy / "wiki" / "live" / "indices" / "GAPS.md").exists())
            self.assertTrue((health_copy / "outputs" / "health" / "graph-snapshot.json").exists())
            self.assertTrue((health_copy / "outputs" / "audit" / "operations.jsonl").exists())

        with tempfile.TemporaryDirectory() as tmp:
            episode_copy = Path(tmp) / "episodic-consolidation"
            shutil.copytree(FIXTURES_DIR / "episodic-consolidation", episode_copy)
            payload = run_json_script("run_automation_hooks.py", str(episode_copy), "--mode", "session-end", "--write")

            self.assertEqual(payload["mode"], "session-end")
            self.assertIn("episodes", payload)
            self.assertTrue((episode_copy / "outputs" / "episodes" / "2026-04-10-review-gate-runtime.md").exists())
            self.assertTrue((episode_copy / "outputs" / "audit" / "operations.jsonl").exists())



if __name__ == "__main__":
    unittest.main()
