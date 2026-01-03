import pytest
import tempfile
import json
import os
from assay import AssayClient

def test_record_trace():
    with tempfile.NamedTemporaryFile(suffix=".jsonl", delete=False) as tmp:
        path = tmp.name

    try:
        client = AssayClient(path)

        # Record trace 1
        trace1 = {"type": "call_tool", "tool": "ToolA", "test_id": "t1"}
        client.record_trace(trace1)

        # Record trace 2
        trace2 = {"type": "call_tool", "tool": "ToolB", "test_id": "t1"}
        client.record_trace(trace2)

        # Verify file content
        with open(path, "r") as f:
            lines = f.readlines()

        assert len(lines) == 2
        obj1 = json.loads(lines[0])
        obj2 = json.loads(lines[1])

        assert obj1["tool"] == "ToolA"
        assert obj2["tool"] == "ToolB"

    finally:
        if os.path.exists(path):
            os.remove(path)
