# shellcheck shell=bash
# Shared settings (bash) for the secret-scanning git hooks and scripts/setup-hooks.sh.
# Sourced, not run. Git only runs files named after hooks, so this file in
# .githooks/ is never executed as a hook itself.
#
# Keep GITLEAKS_VERSION in step with the `secrets` job in
# .github/workflows/pr-checks.yml so local hooks and CI agree.

GITLEAKS_VERSION="8.30.1"

# Where scripts/setup-hooks.sh installs gitleaks, and where the hooks look first.
GITLEAKS_INSTALL_DIR="${GITLEAKS_INSTALL_DIR:-$HOME/.local/bin}"

# sha256 of each release tarball, from gitleaks_8.30.1_checksums.txt on the
# official release. linux_x64 is the same value CI pins.
gitleaks_sha256() {
  case "$1" in
    darwin_arm64) echo "b40ab0ae55c505963e365f271a8d3846efbc170aa17f2607f13df610a9aeb6a5" ;;
    darwin_x64) echo "dfe101a4db2255fc85120ac7f3d25e4342c3c20cf749f2c20a18081af1952709" ;;
    linux_arm64) echo "e4a487ee7ccd7d3a7f7ec08657610aa3606637dab924210b3aee62570fb4b080" ;;
    linux_x64) echo "551f6fc83ea457d62a0d98237cbad105af8d557003051f41f3e7ca7b3f2470eb" ;;
    *) return 1 ;;
  esac
}

# Prints the path of a gitleaks binary at exactly GITLEAKS_VERSION, or fails.
# Checks the user-local install first so a different version on PATH (e.g.
# Homebrew) doesn't shadow it.
find_gitleaks() {
  for candidate in "$GITLEAKS_INSTALL_DIR/gitleaks" "$(command -v gitleaks 2>/dev/null)"; do
    if [ -z "$candidate" ] || [ ! -x "$candidate" ]; then
      continue
    fi
    if [ "$("$candidate" version 2>/dev/null)" = "$GITLEAKS_VERSION" ]; then
      echo "$candidate"
      return 0
    fi
  done
  return 1
}

# Fills the GITLEAKS_REPO_ARGS array with the repo's gitleaks config and
# allowlist, the same files CI uses (bash: callers are bash scripts).
gitleaks_repo_args() {
  local root="$1"
  GITLEAKS_REPO_ARGS=()
  if [ -f "$root/.gitleaks.toml" ]; then
    GITLEAKS_REPO_ARGS+=("--config=$root/.gitleaks.toml")
  fi
  if [ -f "$root/.github/gitleaks/.gitleaksignore" ]; then
    GITLEAKS_REPO_ARGS+=("--gitleaks-ignore-path=$root/.github/gitleaks/.gitleaksignore")
  fi
}

gitleaks_missing_msg() {
  echo "secret scan: gitleaks $GITLEAKS_VERSION not found (missing or wrong version); run scripts/setup-hooks.sh and retry." >&2
}
