use anyhow::Result;

use crate::payload::DoctorPayload;
use crate::skills::bundle_available;

pub struct RuntimeInfo {
    pub name: &'static str,
    pub version: String,
    pub binary_path: String,
    pub runtime_mode: &'static str,
}

pub fn doctor_report() -> Result<DoctorPayload> {
    let runtime = runtime_info()?;
    let bundle = bundle_available();
    Ok(DoctorPayload {
        version: runtime.version,
        binary_path: runtime.binary_path,
        embedded_assets_detected: bundle,
        skill_bundle_available: bundle,
        runtime_mode: runtime.runtime_mode.to_string(),
        needs_python: false,
        python_detected: None,
        missing_steps: if bundle {
            Vec::new()
        } else {
            vec!["reinstall_onkb".to_string()]
        },
    })
}

pub fn runtime_info() -> Result<RuntimeInfo> {
    Ok(RuntimeInfo {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION").to_string(),
        binary_path: std::env::current_exe()?.display().to_string(),
        runtime_mode: "standalone-cli",
    })
}

pub fn format_doctor_report(payload: &DoctorPayload) -> String {
    let missing_steps = if payload.missing_steps.is_empty() {
        "- none".to_string()
    } else {
        payload
            .missing_steps
            .iter()
            .map(|step| format!("- {}", humanize_missing_step(step)))
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!(
        "onkb doctor\nversion: {}\nbinary: {}\nruntime: {}\nbundle: {}\npython: {}\nstatus: {}\nmissing steps:\n{}",
        payload.version,
        crate_path(&payload.binary_path),
        payload.runtime_mode,
        format_bundle_status(payload),
        format_python_status(payload),
        format_doctor_status(payload),
        missing_steps,
    )
}

fn format_bundle_status(payload: &DoctorPayload) -> String {
    match (
        payload.embedded_assets_detected,
        payload.skill_bundle_available,
    ) {
        (true, true) => "ready".to_string(),
        (true, false) => "embedded assets detected; skill bundle missing".to_string(),
        (false, true) => "skill bundle available; embedded assets missing".to_string(),
        (false, false) => "embedded assets and skill bundle missing".to_string(),
    }
}

fn format_python_status(payload: &DoctorPayload) -> String {
    match (payload.needs_python, &payload.python_detected) {
        (false, _) => "not required".to_string(),
        (true, Some(value)) => format!("required; detected {value}"),
        (true, None) => "required; not detected".to_string(),
    }
}

fn format_doctor_status(payload: &DoctorPayload) -> &'static str {
    if payload.missing_steps.is_empty() {
        "ready"
    } else {
        "needs attention"
    }
}

fn humanize_missing_step(step: &str) -> String {
    match step {
        "reinstall_onkb" => "reinstall onkb so the embedded bundle is refreshed".to_string(),
        _ => step.replace('_', " "),
    }
}

fn crate_path(value: &str) -> String {
    value.replace('\\', "/")
}
