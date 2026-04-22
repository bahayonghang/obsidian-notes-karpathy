use std::env;
use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use serde_json::{Value, json};

use crate::{
    automation::run_automation,
    compile::{build_draft_packages, scan_compile_delta},
    dev::{
        contract::validate_bundle,
        current_repo_root, load_registry,
        reference_blocks::render_shared_reference_block,
        runtime_eval::{RuntimeEvalOptions, run_runtime_eval},
        skill_audit::build_payload as build_skill_audit_payload,
        trigger_eval::{TriggerEvalOptions, run_trigger_eval},
    },
    governance::{build_governance_indices, write_governance_indices},
    graph::{build_graph_snapshot, write_graph_snapshot},
    health::audit_vault_mechanics,
    ingest::{scan_ingest_delta, sync_source_manifest},
    init::{describe_vault_status, migrate_legacy_vault, scaffold_review_gated_vault},
    payload::{DoctorPayload, VersionPayload},
    query::{query_scope, rank_query_candidates},
    render::render_artifact,
    review::scan_review_queue,
    skills::{bundle_available, install_skills, list_skills, show_skill},
};

#[derive(Parser)]
#[command(
    name = "onkb",
    about = "Review-gated Obsidian knowledge-base CLI",
    version
)]
struct Cli {
    #[arg(long, global = true)]
    json: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Version,
    Doctor,
    Skill {
        #[command(subcommand)]
        command: SkillCommand,
    },
    Status {
        vault: PathBuf,
    },
    Init(InitArgs),
    Migrate(MigrateArgs),
    Ingest {
        #[command(subcommand)]
        command: IngestCommand,
    },
    Compile {
        #[command(subcommand)]
        command: CompileCommand,
    },
    Review {
        #[command(subcommand)]
        command: ReviewCommand,
    },
    Query {
        #[command(subcommand)]
        command: QueryCommand,
    },
    Render(RenderArgs),
    Dev {
        #[command(subcommand)]
        command: DevCommand,
    },
}

#[derive(Subcommand)]
enum SkillCommand {
    Install(SkillInstallArgs),
    List,
    Show { name: String },
}

#[derive(Args)]
struct SkillInstallArgs {
    #[arg(long)]
    claude: bool,
    #[arg(long)]
    codex: bool,
    #[arg(long)]
    dir: Option<PathBuf>,
    #[arg(long)]
    overwrite: bool,
}

#[derive(Args)]
struct InitArgs {
    vault: PathBuf,
    #[arg(long, default_value = "governed-team")]
    profile: String,
    #[arg(long)]
    include_governance: bool,
    #[arg(long)]
    include_full_outputs: bool,
    #[arg(long)]
    include_latest_outputs: bool,
    #[arg(long)]
    overwrite: bool,
    #[arg(long)]
    skip_memory: bool,
}

#[derive(Args)]
struct MigrateArgs {
    vault: PathBuf,
    #[arg(long, default_value = "governed-team")]
    profile: String,
    #[arg(long, default_value_t = true)]
    include_governance: bool,
    #[arg(long)]
    include_full_outputs: bool,
    #[arg(long)]
    include_latest_outputs: bool,
}

#[derive(Subcommand)]
enum IngestCommand {
    Scan {
        vault: PathBuf,
    },
    Sync {
        vault: PathBuf,
        #[arg(long)]
        write: bool,
    },
}

#[derive(Subcommand)]
enum CompileCommand {
    Scan {
        vault: PathBuf,
    },
    Build {
        vault: PathBuf,
        #[arg(long)]
        write: bool,
    },
}

#[derive(Subcommand)]
enum ReviewCommand {
    Queue {
        vault: PathBuf,
    },
    Lint {
        vault: PathBuf,
    },
    Governance {
        vault: PathBuf,
        #[arg(long)]
        write: bool,
    },
    Graph {
        vault: PathBuf,
        #[arg(long)]
        write: bool,
    },
    Automation {
        vault: PathBuf,
        #[arg(long)]
        mode: String,
        #[arg(long)]
        write: bool,
    },
}

#[derive(Subcommand)]
enum QueryCommand {
    Scope { vault: PathBuf },
    Rank { vault: PathBuf, query: String },
}

#[derive(Args)]
struct RenderArgs {
    vault: PathBuf,
    #[arg(long)]
    mode: String,
    #[arg(long = "source")]
    sources: Vec<String>,
    #[arg(long)]
    output: Option<String>,
    #[arg(long)]
    title: Option<String>,
    #[arg(long)]
    write: bool,
}

