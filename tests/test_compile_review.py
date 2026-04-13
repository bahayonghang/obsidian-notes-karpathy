# pyright: reportMissingImports=false
import unittest
import shutil
import tempfile
from pathlib import Path

from _bundle_test_support import FIXTURES_DIR, accepted_raw_sources, run_json_script


class CompileReviewTests(unittest.TestCase):
    def test_scan_ingest_delta_reports_manifest_drift(self) -> None:
        payload = run_json_script("scan_ingest_delta.py", str(FIXTURES_DIR / "needs-ingest"))

        self.assertTrue(payload["manifest_present"])
        self.assertEqual(payload["manifest_status"], "stale")
        self.assertTrue(payload["needs_ingest"])
        self.assertEqual(payload["counts"]["new"], 1)
        self.assertEqual(payload["counts"]["removed"], 1)

    def test_scan_compile_delta_reports_review_gated_capture_trust(self) -> None:
        review_ready = run_json_script("scan_compile_delta.py", str(FIXTURES_DIR / "needs-review"))

        self.assertEqual(review_ready["layout_family"], "review-gated")
        self.assertEqual(review_ready["counts"]["new"], 0)
        self.assertEqual(review_ready["counts"]["changed"], 0)
        self.assertEqual(review_ready["counts"]["unchanged"], 2)

        items = {item["path"]: item for item in review_ready["items"]}
        human_item = items["raw/human/articles/2026-04-01-compiler-safety.md"]
        agent_item = items["raw/agents/researcher/2026-04-02-review-gates.md"]

        self.assertEqual(human_item["capture_source"], "human")
        self.assertEqual(human_item["capture_trust"], "curated")
        self.assertEqual(
            human_item["summary_path"],
            "wiki/drafts/summaries/human/articles/2026-04-01-compiler-safety.md",
        )
        self.assertEqual(agent_item["capture_source"], "agent")
        self.assertEqual(agent_item["capture_trust"], "untrusted")
        self.assertEqual(agent_item["agent_role"], "researcher")
        self.assertEqual(
            agent_item["summary_path"],
            "wiki/drafts/summaries/agents/researcher/2026-04-02-review-gates.md",
        )

    def test_scan_compile_delta_keeps_pdf_directory_policy(self) -> None:
        skill_home_root = FIXTURES_DIR / "companion-skill-homes"
        pdf_only = run_json_script(
            "scan_compile_delta.py",
            str(FIXTURES_DIR / "paper-intake-with-handle"),
            env={"KB_COMPANION_SKILL_PATHS": str(skill_home_root / "both")},
        )

        self.assertEqual(pdf_only["layout_family"], "legacy-layout")
        self.assertEqual(pdf_only["items"][0]["ingest_plan"], "paper-workbench")
        self.assertEqual(pdf_only["items"][0]["ingest_reason"], "paper_workbench_directory_policy")
        self.assertEqual(pdf_only["items"][0]["paper_handle"], "1706.03762")
        self.assertEqual(
            pdf_only["items"][0]["metadata_path"],
            "raw/papers/2026-04-08-transformers.source.md",
        )

    def test_scan_compile_delta_reports_skipped_pdf_without_companion_skill(self) -> None:
        pdf_only = run_json_script(
            "scan_compile_delta.py",
            str(FIXTURES_DIR / "paper-intake-with-handle"),
            env={"KB_COMPANION_SKILL_PATHS": ""},
        )

        self.assertEqual(pdf_only["counts"]["new"], 1)
        self.assertEqual(pdf_only["counts"]["skipped"], 1)
        self.assertEqual(pdf_only["items"][0]["ingest_plan"], "skip")
        self.assertEqual(
            pdf_only["items"][0]["ingest_reason"],
            "paper_workbench_required_for_raw_papers",
        )

    def test_scan_review_queue_applies_mixed_gate(self) -> None:
        queue = run_json_script("scan_review_queue.py", str(FIXTURES_DIR / "review-conflict-mixed-gate"))

        self.assertEqual(queue["counts"]["approve"], 1)
        self.assertEqual(queue["counts"]["reject"], 1)
        self.assertEqual(queue["counts"]["needs-human"], 1)

        decisions = {item["path"]: item["decision"] for item in queue["items"]}
        self.assertEqual(
            decisions["wiki/drafts/summaries/human/articles/2026-04-03-grounded-article.md"],
            "approve",
        )
        self.assertEqual(
            decisions["wiki/drafts/summaries/agents/scout/2026-04-03-hallucinated-bridge.md"],
            "reject",
        )
        self.assertEqual(
            decisions["wiki/drafts/summaries/agents/editor/2026-04-03-contradictory-summary.md"],
            "needs-human",
        )

    def test_accepted_raw_sources_supports_review_gated_layout_and_skips_pdf_sidecars(self) -> None:
        v2_sources = [
            path.relative_to(FIXTURES_DIR / "needs-review").as_posix()
            for path in accepted_raw_sources(FIXTURES_DIR / "needs-review")
        ]
        pdf_sources = [
            path.relative_to(FIXTURES_DIR / "paper-intake-with-handle").as_posix()
            for path in accepted_raw_sources(FIXTURES_DIR / "paper-intake-with-handle")
        ]

        self.assertEqual(
            v2_sources,
            [
                "raw/agents/researcher/2026-04-02-review-gates.md",
                "raw/human/articles/2026-04-01-compiler-safety.md",
            ],
        )
        self.assertEqual(pdf_sources, ["raw/papers/2026-04-08-transformers.pdf"])

    def test_build_draft_packages_writes_topics_and_review_meta(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            fixture_copy = Path(tmp) / "needs-compilation"
            shutil.copytree(FIXTURES_DIR / "needs-compilation", fixture_copy)
            run_json_script("sync_source_manifest.py", str(fixture_copy))
            payload = run_json_script("build_draft_packages.py", str(fixture_copy), "--write")

            self.assertGreaterEqual(payload["package_count"], 1)
            self.assertTrue((fixture_copy / "wiki" / "drafts" / "topics").exists())
            self.assertTrue((fixture_copy / "wiki" / "drafts" / "indices" / "packages").exists())
            self.assertTrue(any(path.startswith("wiki/drafts/topics/") for path in payload["written_paths"]))
            self.assertTrue(any(path.startswith("wiki/drafts/indices/packages/") for path in payload["written_paths"]))

    def test_sync_manifest_tracks_asset_and_data_sources(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            fixture_copy = Path(tmp) / "ready-for-query"
            shutil.copytree(FIXTURES_DIR / "ready-for-query", fixture_copy)
            asset_path = fixture_copy / "raw" / "human" / "assets" / "diagram.png"
            data_path = fixture_copy / "raw" / "human" / "data" / "stats.json"
            asset_path.parent.mkdir(parents=True, exist_ok=True)
            data_path.parent.mkdir(parents=True, exist_ok=True)
            asset_path.write_bytes(b"png-bytes")
            data_path.write_text('{"count": 3}', encoding="utf-8")

            manifest_payload = run_json_script("sync_source_manifest.py", str(fixture_copy))
            compile_payload = run_json_script("scan_compile_delta.py", str(fixture_copy))

            manifest_text = (fixture_copy / manifest_payload["written_manifest"]).read_text(encoding="utf-8")
            self.assertIn("raw/human/assets/diagram.png", manifest_text)
            self.assertIn("raw/human/data/stats.json", manifest_text)
            source_items = {item["path"]: item for item in compile_payload["items"]}
            self.assertEqual(source_items["raw/human/assets/diagram.png"]["source_class"], "image_asset")
            self.assertEqual(source_items["raw/human/data/stats.json"]["source_class"], "data_asset")


if __name__ == "__main__":
    unittest.main()
