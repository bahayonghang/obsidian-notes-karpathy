# pyright: reportMissingImports=false
import shutil
import tempfile
import unittest
from pathlib import Path

from _bundle_test_support import FIXTURES_DIR, accepted_raw_sources, run_json_script


class InitWorkflowTests(unittest.TestCase):
    def test_bootstrap_review_gated_vault_creates_canonical_starter_assets(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            fixture_copy = Path(tmp) / "needs-setup"
            shutil.copytree(FIXTURES_DIR / "needs-setup", fixture_copy)

            payload = run_json_script(
                "bootstrap_review_gated_vault.py",
                str(fixture_copy),
                "--include-governance",
            )

            self.assertIn("raw/human/articles", payload["created_dirs"])
            self.assertTrue((fixture_copy / "AGENTS.md").exists())
            self.assertTrue((fixture_copy / "CLAUDE.md").exists())
            self.assertTrue((fixture_copy / "MEMORY.md").exists())
            self.assertTrue((fixture_copy / "raw" / "_manifest.yaml").exists())
            self.assertTrue((fixture_copy / "wiki" / "index.md").exists())
            self.assertTrue((fixture_copy / "wiki" / "log.md").exists())
            self.assertTrue((fixture_copy / "wiki" / "briefings" / "researcher.md").exists())
            self.assertTrue((fixture_copy / "wiki" / "live" / "indices" / "QUESTIONS.md").exists())
            self.assertIn('profile: "governed-team"', (fixture_copy / "raw" / "_manifest.yaml").read_text(encoding="utf-8"))

    def test_bootstrap_preserves_existing_support_files_while_repairing_structure(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            fixture_copy = Path(tmp) / "needs-repair-live-only"
            shutil.copytree(FIXTURES_DIR / "needs-repair-live-only", fixture_copy)

            original_live_text = (
                fixture_copy / "wiki" / "live" / "summaries" / "human" / "articles" / "2026-04-07-incomplete-live.md"
            ).read_text(encoding="utf-8")
            payload = run_json_script("bootstrap_review_gated_vault.py", str(fixture_copy))

            self.assertIn("wiki/index.md", payload["preserved_files"])
            self.assertTrue((fixture_copy / "wiki" / "drafts").exists())
            self.assertTrue((fixture_copy / "wiki" / "briefings").exists())
            self.assertTrue((fixture_copy / "outputs" / "reviews").exists())
            repaired_live_text = (
                fixture_copy / "wiki" / "live" / "summaries" / "human" / "articles" / "2026-04-07-incomplete-live.md"
            ).read_text(encoding="utf-8")
            self.assertEqual(original_live_text, repaired_live_text)

    def test_bootstrap_surfaces_profile_choice_in_scaffolded_files(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            fixture_copy = Path(tmp) / "needs-setup"
            shutil.copytree(FIXTURES_DIR / "needs-setup", fixture_copy)

            run_json_script(
                "bootstrap_review_gated_vault.py",
                str(fixture_copy),
                "--profile",
                "fast-personal",
                "--include-governance",
            )

            manifest_text = (fixture_copy / "raw" / "_manifest.yaml").read_text(encoding="utf-8")
            index_text = (fixture_copy / "wiki" / "index.md").read_text(encoding="utf-8")
            briefing_text = (fixture_copy / "wiki" / "briefings" / "researcher.md").read_text(encoding="utf-8")

            self.assertIn('profile: "fast-personal"', manifest_text)
            self.assertIn('kb_profile: "fast-personal"', index_text)
            self.assertIn('kb_profile: "fast-personal"', briefing_text)

    def test_migrate_legacy_vault_preserves_originals_and_syncs_manifest(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            fixture_copy = Path(tmp) / "needs-migration"
            shutil.copytree(FIXTURES_DIR / "needs-migration", fixture_copy)

            payload = run_json_script("migrate_legacy_vault.py", str(fixture_copy))

            self.assertTrue((fixture_copy / "raw" / "articles" / "2026-03-20-old-pattern.md").exists())
            self.assertTrue((fixture_copy / "raw" / "human" / "articles" / "2026-03-20-old-pattern.md").exists())
            self.assertTrue((fixture_copy / "wiki" / "summaries" / "2026-03-20-old-pattern.md").exists())
            self.assertTrue((fixture_copy / "wiki" / "live" / "summaries" / "2026-03-20-old-pattern.md").exists())
            self.assertTrue((fixture_copy / payload["migration_report"]).exists())
            manifest_text = (fixture_copy / payload["written_manifest"]).read_text(encoding="utf-8")
            self.assertIn("raw/human/articles/2026-03-20-old-pattern.md", manifest_text)

            migrated_sources = [path.relative_to(fixture_copy).as_posix() for path in accepted_raw_sources(fixture_copy)]
            self.assertIn("raw/human/articles/2026-03-20-old-pattern.md", migrated_sources)
            self.assertNotIn("raw/articles/2026-03-20-old-pattern.md", migrated_sources)

    def test_vault_status_reports_stage_route_and_counts_for_key_states(self) -> None:
        expectations = {
            "needs-setup": "needs-setup",
            "needs-ingest": "needs-ingest",
            "needs-review": "needs-review",
            "needs-maintenance": "needs-maintenance",
            "ready-for-query": "ready-for-query",
        }

        for fixture_name, expected_state in expectations.items():
            with self.subTest(fixture=fixture_name):
                payload = run_json_script("vault_status.py", str(FIXTURES_DIR / fixture_name), "--format", "json")
                self.assertEqual(payload["state"], expected_state)
                self.assertIn("raw_sources", payload["counts"])
                self.assertIn("live_pages", payload["counts"])
                self.assertIn("Next step:", payload["summary"])


if __name__ == "__main__":
    unittest.main()
