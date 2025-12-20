use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Parser)]
#[command(
    name = "verdict",
    version,
    about = "CI-first PR regression gate for RAG pipelines (skeleton)"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    Run(RunArgs),
    Ci(CiArgs),
    Init(InitArgs),
    Quarantine(QuarantineArgs),
    Version,
}

#[derive(Parser, Clone)]
struct RunArgs {
    #[arg(long, default_value = "eval.yaml")]
    config: PathBuf,
    #[arg(long, default_value = ".eval/eval.db")]
    db: PathBuf,

    #[arg(long, default_value_t = 0)]
    rerun_failures: u32,

    /// quarantine mode: off|warn|strict (controls status of quarantined tests)
    #[arg(long, default_value = "warn")]
    quarantine_mode: String,

    #[arg(long)]
    trace_file: Option<PathBuf>,

    #[arg(long)]
    redact_prompts: bool,

    /// strict mode (controls exit code policy: warn/flaky -> exit 1)
    #[arg(long)]
    strict: bool,
}

#[derive(Parser, Clone)]
struct CiArgs {
    #[arg(long, default_value = "eval.yaml")]
    config: PathBuf,
    #[arg(long, default_value = ".eval/eval.db")]
    db: PathBuf,
    #[arg(long, default_value = "junit.xml")]
    junit: PathBuf,
    #[arg(long, default_value = "sarif.json")]
    sarif: PathBuf,

    #[arg(long, default_value_t = 1)]
    rerun_failures: u32,
    #[arg(long, default_value = "warn")]
    quarantine_mode: String,

    #[arg(long)]
    otel_jsonl: Option<PathBuf>,

    #[arg(long)]
    trace_file: Option<PathBuf>,

    #[arg(long)]
    redact_prompts: bool,

    /// strict mode (controls exit code policy: warn/flaky -> exit 1)
    #[arg(long)]
    strict: bool,
}

#[derive(Parser, Clone)]
struct InitArgs {
    #[arg(long, default_value = "eval.yaml")]
    config: PathBuf,

    /// generate CI scaffolding (smoke test, traces, workflow)
    #[arg(long)]
    ci: bool,

    /// generate .gitignore for artifacts/db
    #[arg(long)]
    gitignore: bool,
}

#[derive(Parser)]
struct QuarantineArgs {
    #[command(subcommand)]
    cmd: QuarantineSub,
    #[arg(long, default_value = ".eval/eval.db")]
    db: PathBuf,
    #[arg(long, default_value = "demo")]
    suite: String,
}

#[derive(Subcommand)]
enum QuarantineSub {
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

mod exit_codes {
    pub const OK: i32 = 0;
    pub const TEST_FAILED: i32 = 1;
    pub const CONFIG_ERROR: i32 = 2;
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let cli = Cli::parse();
    let code = match dispatch(cli).await {
        Ok(code) => code,
        Err(e) => {
            eprintln!("fatal: {e:?}");
            exit_codes::CONFIG_ERROR
        }
    };
    std::process::exit(code);
}

async fn dispatch(cli: Cli) -> anyhow::Result<i32> {
    match cli.cmd {
        Command::Init(args) => cmd_init(args).await,
        Command::Run(args) => cmd_run(args).await,
        Command::Ci(args) => cmd_ci(args).await,
        Command::Quarantine(args) => cmd_quarantine(args).await,
        Command::Version => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            Ok(exit_codes::OK)
        }
    }
}

