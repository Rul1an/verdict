use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "verdict",
    version,
    about = "CI-first PR regression gate for RAG pipelines (skeleton)"
)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Run(RunArgs),
    Ci(CiArgs),
    Init(InitArgs),
    Quarantine(QuarantineArgs),
    Version,
}

#[derive(Parser, Clone)]
pub struct RunArgs {
    #[arg(long, default_value = "eval.yaml")]
    pub config: PathBuf,
    #[arg(long, default_value = ".eval/eval.db")]
    pub db: PathBuf,

    #[arg(long, default_value_t = 0)]
    pub rerun_failures: u32,

    /// quarantine mode: off|warn|strict (controls status of quarantined tests)
    #[arg(long, default_value = "warn")]
    pub quarantine_mode: String,

    #[arg(long)]
    pub trace_file: Option<PathBuf>,

    #[arg(long)]
    pub redact_prompts: bool,

    #[arg(long)]
    pub baseline: Option<PathBuf>,

    #[arg(long)]
    pub export_baseline: Option<PathBuf>,

    /// strict mode (controls exit code policy: warn/flaky -> exit 1)
    #[arg(long)]
    pub strict: bool,

    /// embedder provider (none|openai|fake)
    #[arg(long, default_value = "none")]
    pub embedder: String,

    /// embedding model name
    #[arg(long, default_value = "text-embedding-3-small")]
    pub embedding_model: String,

    /// force refresh of embeddings (ignore cache)
    #[arg(long)]
    pub refresh_embeddings: bool,

    #[command(flatten)]
    pub judge: JudgeArgs,
}

#[derive(Parser, Clone)]
pub struct CiArgs {
    #[arg(long, default_value = "eval.yaml")]
    pub config: PathBuf,
    #[arg(long, default_value = ".eval/eval.db")]
    pub db: PathBuf,
    #[arg(long, default_value = "junit.xml")]
    pub junit: PathBuf,
    #[arg(long, default_value = "sarif.json")]
    pub sarif: PathBuf,

    #[arg(long, default_value_t = 1)]
    pub rerun_failures: u32,
    #[arg(long, default_value = "warn")]
    pub quarantine_mode: String,

    #[arg(long)]
    pub otel_jsonl: Option<PathBuf>,

    #[arg(long)]
    pub trace_file: Option<PathBuf>,

    #[arg(long)]
    pub redact_prompts: bool,

    #[arg(long)]
    pub baseline: Option<PathBuf>,

    #[arg(long)]
    pub export_baseline: Option<PathBuf>,

    /// strict mode (controls exit code policy: warn/flaky -> exit 1)
    #[arg(long)]
    pub strict: bool,

    #[arg(long, default_value = "none")]
    pub embedder: String,

    #[arg(long, default_value = "text-embedding-3-small")]
    pub embedding_model: String,

    #[arg(long)]
    pub refresh_embeddings: bool,

    #[command(flatten)]
    pub judge: JudgeArgs,
}

#[derive(clap::Args, Clone)]
pub struct JudgeArgs {
    /// Enable or disable LLM-as-judge evaluation
    /// - none: judge calls disabled (replay/trace-only)
    /// - openai: live judge calls via OpenAI
    /// - fake: deterministic fake judge (tests/dev)
    #[arg(long, default_value = "none", env = "VERDICT_JUDGE")]
    pub judge: String,

    /// Alias for --judge none
    #[arg(long, conflicts_with = "judge")]
    pub no_judge: bool,

    /// Judge model identifier (provider-specific)
    /// Example: gpt-4o-mini
    #[arg(long, env = "VERDICT_JUDGE_MODEL")]
    pub judge_model: Option<String>,

    /// Number of judge samples per test (majority vote)
    /// Default: 3
    /// Tip: for critical production gates consider: --judge-samples 5
    #[arg(long, default_value_t = 3, env = "VERDICT_JUDGE_SAMPLES")]
    pub judge_samples: u32,

    /// Ignore judge cache and re-run judge calls (live mode only)
    #[arg(long)]
    pub judge_refresh: bool,

    /// Temperature used for judge calls (affects cache key)
    /// Default: 0.0
    #[arg(long, default_value_t = 0.0, env = "VERDICT_JUDGE_TEMPERATURE")]
    pub judge_temperature: f32,

    /// Max tokens for judge response (affects cache key)
    /// Default: 800
    #[arg(long, default_value_t = 800, env = "VERDICT_JUDGE_MAX_TOKENS")]
    pub judge_max_tokens: u32,

    /// Start with env (VERDICT_JUDGE_API_KEY could be supported but OPENAI_API_KEY is primary)
    #[arg(long, hide = true)]
    pub judge_api_key: Option<String>,
}

#[derive(Parser, Clone)]
pub struct InitArgs {
    #[arg(long, default_value = "eval.yaml")]
    pub config: PathBuf,

    /// generate CI scaffolding (smoke test, traces, workflow)
    #[arg(long)]
    pub ci: bool,

    /// generate .gitignore for artifacts/db
    #[arg(long)]
    pub gitignore: bool,
}

#[derive(Parser, Clone)]
pub struct QuarantineArgs {
    #[command(subcommand)]
    pub cmd: QuarantineSub,
    #[arg(long, default_value = ".eval/eval.db")]
    pub db: PathBuf,
    #[arg(long, default_value = "demo")]
    pub suite: String,
}

#[derive(Subcommand, Clone)]
pub enum QuarantineSub {
    Add {
        #[arg(long)]
        test_id: String,
        #[arg(long)]
        reason: String,
    },
    Remove {
        #[arg(long)]
        test_id: String,
    },
    List,
}
