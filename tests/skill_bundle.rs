mod common;

use std::fs;

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