async fn cmd_init(args: InitArgs) -> anyhow::Result<i32> {
    // 1. Basic Config
    if !args.config.exists() {
        if let Some(parent) = args.config.parent() {
            std::fs::create_dir_all(parent)?;
        }
        verdict_core::config::write_sample_config(&args.config)?;
        eprintln!("created {}", args.config.display());
    } else {
        eprintln!("note: {} already exists", args.config.display());
    }

    // 2. Gitignore
    if args.gitignore {
        let gi_path = std::path::Path::new(".gitignore");
        if !gi_path.exists() {
            std::fs::write(gi_path, "/.eval/\n/out/\n*.db\n*.db-shm\n*.db-wal\n/verdict\n")?;
            eprintln!("created .gitignore");
        } else {
            eprintln!("note: .gitignore already exists (skipped)");
        }
    }

    // 3. CI Scaffolding
    if args.ci {
        // ci-eval.yaml
        let ci_yaml = std::path::Path::new("ci-eval.yaml");
        if !ci_yaml.exists() {
             std::fs::write(ci_yaml, r#"version: 1
suite: "ci_smoke"
model: "trace"
tests:
  - id: "ci_smoke_regex"
    input:
      prompt: "ci_regex"
    expected:
      type: regex_match
      pattern: "Hello\\s+CI"
      flags: ["i"]
  - id: "ci_smoke_schema"
    input:
      prompt: "ci_schema"
    expected:
      type: json_schema
      json_schema: "{}"
      schema_file: "schemas/ci_answer.schema.json"
"#)?;
             eprintln!("created ci-eval.yaml");
        }

        // schemas/ci_answer.schema.json
        ensure_parent_dir(&std::path::Path::new("schemas/x"))?;
        let schema_path = std::path::Path::new("schemas/ci_answer.schema.json");
        if !schema_path.exists() {
            std::fs::write(schema_path, r#"{
  "type": "object",
  "required": ["answer"],
  "properties": {
    "answer": { "type": "string" }
  },
  "additionalProperties": false
}"#)?;
            eprintln!("created schemas/ci_answer.schema.json");
        }

        // traces/ci.jsonl
        ensure_parent_dir(&std::path::Path::new("traces/x"))?;
        let trace_path = std::path::Path::new("traces/ci.jsonl");
        if !trace_path.exists() {
            std::fs::write(trace_path, r#"{"schema_version": 1, "type": "verdict.trace", "request_id": "ci_1", "prompt": "ci_regex", "response": "hello   ci", "model": "trace", "provider": "trace"}
{"schema_version": 1, "type": "verdict.trace", "request_id": "ci_2", "prompt": "ci_schema", "response": "{\"answer\":\"ok\"}", "model": "trace", "provider": "trace"}
"#)?;
            eprintln!("created traces/ci.jsonl");
        }

        // .github/workflows/verdict.yml
        ensure_parent_dir(&std::path::Path::new(".github/workflows/x"))?;
        let workflow_path = std::path::Path::new(".github/workflows/verdict.yml");
        if !workflow_path.exists() {
            std::fs::write(workflow_path, r#"name: Verdict Gate
on: [push, pull_request]
jobs:
  verdict:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      # Note: In real usage, you would install verdict binary or use a container.
      # This example assumes verdict is available or you build it.
      - name: Run Verdict Smoke Test
        run: |
          # Example: download release or build
          # cargo install --git https://github.com/Rul1an/verdict.git verdict-cli
          # verdict ci --config ci-eval.yaml --trace-file traces/ci.jsonl --strict
          echo "Customize this workflow to install verdict!"
"#)?;
             eprintln!("created .github/workflows/verdict.yml (skeleton)");
        }
    }

    Ok(exit_codes::OK)
}

async fn cmd_run(args: RunArgs) -> anyhow::Result<i32> {
    ensure_parent_dir(&args.db)?;
    let cfg = verdict_core::config::load_config(&args.config).map_err(|e| anyhow::anyhow!(e))?;
    let runner = build_runner(
        &args.db,
        &args.trace_file,
        &cfg,
        args.rerun_failures,
        &args.quarantine_mode,
    ).await?;

    let artifacts = runner.run_suite(&cfg).await?;

    verdict_core::report::console::print_summary(&artifacts.results);
    Ok(decide_exit_code(&artifacts.results, args.strict))
}

async fn cmd_ci(args: CiArgs) -> anyhow::Result<i32> {
    ensure_parent_dir(&args.db)?;
    let cfg = verdict_core::config::load_config(&args.config).map_err(|e| anyhow::anyhow!(e))?;
    let runner = build_runner(
        &args.db,
        &args.trace_file,
        &cfg,
        args.rerun_failures,
        &args.quarantine_mode,
    ).await?;

    let artifacts = runner.run_suite(&cfg).await?;

    verdict_core::report::junit::write_junit(&cfg.suite, &artifacts.results, &args.junit)?;
    verdict_core::report::sarif::write_sarif("verdict", &artifacts.results, &args.sarif)?;
    verdict_core::report::json::write_json(&artifacts, &PathBuf::from("run.json"))?;

    let otel_cfg = verdict_core::otel::OTelConfig {
        jsonl_path: args.otel_jsonl.clone(),
        redact_prompts: args.redact_prompts,
    };
    let _ = verdict_core::otel::export_jsonl(&otel_cfg, &cfg.suite, &artifacts.results);

    Ok(decide_exit_code(&artifacts.results, args.strict))
}

async fn cmd_quarantine(args: QuarantineArgs) -> anyhow::Result<i32> {
    ensure_parent_dir(&args.db)?;
    let store = verdict_core::storage::Store::open(&args.db)?;
    store.init_schema()?;
    let svc = verdict_core::quarantine::QuarantineService::new(store);

    match args.cmd {
        QuarantineSub::Add { test_id, reason } => {
            svc.add(&args.suite, &test_id, &reason)?;
            eprintln!("quarantine added: suite={} test_id={}", args.suite, test_id);
        }
        QuarantineSub::Remove { test_id } => {
            svc.remove(&args.suite, &test_id)?;
            eprintln!(
                "quarantine removed: suite={} test_id={}",
                args.suite, test_id
            );
        }
        QuarantineSub::List => {
            eprintln!("quarantine list: TODO (skeleton)");
        }
    }
    Ok(exit_codes::OK)
}

fn decide_exit_code(results: &[verdict_core::model::TestResultRow], strict: bool) -> i32 {
    use verdict_core::model::TestStatus;
    let mut has_fail = false;
    let mut has_error = false;
    let mut has_warn = false;
    let mut has_flaky = false;
    let mut has_config_error = false;

    for r in results {
        match r.status {
            TestStatus::Pass => {}
            TestStatus::Warn => has_warn = true,
            TestStatus::Flaky => has_flaky = true,
            TestStatus::Fail => has_fail = true,
            TestStatus::Error => {
                 if r.message.starts_with("config error:") {
                     has_config_error = true;
                 }
                 has_error = true;
            },
        }
    }

    if has_config_error {
        return exit_codes::CONFIG_ERROR;
    }

    if has_error || has_fail {
        return exit_codes::TEST_FAILED;
    }

    if strict && (has_warn || has_flaky) {
        return exit_codes::TEST_FAILED;
    }

    exit_codes::OK
}

async fn build_runner(
    db_path: &std::path::Path,
    trace_file: &Option<PathBuf>,
    cfg: &verdict_core::model::EvalConfig,
    rerun_failures_arg: u32,
    quarantine_mode_str: &str,
) -> anyhow::Result<verdict_core::engine::runner::Runner> {
    let store = verdict_core::storage::Store::open(db_path)?;
    store.init_schema()?;
    let cache = verdict_core::cache::vcr::VcrCache::new(store.clone());

    let client: Arc<dyn verdict_core::providers::llm::LlmClient> =
        if let Some(trace_path) = trace_file {
            Arc::new(
                verdict_core::providers::trace::TraceClient::from_path(trace_path)
                    .map_err(|e| anyhow::anyhow!(e))?,
            )
        } else {
            Arc::new(DummyClient::new(&cfg.model))
        };
    let metrics = verdict_metrics::default_metrics();

    let replay_mode = trace_file.is_some();
    let rerun_failures = if replay_mode {
        if rerun_failures_arg > 0 {
            eprintln!("note: replay mode active; forcing --rerun-failures=0 for determinism");
        }
        0
    } else {
        rerun_failures_arg
    };

    let policy = verdict_core::engine::runner::RunPolicy {
        rerun_failures,
        quarantine_mode: verdict_core::quarantine::QuarantineMode::parse(quarantine_mode_str),
    };

    Ok(verdict_core::engine::runner::Runner {
        store,
        cache,
        client,
        metrics,
        policy,
    })
}

fn ensure_parent_dir(path: &std::path::Path) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}

#[derive(Clone)]
struct DummyClient {
    model: String,
}

impl DummyClient {
    fn new(model: &str) -> Self {
        Self {
            model: model.to_string(),
        }
    }
}

#[async_trait::async_trait]
impl verdict_core::providers::llm::LlmClient for DummyClient {
    async fn complete(
        &self,
        prompt: &str,
        _context: Option<&[String]>,
    ) -> anyhow::Result<verdict_core::model::LlmResponse> {
        let text = format!("hello from {} :: {}", self.model, prompt);
        Ok(verdict_core::model::LlmResponse {
            text,
            provider: self.provider_name().to_string(),
            model: self.model.clone(),
            cached: false,
            meta: serde_json::json!({"dummy": true}),
        })
    }

    fn provider_name(&self) -> &'static str {
        "dummy"
    }
}
