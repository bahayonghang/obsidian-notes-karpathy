use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "onkb",
    about = "Review-gated Obsidian knowledge-base CLI",
    version
)]
pub struct Cli {
    #[arg(long, global = true)]
    pub json: bool,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
pub enum SkillCommand {
    Install(SkillInstallArgs),
    List,
    Show { name: String },
}

#[derive(Args)]
pub struct SkillInstallArgs {
    #[arg(long)]
    pub claude: bool,
    #[arg(long)]
    pub codex: bool,
    #[arg(long)]
    pub dir: Option<PathBuf>,
    #[arg(long)]
    pub overwrite: bool,
}

#[derive(Args)]
pub struct InitArgs {
    pub vault: PathBuf,
    #[arg(long, default_value = "governed-team")]
    pub profile: String,
    #[arg(long)]
    pub include_governance: bool,
    #[arg(long)]
    pub include_full_outputs: bool,
    #[arg(long)]
    pub include_latest_outputs: bool,
    #[arg(long)]
    pub overwrite: bool,
    #[arg(long)]
    pub skip_memory: bool,
}

#[derive(Args)]
pub struct MigrateArgs {
    pub vault: PathBuf,
    #[arg(long, default_value = "governed-team")]
    pub profile: String,
    #[arg(long, default_value_t = true)]
    pub include_governance: bool,
    #[arg(long)]
    pub include_full_outputs: bool,
    #[arg(long)]
    pub include_latest_outputs: bool,
}

#[derive(Subcommand)]
pub enum IngestCommand {
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
pub enum CompileCommand {
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
pub enum ReviewCommand {
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
pub enum QueryCommand {
    Scope { vault: PathBuf },
    Rank { vault: PathBuf, query: String },
}

#[derive(Args)]
pub struct RenderArgs {
    pub vault: PathBuf,
    #[arg(long)]
    pub mode: String,
    #[arg(long = "source")]
    pub sources: Vec<String>,
    #[arg(long)]
    pub output: Option<String>,
    #[arg(long)]
    pub title: Option<String>,
    #[arg(long)]
    pub write: bool,
}

#[derive(Subcommand)]
pub enum DevCommand {
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
