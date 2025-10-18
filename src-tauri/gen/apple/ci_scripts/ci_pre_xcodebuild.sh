#!/bin/sh

set -e

echo "========================================="
echo "Setting Build Environment Variables"
echo "========================================="

# Add Node.js to PATH (installed by post-clone script)
export PATH="/usr/local/opt/node@20/bin:$PATH"

# Add Rust/Cargo to PATH (installed by post-clone script)
export PATH="$HOME/.cargo/bin:$PATH"

# Force the Tauri CLI socket host to resolve locally for Xcode Cloud builds
APP_IDENTIFIER="com.unicel.app"
ADDR_FILENAME="${APP_IDENTIFIER}-server-addr"
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

# Known directories that may contain the addr file
if [ -n "${TMPDIR:-}" ]; then
    patch_addr_file "${TMPDIR%/}/$ADDR_FILENAME"
fi
if [ -n "${CI_WORKSPACE:-}" ]; then
    patch_addr_file "${CI_WORKSPACE%/}/tmp/$ADDR_FILENAME"
fi
patch_addr_file "/tmp/$ADDR_FILENAME"

# Fallback: attempt to locate via find if not already patched
if [ -z "$SERVER_ADDR_FILE_FOUND" ] || ! grep -q '127.0.0.1' "$SERVER_ADDR_FILE_FOUND" 2>/dev/null; then
    found_file="$(find "${CI_WORKSPACE:-/Volumes/workspace}" -name "$ADDR_FILENAME" 2>/dev/null | head -n 1 || true)"
    if [ -n "$found_file" ]; then
        patch_addr_file "$found_file"
    fi
fi

# Verify tools are accessible
echo "  ✓ node: $(node --version 2>/dev/null || echo 'NOT FOUND')"
echo "  ✓ npm: $(npm --version 2>/dev/null || echo 'NOT FOUND')"
echo "  ✓ rustc: $(rustc --version 2>/dev/null || echo 'NOT FOUND')"
echo "  ✓ cargo: $(cargo --version 2>/dev/null || echo 'NOT FOUND')"

echo "========================================="
echo "✓ Build environment configured"
echo "========================================="
