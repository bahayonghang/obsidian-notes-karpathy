import json
import os
import subprocess
import sys
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
ENTRY_SKILL_ROOT = REPO_ROOT / "skills" / "obsidian-notes-karpathy"
SCRIPTS_DIR = ENTRY_SKILL_ROOT / "scripts"
FIXTURES_DIR = ENTRY_SKILL_ROOT / "evals" / "fixtures"
sys.path.insert(0, str(SCRIPTS_DIR))

from _vault_utils import accepted_raw_sources, summarize_local_guidance


def run_json_script(script_name: str, *args: str, env: dict[str, str] | None = None) -> dict:
    script_path = SCRIPTS_DIR / script_name
    command_env = os.environ.copy()
    if env:
        command_env.update(env)
    result = subprocess.run(
        [sys.executable, str(script_path), *args],
        cwd=REPO_ROOT,
        check=True,
        capture_output=True,
        text=True,
        env=command_env,
    )
    return json.loads(result.stdout)


class SkillBundleContractTests(unittest.TestCase):
    def test_detect_lifecycle_routes_structural_states(self) -> None:
        fresh = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "fresh-vault"))
        partial = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "partial-vault"))
        compiled = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "compiled-vault"))
        drift = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "drift-vault"))
        pdf_only = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "pdf-only-vault"))
        missing_agents = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "missing-agents-vault"))
        root_raw = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "root-raw-vault"))

        self.assertEqual(fresh["route"], "kb-init")
        self.assertEqual(fresh["state"], "fresh")

        self.assertEqual(partial["route"], "kb-init")
        self.assertEqual(partial["state"], "partial")
        self.assertIn("missing_support_layer", partial["signals"])

        self.assertEqual(compiled["route"], "kb-compile")
        self.assertEqual(compiled["state"], "compile-ready")
        self.assertGreaterEqual(compiled["compile_delta"]["new_count"], 1)
        self.assertTrue(compiled["guidance_status"]["agents"]["present"])
        self.assertFalse(compiled["guidance_status"]["claude"]["present"])
        self.assertIn("missing_claude_guidance", compiled["guidance_warnings"])

        self.assertEqual(drift["route"], "kb-health")
        self.assertEqual(drift["state"], "health-first")
        self.assertTrue(drift["health_flags"])
        self.assertTrue(drift["guidance_status"]["agents"]["present"])
        self.assertIn("missing_claude_guidance", drift["guidance_warnings"])

        self.assertEqual(pdf_only["route"], "kb-compile")
        self.assertEqual(pdf_only["state"], "compile-ready")
        self.assertGreaterEqual(pdf_only["compile_delta"]["new_count"], 1)
        self.assertTrue(pdf_only["guidance_status"]["agents"]["present"])
        self.assertIn("missing_claude_guidance", pdf_only["guidance_warnings"])

        self.assertEqual(missing_agents["route"], "kb-init")
        self.assertEqual(missing_agents["state"], "partial")
        self.assertIn("missing_agents_guidance", missing_agents["signals"])
        self.assertIn("AGENTS.md", missing_agents["missing_support_files"])
        self.assertIn("missing_claude_guidance", missing_agents["guidance_warnings"])

        self.assertEqual(root_raw["route"], "kb-compile")
        self.assertEqual(root_raw["state"], "compile-ready")
        self.assertIn("missing_claude_guidance", root_raw["guidance_warnings"])

    def test_scan_compile_delta_reports_new_changed_and_unchanged(self) -> None:
        compiled = run_json_script("scan_compile_delta.py", str(FIXTURES_DIR / "compiled-vault"))
        root_raw = run_json_script("scan_compile_delta.py", str(FIXTURES_DIR / "root-raw-vault"))
        pdf_only = run_json_script("scan_compile_delta.py", str(FIXTURES_DIR / "pdf-only-vault"))

        self.assertEqual(compiled["counts"]["new"], 2)
        self.assertEqual(compiled["counts"]["changed"], 1)
        self.assertEqual(compiled["counts"]["unchanged"], 0)
        self.assertEqual(compiled["counts"]["skipped"], 0)

        compiled_paths = {item["path"] for item in compiled["items"]}
        self.assertIn(
            "raw/articles/2026-04-03-knowledge-compilers.md",
            compiled_paths,
        )
        self.assertIn("raw/repos/2026-04-05-qmd-local-search.md", compiled_paths)

        self.assertEqual(root_raw["counts"]["new"], 2)
        self.assertEqual(root_raw["counts"]["changed"], 0)

        self.assertEqual(pdf_only["counts"]["new"], 1)
        self.assertEqual(pdf_only["counts"]["changed"], 0)
        self.assertEqual(pdf_only["counts"]["unchanged"], 0)
        self.assertEqual(pdf_only["counts"]["skipped"], 0)
        self.assertEqual(pdf_only["items"][0]["path"], "raw/papers/2026-04-08-transformers.pdf")
        self.assertEqual(pdf_only["items"][0]["summary_path"], "wiki/summaries/2026-04-08-transformers.md")
        self.assertEqual(pdf_only["items"][0]["source_class"], "paper_pdf")
        self.assertIn(pdf_only["items"][0]["ingest_plan"], {"alphaxiv", "pdf", "skip"})

    def test_scan_compile_delta_builds_pdf_ingest_manifest(self) -> None:
        skill_home_root = FIXTURES_DIR / "companion-skill-homes"
        both_companions = run_json_script(
            "scan_compile_delta.py",
            str(FIXTURES_DIR / "pdf-only-vault"),
            env={"KB_COMPANION_SKILL_PATHS": str(skill_home_root / "both")},
        )
        pdf_fallback = run_json_script(
            "scan_compile_delta.py",
            str(FIXTURES_DIR / "pdf-no-handle-vault"),
            env={"KB_COMPANION_SKILL_PATHS": str(skill_home_root / "pdf-only")},
        )
        skipped_pdf = run_json_script(
            "scan_compile_delta.py",
            str(FIXTURES_DIR / "pdf-only-vault"),
            env={"KB_COMPANION_SKILL_PATHS": str(skill_home_root / "empty")},
        )

        alphaxiv_item = both_companions["items"][0]
        self.assertEqual(alphaxiv_item["ingest_plan"], "alphaxiv")
        self.assertEqual(alphaxiv_item["ingest_reason"], "paper_handle_available")
        self.assertEqual(alphaxiv_item["paper_handle"], "1706.03762")
        self.assertEqual(alphaxiv_item["paper_handle_source"], "paper_id")
        self.assertEqual(
            alphaxiv_item["metadata_path"],
            "raw/papers/2026-04-08-transformers.source.md",
        )
        self.assertTrue(both_companions["companion_skills"]["skills"]["alphaxiv-paper-lookup"])
        self.assertTrue(both_companions["companion_skills"]["skills"]["pdf"])

        pdf_item = pdf_fallback["items"][0]
        self.assertEqual(pdf_item["ingest_plan"], "pdf")
        self.assertEqual(pdf_item["ingest_reason"], "missing_paper_handle")
        self.assertIsNone(pdf_item["paper_handle"])
        self.assertFalse(pdf_fallback["companion_skills"]["skills"]["alphaxiv-paper-lookup"])
        self.assertTrue(pdf_fallback["companion_skills"]["skills"]["pdf"])

        skipped_item = skipped_pdf["items"][0]
        self.assertEqual(skipped_item["ingest_plan"], "skip")
        self.assertEqual(skipped_item["ingest_reason"], "missing_companion_skills")
        self.assertEqual(skipped_pdf["ingest_counts"]["skip"], 1)

    def test_accepted_raw_sources_skips_pdf_sidecars(self) -> None:
        pdf_only_sources = [
            path.relative_to(FIXTURES_DIR / "pdf-only-vault").as_posix()
            for path in accepted_raw_sources(FIXTURES_DIR / "pdf-only-vault")
        ]

        self.assertEqual(
            pdf_only_sources,
            ["raw/papers/2026-04-08-transformers.pdf"],
        )

    def test_lint_obsidian_mechanics_catches_render_and_alias_issues(self) -> None:
        broken = run_json_script("lint_obsidian_mechanics.py", str(FIXTURES_DIR / "broken-render-vault"))
        drift = run_json_script("lint_obsidian_mechanics.py", str(FIXTURES_DIR / "drift-vault"))

        alias_issues = [issue for issue in broken["issues"] if issue["kind"] == "alias_wikilink_in_table"]
        self.assertTrue(alias_issues)
        self.assertTrue(
            any(issue["path"] == "wiki/indices/SOURCES.md" for issue in alias_issues)
        )

        duplicate_concepts = [issue for issue in drift["issues"] if issue["kind"] == "duplicate_concept"]
        self.assertTrue(duplicate_concepts)
        self.assertTrue(
            any("retrieval-augmented-generation" in issue["normalized_key"] for issue in duplicate_concepts)
        )

    def test_guidance_contract_status_treats_noncanonical_names_as_warnings_but_duplicates_as_blocking(self) -> None:
        noncanonical = summarize_local_guidance(["agents.md", "CLAUDE.md"])
        self.assertTrue(noncanonical["agents"]["present"])
        self.assertFalse(noncanonical["agents"]["canonical"])
        self.assertIn("noncanonical_agents_guidance_name", noncanonical["warnings"])
        self.assertNotIn("missing_agents_guidance", noncanonical["blocking_issues"])

        duplicates = summarize_local_guidance(["AGENTS.md", "claude.md", "CLAUDE.md"])
        self.assertTrue(duplicates["claude"]["present"])
        self.assertIn("duplicate_claude_guidance_files", duplicates["blocking_issues"])
        self.assertIn("duplicate_claude_guidance_files", duplicates["warnings"])

    def test_bundle_docs_and_trigger_evals_stay_consistent(self) -> None:
        claude_md = (REPO_ROOT / "CLAUDE.md").read_text(encoding="utf-8")
        readme = (REPO_ROOT / "README.md").read_text(encoding="utf-8")
        readme_cn = (REPO_ROOT / "README_CN.md").read_text(encoding="utf-8")
        installation = (REPO_ROOT / "docs" / "guide" / "installation.md").read_text(encoding="utf-8")
        installation_cn = (REPO_ROOT / "docs" / "zh" / "guide" / "installation.md").read_text(encoding="utf-8")
        trigger_eval_path = ENTRY_SKILL_ROOT / "evals" / "trigger-evals.json"

        self.assertNotIn(
            "skills/obsidian-notes-karpathy/obsidian-notes-karpathy/",
            claude_md,
        )
        self.assertIn("skills/obsidian-notes-karpathy/SKILL.md", claude_md)

        self.assertIn("~/.claude/skills/", readme)
        self.assertIn("~/.codex/skills/", readme)
        self.assertIn("~/.claude/skills/", readme_cn)
        self.assertIn("~/.codex/skills/", readme_cn)
        self.assertIn("~/.codex/skills/", installation)
        self.assertIn("alphaxiv-paper-lookup", readme)
        self.assertIn("alphaxiv-paper-lookup", readme_cn)
        self.assertIn("alphaxiv-paper-lookup", installation)
        self.assertIn("alphaxiv-paper-lookup", installation_cn)
        self.assertIn("pdf", readme)
        self.assertIn("pdf", readme_cn)
        self.assertIn("pdf", installation)
        self.assertIn("pdf", installation_cn)

        self.assertTrue(trigger_eval_path.exists())
        trigger_evals = json.loads(trigger_eval_path.read_text(encoding="utf-8"))
        self.assertGreaterEqual(len(trigger_evals), 20)


if __name__ == "__main__":
    unittest.main()
