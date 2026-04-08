# pyright: reportMissingImports=false
import json
import unittest

from _bundle_test_support import REPO_ROOT, SCRIPTS_DIR, run_repo_command
from runtime_eval import build_prompt, classify_failure, fallback_runner_for


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
        )
        self.assertIn("On Windows, avoid shell globs", prompt)
        self.assertIn("Read and follow the skill", prompt)

    def test_runtime_eval_prefers_claude_fallback_for_codex(self) -> None:
        fallback = fallback_runner_for("codex")
        self.assertIn(fallback, {None, "claude"})
        self.assertIsNone(fallback_runner_for("claude"))


if __name__ == "__main__":
    unittest.main()