#[derive(Subcommand)]
enum DevCommand {
    ContractValidate,
    AuditSkills {
        #[arg(long)]
        json: bool,
        #[arg(long = "skill")]
        skills: Vec<String>,
    },
    EvalTrigger {
        #[arg(long = "eval-set")]
        eval_set: Option<PathBuf>,
        #[arg(long)]
        runner: Option<String>,
        #[arg(long)]
        workspace: Option<PathBuf>,
        #[arg(long = "skill")]
        skills: Vec<String>,
        #[arg(long)]
        dry_run: bool,
        #[arg(long)]
        limit: Option<usize>,
        #[arg(long = "timeout-sec", default_value_t = 90)]
        timeout_sec: u64,
    },
    EvalRuntime {
        #[arg(long)]
        manifest: Option<PathBuf>,
        #[arg(long)]
        writable: bool,
        #[arg(long)]
        dry_run: bool,
        #[arg(long)]
        runner: Option<String>,
        #[arg(long)]
        workspace: Option<PathBuf>,
        #[arg(long = "reuse-baseline-from")]
        reuse_baseline_from: Option<PathBuf>,
        #[arg(long = "skill")]
        skills: Vec<String>,
        #[arg(long = "eval-id")]
        eval_ids: Vec<String>,
        #[arg(long)]
        limit: Option<usize>,
        #[arg(long = "timeout-sec", default_value_t = 180)]
        timeout_sec: u64,
    },
    RenderReferenceBlock {
        skill: String,
    },
}

#[derive(Clone, Copy)]
enum OutputView {
    Default,
    Doctor,
    Version,
}

struct RuntimeInfo {
    name: &'static str,
    version: String,
    binary_path: String,
    runtime_mode: &'static str,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    let output_view = match &cli.command {
        Commands::Doctor => OutputView::Doctor,
        Commands::Version => OutputView::Version,
        _ => OutputView::Default,
    };
    let force_json_output = cli.json
        || matches!(
            &cli.command,
            Commands::Skill {
                command: SkillCommand::List
            } | Commands::Skill {
                command: SkillCommand::Install(_)
            }
        );
    let payload = match cli.command {
        Commands::Version => serde_json::to_value(version_report()?)?,
        Commands::Doctor => serde_json::to_value(doctor_report()?)?,
        Commands::Skill { command } => match command {
            SkillCommand::Install(args) => install_skills(
                &args.dir.unwrap_or(env::current_dir()?),
                args.claude,
                args.codex,
                args.overwrite,
            )?,
            SkillCommand::List => json!({"skills": list_skills()}),
            SkillCommand::Show { name } => json!({"name": name, "content": show_skill(&name)?}),
        },
        Commands::Status { vault } => describe_vault_status(&vault)?,
        Commands::Init(args) => scaffold_review_gated_vault(
            &args.vault,
            &args.profile,
            args.include_governance,
            args.include_full_outputs,
            args.include_latest_outputs,
            args.overwrite,
            args.skip_memory,
        )?,
        Commands::Migrate(args) => migrate_legacy_vault(
            &args.vault,
            &args.profile,
            args.include_governance,
            args.include_full_outputs,
            args.include_latest_outputs,
        )?,
        Commands::Ingest { command } => match command {
            IngestCommand::Scan { vault } => scan_ingest_delta(&vault)?,
            IngestCommand::Sync { vault, write } => {
                if write {
                    sync_source_manifest(&vault)?
                } else {
                    scan_ingest_delta(&vault)?
                }
            }
        },
        Commands::Compile { command } => match command {
            CompileCommand::Scan { vault } => scan_compile_delta(&vault)?,
            CompileCommand::Build { vault, write } => build_draft_packages(&vault, write)?,
        },
        Commands::Review { command } => match command {
            ReviewCommand::Queue { vault } => scan_review_queue(&vault)?,
            ReviewCommand::Lint { vault } => audit_vault_mechanics(&vault)?,
            ReviewCommand::Governance { vault, write } => {
                let payload = build_governance_indices(&vault)?;
                if write {
                    write_governance_indices(&vault, &payload)?;
                }
                payload
            }
            ReviewCommand::Graph { vault, write } => {
                let mut payload = build_graph_snapshot(&vault)?;
                if write {
                    payload["output_path"] = json!(write_graph_snapshot(&vault, &payload)?);
                }
                payload
            }
            ReviewCommand::Automation { vault, mode, write } => {
                run_automation(&vault, &mode, write)?
            }
        },
        Commands::Query { command } => match command {
            QueryCommand::Scope { vault } => query_scope(&vault)?,
            QueryCommand::Rank { vault, query } => rank_query_candidates(&vault, &query)?,
        },
        Commands::Render(args) => render_artifact(
            &args.vault,
            &args.mode,
            &args.sources,
            args.output.as_deref(),
            args.title.as_deref(),
            args.write,
        )?,
        Commands::Dev { command } => match command {
            DevCommand::ContractValidate => validate_bundle(&current_repo_root()?)?,
            DevCommand::AuditSkills { skills, .. } => {
                build_skill_audit_payload(&current_repo_root()?, &skills)?
            }
            DevCommand::EvalTrigger {
                eval_set,
                runner,
                workspace,
                skills,
                dry_run,
                limit,
                timeout_sec,
            } => run_trigger_eval(
                &current_repo_root()?,
                TriggerEvalOptions {
                    eval_set: eval_set.as_deref(),
                    runner: runner.as_deref(),
                    workspace: workspace.as_deref(),
                    skills: &skills,
                    dry_run,
                    limit,
                    timeout_sec,
                },
            )?,
            DevCommand::EvalRuntime {
                manifest,
                writable,
                dry_run,
                runner,
                workspace,
                reuse_baseline_from,
                skills,
                eval_ids,
                limit,
                timeout_sec,
            } => {
                let repo_root = current_repo_root()?;
                let registry = load_registry(&repo_root)?;
                run_runtime_eval(
                    &repo_root,
                    &registry,
                    RuntimeEvalOptions {
                        manifest_override: manifest.as_deref(),
                        runner: runner.as_deref(),
                        workspace: workspace.as_deref(),
                        reuse_baseline_from: reuse_baseline_from.as_deref(),
                        skills: &skills,
                        eval_ids: &eval_ids,
                        dry_run,
                        limit,
                        timeout_sec,
                        writable,
                    },
                )?
            }
            DevCommand::RenderReferenceBlock { skill } => {
                let repo_root = current_repo_root()?;
                let registry = load_registry(&repo_root)?;
                json!({
                    "skill": skill,
                    "content": render_shared_reference_block(&skill, &registry)?,
                })
            }
        },
    };

    print_payload(&payload, output_view, force_json_output)?;
    Ok(())
}

