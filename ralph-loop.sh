#!/usr/bin/env bash
# ralph-loop.sh — Autonomous implementation loop for ezpdf v1
# Based on: ~/Library/CloudStorage/Dropbox/ObsidianVault/guides/ralph-loop.md
#
# Usage:
#   chmod +x ralph-loop.sh
#   ./ralph-loop.sh
#
# Stop with: Ctrl+C  (or see "Killing a Stuck Loop" below)
#
# Monitor in another terminal:
#   tail -f ralph-loop.log
#
# Killing a stuck loop:
#   ps aux | grep -E "ralph|claude.*continue" | grep -v grep | awk '{print $2}' | xargs kill -9

set -euo pipefail

WORKSPACE="$(cd "$(dirname "$0")" && pwd)"
PROMPT_FILE="$WORKSPACE/PROMPT.md"
LOG_FILE="$WORKSPACE/ralph-loop.log"
COMPLETION_SIGNAL="EZPDF V1 COMPLETE"
MAX_ITERATIONS=0   # 0 = unlimited
ITERATION=0

# Sanity checks
if [[ ! -f "$PROMPT_FILE" ]]; then
  echo "ERROR: PROMPT.md not found at $PROMPT_FILE"
  exit 1
fi

if [[ ! -f "$WORKSPACE/task_plan.md" ]]; then
  echo "ERROR: task_plan.md not found at $WORKSPACE/task_plan.md"
  exit 1
fi

echo "=== ezpdf ralph-loop starting ===" | tee -a "$LOG_FILE"
echo "Workspace: $WORKSPACE" | tee -a "$LOG_FILE"
echo "Prompt: $PROMPT_FILE" | tee -a "$LOG_FILE"
echo "Log: $LOG_FILE" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"

while :; do
  ITERATION=$((ITERATION + 1))
  echo "--- Iteration $ITERATION [$(date '+%Y-%m-%d %H:%M:%S')] ---" | tee -a "$LOG_FILE"

  # Heartbeat dots so the terminal doesn't look frozen (dots every 10s = alive)
  ( while true; do sleep 10; printf "."; done ) &
  HEARTBEAT_PID=$!

  OUTPUT=$(claude \
    --dangerously-skip-permissions \
    --continue \
    --output-format stream-json \
    --verbose \
    -p "$(cat "$PROMPT_FILE")" 2>&1 | tee -a "$LOG_FILE")

  kill "$HEARTBEAT_PID" 2>/dev/null
  wait "$HEARTBEAT_PID" 2>/dev/null || true
  printf "\n"

  if echo "$OUTPUT" | grep -qF "<promise>$COMPLETION_SIGNAL</promise>"; then
    echo "" | tee -a "$LOG_FILE"
    echo "=== ✅ Done after $ITERATION iteration(s). ===" | tee -a "$LOG_FILE"
    echo "=== ezpdf v1 is complete! ===" | tee -a "$LOG_FILE"
    exit 0
  fi

  if [[ "$MAX_ITERATIONS" -gt 0 && "$ITERATION" -ge "$MAX_ITERATIONS" ]]; then
    echo "=== Max iterations ($MAX_ITERATIONS) reached. ===" | tee -a "$LOG_FILE"
    exit 0
  fi

  # Brief pause between iterations to avoid hammering the API
  sleep 2
done
