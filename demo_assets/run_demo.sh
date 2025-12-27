#!/bin/bash
# Assay Demo "Actor" Script
# Usage: ./run_demo.sh
# Dependencies: 'pv' (brew install pv) for typing effect. If not found, falls back to direct echo.

# Colors
GREEN='\033[0;32m'
CYAN='\033[0;36m'
GREY='\033[0;90m'
RESET='\033[0m'
BOLD='\033[1m'

# Setup
clear
# Check for simulated typing tool 'pv', otherwise define fallback
type_cmd() {
    if command -v pv &> /dev/null; then
        echo -n "$1" | pv -qL 15
    else
        echo -n "$1"
    fi
    echo ""
}

prompt() {
    echo -n -e "${CYAN}➜${RESET} ${CYAN}assay${RESET} "
    sleep 0.8
    type_cmd "$1"
    sleep 0.5
}

# Scenario
echo -e "${GREY}# Step 1: Record a successful session${RESET}"
sleep 1
prompt "import --format mcp-inspector session.json --init"

sleep 0.5
echo "Imported 4 tool calls"
echo "Created: mcp-eval.yaml (with policies)"
echo "Created: traces/session-2025-12-27.jsonl"
echo ""

sleep 2
echo -e "${GREY}# Step 2: Run tests (instant, offline)${RESET}"
sleep 1
prompt "run --config mcp-eval.yaml --strict"

sleep 0.5
echo -e "${BOLD}Assay v0.8.0 — Zero-Flake CI for AI Agents${RESET}"
echo ""
echo "Suite: mcp-basics"
echo "Trace: traces/session-2025-12-27.jsonl"
echo ""

# Simulate results appearing fast but readable
echo "┌───────────────────┬────────┬─────────────────────────┐"
echo "│ Test              │ Status │ Details                 │"
echo "├───────────────────┼────────┼─────────────────────────┤"
sleep 0.2
echo -e "│ args_valid        │ ${GREEN}✅ PASS${RESET} │ 2ms                     │"
sleep 0.1
echo -e "│ sequence_valid    │ ${GREEN}✅ PASS${RESET} │ 1ms                     │"
sleep 0.1
echo -e "│ tool_blocklist    │ ${GREEN}✅ PASS${RESET} │ 0ms                     │"
echo "└───────────────────┴────────┴─────────────────────────┘"
echo ""
echo -e "Total: ${BOLD}${GREEN}3ms${RESET} | 2 passed, 0 failed"
echo ""
