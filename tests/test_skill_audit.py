# pyright: reportMissingImports=false
import json
import unittest

from _bundle_test_support import REPO_ROOT, run_repo_command


class SkillAuditTests(unittest.TestCase):
    def test_skill_audit_reports_full_runtime_and_trigger_coverage(self) -> None:
        result = run_repo_command("py", "-3", "skills/obsidian-notes-karpathy/scripts/audit_skills.py", "--json")
        payload = json.loads(result.stdout)

        self.assertEqual(payload["status"], "ok")
        self.assertEqual(payload["summary"]["skill_count"], 7)
        self.assertEqual(payload["summary"]["trigger_eval_covered"], 7)
        self.assertEqual(payload["summary"]["runtime_eval_covered"], 7)
        self.assertEqual(payload["summary"]["blocking_issue_count"], 0)
        self.assertGreaterEqual(payload["summary"]["average_score"], 0.9)

        skills = {item["name"]: item for item in payload["skills"]}
        self.assertIn("obsidian-notes-karpathy", skills)
        self.assertTrue(skills["obsidian-notes-karpathy"]["checks"]["description_has_multilingual_trigger"])
        self.assertTrue(skills["obsidian-notes-karpathy"]["checks"]["runtime_eval_covered"])
        self.assertTrue(skills["kb-init"]["checks"]["writable_runtime_covered"])
        self.assertTrue(skills["kb-ingest"]["checks"]["writable_runtime_covered"])
        self.assertTrue(skills["kb-review"]["checks"]["writable_runtime_covered"])
        self.assertTrue(skills["kb-render"]["checks"]["writable_runtime_covered"])

    def test_runtime_eval_manifest_now_includes_router_skill(self) -> None:
        result = run_repo_command(
            "py",
            "-3",
            "skills/obsidian-notes-karpathy/scripts/runtime_eval.py",
            "--dry-run",
            "--runner",
            "codex",
            "--workspace",
            str(REPO_ROOT / ".runtime-eval-skill-audit"),
        )
        payload = json.loads(result.stdout)

        self.assertIn("obsidian-notes-karpathy", payload["skills"])
        self.assertGreaterEqual(payload["eval_count"], 17)


if __name__ == "__main__":
    unittest.main()
