# Python SDK Reference

The `assay-it` package allows you to record traces from your Python agents for use with Assay.

## Installation

```bash
pip install assay-it
```

## Quick Start

Wrap your OpenAI client to automatically record traces:

```python
import os
from assay import record_chat_completions_with_tools, TraceWriter
import openai

# 1. Setup
client = openai.OpenAI()
writer = TraceWriter("traces/my_trace.jsonl")

# 2. Record
result = record_chat_completions_with_tools(
    writer=writer,
    client=client,
    model="gpt-4o",
    messages=[{"role": "user", "content": "Hello"}],
    tools=[...],
    tool_executors={...},
    test_id="basic_greeting"
)
```

## Core Functions

### `record_chat_completions_with_tools`

Executes the agent loop (call LLM -> execute tools -> call LLM) and records the entire interaction to the `writer`.

**Arguments:**

*   `writer`: `TraceWriter` instance.
*   `client`: `openai.Client`.
*   `model`: Model ID string.
*   `messages`: List of initial messages.
*   `tools`: JSON schema for tools.
*   `tool_executors`: Dictionary mapping tool names to python functions.
*   `test_id`: (Optional) ID to link this trace to a test case in `eval.yaml`.

### `TraceWriter`

Handles writing traces to disk in the correct JSONL format.

```python
writer = TraceWriter("path/to/trace.jsonl", mode="a") # Append mode
```

## Integrations

### LangChain / LlamaIndex

For framework integration, we recommend using the `assay import` command with OpenTelemetry or custom callbacks, rather than wrapping the client directly, as frameworks often abstract the client access.

See [Importing Traces](../cli/import.md).
