use anyhow::Result;
use serde_json::Value;

use crate::payload::{DoctorPayload, VersionPayload};

#[derive(Clone, Copy)]
pub enum RenderMode {
    Default,
    Doctor,
    Version,
}

pub struct CommandOutcome {
    pub payload: Value,
    pub render_mode: RenderMode,
    pub force_json_output: bool,
}

pub fn print_outcome(outcome: &CommandOutcome) -> Result<()> {
    if outcome.force_json_output {
        println!("{}", serde_json::to_string_pretty(&outcome.payload)?);
        return Ok(());
    }

    match outcome.render_mode {
        RenderMode::Doctor => {
            let report: DoctorPayload = serde_json::from_value(outcome.payload.clone())?;
            println!("{}", crate::cli::doctor::format_doctor_report(&report));
        }
        RenderMode::Version => {
            let report: VersionPayload = serde_json::from_value(outcome.payload.clone())?;
            println!("{}", crate::cli::version::format_version_report(&report));
        }
        RenderMode::Default => {
            if let Some(summary) = outcome.payload.get("summary").and_then(Value::as_str) {
                println!("{summary}");
            } else if let Some(content) = outcome.payload.get("content").and_then(Value::as_str) {
                println!("{content}");
            } else {
                println!("{}", serde_json::to_string_pretty(&outcome.payload)?);
            }
        }
    }

    Ok(())
}
