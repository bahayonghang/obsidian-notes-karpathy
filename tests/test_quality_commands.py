# pyright: reportMissingImports=false
import json
import unittest

from _bundle_test_support import REPO_ROOT, SCRIPTS_DIR, run_repo_command
from _vault_common import parse_simple_yaml
from runtime_eval import build_prompt, classify_failure, detect_root_leakage, fallback_runner_for, validate_manifest
from trigger_eval import load_skill_catalog, parse_selected_skill, summarize_trigger_results


class QualityCommandTests(unittest.TestCase):
    def test_validate_bundle_contract_script_passes(self) -> None:
        result = run_repo_command("py", "-3", str(SCRIPTS_DIR / "validate_bundle_contract.py"), "--json")
        payload = json.loads(result.stdout)
        self.assertEqual(payload["status"], "ok")
        self.assertGreaterEqual(payload["checks"], 10)

    def test_runtime_eval_supports_dry_run_without_execution(self) -> None:
        result = run_repo_command(
            "py",
            "-3",
            str(SCRIPTS_DIR / "runtime_eval.py"),
            "--dry-run",
            "--runner",
            "codex",
            "--workspace",
            str(REPO_ROOT / ".runtime-eval-test"),
        )
        payload = json.loads(result.stdout)
        self.assertIn(payload["status"], {"planned", "skipped"})
        self.assertGreaterEqual(payload["eval_count"], 10)

    def test_runtime_eval_classifies_infra_failures(self) -> None:
        self.assertEqual(
            classify_failure(1, "", "ERROR: stream disconnected before completion"),
            "infra_failure",
        )
        self.assertEqual(
            classify_failure(124, "", "Runtime eval attempt timed out after 60 seconds."),
            "infra_failure",
        )
        self.assertEqual(
            classify_failure(1, "", "profile.ps1: Cannot dot-source this command"),
            "infra_failure",
        )
        self.assertEqual(classify_failure(1, "", "ordinary tool failure"), "runner_failure")
        self.assertEqual(classify_failure(0, "ok", ""), "ok")

    def test_runtime_eval_prompt_includes_windows_safety_guidance(self) -> None:
        prompt = build_prompt(
            use_skill=True,
            skill="kb-init",
            prompt="Explain the next step.",
            files=["evals/fixtures/needs-setup/README.md"],
            vault_root="evals/fixtures/needs-setup",
            mode="read-only",
        )
        self.assertIn("On Windows, avoid shell globs", prompt)
        self.assertIn("Read and follow the skill", prompt)
        self.assertIn("only target vault root", prompt)

    def test_runtime_eval_prompt_respects_writable_copy_mode(self) -> None:
        prompt = build_prompt(
            use_skill=False,
            skill="kb-init",
            prompt="Repair the copied vault.",
            files=[".runtime-evals/kb-init/README.md"],
            vault_root=".runtime-evals/kb-init/target",
            mode="writable-copy",
        )
        self.assertIn("modify files only under the declared vault root", prompt)
        self.assertNotIn("Work in read-only mode", prompt)

    def test_runtime_eval_prefers_claude_fallback_for_codex(self) -> None:
        fallback = fallback_runner_for("codex")
        self.assertIn(fallback, {None, "claude"})
        self.assertIsNone(fallback_runner_for("claude"))

    def test_reference_block_renderer_uses_registry_driven_shared_refs(self) -> None:
        result = run_repo_command(
            "py",
            "-3",
            str(SCRIPTS_DIR / "render_reference_block.py"),
            "kb-init",
        )
        self.assertIn("../obsidian-notes-karpathy/scripts/skill-contract-registry.json", result.stdout)
        self.assertIn("../obsidian-notes-karpathy/references/source-manifest-contract.md", result.stdout)

    def test_runtime_eval_detects_repo_root_leakage(self) -> None:
        leakage = detect_root_leakage(
            f"The current workspace root is {REPO_ROOT.as_posix()} and AGENTS.md lives there.",
            REPO_ROOT / "skills" / "obsidian-notes-karpathy" / "evals" / "fixtures" / "needs-setup",
        )
        self.assertTrue(leakage["detected"])
        self.assertIn("repo_root_path_mentioned", leakage["reasons"])

    def test_runtime_eval_ignores_repo_root_inside_markdown_link_targets(self) -> None:
        leakage = detect_root_leakage(
            f"Used [report]({REPO_ROOT.as_posix()}/outputs/health/report.md) to summarize the result.",
            REPO_ROOT / "skills" / "obsidian-notes-karpathy" / "evals" / "fixtures" / "needs-setup",
        )
        self.assertFalse(leakage["detected"])

    def test_runtime_eval_manifest_validation_rejects_mixed_fixture_roots(self) -> None:
        manifest = {
            "evals": [
                {
                    "id": "mixed-roots",
                    "skill": "kb-init",
                    "prompt": "Inspect the fixture vault.",
                    "vault_root": "evals/fixtures/needs-setup",
                    "files": [
                        "evals/fixtures/needs-setup/README.md",
                        "evals/fixtures/needs-review/AGENTS.md",
                    ],
                }
            ]
        }
        errors = validate_manifest(manifest)
        self.assertTrue(errors)
        self.assertTrue(any("files must belong to exactly one fixture root" in error for error in errors))

    def test_trigger_eval_loads_catalog_and_parses_json_output(self) -> None:
        catalog = load_skill_catalog()
        self.assertIn("kb-init", catalog)
        self.assertIsNone(parse_selected_skill('{"selected_skill":"none","reason":"outside scope"}'))
        self.assertEqual(
            parse_selected_skill('{"selected_skill":"kb-query","reason":"live-grounded request"}'),
            "kb-query",
        )

    def test_trigger_eval_summary_reports_precision_and_recall(self) -> None:
        summary = summarize_trigger_results(
            [
                {"expected_skill": "kb-init", "actual_skill": "kb-init", "matched": True},
                {"expected_skill": "kb-init", "actual_skill": "kb-query", "matched": False},
                {"expected_skill": None, "actual_skill": "kb-query", "matched": False},
            ],
            ["kb-init", "kb-query"],
        )
        self.assertEqual(summary["matched"], 1)
        self.assertEqual(summary["per_skill"]["kb-init"]["true_positive"], 1)
        self.assertEqual(summary["per_skill"]["kb-query"]["false_positive"], 2)

    def test_parse_simple_yaml_handles_empty_values_and_lists(self) -> None:
        parsed = parse_simple_yaml(
            """title: Test Note\naliases:\n  - alpha\n  - beta\nempty_list: []\nowner:\n"""
        )
        self.assertEqual(parsed["title"], "Test Note")
        self.assertEqual(parsed["aliases"], ["alpha", "beta"])
        self.assertEqual(parsed["empty_list"], [])
        self.assertEqual(parsed["owner"], [])

    def test_parse_simple_yaml_preserves_colons_and_quoted_special_characters(self) -> None:
        parsed = parse_simple_yaml(
            """note: keep:the:rest\nurl: \"https://example.com/a:b\"\nlabel: 'RAG vs. wiki: tradeoffs'\nitems: [one, two, three]\n"""
        )
        self.assertEqual(parsed["note"], "keep:the:rest")
        self.assertEqual(parsed["url"], "https://example.com/a:b")
        self.assertEqual(parsed["label"], "RAG vs. wiki: tradeoffs")
        self.assertEqual(parsed["items"], ["one", "two", "three"])

    def test_parse_simple_yaml_ignores_comments_and_invalid_lines(self) -> None:
        parsed = parse_simple_yaml(
            """# comment\ntitle: Valid\nthis line has no separator\ncount: 3\n"""
        )
        self.assertEqual(parsed, {"title": "Valid", "count": "3"})


if __name__ == "__main__":
    unittest.main()
