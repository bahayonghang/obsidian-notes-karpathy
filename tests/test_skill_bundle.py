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
    def test_detect_lifecycle_routes_v2_and_legacy_states(self) -> None:
        fresh = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "v2-fresh"))
        partial = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "partial-vault"))
        review_ready = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "v2-review-ready"))
        live_ready = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "v2-live-ready"))
        briefing_stale = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "v2-briefing-stale"))
        legacy = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "v1-legacy"))

        self.assertEqual(fresh["state"], "fresh")
        self.assertEqual(fresh["route"], "kb-init")
        self.assertEqual(fresh["contract_version"], "v2")

        self.assertEqual(partial["state"], "partial")
        self.assertEqual(partial["route"], "kb-init")
        self.assertIn("missing_support_layer", partial["signals"])

        self.assertEqual(review_ready["contract_version"], "v2")
        self.assertEqual(review_ready["state"], "review-ready")
        self.assertEqual(review_ready["route"], "kb-review")
        self.assertIn("drafts_pending_review", review_ready["signals"])

        self.assertEqual(live_ready["state"], "query-ready")
        self.assertEqual(live_ready["route"], "kb-query")

        self.assertEqual(briefing_stale["state"], "briefing-stale")
        self.assertEqual(briefing_stale["route"], "kb-review")
        self.assertIn("briefing_stale", briefing_stale["signals"])

        self.assertEqual(legacy["contract_version"], "v1")
        self.assertEqual(legacy["state"], "legacy-v1")
        self.assertEqual(legacy["route"], "kb-init")
        self.assertIn("legacy_v1_contract", legacy["signals"])

    def test_scan_compile_delta_reports_v2_capture_trust(self) -> None:
        review_ready = run_json_script("scan_compile_delta.py", str(FIXTURES_DIR / "v2-review-ready"))

        self.assertEqual(review_ready["contract_version"], "v2")
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
            str(FIXTURES_DIR / "pdf-only-vault"),
            env={"KB_COMPANION_SKILL_PATHS": str(skill_home_root / "both")},
        )

        self.assertEqual(pdf_only["contract_version"], "v1")
        self.assertEqual(pdf_only["items"][0]["ingest_plan"], "paper-workbench")
        self.assertEqual(pdf_only["items"][0]["ingest_reason"], "paper_workbench_directory_policy")
        self.assertEqual(pdf_only["items"][0]["paper_handle"], "1706.03762")
        self.assertEqual(
            pdf_only["items"][0]["metadata_path"],
            "raw/papers/2026-04-08-transformers.source.md",
        )

    def test_scan_review_queue_applies_mixed_gate(self) -> None:
        queue = run_json_script("scan_review_queue.py", str(FIXTURES_DIR / "v2-conflict-review"))

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

    def test_query_scope_ignores_raw_and_drafts_for_v2(self) -> None:
        scope = run_json_script("scan_query_scope.py", str(FIXTURES_DIR / "v2-live-ready"))

        self.assertEqual(scope["contract_version"], "v2")
        self.assertIn("wiki/live/summaries/human/articles/2026-04-05-approved-summary.md", scope["included_paths"])
        self.assertIn("wiki/briefings/researcher.md", scope["included_paths"])
        self.assertIn("outputs/qa/2026-04-06-review-gate-benefits.md", scope["included_paths"])
        self.assertIn("raw/human/articles/2026-04-05-approved-summary.md", scope["excluded_paths"])
        self.assertIn("wiki/drafts/summaries/human/articles/2026-04-05-approved-summary.md", scope["excluded_paths"])

    def test_lint_obsidian_mechanics_flags_v2_staleness_and_backlog(self) -> None:
        review_ready = run_json_script("lint_obsidian_mechanics.py", str(FIXTURES_DIR / "v2-review-ready"))
        briefing_stale = run_json_script("lint_obsidian_mechanics.py", str(FIXTURES_DIR / "v2-briefing-stale"))

        review_issue_kinds = {issue["kind"] for issue in review_ready["issues"]}
        stale_issue_kinds = {issue["kind"] for issue in briefing_stale["issues"]}

        self.assertIn("review_backlog", review_issue_kinds)
        self.assertIn("stale_briefing", stale_issue_kinds)

    def test_accepted_raw_sources_supports_v2_and_skips_pdf_sidecars(self) -> None:
        v2_sources = [
            path.relative_to(FIXTURES_DIR / "v2-review-ready").as_posix()
            for path in accepted_raw_sources(FIXTURES_DIR / "v2-review-ready")
        ]
        pdf_sources = [
            path.relative_to(FIXTURES_DIR / "pdf-only-vault").as_posix()
            for path in accepted_raw_sources(FIXTURES_DIR / "pdf-only-vault")
        ]

        self.assertEqual(
            v2_sources,
            [
                "raw/agents/researcher/2026-04-02-review-gates.md",
                "raw/human/articles/2026-04-01-compiler-safety.md",
            ],
        )
        self.assertEqual(pdf_sources, ["raw/papers/2026-04-08-transformers.pdf"])

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
        entry_skill = (ENTRY_SKILL_ROOT / "SKILL.md").read_text(encoding="utf-8")
        compile_skill = (REPO_ROOT / "skills" / "kb-compile" / "SKILL.md").read_text(encoding="utf-8")
        review_skill = (REPO_ROOT / "skills" / "kb-review" / "SKILL.md").read_text(encoding="utf-8")
        evals_text = (ENTRY_SKILL_ROOT / "evals" / "evals.json").read_text(encoding="utf-8")
        trigger_eval_path = ENTRY_SKILL_ROOT / "evals" / "trigger-evals.json"

        for text in (claude_md, readme, readme_cn, entry_skill, compile_skill, review_skill, evals_text):
            self.assertIn("wiki/drafts", text)
            self.assertIn("wiki/live", text)
            self.assertIn("wiki/briefings", text)
            self.assertIn("outputs/reviews", text)

        self.assertIn("kb-review", readme)
        self.assertIn("kb-review", readme_cn)
        self.assertIn("kb-review", entry_skill)
        self.assertTrue(trigger_eval_path.exists())
        trigger_evals = json.loads(trigger_eval_path.read_text(encoding="utf-8"))
        self.assertGreaterEqual(len(trigger_evals), 20)


if __name__ == "__main__":
    unittest.main()
