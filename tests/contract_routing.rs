mod common;

use common::{entry_skill_root, fixtures_root, read_text, run_json};
use onkb::dev::{
    load_registry,
    reference_blocks::{
        build_on_demand_bullets, build_shared_reference_bullets, extract_on_demand_bullets,
    },
};
use onkb::guidance::summarize_local_guidance;

#[test]
fn status_routes_key_lifecycle_states() {
    let cases = [
        ("needs-compilation", "needs-compilation", "kb-compile"),
        ("needs-setup", "needs-setup", "kb-init"),
        ("needs-maintenance", "needs-maintenance", "kb-review"),
        ("needs-review", "needs-review", "kb-review"),
        ("needs-ingest", "needs-ingest", "kb-ingest"),
        ("ready-for-query", "needs-maintenance", "kb-review"),
        ("needs-migration", "needs-migration", "kb-init"),
    ];

    for (fixture, expected_state, expected_route) in cases {
        let payload = run_json(&[
            "--json",
            "status",
            fixtures_root().join(fixture).to_str().unwrap(),
        ]);
        assert_eq!(payload["state"].as_str(), Some(expected_state));
        assert_eq!(payload["route"].as_str(), Some(expected_route));
    }
}

#[test]
fn guidance_contract_status_treats_noncanonical_names_as_warnings() {
    let noncanonical =
        summarize_local_guidance(&["agents.md".to_string(), "CLAUDE.md".to_string()]);
    assert_eq!(noncanonical["agents"]["present"].as_bool(), Some(true));
    assert_eq!(noncanonical["agents"]["canonical"].as_bool(), Some(false));
    assert!(
        noncanonical["warnings"]
            .as_array()
            .expect("warnings")
            .iter()
            .any(|item| item.as_str() == Some("noncanonical_agents_guidance_name"))
    );
    assert!(
        !noncanonical["blocking_issues"]
            .as_array()
            .expect("blocking issues")
            .iter()
            .any(|item| item.as_str() == Some("missing_agents_guidance"))
    );

    let duplicates = summarize_local_guidance(&[
        "AGENTS.md".to_string(),
        "claude.md".to_string(),
        "CLAUDE.md".to_string(),
    ]);
    assert_eq!(duplicates["claude"]["present"].as_bool(), Some(true));
    assert!(
        duplicates["blocking_issues"]
            .as_array()
            .expect("blocking issues")
            .iter()
            .any(|item| item.as_str() == Some("duplicate_claude_guidance_files"))
    );
}

#[test]
fn registry_matches_skill_docs_and_shared_reference_blocks() {
    let repo_root = common::repo_root();
    let registry = load_registry(&repo_root).expect("registry");
    assert_eq!(registry.contract_family, "review-gated");
    assert!(
        registry
            .skills
            .get("kb-init")
            .expect("kb-init")
            .writes
            .contains(&"MEMORY.md".to_string())
    );
    assert!(
        registry
            .skills
            .get("kb-review")
            .expect("kb-review")
            .writes
            .contains(&"wiki/live/procedures/".to_string())
    );

    for (skill_name, skill_path) in onkb::dev::skill_paths(&repo_root).expect("skill paths") {
        let skill_text = read_text(&skill_path);
        let referenced_paths = onkb::dev::reference_blocks::extract_reference_bullets(&skill_text);
        let expected_bullets =
            build_shared_reference_bullets(&skill_name, &registry).expect("bullets");
        for bullet in expected_bullets {
            let target = bullet.trim_start_matches("- `").trim_end_matches('`');
            assert!(referenced_paths.contains(&target.to_string()));
        }
        let baseline_command = &registry
            .skills
            .get(&skill_name)
            .expect("registry skill")
            .baseline_command;
        assert!(skill_text.contains(baseline_command));
    }

    let router_skill = read_text(&entry_skill_root().join("SKILL.md"));
    assert!(router_skill.contains("creator knowledge compiler"));
}

#[test]
fn registry_layers_reads_into_core_and_on_demand() {
    let repo_root = common::repo_root();
    let registry = load_registry(&repo_root).expect("registry");
    let references_root = entry_skill_root().join("references");

    for (skill_name, entry) in &registry.skills {
        assert!(
            entry.reads.len() <= 6,
            "{skill_name}: unconditional reads cap is 6, got {}",
            entry.reads.len()
        );
        let core: std::collections::BTreeSet<_> = entry.reads.iter().collect();
        for on_demand in &entry.reads_on_demand {
            assert!(
                !core.contains(&on_demand.file),
                "{skill_name}: {} is in both reads and reads_on_demand",
                on_demand.file
            );
            assert!(
                references_root.join(&on_demand.file).exists(),
                "{skill_name}: on-demand read missing on disk: {}",
                on_demand.file
            );
            assert!(
                !on_demand.when.trim().is_empty(),
                "{skill_name}: {} has an empty trigger condition",
                on_demand.file
            );
        }
    }

    // SKILL.md 的 Load on demand 小节与 registry 期望逐条一致（round-trip）。
    for (skill_name, skill_path) in onkb::dev::skill_paths(&repo_root).expect("skill paths") {
        let skill_text = read_text(&skill_path);
        let actual = extract_on_demand_bullets(&skill_text);
        let expected = build_on_demand_bullets(&skill_name, &registry).expect("bullets");
        for bullet in &expected {
            assert!(
                actual.contains(bullet),
                "{skill_name}: missing on-demand bullet {bullet}"
            );
        }
        if !expected.is_empty() {
            assert!(
                skill_text.contains("## Load on demand"),
                "{skill_name}: missing Load on demand section"
            );
        }
    }
}
