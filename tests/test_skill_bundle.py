import json
import subprocess
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
ENTRY_SKILL_ROOT = REPO_ROOT / "skills" / "obsidian-notes-karpathy"
SCRIPTS_DIR = ENTRY_SKILL_ROOT / "scripts"
FIXTURES_DIR = ENTRY_SKILL_ROOT / "evals" / "fixtures"


def run_json_script(script_name: str, *args: str) -> dict:
    script_path = SCRIPTS_DIR / script_name
    result = subprocess.run(
        ["python3", str(script_path), *args],
        cwd=REPO_ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    return json.loads(result.stdout)


class SkillBundleContractTests(unittest.TestCase):
    def test_detect_lifecycle_routes_structural_states(self) -> None:
        fresh = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "fresh-vault"))
        partial = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "partial-vault"))
        compiled = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "compiled-vault"))
        drift = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "drift-vault"))

        self.assertEqual(fresh["route"], "kb-init")
        self.assertEqual(fresh["state"], "fresh")

        self.assertEqual(partial["route"], "kb-init")
        self.assertEqual(partial["state"], "partial")
        self.assertIn("missing_support_layer", partial["signals"])

        self.assertEqual(compiled["route"], "kb-compile")
        self.assertEqual(compiled["state"], "compile-ready")
        self.assertGreaterEqual(compiled["compile_delta"]["new_count"], 1)

        self.assertEqual(drift["route"], "kb-health")
        self.assertEqual(drift["state"], "health-first")
        self.assertTrue(drift["health_flags"])

    def test_scan_compile_delta_reports_new_changed_and_unchanged(self) -> None:
        compiled = run_json_script("scan_compile_delta.py", str(FIXTURES_DIR / "compiled-vault"))
        root_raw = run_json_script("scan_compile_delta.py", str(FIXTURES_DIR / "root-raw-vault"))

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

    def test_bundle_docs_and_trigger_evals_stay_consistent(self) -> None:
        claude_md = (REPO_ROOT / "CLAUDE.md").read_text()
        readme = (REPO_ROOT / "README.md").read_text()
        readme_cn = (REPO_ROOT / "README_CN.md").read_text()
        installation = (REPO_ROOT / "docs" / "guide" / "installation.md").read_text()
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

        self.assertTrue(trigger_eval_path.exists())
        trigger_evals = json.loads(trigger_eval_path.read_text())
        self.assertGreaterEqual(len(trigger_evals), 20)


if __name__ == "__main__":
    unittest.main()
