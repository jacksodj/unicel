#!/bin/sh

set -e

echo "========================================="
echo "Setting Build Environment Variables"
echo "========================================="

# Add Node.js to PATH (installed by post-clone script)
export PATH="/usr/local/opt/node@20/bin:$PATH"

# Add Rust/Cargo to PATH (installed by post-clone script)
export PATH="$HOME/.cargo/bin:$PATH"

# Start the lightweight options daemon that mimics `tauri-cli` socket hand-off
APP_IDENTIFIER="com.unicel.app"
ADDR_FILENAME="${APP_IDENTIFIER}-server-addr"
WORK_TMP="${CI_WORKSPACE:-/Volumes/workspace}/tmp"
mkdir -p "$WORK_TMP"

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || (cd "$(dirname "$0")/../../.." && pwd))"
DAEMON_BIN="$REPO_ROOT/target/release/tauri-options-daemon"
DAEMON_LOG="$WORK_TMP/tauri-options-daemon.log"

echo "Starting Tauri CLI options daemon..."
echo "  - Building helper binary (release)..."
(cd "$REPO_ROOT" && cargo build -p tauri-options-daemon --release)

if [ ! -x "$DAEMON_BIN" ]; then
    echo "  ✗ Helper binary not found at $DAEMON_BIN"
    exit 1
fi

UNICEL_REPO_ROOT="$REPO_ROOT" nohup "$DAEMON_BIN" > "$DAEMON_LOG" 2>&1 &
DAEMON_PID=$!
echo "$DAEMON_PID" > "$WORK_TMP/tauri-options-daemon.pid"

SERVER_ADDR_FILE_FOUND=""

# Helper to normalize and patch a candidate addr file
patch_addr_file() {
    file="$1"
    if [ ! -f "$file" ]; then
        return
    fi
    SERVER_ADDR_FILE_FOUND="$file"

    raw=""
    raw="$(cat "$file" 2>/dev/null | tr -d '\r\n')"
    if [ -z "$raw" ]; then
        return
    fi

    host="${raw%:*}"
    port="${raw##*:}"
    if [ -z "$host" ] || [ "$host" = "$raw" ] || [ -z "$port" ]; then
        return
    fi

    if printf '%s' "$host" | grep -Eq '^[0-9.]+$'; then
        echo "  ✓ CLI server address already uses IPv4 ($raw)"
        return
    fi

    echo "  - Overriding CLI server host '$host' → 127.0.0.1 ($file)"
    printf '127.0.0.1:%s' "$port" > "$file"
}

# Wait for the daemon to write the address file
ADDR_DIRS=""
if [ -n "${TMPDIR:-}" ]; then
    ADDR_DIRS="${ADDR_DIRS} ${TMPDIR%/}"
fi
if [ -n "${CI_WORKSPACE:-}" ]; then
    ADDR_DIRS="${ADDR_DIRS} ${CI_WORKSPACE%/}/tmp"
fi
ADDR_DIRS="${ADDR_DIRS} /tmp"

for _ in $(seq 1 30); do
    for dir in $ADDR_DIRS; do
        patch_addr_file "${dir%/}/$ADDR_FILENAME"
        if [ -n "$SERVER_ADDR_FILE_FOUND" ]; then
            break 2
        fi
    done
    sleep 1
done

if [ -z "$SERVER_ADDR_FILE_FOUND" ]; then
    echo "  ✗ Failed to detect CLI options daemon address file ($ADDR_FILENAME)"
    echo "  - Daemon log (tail):"
    tail -n 20 "$DAEMON_LOG" 2>/dev/null || true
    exit 1
else
    echo "  ✓ CLI options daemon ready ($SERVER_ADDR_FILE_FOUND)"
fi

# Verify tools are accessible
echo "  ✓ node: $(node --version 2>/dev/null || echo 'NOT FOUND')"
echo "  ✓ npm: $(npm --version 2>/dev/null || echo 'NOT FOUND')"
echo "  ✓ rustc: $(rustc --version 2>/dev/null || echo 'NOT FOUND')"
echo "  ✓ cargo: $(cargo --version 2>/dev/null || echo 'NOT FOUND')"

echo "========================================="
echo "✓ Build environment configured"
echo "========================================="
