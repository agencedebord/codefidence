#!/usr/bin/env bash
set -euo pipefail

# Reproducible demo for codefidence v0.2
# Replays the full product loop on the fixture project.
#
# Usage:
#   WIKI_BIN=/path/to/codefidence bash fixtures/run-demo.sh
#
# Or build first:
#   cargo build --release
#   WIKI_BIN=./target/release/codefidence bash fixtures/run-demo.sh

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
FIXTURE="$SCRIPT_DIR/demo-project"
WIKI_BIN="${WIKI_BIN:-codefidence}"

echo "=== codefidence v0.2 — Reproducible Demo ==="
echo ""
echo "Fixture: $FIXTURE"
echo "Binary:  $WIKI_BIN"
echo ""

cd "$FIXTURE"

# Clean any previous run
rm -rf .wiki .git

# Initialize git (check-diff requires a git repo)
git init -q
git add .
git commit -q -m "initial fixture commit"

echo "--- Step 1: Initialize wiki (scan codebase) ---"
$WIKI_BIN init --scan
echo ""

echo "--- Step 2: Generate candidates ---"
$WIKI_BIN generate-candidates
echo ""

echo "--- Step 3: Show candidates ---"
if [ -f .wiki/_candidates.md ]; then
  echo "Candidates found:"
  cat .wiki/_candidates.md
else
  echo "No candidates file generated."
fi
echo ""

echo "--- Step 4: Promote first candidate (if exists) ---"
CANDIDATE_ID=$(grep -oE '### [a-z_]+-[0-9]+' .wiki/_candidates.md 2>/dev/null | head -1 | sed 's/### //' || true)
if [ -n "$CANDIDATE_ID" ]; then
  echo "Promoting: $CANDIDATE_ID"
  $WIKI_BIN promote "$CANDIDATE_ID"
else
  echo "No candidates to promote."
fi
echo ""

echo "--- Step 5: Context for a billing file ---"
$WIKI_BIN context --file src/services/billing/invoice.ts
echo ""

echo "--- Step 6: Check-diff (specific files) ---"
$WIKI_BIN check-diff src/services/billing/invoice.ts src/services/billing/legacy_pricing.ts
echo ""

echo "--- Step 7: Validate wiki health ---"
$WIKI_BIN validate
echo ""

echo "=== Demo complete ==="
