use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{anyhow, Context, Result};
use serde_json::{json, Value};

use super::paths::INFRA_FAILURE_PATTERNS;

fn candidate_extensions() -> Vec<String> {
    if cfg!(windows) {
        env::var("PATHEXT")
            .unwrap_or_else(|_| ".COM;.EXE;.BAT;.CMD;.PS1".to_string())
            .split(';')
            .map(|value| value.trim().to_ascii_lowercase())
            .filter(|value| !value.is_empty())
            .collect()
    } else {
        vec![String::new()]
    }
}

pub fn find_executable(name: &str) -> Option<PathBuf> {
    let path = PathBuf::from(name);
    if path.is_absolute() || path.components().count() > 1 {
        if path.exists() {
            return Some(path);
        }
        return None;
    }

    let path_var = env::var_os("PATH")?;
    let extensions = candidate_extensions();
    for directory in env::split_paths(&path_var) {
        if cfg!(windows) {
            let lowered = name.to_ascii_lowercase();
            let has_extension = extensions
                .iter()
                .any(|extension| lowered.ends_with(extension));
            if has_extension {
                let candidate = directory.join(name);
                if candidate.is_file() {
                    return Some(candidate);
                }
            } else {
                for extension in &extensions {
                    let candidate = directory.join(format!("{name}{extension}"));
                    if candidate.is_file() {
                        return Some(candidate);
                    }
                }
            }
        } else {
            let candidate = directory.join(name);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}

pub fn detect_runner(preferred: Option<&str>) -> Option<String> {
    if let Some(preferred) = preferred {
        return find_executable(preferred).map(|_| preferred.to_string());
    }

    for candidate in ["codex", "claude"] {
        if find_executable(candidate).is_some() {
            return Some(candidate.to_string());
        }
    }
    None
}

pub fn resolve_runner_invocation(runner: &str) -> Result<Vec<String>> {
    let resolved = find_executable(runner).ok_or_else(|| anyhow!("Runner not found: {runner}"))?;
    let suffix = resolved
        .extension()
        .map(|value| format!(".{}", value.to_string_lossy().to_ascii_lowercase()))
        .unwrap_or_default();

    if cfg!(windows) && suffix == ".ps1" {
        return Ok(vec![
            "powershell".to_string(),
            "-NoProfile".to_string(),
            "-ExecutionPolicy".to_string(),
            "Bypass".to_string(),
            "-File".to_string(),
            resolved.to_string_lossy().into_owned(),
        ]);
    }
    if cfg!(windows) && matches!(suffix.as_str(), ".cmd" | ".bat") {
        return Ok(vec![resolved.to_string_lossy().into_owned()]);
    }
    Ok(vec![resolved.to_string_lossy().into_owned()])
}

pub fn codex_command(
    output_path: &Path,
    sandbox_mode: &str,
    repo_root: &Path,
) -> Result<Vec<String>> {
    let mut command = resolve_runner_invocation("codex")?;
    command.extend([
        "exec".to_string(),
        "--ephemeral".to_string(),
        "-s".to_string(),
        sandbox_mode.to_string(),
        "-C".to_string(),
        repo_root.to_string_lossy().into_owned(),
        "--output-last-message".to_string(),
        output_path.to_string_lossy().into_owned(),
        "-".to_string(),
    ]);
    Ok(command)
}

pub fn claude_command(prompt: &str) -> Result<Vec<String>> {
    let mut command = resolve_runner_invocation("claude")?;
    command.extend([
        "-p".to_string(),
        "--permission-mode".to_string(),
        "default".to_string(),
        prompt.to_string(),
    ]);
    Ok(command)
}

pub fn classify_failure(returncode: i32, stdout: &str, stderr: &str) -> String {
    if returncode == 0 {
        return "ok".to_string();
    }
    let combined = format!("{stdout}\n{stderr}").to_ascii_lowercase();
    if INFRA_FAILURE_PATTERNS
        .iter()
        .any(|pattern| combined.contains(pattern))
    {
        return "infra_failure".to_string();
    }
    "runner_failure".to_string()
}

pub fn fallback_runner_for(runner: &str) -> Option<String> {
    if runner == "codex" && detect_runner(Some("claude")).is_some() {
        return Some("claude".to_string());
    }
    None
}

fn spawn_and_capture(
    command: &[String],
    cwd: &Path,
    input: Option<&str>,
    timeout_sec: u64,
) -> Result<(i32, String, String)> {
    let mut child = Command::new(&command[0])
        .args(&command[1..])
        .current_dir(cwd)
        .stdin(if input.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("spawn {}", command.join(" ")))?;

    if let Some(input) = input {
        if let Some(stdin) = child.stdin.as_mut() {
            stdin
                .write_all(input.as_bytes())
                .context("write runner stdin")?;
        }
    }

    let start = Instant::now();
    let mut timed_out = false;
    loop {
        if child.try_wait()?.is_some() {
            break;
        }
        if start.elapsed() >= Duration::from_secs(timeout_sec) {
            timed_out = true;
            let _ = child.kill();
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }

    let output = child.wait_with_output().context("wait for runner output")?;
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let mut stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    let returncode = if timed_out {
        if !stderr.is_empty() {
            stderr.push('\n');
        }
        stderr.push_str(&format!(
            "Runtime eval attempt timed out after {timeout_sec} seconds."
        ));
        124
    } else {
        output.status.code().unwrap_or(1)
    };
    Ok((returncode, stdout, stderr))
}

pub fn execute_attempt(
    runner: &str,
    prompt: &str,
    run_dir: &Path,
    timeout_sec: u64,
    sandbox_mode: &str,
    repo_root: &Path,
) -> Result<Value> {
    fs::create_dir_all(run_dir).with_context(|| format!("create {}", run_dir.display()))?;
    let output_path = run_dir.join("last_message.txt");
    let started = Instant::now();
    let command = match runner {
        "codex" => codex_command(&output_path, sandbox_mode, repo_root)?,
        "claude" => claude_command(prompt)?,
        _ => return Err(anyhow!("Unsupported runner: {runner}")),
    };
    let input = if runner == "codex" {
        Some(prompt)
    } else {
        None
    };
    let (returncode, stdout, stderr) = spawn_and_capture(&command, repo_root, input, timeout_sec)?;
    let duration_ms = started.elapsed().as_millis() as i64;

    if runner == "claude" && !output_path.exists() {
        fs::write(&output_path, &stdout)
            .with_context(|| format!("write {}", output_path.display()))?;
    }
    if !output_path.exists() {
        fs::write(&output_path, "").with_context(|| format!("write {}", output_path.display()))?;
    }
    fs::write(run_dir.join("stdout.txt"), &stdout)
        .with_context(|| format!("write {}", run_dir.join("stdout.txt").display()))?;
    fs::write(run_dir.join("stderr.txt"), &stderr)
        .with_context(|| format!("write {}", run_dir.join("stderr.txt").display()))?;

    let payload = json!({
        "runner": runner,
        "returncode": returncode,
        "duration_ms": duration_ms,
        "output_path": "last_message.txt",
        "output_captured": !fs::read_to_string(&output_path).unwrap_or_default().trim().is_empty(),
        "failure_kind": classify_failure(returncode, &stdout, &stderr),
    });
    fs::write(
        run_dir.join("result.json"),
        serde_json::to_string_pretty(&payload)?,
    )
    .with_context(|| format!("write {}", run_dir.join("result.json").display()))?;
    Ok(payload)
}

fn copy_attempt_outputs(attempt_dir: &Path, run_dir: &Path) -> Result<()> {
    for file_name in ["last_message.txt", "stdout.txt", "stderr.txt"] {
        let source = attempt_dir.join(file_name);
        if source.exists() {
            fs::copy(&source, run_dir.join(file_name))
                .with_context(|| format!("copy {}", source.display()))?;
        }
    }
    Ok(())
}

pub fn execute_run(
    runner: &str,
    prompt: &str,
    run_dir: &Path,
    timeout_sec: u64,
    sandbox_mode: &str,
    repo_root: &Path,
) -> Result<Value> {
    fs::create_dir_all(run_dir).with_context(|| format!("create {}", run_dir.display()))?;
    let primary_dir = run_dir.join(format!("attempt-1-{runner}"));
    let primary = execute_attempt(
        runner,
        prompt,
        &primary_dir,
        timeout_sec,
        sandbox_mode,
        repo_root,
    )?;
    let mut attempts = vec![primary.clone()];
    let mut final_attempt = primary;

    if final_attempt.get("failure_kind").and_then(Value::as_str) == Some("infra_failure") {
        if let Some(fallback_runner) = fallback_runner_for(runner) {
            let fallback_dir = run_dir.join(format!("attempt-2-{fallback_runner}"));
            let fallback = execute_attempt(
                &fallback_runner,
                prompt,
                &fallback_dir,
                timeout_sec,
                sandbox_mode,
                repo_root,
            )?;
            attempts.push(fallback.clone());
            final_attempt = fallback;
        }
    }

    let final_attempt_dir = run_dir.join(format!(
        "attempt-{}-{}",
        attempts.len(),
        final_attempt
            .get("runner")
            .and_then(Value::as_str)
            .unwrap_or(runner)
    ));
    copy_attempt_outputs(&final_attempt_dir, run_dir)?;

    let payload = json!({
        "requested_runner": runner,
        "runner": final_attempt.get("runner").cloned().unwrap_or_else(|| Value::String(runner.to_string())),
        "returncode": final_attempt.get("returncode").cloned().unwrap_or(Value::Null),
        "duration_ms": final_attempt.get("duration_ms").cloned().unwrap_or(Value::Null),
        "output_path": "last_message.txt",
        "output_captured": !fs::read_to_string(run_dir.join("last_message.txt")).unwrap_or_default().trim().is_empty(),
        "failure_kind": final_attempt.get("failure_kind").cloned().unwrap_or_else(|| Value::String("runner_failure".to_string())),
        "fallback_used": attempts.len() > 1,
        "attempts": attempts,
    });
    fs::write(
        run_dir.join("result.json"),
        serde_json::to_string_pretty(&payload)?,
    )
    .with_context(|| format!("write {}", run_dir.join("result.json").display()))?;
    Ok(payload)
}
