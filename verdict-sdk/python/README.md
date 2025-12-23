# Verdict Python SDK

**Record deterministic traces from your Python agents for regression gating.**

## Installation

```bash
pip install verdict-sdk
```

Optional: Install with OpenAI support:
```bash
pip install verdict-sdk[openai]
```

## Quick Start (OpenAI)

Record a trace using the instrumentor:

```python
import openai
from verdict_sdk import TraceWriter, record_chat_completions_with_tools

client = openai.OpenAI()
writer = TraceWriter("traces/agent.jsonl")

# Executes loop and records all tool calls/results automatically
result = record_chat_completions_with_tools(
    writer=writer,
    client=client,
    model="gpt-4o",
    messages=[{"role": "user", "content": "What's the weather in Tokyo?"}],
    tool_executors={"get_weather": lambda args: {"temp": 22}},
    episode_id="weather_demo"
)
```

## Run Regression Gate

Use the Verdict CLI to validate the recorded trace:

```bash
verdict ci \
  --config verdict.yaml \
  --trace-file traces/agent.jsonl \
  --replay-strict \
  --db :memory:
```

## Features

- **EpisodeRecorder**: Context manager for manual trace recording.
- **TraceWriter**: Deterministic, append-only JSONL generator with sorted keys.
- **OpenAI Instrumentor**: Seamless bridge between OpenAI's SDK and Verdict's storage format.
- **Python 3.8+ Compatible**: Supports modern and legacy Python environments.
