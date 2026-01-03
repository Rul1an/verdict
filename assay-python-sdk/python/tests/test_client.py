import pytest
import tempfile
import json
import os
from assay import AssayClient

def test_record_trace_success():
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

def test_record_trace_no_file():
    # Client initialized without file
    client = AssayClient(None)

    # Should raise RuntimeError (mapped from PyRuntimeError)
    with pytest.raises(RuntimeError, match="trace_file is not configured"):
        client.record_trace({"foo": "bar"})

def test_record_trace_invalid_object():
    with tempfile.NamedTemporaryFile(suffix=".jsonl", delete=False) as tmp:
        path = tmp.name

    try:
        client = AssayClient(path)

        class NotSerializable:
            pass

        # Should raise ValueError (mapped from PyValueError)
        # However, pythonize or serde might raise different errors.
        # Simple non-serializable might be a set (which isn't strictly JSON, but maybe supported?)
        # A circular reference or custom object without dict.

        # Let's try a set, usually not valid JSON unless special handling.
        # Or better, an object.
        with pytest.raises(ValueError):
           client.record_trace(NotSerializable())

    finally:
        if os.path.exists(path):
            os.remove(path)
