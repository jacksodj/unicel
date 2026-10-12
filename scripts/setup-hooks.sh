#!/usr/bin/env bash
# One-time (idempotent) setup for a clone: turns on the secret-scanning git
# hooks in .githooks/ and installs the pinned gitleaks if it is missing.
#
#   scripts/setup-hooks.sh
#
# Run it in every clone: your laptop, a cloud agent's environment setup, a
# codespace. Safe to re-run. Installs gitleaks into $GITLEAKS_INSTALL_DIR
# (default ~/.local/bin); nothing is installed system-wide.
set -euo pipefail

root="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
# shellcheck source=SCRIPTDIR/../.githooks/gitleaks-env.sh
. "$root/.githooks/gitleaks-env.sh"

# 1. Point git at the tracked hooks.
if [ "$(git -C "$root" config --local --get core.hooksPath || true)" != ".githooks" ]; then
  git -C "$root" config --local core.hooksPath .githooks
fi
chmod +x "$root/.githooks/pre-commit" "$root/.githooks/pre-push" 2>/dev/null || true

# 2. Install gitleaks if the pinned version isn't already available.
if ! gitleaks="$(find_gitleaks)"; then
  case "$(uname -s)" in
    Darwin) os=darwin ;;
    Linux) os=linux ;;
    *) echo "setup-hooks: unsupported OS $(uname -s); install gitleaks $GITLEAKS_VERSION manually into $GITLEAKS_INSTALL_DIR." >&2; exit 1 ;;
  esac
  case "$(uname -m)" in
    x86_64 | amd64) arch=x64 ;;
    arm64 | aarch64) arch=arm64 ;;
    *) echo "setup-hooks: unsupported CPU $(uname -m); install gitleaks $GITLEAKS_VERSION manually into $GITLEAKS_INSTALL_DIR." >&2; exit 1 ;;
  esac
  platform="${os}_${arch}"
  expected="$(gitleaks_sha256 "$platform")"
  tarball="gitleaks_${GITLEAKS_VERSION}_${platform}.tar.gz"
  url="https://github.com/gitleaks/gitleaks/releases/download/v${GITLEAKS_VERSION}/${tarball}"

  tmp="$(mktemp -d)"
  trap 'rm -rf "$tmp"' EXIT
  echo "setup-hooks: downloading $tarball"
  curl -fsSL --retry 3 -o "$tmp/$tarball" "$url"

  if command -v sha256sum >/dev/null 2>&1; then
    actual="$(sha256sum "$tmp/$tarball" | awk '{print $1}')"
  else
    actual="$(shasum -a 256 "$tmp/$tarball" | awk '{print $1}')"
  fi
  if [ "$actual" != "$expected" ]; then
    echo "setup-hooks: checksum mismatch for $tarball (expected $expected, got $actual); not installing." >&2
    exit 1
  fi

  tar -xzf "$tmp/$tarball" -C "$tmp" gitleaks
  mkdir -p "$GITLEAKS_INSTALL_DIR"
  install -m 0755 "$tmp/gitleaks" "$GITLEAKS_INSTALL_DIR/gitleaks"
  gitleaks="$(find_gitleaks)" || { echo "setup-hooks: installed gitleaks but it does not report version $GITLEAKS_VERSION." >&2; exit 1; }
fi

echo "setup-hooks: core.hooksPath=$(git -C "$root" config --get core.hooksPath); gitleaks $("$gitleaks" version) at $gitleaks"
echo "setup-hooks: commits and pushes in $root are now scanned for secrets."
