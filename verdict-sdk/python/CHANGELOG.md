# Changelog

All notable changes to the Verdict Python SDK.

## [0.3.0] - 2025-12-23

### Features
*   **Redaction Hooks**: `TraceWriter` now supports a `redact_fn` for "last mile" filtering of sensitive data before writing to disk.
*   **Streaming Support**: Added `record_chat_completions_stream` and `record_chat_completions_stream_with_tools` to capture and aggregate OpenAI `stream=True` responses.
*   **Edge Case Hardening**: Improved robustness for fragmented tool arguments, interleaved tool calls, and case-insensitive header redaction.

### Internal
*   **Performance**: Optimized `StreamAccumulator` to use O(1) ID lookup for large streams.
*   **Packaging**: Confirmed compatibility with `pip install .[openai]` without `PYTHONPATH`.

## [0.2.0] - 2025-12-21

### Features
*   **Tool Loop Support**: Added `record_chat_completions_with_tools` for automatic tool execution and capture in a loop.
*   **Determinism**: `TraceWriter` enforces sorted keys and byte-stable JSON serialization.
