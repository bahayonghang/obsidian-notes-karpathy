use std::env;

use anyhow::Result;
use clap::Parser;
use serde_json::json;

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
    query::{query_scope, rank_query_candidates},
    render::render_artifact,
    review::scan_review_queue,
    skills::{install_skills, list_skills, show_skill},
};

use super::args::{
    Cli, Commands, CompileCommand, DevCommand, IngestCommand, QueryCommand, ReviewCommand,
    SkillCommand,
};
use super::doctor::doctor_report;
use super::output::{CommandOutcome, RenderMode, print_outcome};
use super::version::version_report;

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    let outcome = dispatch(cli)?;
    print_outcome(&outcome)
}

fn dispatch(cli: Cli) -> Result<CommandOutcome> {
    let render_mode = match &cli.command {
        Commands::Doctor => RenderMode::Doctor,
        Commands::Version => RenderMode::Version,
        _ => RenderMode::Default,
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
        Commands::Skill { command } => dispatch_skill(command)?,
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
        Commands::Ingest { command } => dispatch_ingest(command)?,
        Commands::Compile { command } => dispatch_compile(command)?,
        Commands::Review { command } => dispatch_review(command)?,
        Commands::Query { command } => dispatch_query(command)?,
        Commands::Render(args) => render_artifact(
            &args.vault,
            &args.mode,
            &args.sources,
            args.output.as_deref(),
            args.title.as_deref(),
            args.write,
        )?,
        Commands::Dev { command } => dispatch_dev(command)?,
    };

    Ok(CommandOutcome {
        payload,
        render_mode,
        force_json_output,
    })
}

fn dispatch_skill(command: SkillCommand) -> Result<serde_json::Value> {
    match command {
        SkillCommand::Install(args) => install_skills(
            &args.dir.unwrap_or(env::current_dir()?),
            args.claude,
            args.codex,
            args.overwrite,
        ),
        SkillCommand::List => Ok(json!({"skills": list_skills()})),
        SkillCommand::Show { name } => Ok(json!({"name": name, "content": show_skill(&name)?})),
    }
}

fn dispatch_ingest(command: IngestCommand) -> Result<serde_json::Value> {
    match command {
        IngestCommand::Scan { vault } => scan_ingest_delta(&vault),
        IngestCommand::Sync { vault, write } => {
            if write {
                sync_source_manifest(&vault)
            } else {
                scan_ingest_delta(&vault)
            }
        }
    }
}

fn dispatch_compile(command: CompileCommand) -> Result<serde_json::Value> {
    match command {
        CompileCommand::Scan { vault } => scan_compile_delta(&vault),
        CompileCommand::Build { vault, write } => build_draft_packages(&vault, write),
    }
}

fn dispatch_review(command: ReviewCommand) -> Result<serde_json::Value> {
    match command {
        ReviewCommand::Queue { vault } => scan_review_queue(&vault),
        ReviewCommand::Lint { vault } => audit_vault_mechanics(&vault),
        ReviewCommand::Governance { vault, write } => {
            let payload = build_governance_indices(&vault)?;
            if write {
                write_governance_indices(&vault, &payload)?;
            }
            Ok(payload)
        }
        ReviewCommand::Graph { vault, write } => {
            let mut payload = build_graph_snapshot(&vault)?;
            if write {
                payload["output_path"] = json!(write_graph_snapshot(&vault, &payload)?);
            }
            Ok(payload)
        }
        ReviewCommand::Automation { vault, mode, write } => run_automation(&vault, &mode, write),
    }
}

fn dispatch_query(command: QueryCommand) -> Result<serde_json::Value> {
    match command {
        QueryCommand::Scope { vault } => query_scope(&vault),
        QueryCommand::Rank { vault, query } => rank_query_candidates(&vault, &query),
    }
}

fn dispatch_dev(command: DevCommand) -> Result<serde_json::Value> {
    match command {
        DevCommand::ContractValidate => validate_bundle(&current_repo_root()?),
        DevCommand::AuditSkills { skills, .. } => {
            build_skill_audit_payload(&current_repo_root()?, &skills)
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
        ),
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
            )
        }
        DevCommand::RenderReferenceBlock { skill } => {
            let repo_root = current_repo_root()?;
            let registry = load_registry(&repo_root)?;
            Ok(json!({
                "skill": skill,
                "content": render_shared_reference_block(&skill, &registry)?,
            }))
        }
    }
}
