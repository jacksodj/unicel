#!/bin/sh

set -e

echo "========================================="
echo "Setting Build Environment Variables"
echo "========================================="

# Add Node.js to PATH (installed by post-clone script)
export PATH="/usr/local/opt/node@20/bin:$PATH"

# Add Rust/Cargo to PATH (installed by post-clone script)
export PATH="$HOME/.cargo/bin:$PATH"

# Verify tools are accessible
echo "  ✓ node: $(node --version 2>/dev/null || echo 'NOT FOUND')"
echo "  ✓ npm: $(npm --version 2>/dev/null || echo 'NOT FOUND')"
echo "  ✓ rustc: $(rustc --version 2>/dev/null || echo 'NOT FOUND')"
echo "  ✓ cargo: $(cargo --version 2>/dev/null || echo 'NOT FOUND')"

echo "========================================="
echo "✓ Build environment configured"
echo "========================================="
