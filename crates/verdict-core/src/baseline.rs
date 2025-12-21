use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Baseline {
    pub schema_version: u32,
    pub suite: String,
    pub verdict_version: String,
    pub created_at: String,
    pub config_fingerprint: String,
    pub entries: Vec<BaselineEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BaselineEntry {
    pub test_id: String,
    pub metric: String,
    pub score: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl Baseline {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let file = File::open(path)
            .with_context(|| format!("failed to open baseline file: {}", path.display()))?;
        let baseline: Baseline =
            serde_json::from_reader(file).context("failed to parse baseline JSON")?;

        if baseline.schema_version != 1 {
            anyhow::bail!(
                "config error: unsupported baseline schema version {}",
                baseline.schema_version
            );
        }

        // Hardening: Suite mismatch
        // Note: We need access to current suite name to check this.
        // `load` currently only takes path.
        // We might need to change signature of `load` OR check it outside.
        // Let's keep `load` simple-ish but maybe we add verification methods?
        // Actually, the plan said "Hardening (Exit 2 on schema/suite mismatch)".
        // If we want to check suite, we need the expected suite.
        // Let's modify `load` signature? No, that breaks callers unless we update them.
        // `load` is called in `commands.rs`. We can check suite *after* load there.
        // BUT `schema_version` is structural, so `load` handles it.
        // Let's stick to `schema_version` here. Suite check should be in `commands.rs` or `runner.rs` where we have context.
        // Wait, the plan said "Core: Hardening".
        // Let's verify compatibility *in* `baseline.rs` but maybe as a separate method `validate(&self, current_suite: &str, current_version: &str)`.

        Ok(baseline)
    }

    pub fn validate(&self, current_suite: &str) -> Result<()> {
        if self.suite != current_suite {
            anyhow::bail!(
                "config error: baseline suite mismatch (expected '{}', found '{}')",
                current_suite,
                self.suite
            );
        }

        let current_ver = env!("CARGO_PKG_VERSION");
        if self.verdict_version != current_ver {
            eprintln!(
                "warning: baseline generated with verdict v{} (current: v{})",
                self.verdict_version, current_ver
            );
        }

        Ok(())
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let file = File::create(path)
            .with_context(|| format!("failed to create baseline file: {}", path.display()))?;
        // Use pretty print for git diffability
        serde_json::to_writer_pretty(file, self).context("failed to write baseline JSON")?;
        Ok(())
    }

    // Helper to get score for a test+metric
    pub fn get_score(&self, test_id: &str, metric: &str) -> Option<f64> {
        self.entries
            .iter()
            .find(|e| e.test_id == test_id && e.metric == metric)
            .map(|e| e.score)
    }
}

// Fingerprint logic could go here or in util
pub fn compute_config_fingerprint(config_path: &Path) -> String {
    // For MVP, just hash the config file content if it exists.
    // In future, canonicalize logic.
    if let Ok(content) = std::fs::read(config_path) {
        let digest = md5::compute(content);
        format!("md5:{:x}", digest)
    } else {
        "md5:unknown".to_string()
    }
}