fn print_payload(payload: &Value, output_view: OutputView, force_json_output: bool) -> Result<()> {
    if force_json_output {
        println!("{}", serde_json::to_string_pretty(payload)?);
    } else {
        match output_view {
            OutputView::Doctor => {
                let report: DoctorPayload = serde_json::from_value(payload.clone())?;
                println!("{}", format_doctor_report(&report));
            }
            OutputView::Version => {
                let report: VersionPayload = serde_json::from_value(payload.clone())?;
                println!("{}", format_version_report(&report));
            }
            OutputView::Default => {
                if let Some(summary) = payload.get("summary").and_then(Value::as_str) {
                    println!("{summary}");
                } else if let Some(content) = payload.get("content").and_then(Value::as_str) {
                    println!("{content}");
                } else {
                    println!("{}", serde_json::to_string_pretty(payload)?);
                }
            }
        }
    }
    Ok(())
}

fn version_report() -> Result<VersionPayload> {
    let runtime = runtime_info()?;
    Ok(VersionPayload {
        name: runtime.name.to_string(),
        version: runtime.version,
    })
}

fn doctor_report() -> Result<DoctorPayload> {
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

fn runtime_info() -> Result<RuntimeInfo> {
    let binary_path = env::current_exe()?.to_string_lossy().into_owned();
    Ok(RuntimeInfo {
        name: "onkb",
        version: env!("CARGO_PKG_VERSION").to_string(),
        binary_path: crate_path(&binary_path),
        runtime_mode: "standalone-cli",
    })
}

fn format_version_report(payload: &VersionPayload) -> String {
    format!("{} {}", payload.name, payload.version)
}

fn format_doctor_report(payload: &DoctorPayload) -> String {
    let mut lines = vec![
        "onkb doctor".to_string(),
        format!("version: {}", payload.version),
        format!("binary: {}", payload.binary_path),
        format!("runtime: {}", payload.runtime_mode),
        format!("bundle: {}", format_bundle_status(payload)),
        format!("python: {}", format_python_status(payload)),
        format!("status: {}", format_doctor_status(payload)),
    ];

    if !payload.missing_steps.is_empty() {
        lines.push("next:".to_string());
        lines.extend(
            payload
                .missing_steps
                .iter()
                .map(|step| format!("  - {}", humanize_missing_step(step))),
        );
    }

    lines.join("\n")
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
