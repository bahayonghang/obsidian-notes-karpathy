#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use serde_json::Value;
use walkdir::WalkDir;

pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn entry_skill_root() -> PathBuf {
    repo_root().join("skills").join("obsidian-notes-karpathy")
}

pub fn fixtures_root() -> PathBuf {
    entry_skill_root().join("evals").join("fixtures")
}

pub fn run_json(args: &[&str]) -> Value {
    let output = Command::cargo_bin("onkb")
        .expect("binary")
        .args(args)
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    serde_json::from_slice(&output).expect("valid json")
}

pub fn run_json_in_dir(args: &[&str], cwd: &Path) -> Value {
    let output = Command::cargo_bin("onkb")
        .expect("binary")
        .current_dir(cwd)
        .args(args)
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    serde_json::from_slice(&output).expect("valid json")
}

pub fn run_json_with_env(args: &[&str], cwd: &Path, envs: &[(&str, &str)]) -> Value {
    let mut command = Command::cargo_bin("onkb").expect("binary");
    command.current_dir(cwd).args(args);
    for (key, value) in envs {
        command.env(key, value);
    }
    let output = command.assert().success().get_output().stdout.clone();
    serde_json::from_slice(&output).expect("valid json")
}

pub fn run_stdout(args: &[&str]) -> String {
    String::from_utf8(
        Command::cargo_bin("onkb")
            .expect("binary")
            .args(args)
            .assert()
            .success()
            .get_output()
            .stdout
            .clone(),
    )
    .expect("utf8 stdout")
}

pub fn copy_dir(source: &Path, target: &Path) {
    for entry in WalkDir::new(source) {
        let entry = entry.expect("walkdir");
        let rel = entry.path().strip_prefix(source).expect("relative");
        let dest = target.join(rel);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&dest).expect("create dir");
        } else {
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent).expect("create parent");
            }
            fs::copy(entry.path(), &dest).expect("copy file");
        }
    }
}

pub fn read_text(path: &Path) -> String {
    fs::read_to_string(path).expect("read text")
}

pub fn load_json_file(path: &Path) -> Value {
    serde_json::from_str(&read_text(path)).expect("valid json file")
}

pub fn tree_snapshot(root: &Path) -> BTreeMap<String, Vec<u8>> {
    let mut snapshot = BTreeMap::new();
    if !root.exists() {
        return snapshot;
    }
    for entry in WalkDir::new(root) {
        let entry = entry.expect("walkdir");
        if !entry.file_type().is_file() {
            continue;
        }
        snapshot.insert(
            entry
                .path()
                .strip_prefix(root)
                .expect("relative")
                .to_string_lossy()
                .replace('\\', "/"),
            fs::read(entry.path()).expect("read bytes"),
        );
    }
    snapshot
}
