#!/usr/bin/env bash
# Claude Code PreToolUse hook (Bash tool): before a `git commit` or `git push`,
# make sure this clone's secret-scanning git hooks are on (core.hooksPath =
# .githooks) and gitleaks is installed. It never blocks the command itself;
# the git hooks in .githooks/ do the scanning and block on a finding.
#
# Input: the hook event JSON on stdin; the command is .tool_input.command.

input="$(cat)"

if command -v jq >/dev/null 2>&1; then
  cmd="$(printf '%s' "$input" | jq -r '.tool_input.command // empty' 2>/dev/null)"
elif command -v python3 >/dev/null 2>&1; then
  cmd="$(printf '%s' "$input" | python3 -c 'import json,sys; print(json.load(sys.stdin).get("tool_input",{}).get("command",""))' 2>/dev/null)"
else
  cmd="$input"  # no JSON parser: match on the raw event (may over-match, harmless)
fi

# Act only when the command runs git commit / git push (incl. `git -C dir push`).
printf '%s' "$cmd" | grep -Eq '(^|[^[:alnum:]_./-])git([[:space:]]+-[^[:space:]]+([[:space:]]+[^-[:space:]][^[:space:]]*)?)*[[:space:]]+(commit|push)([[:space:]]|$)' || exit 0

root="${CLAUDE_PROJECT_DIR:-$(pwd)}"
root="$(git -C "$root" rev-parse --show-toplevel 2>/dev/null)" || exit 0
[ -f "$root/.githooks/gitleaks-env.sh" ] || exit 0
# shellcheck source=SCRIPTDIR/../../.githooks/gitleaks-env.sh
. "$root/.githooks/gitleaks-env.sh"

if [ "$(git -C "$root" config --get core.hooksPath)" != ".githooks" ] || ! find_gitleaks >/dev/null; then
  echo "ensure-secret-hooks: enabling secret-scanning git hooks via scripts/setup-hooks.sh" >&2
  if ! "$root/scripts/setup-hooks.sh" >&2; then
    # Setup failed (e.g. offline). Still make sure the hooks are on: they fail
    # closed without gitleaks, so the commit/push is blocked, not unscanned.
    git -C "$root" config --local core.hooksPath .githooks
  fi
fi
exit 0
