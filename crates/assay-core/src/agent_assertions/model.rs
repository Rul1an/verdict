use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TraceAssertion {
    #[serde(rename = "trace_must_call_tool")]
    TraceMustCallTool {
        tool: String,
        min_calls: Option<u32>,
    },
    #[serde(rename = "trace_must_not_call_tool")]
    TraceMustNotCallTool { tool: String },
    #[serde(rename = "trace_tool_sequence")]
    TraceToolSequence {
        sequence: Vec<String>,
        allow_other_tools: bool,
    },
    #[serde(rename = "trace_max_steps")]
    TraceMaxSteps { max: u32 },
    #[serde(rename = "args_valid")]
    ArgsValid {
        tool: String,
        #[serde(default)]
        test_args: Option<serde_json::Value>,
        #[serde(default)]
        policy: Option<serde_json::Value>,
        #[serde(default)]
        expect: Option<String>,
    },
    #[serde(rename = "sequence_valid")]
    SequenceValid {
        #[serde(default)]
        test_trace: Option<Vec<crate::storage::rows::ToolCallRow>>, // Reusing existing struct or simplified Value
        // If the user uses simplified structure in yaml, we might need a custom struct or Value.
        // fp_suite uses: - tool: VerifyIdentity, args: {}
        // ToolCallRow is a bit heavy, let's use Value for flexibility if model mismatch.
        // But for safety, let's look at strict parsing.
        // Example: { tool: "VerifyIdentity", args: {} }
        #[serde(default)]
        test_trace_raw: Option<Vec<serde_json::Value>>,
        #[serde(default)]
        policy: Option<serde_json::Value>,
        #[serde(default)]
        expect: Option<String>,
    },
    #[serde(rename = "tool_blocklist")]
    ToolBlocklist {
        #[serde(default)]
        test_tool_calls: Option<Vec<String>>,
        #[serde(default)]
        policy: Option<serde_json::Value>,
        #[serde(default)]
        expect: Option<String>,
    },
}
