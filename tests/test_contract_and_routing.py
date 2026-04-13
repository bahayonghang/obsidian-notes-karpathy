# pyright: reportMissingImports=false
import unittest

from _bundle_test_support import (
    FIXTURES_DIR,
    SKILL_PATHS,
    build_shared_reference_bullets,
    extract_reference_bullets,
    load_registry,
    run_json_script,
    summarize_local_guidance,
)


class ContractAndRoutingTests(unittest.TestCase):
    def test_detect_lifecycle_routes_semantic_states(self) -> None:
        compile_ready = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "needs-compilation"))
        fresh = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "needs-setup"))
        maintenance = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "needs-maintenance"))
        incomplete_live = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "needs-repair-live-only"))
        missing_agents = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "missing-contract-guidance"))
        partial = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "needs-repair"))
        review_ready = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "needs-review"))
        root_raw = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "bootstrap-raw-intake"))
        live_ready = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "ready-for-query"))
        briefing_refresh = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "needs-briefing-refresh"))
        migration = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "needs-migration"))
        needs_ingest = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "needs-ingest"))
        confidence_decay = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "confidence-decay"))
        supersession_chain = run_json_script("detect_lifecycle.py", str(FIXTURES_DIR / "supersession-chain"))

        self.assertEqual(compile_ready["state"], "needs-compilation")
        self.assertEqual(compile_ready["route"], "kb-compile")
        self.assertIn("new_raw_sources", compile_ready["signals"])

        self.assertEqual(fresh["state"], "needs-setup")
        self.assertEqual(fresh["route"], "kb-init")
        self.assertEqual(fresh["layout_family"], "uninitialized")

        self.assertEqual(maintenance["state"], "needs-maintenance")
        self.assertEqual(maintenance["route"], "kb-review")
        self.assertEqual(maintenance["route_mode"], "maintenance")
        self.assertIn("maintenance_needed", maintenance["signals"])
        self.assertIn("stale_qa", maintenance["health_flags"])

        self.assertEqual(incomplete_live["state"], "needs-repair")
        self.assertEqual(incomplete_live["route"], "kb-init")
        self.assertIn("missing_review_gate_support", incomplete_live["signals"])
        self.assertCountEqual(
            incomplete_live["missing_support_files"],
            ["outputs/reviews", "wiki/briefings", "wiki/drafts"],
        )

        self.assertEqual(missing_agents["state"], "needs-repair")
        self.assertEqual(missing_agents["route"], "kb-init")
        self.assertIn("AGENTS.md", missing_agents["missing_support_files"])

        self.assertEqual(partial["state"], "needs-repair")
        self.assertEqual(partial["route"], "kb-init")
        self.assertIn("missing_support_layer", partial["signals"])

        self.assertEqual(review_ready["layout_family"], "review-gated")
        self.assertEqual(review_ready["state"], "needs-review")
        self.assertEqual(review_ready["route"], "kb-review")
        self.assertEqual(review_ready["route_mode"], "gate")
        self.assertIn("drafts_pending_review", review_ready["signals"])

        self.assertEqual(root_raw["state"], "needs-compilation")
        self.assertEqual(root_raw["route"], "kb-compile")
        self.assertIn("missing_claude_guidance", root_raw["guidance_warnings"])

        self.assertEqual(needs_ingest["state"], "needs-ingest")
        self.assertEqual(needs_ingest["route"], "kb-ingest")
        self.assertIn("new_sources_not_registered", needs_ingest["signals"])

        self.assertEqual(live_ready["state"], "ready-for-query")
        self.assertEqual(live_ready["route"], "kb-query")

        self.assertEqual(briefing_refresh["state"], "needs-briefing-refresh")
        self.assertEqual(briefing_refresh["route"], "kb-review")
        self.assertEqual(briefing_refresh["route_mode"], "gate")
        self.assertIn("briefing_refresh_required", briefing_refresh["signals"])

        self.assertEqual(confidence_decay["state"], "needs-maintenance")
        self.assertEqual(confidence_decay["route"], "kb-review")
        self.assertEqual(confidence_decay["route_mode"], "maintenance")
        self.assertIn("missing_confidence_metadata", confidence_decay["health_flags"])
        self.assertIn("confidence_decay_due", confidence_decay["health_flags"])

        self.assertEqual(supersession_chain["state"], "needs-maintenance")
        self.assertEqual(supersession_chain["route"], "kb-review")
        self.assertEqual(supersession_chain["route_mode"], "maintenance")
        self.assertIn("supersession_gap", supersession_chain["health_flags"])

        self.assertEqual(migration["layout_family"], "legacy-layout")
        self.assertEqual(migration["state"], "needs-migration")
        self.assertEqual(migration["route"], "kb-init")
        self.assertIn("legacy_layout_detected", migration["signals"])

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

    def test_skill_contract_registry_matches_skill_docs(self) -> None:
        registry = load_registry()
        self.assertEqual(registry["contract_family"], "review-gated")
        self.assertEqual(sorted(registry["skills"].keys()), sorted(SKILL_PATHS.keys()))
        self.assertIn("MEMORY.md", registry["skills"]["kb-init"]["writes"])
        self.assertIn("outputs/episodes/", registry["skills"]["kb-init"]["writes"])
        self.assertIn("raw/_manifest.yaml", registry["skills"]["kb-ingest"]["writes"])
        self.assertIn("wiki/live/procedures/", registry["skills"]["kb-review"]["writes"])
        self.assertIn("wiki/live/topics/", registry["skills"]["kb-review"]["writes"])
        self.assertIn(
            "wiki/live/indices/ALIASES.md",
            registry["skills"]["kb-review"]["writes"],
        )
        self.assertIn("wiki/live/indices/RELATIONSHIPS.md", registry["skills"]["kb-review"]["writes"])
        self.assertIn("outputs/health/graph-snapshot.json", registry["skills"]["kb-review"]["outputs"])
        self.assertIn("wiki/live/indices/", registry["skills"]["kb-review"]["outputs"])

        shared_refs = set(registry["shared_references"])
        for reference in shared_refs:
            self.assertTrue((SKILL_PATHS["obsidian-notes-karpathy"].parent / "references" / reference).exists(), reference)

        for skill_name, skill_path in SKILL_PATHS.items():
            skill_text = skill_path.read_text(encoding="utf-8")
            referenced_paths = extract_reference_bullets(skill_text)
            expected = registry["skills"][skill_name]

            if skill_name == "obsidian-notes-karpathy":
                expected_bullets = build_shared_reference_bullets(skill_name, registry)
            else:
                expected_bullets = build_shared_reference_bullets(skill_name, registry)
            for bullet in expected_bullets:
                self.assertIn(bullet.removeprefix("- `").removesuffix("`"), referenced_paths)

            self.assertIn(expected["baseline_script"], skill_text)


if __name__ == "__main__":
    unittest.main()
