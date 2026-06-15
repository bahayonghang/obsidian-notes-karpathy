mod common;

use std::fs;

use assert_cmd::Command;
use common::run_json;
use tempfile::tempdir;

#[test]
fn skill_install_excludes_repo_only_eval_assets() {
    let temp = tempdir().expect("tempdir");
    let payload = run_json(&[
        "--json",
        "skill",
        "install",
        "--dir",
        temp.path().to_str().unwrap(),
        "--claude",
        "--codex",
    ]);

    assert_eq!(
        payload["claude"]["installed"]
            .as_array()
            .map(Vec::len)
            .unwrap_or_default(),
        7
    );
    assert_eq!(
        payload["codex"]["installed"]
            .as_array()
            .map(Vec::len)
            .unwrap_or_default(),
        7
    );

    for runtime_home in [
        temp.path().join(".claude").join("skills"),
        temp.path().join(".agents").join("skills"),
    ] {
        assert!(
            runtime_home
                .join("obsidian-notes-karpathy")
                .join("references")
                .exists()
        );
        assert!(
            runtime_home
                .join("obsidian-notes-karpathy")
                .join("scripts")
                .exists()
        );
        assert!(
            !runtime_home
                .join("obsidian-notes-karpathy")
                .join("evals")
                .exists()
        );
        assert!(!runtime_home.join("evals").exists());

        let installed_dirs = fs::read_dir(&runtime_home)
            .expect("read installed skill home")
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_dir())
            .count();
        assert_eq!(installed_dirs, 7);
    }
}

#[test]
fn skill_install_default_keeps_legacy_local_targets_and_reports_targets_array() {
    let temp = tempdir().expect("tempdir");
    let payload = run_json(&[
        "--json",
        "skill",
        "install",
        "--dir",
        temp.path().to_str().unwrap(),
    ]);

    let targets = payload["targets"].as_array().expect("targets");
    assert_eq!(targets.len(), 2);
    assert_eq!(targets[0]["key"].as_str(), Some("claude"));
    assert_eq!(targets[0]["scope"].as_str(), Some("local"));
    assert_eq!(targets[1]["key"].as_str(), Some("codex"));
    assert_eq!(targets[1]["scope"].as_str(), Some("local"));
    assert!(payload["claude"].is_object());
    assert!(payload["codex"].is_object());
    assert!(temp.path().join(".claude").join("skills").exists());
    assert!(temp.path().join(".agents").join("skills").exists());
}

#[test]
fn skill_install_explicit_local_targets_install_only_selected_targets() {
    let temp = tempdir().expect("tempdir");
    let payload = run_json(&[
        "--json",
        "skill",
        "install",
        "--dir",
        temp.path().to_str().unwrap(),
        "--cursor",
        "--windsurf",
        "--kiro",
        "--pi",
    ]);

    let keys = payload["targets"]
        .as_array()
        .expect("targets")
        .iter()
        .map(|target| target["key"].as_str().unwrap_or_default())
        .collect::<Vec<_>>();
    assert_eq!(keys, vec!["cursor", "windsurf", "kiro", "pi"]);
    assert!(temp.path().join(".cursor").join("skills").exists());
    assert!(temp.path().join(".windsurf").join("skills").exists());
    assert!(temp.path().join(".kiro").join("skills").exists());
    assert!(temp.path().join(".pi").join("skills").exists());
    assert!(!temp.path().join(".claude").join("skills").exists());
    assert!(!temp.path().join(".agents").join("skills").exists());
    assert!(payload["claude"].is_null());
    assert!(payload["codex"].is_null());
}

#[test]
fn skill_install_all_local_installs_every_local_target() {
    let temp = tempdir().expect("tempdir");
    let payload = run_json(&[
        "--json",
        "skill",
        "install",
        "--dir",
        temp.path().to_str().unwrap(),
        "--all-local",
    ]);

    let keys = payload["targets"]
        .as_array()
        .expect("targets")
        .iter()
        .map(|target| target["key"].as_str().unwrap_or_default())
        .collect::<Vec<_>>();
    assert_eq!(
        keys,
        vec!["claude", "codex", "cursor", "windsurf", "kiro", "pi"]
    );
    for path in [
        ".claude/skills",
        ".agents/skills",
        ".cursor/skills",
        ".windsurf/skills",
        ".kiro/skills",
        ".pi/skills",
    ] {
        assert!(temp.path().join(path).exists(), "missing {path}");
    }
}

#[test]
fn skill_install_skips_existing_targets_without_overwrite() {
    let temp = tempdir().expect("tempdir");
    let first = run_json(&[
        "--json",
        "skill",
        "install",
        "--dir",
        temp.path().to_str().unwrap(),
        "--claude",
    ]);
    assert_eq!(
        first["targets"][0]["installed"]
            .as_array()
            .map(Vec::len)
            .unwrap_or_default(),
        7
    );

    let second = run_json(&[
        "--json",
        "skill",
        "install",
        "--dir",
        temp.path().to_str().unwrap(),
        "--claude",
    ]);
    assert_eq!(
        second["targets"][0]["installed"]
            .as_array()
            .map(Vec::len)
            .unwrap_or_default(),
        0
    );
    assert_eq!(
        second["targets"][0]["skipped"]
            .as_array()
            .map(Vec::len)
            .unwrap_or_default(),
        7
    );
}

#[test]
fn skill_install_overwrite_replaces_existing_targets() {
    let temp = tempdir().expect("tempdir");
    run_json(&[
        "--json",
        "skill",
        "install",
        "--dir",
        temp.path().to_str().unwrap(),
        "--claude",
    ]);
    let marker = temp
        .path()
        .join(".claude")
        .join("skills")
        .join("kb-query")
        .join("marker.txt");
    fs::write(&marker, "stale").expect("write marker");

    let payload = run_json(&[
        "--json",
        "skill",
        "install",
        "--dir",
        temp.path().to_str().unwrap(),
        "--claude",
        "--overwrite",
    ]);

    assert_eq!(
        payload["targets"][0]["installed"]
            .as_array()
            .map(Vec::len)
            .unwrap_or_default(),
        7
    );
    assert!(!marker.exists());
}

#[test]
fn skill_install_rejects_global_with_dir_to_avoid_ambiguous_writes() {
    let temp = tempdir().expect("tempdir");
    let output = Command::cargo_bin("onkb")
        .expect("binary")
        .args([
            "--json",
            "skill",
            "install",
            "--global",
            "--dir",
            temp.path().to_str().unwrap(),
            "--claude",
        ])
        .assert()
        .failure()
        .get_output()
        .clone();
    let stderr = String::from_utf8(output.stderr).expect("stderr utf8");
    assert!(stderr.contains("--dir cannot be combined with --global"));
}
