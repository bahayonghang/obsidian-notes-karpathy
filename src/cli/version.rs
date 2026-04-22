use anyhow::Result;

use crate::payload::VersionPayload;

use super::doctor::runtime_info;

pub fn version_report() -> Result<VersionPayload> {
    let runtime = runtime_info()?;
    Ok(VersionPayload {
        name: runtime.name.to_string(),
        version: runtime.version,
    })
}

pub fn format_version_report(report: &VersionPayload) -> String {
    format!("{} {}", report.name, report.version)
}
