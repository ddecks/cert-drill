#!/usr/bin/env bash
# Integration test for cert-drill — exercises all major non-interactive commands
# against a baseline test exam. Run from repo root.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BINARY="$REPO_ROOT/target/release/cert-drill"
FIXTURES="$REPO_ROOT/tests/fixtures"
PASS=0
FAIL=0

inc_pass() { PASS=$((PASS + 1)); }
inc_fail() { FAIL=$((FAIL + 1)); }

# Work in a temp directory so we don't pollute real data
WORKDIR="$(mktemp -d)"
trap 'rm -rf "$WORKDIR"' EXIT
cd "$WORKDIR"

# Build first
echo "=== Building release binary ==="
cargo build --release --manifest-path "$REPO_ROOT/Cargo.toml" 2>&1 | tail -3

assert_ok() {
    local desc="$1"; shift
    if "$@" > /dev/null 2>&1; then
        echo "  ✅ $desc"
        inc_pass
    else
        echo "  ❌ $desc (exit code $?)"
        inc_fail
    fi
}

assert_fail() {
    local desc="$1"; shift
    if "$@" > /dev/null 2>&1; then
        echo "  ❌ $desc (expected failure, got success)"
        inc_fail
    else
        echo "  ✅ $desc (correctly failed)"
        inc_pass
    fi
}

assert_output_contains() {
    local desc="$1"; shift
    local pattern="$1"; shift
    local output
    output=$("$@" 2>&1) || true
    if echo "$output" | grep -q "$pattern"; then
        echo "  ✅ $desc"
        inc_pass
    else
        echo "  ❌ $desc (expected '$pattern' in output)"
        echo "     got: $(echo "$output" | head -3)"
        inc_fail
    fi
}

echo ""
echo "=== list (empty) ==="
assert_output_contains "list with no exams" "No exams loaded" "$BINARY" list

echo ""
echo "=== load ==="
assert_ok "load test exam" "$BINARY" load "$FIXTURES/test-exam"
assert_ok "load idempotent (reload)" "$BINARY" load "$FIXTURES/test-exam"

echo ""
echo "=== list ==="
assert_output_contains "list shows test-exam" "test-exam" "$BINARY" list
assert_output_contains "list --domains shows domains" "Domain 1" "$BINARY" list --domains
assert_output_contains "list --name detail" "Integration Test Exam" "$BINARY" list --name test-exam

echo ""
echo "=== import (attempt 1: 3/5) ==="
assert_ok "import attempt 1" "$BINARY" import test-exam "$FIXTURES/answers-attempt1.md"

echo ""
echo "=== grade ==="
assert_output_contains "grade shows score" "3/5" "$BINARY" grade test-exam
assert_ok "grade --score-only" "$BINARY" grade test-exam --score-only
assert_output_contains "grade --missed shows wrong" "❌" "$BINARY" grade test-exam --missed

echo ""
echo "=== export ==="
assert_output_contains "export markdown" "test-exam" "$BINARY" export test-exam
assert_output_contains "export --missed" "❌" "$BINARY" export test-exam --missed
assert_output_contains "export --ai-context" "misconceptions" "$BINARY" export test-exam --ai-context

echo ""
echo "=== review ==="
assert_output_contains "review shows results" "Q1" "$BINARY" review test-exam
assert_output_contains "review --missed shows wrong" "❌" "$BINARY" review test-exam --missed

echo ""
echo "=== progress ==="
assert_output_contains "progress shows domains" "Domain 1" "$BINARY" progress test-exam

echo ""
echo "=== import (attempt 2: 4/5) ==="
assert_ok "import attempt 2" "$BINARY" import test-exam "$FIXTURES/answers-attempt2.md"
assert_ok "grade attempt 2" "$BINARY" grade test-exam

echo ""
echo "=== review (all attempts) ==="
assert_output_contains "review shows attempt 1" "Attempt 1" "$BINARY" review test-exam
assert_output_contains "review shows attempt 2" "Attempt 2" "$BINARY" review test-exam

echo ""
echo "=== progress (multi-attempt) ==="
assert_output_contains "progress after 2 attempts" "Domain 2" "$BINARY" progress test-exam

echo ""
echo "=== error handling ==="
assert_fail "grade nonexistent exam" "$BINARY" grade no-such-exam
assert_fail "import nonexistent file" "$BINARY" import test-exam /no/such/file.md

echo ""
echo "==============================="
echo "  Results: $PASS passed, $FAIL failed"
echo "==============================="

[ "$FAIL" -eq 0 ]
