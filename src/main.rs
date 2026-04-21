use std::env;
use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use serde_json::{json, Value};

use onkb::{
    automation::run_automation,
    compile::{build_draft_packages, scan_compile_delta},
    dev::{
        contract::validate_bundle,
        current_repo_root, load_registry,
        reference_blocks::render_shared_reference_block,
        runtime_eval::{run_runtime_eval, RuntimeEvalOptions},
        skill_audit::build_payload as build_skill_audit_payload,
        trigger_eval::{run_trigger_eval, TriggerEvalOptions},
    },
    governance::{build_governance_indices, write_governance_indices},
    graph::{build_graph_snapshot, write_graph_snapshot},
    health::audit_vault_mechanics,
    ingest::{scan_ingest_delta, sync_source_manifest},
    init::{describe_vault_status, migrate_legacy_vault, scaffold_review_gated_vault},
    payload::DoctorPayload,
    query::{query_scope, rank_query_candidates},
    render::render_artifact,
    review::scan_review_queue,
    skills::{bundle_available, install_skills, list_skills, show_skill},
};

#[derive(Parser)]
#[command(name = "onkb", about = "Review-gated Obsidian knowledge-base CLI")]
struct Cli {
    #[arg(long, global = true)]
    json: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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

fn main() -> Result<()> {
    let cli = Cli::parse();
    let force_json_output = cli.json
        || matches!(
            &cli.command,
            Commands::Doctor
                | Commands::Skill {
                    command: SkillCommand::List
                }
                | Commands::Skill {
                    command: SkillCommand::Install(_)
                }
        );
    let payload = match cli.command {
        Commands::Doctor => doctor_report()?,
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

    if force_json_output {
        println!("{}", serde_json::to_string_pretty(&payload)?);
    } else if let Some(summary) = payload.get("summary").and_then(Value::as_str) {
        println!("{summary}");
    } else if let Some(content) = payload.get("content").and_then(Value::as_str) {
        println!("{content}");
    } else {
        println!("{}", serde_json::to_string_pretty(&payload)?);
    }
    Ok(())
}

fn doctor_report() -> Result<Value> {
    let binary_path = env::current_exe()?.to_string_lossy().into_owned();
    let bundle = bundle_available();
    let payload = DoctorPayload {
        version: env!("CARGO_PKG_VERSION").to_string(),
        binary_path: crate_path(&binary_path),
        embedded_assets_detected: bundle,
        skill_bundle_available: bundle,
        runtime_mode: "standalone-cli".to_string(),
        needs_python: false,
        python_detected: None,
        missing_steps: if bundle {
            Vec::new()
        } else {
            vec!["reinstall_onkb".to_string()]
        },
    };
    Ok(serde_json::to_value(&payload)?)
}

fn crate_path(value: &str) -> String {
    value.replace('\\', "/")
}
