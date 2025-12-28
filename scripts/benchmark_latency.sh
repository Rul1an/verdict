#!/bin/bash
# Assay Latency Benchmark
#
# Measures p50/p95/p99 latency for policy checks.
# Used to verify v1.0 SLA: p95 < 10ms
#
# Usage:
#   ./benchmark_latency.sh [iterations] [policy_file]
#
# Example:
#   ./benchmark_latency.sh 1000 policies/full.yaml
#
# Requirements:
#   - assay CLI built and in PATH
#   - hyperfine (optional, for better stats)
#   - jq (for JSON parsing)

set -euo pipefail

# Configuration
ITERATIONS="${1:-1000}"
POLICY_FILE="${2:-examples/agent-demo-1/demo.yaml}"
TRACE_FILE="${3:-examples/agent-demo-1/trace.jsonl}"
OUTPUT_DIR="benchmark_results"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo "========================================"
echo "Assay Latency Benchmark"
echo "========================================"
echo "Iterations: $ITERATIONS"
echo "Policy:     $POLICY_FILE"
echo "Trace:      $TRACE_FILE"
echo "Timestamp:  $TIMESTAMP"
echo ""

# Verify prerequisites
if ! command -v assay &> /dev/null; then
    echo -e "${RED}Error: 'assay' CLI not found in PATH${NC}"
    echo "Build with: cargo build --release -p assay-cli"
    exit 1
fi

mkdir -p "$OUTPUT_DIR"

# ============================================================
# Method 1: Simple timing loop (always available)
# ============================================================
echo -e "${BLUE}[1/3] Running simple timing loop...${NC}"

TIMINGS_FILE="$OUTPUT_DIR/timings_${TIMESTAMP}.txt"

for i in $(seq 1 "$ITERATIONS"); do
    # Time a single run in milliseconds
    START=$(date +%s%N)
    assay run --config "$POLICY_FILE" --trace-file "$TRACE_FILE" --quiet > /dev/null 2>&1 || true
    END=$(date +%s%N)
    
    # Calculate duration in milliseconds
    DURATION_NS=$((END - START))
    DURATION_MS=$(echo "scale=3; $DURATION_NS / 1000000" | bc)
    
    echo "$DURATION_MS" >> "$TIMINGS_FILE"
    
    # Progress indicator
    if (( i % 100 == 0 )); then
        echo "  Completed $i / $ITERATIONS"
    fi
done

echo ""

# ============================================================
# Method 2: Calculate percentiles
# ============================================================
echo -e "${BLUE}[2/3] Calculating percentiles...${NC}"

# Sort timings and calculate percentiles
SORTED_FILE="$OUTPUT_DIR/sorted_${TIMESTAMP}.txt"
sort -n "$TIMINGS_FILE" > "$SORTED_FILE"

TOTAL=$(wc -l < "$SORTED_FILE")
P50_LINE=$((TOTAL * 50 / 100))
P95_LINE=$((TOTAL * 95 / 100))
P99_LINE=$((TOTAL * 99 / 100))

P50=$(sed -n "${P50_LINE}p" "$SORTED_FILE")
P95=$(sed -n "${P95_LINE}p" "$SORTED_FILE")
P99=$(sed -n "${P99_LINE}p" "$SORTED_FILE")
MIN=$(head -1 "$SORTED_FILE")
MAX=$(tail -1 "$SORTED_FILE")
MEAN=$(awk '{ sum += $1 } END { printf "%.3f", sum/NR }' "$SORTED_FILE")

echo ""
echo "========================================"
echo "RESULTS"
echo "========================================"
echo ""
printf "%-12s %10s\n" "Metric" "Value (ms)"
printf "%-12s %10s\n" "--------" "----------"
printf "%-12s %10s\n" "Min" "$MIN"
printf "%-12s %10s\n" "p50" "$P50"
printf "%-12s %10s\n" "Mean" "$MEAN"
printf "%-12s %10s\n" "p95" "$P95"
printf "%-12s %10s\n" "p99" "$P99"
printf "%-12s %10s\n" "Max" "$MAX"
echo ""

# ============================================================
# Method 3: SLA Check
# ============================================================
echo -e "${BLUE}[3/3] SLA Verification...${NC}"
echo ""

# SLA Targets
P50_TARGET=2
P95_TARGET=10
P99_TARGET=50

check_sla() {
    local metric="$1"
    local value="$2"
    local target="$3"
    
    # Compare floats
    if (( $(echo "$value <= $target" | bc -l) )); then
        echo -e "  $metric: ${GREEN}PASS${NC} ($value ms <= $target ms)"
        return 0
    else
        echo -e "  $metric: ${RED}FAIL${NC} ($value ms > $target ms)"
        return 1
    fi
}

SLA_PASS=true

echo "SLA Checks:"
check_sla "p50" "$P50" "$P50_TARGET" || SLA_PASS=false
check_sla "p95" "$P95" "$P95_TARGET" || SLA_PASS=false
check_sla "p99" "$P99" "$P99_TARGET" || SLA_PASS=false

echo ""

# ============================================================
# Save results
# ============================================================
RESULTS_JSON="$OUTPUT_DIR/benchmark_${TIMESTAMP}.json"

cat > "$RESULTS_JSON" << EOF
{
  "timestamp": "$TIMESTAMP",
  "config": {
    "iterations": $ITERATIONS,
    "policy_file": "$POLICY_FILE",
    "trace_file": "$TRACE_FILE"
  },
  "results": {
    "min_ms": $MIN,
    "p50_ms": $P50,
    "mean_ms": $MEAN,
    "p95_ms": $P95,
    "p99_ms": $P99,
    "max_ms": $MAX
  },
  "sla": {
    "p50_target_ms": $P50_TARGET,
    "p95_target_ms": $P95_TARGET,
    "p99_target_ms": $P99_TARGET,
    "p50_pass": $(echo "$P50 <= $P50_TARGET" | bc -l),
    "p95_pass": $(echo "$P95 <= $P95_TARGET" | bc -l),
    "p99_pass": $(echo "$P99 <= $P99_TARGET" | bc -l)
  }
}
EOF

echo "Results saved to: $RESULTS_JSON"
echo ""

# ============================================================
# Final verdict
# ============================================================
if [ "$SLA_PASS" = true ]; then
    echo -e "${GREEN}========================================"
    echo "✓ ALL SLA TARGETS MET"
    echo "========================================${NC}"
    echo ""
    echo "v1.0 latency requirements satisfied."
    exit 0
else
    echo -e "${RED}========================================"
    echo "✗ SLA TARGETS NOT MET"
    echo "========================================${NC}"
    echo ""
    echo "This is a BLOCKER for v1.0 release."
    echo ""
    echo "Investigate:"
    echo "  1. Check for cold start overhead (first few runs)"
    echo "  2. Profile with 'cargo flamegraph'"
    echo "  3. Review JSON Schema validation complexity"
    echo "  4. Check disk I/O (SQLite cache)"
    exit 1
fi
