#!/bin/sh

# Xcode Cloud Post-Clone Script
# This script runs after cloning the repository and before building
# It installs Node.js, npm, Rust, and all project dependencies

set -e  # Exit on any error

echo "========================================="
echo "Unicel iOS - Xcode Cloud Setup"
echo "========================================="

# Print environment info for debugging
echo ""
echo "Environment Information:"
echo "  Working Directory: $(pwd)"
echo "  CI_WORKSPACE: ${CI_WORKSPACE:-not set}"
echo "  CI_PRIMARY_REPOSITORY_PATH: ${CI_PRIMARY_REPOSITORY_PATH:-not set}"
echo "  CI_PRODUCT_PLATFORM: ${CI_PRODUCT_PLATFORM:-not set}"
echo ""

# Change to repository root
cd "${CI_PRIMARY_REPOSITORY_PATH}"

# ==========================================
# 1. Install Node.js 20 (LTS)
# ==========================================
echo "Step 1: Installing Node.js 20..."

# Check if Homebrew is available (Xcode Cloud provides it)
if ! command -v brew &> /dev/null; then
    echo "ERROR: Homebrew not found. This script requires Homebrew."
    exit 1
fi

# Install Node.js 20 (LTS)
echo "  - Installing node@20 via Homebrew..."
brew install node@20

# Add Node.js to PATH
export PATH="/usr/local/opt/node@20/bin:$PATH"

# Verify installation
if ! command -v node &> /dev/null; then
    echo "ERROR: Node.js installation failed"
    exit 1
fi

NODE_VERSION=$(node --version)
NPM_VERSION=$(npm --version)
echo "  ✓ Node.js installed: $NODE_VERSION"
echo "  ✓ npm installed: $NPM_VERSION"

# ==========================================
# 2. Install npm Dependencies
# ==========================================
echo ""
echo "Step 2: Installing npm dependencies..."

# Use npm ci for faster, reproducible installs
if [ -f "package-lock.json" ]; then
    echo "  - Running 'npm ci' (clean install)..."
    npm ci
else
    echo "  - Running 'npm install'..."
    npm install
fi

echo "  ✓ npm dependencies installed"

# ==========================================
# 3. Install Rust Toolchain
# ==========================================
echo ""
echo "Step 3: Installing Rust toolchain..."

# Check if Rust is already installed
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    echo "  ✓ Rust already installed: $RUST_VERSION"
else
    echo "  - Downloading and installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable

    # Source cargo environment
    source "$HOME/.cargo/env"

    RUST_VERSION=$(rustc --version)
    echo "  ✓ Rust installed: $RUST_VERSION"
fi

# Ensure cargo is in PATH
export PATH="$HOME/.cargo/bin:$PATH"

# ==========================================
# 4. Add iOS Targets
# ==========================================
echo ""
echo "Step 4: Adding iOS compilation targets..."

# Add iOS targets (required for Tauri iOS builds)
echo "  - Adding aarch64-apple-ios (ARM64 iOS)..."
rustup target add aarch64-apple-ios

# Optional: Add simulator target for potential future use
# echo "  - Adding aarch64-apple-ios-sim (ARM64 Simulator)..."
# rustup target add aarch64-apple-ios-sim

echo "  ✓ iOS targets added"

# List installed targets for debugging
echo "  - Installed targets:"
rustup target list --installed | sed 's/^/    /'

# ==========================================
# 5. Verify Build Environment
# ==========================================
echo ""
echo "Step 5: Verifying build environment..."

# Check all required tools
CHECKS_PASSED=true

# Node.js
if command -v node &> /dev/null; then
    echo "  ✓ node: $(node --version)"
else
    echo "  ✗ node: NOT FOUND"
    CHECKS_PASSED=false
fi

# npm
if command -v npm &> /dev/null; then
    echo "  ✓ npm: $(npm --version)"
else
    echo "  ✗ npm: NOT FOUND"
    CHECKS_PASSED=false
fi

# Rust
if command -v rustc &> /dev/null; then
    echo "  ✓ rustc: $(rustc --version)"
else
    echo "  ✗ rustc: NOT FOUND"
    CHECKS_PASSED=false
fi

# Cargo
if command -v cargo &> /dev/null; then
    echo "  ✓ cargo: $(cargo --version)"
else
    echo "  ✗ cargo: NOT FOUND"
    CHECKS_PASSED=false
fi

# Tauri CLI (should be installed via npm)
if npm list @tauri-apps/cli &> /dev/null; then
    TAURI_VERSION=$(npm list @tauri-apps/cli --depth=0 | grep @tauri-apps/cli | awk '{print $2}')
    echo "  ✓ @tauri-apps/cli: $TAURI_VERSION"
else
    echo "  ✗ @tauri-apps/cli: NOT FOUND"
    CHECKS_PASSED=false
fi

# ==========================================
# 6. Build Frontend (Pre-build)
# ==========================================
echo ""
echo "Step 6: Building frontend assets..."

# Build the Vite frontend
echo "  - Running 'npm run build'..."
npm run build

if [ -d "dist" ]; then
    echo "  ✓ Frontend built successfully (dist/ exists)"
else
    echo "  ✗ Frontend build failed (dist/ not found)"
    CHECKS_PASSED=false
fi

# ==========================================
# 7. Patch Tauri Config for Release Build
# ==========================================
echo ""
echo "Step 7: Patching Tauri config for self-contained release build..."

# In Xcode Cloud, we're building a self-contained iOS app with bundled frontend
# Remove devUrl from tauri.conf.json to prevent Tauri from looking for a dev server
echo "  - Removing devUrl from tauri.conf.json..."

# Backup original config
cp src-tauri/tauri.conf.json src-tauri/tauri.conf.json.backup

# Use sed to remove the devUrl line
# This makes Tauri use the bundled frontend instead of looking for a dev server
sed -i '' '/"devUrl":/d' src-tauri/tauri.conf.json

echo "  ✓ Tauri config patched for release build"
echo "  - Config change: devUrl removed (using bundled frontend)"

# ==========================================
# 8. Create Dummy Dev Server Address File
# ==========================================
echo ""
echo "Step 8: Creating dummy dev server address file..."

# xcode-script has a bug where it tries to read this file even in release builds
# This is a known issue (GitHub #10191) - we create a dummy file to satisfy it
# The file content is not used since frontend is bundled, but the file must exist
WORKSPACE_DIR="${CI_WORKSPACE:-/Volumes/workspace}"
TMP_DIR="$WORKSPACE_DIR/tmp"
ADDR_FILE="$TMP_DIR/com.unicel.app-server-addr"

mkdir -p "$TMP_DIR"
echo "http://127.0.0.1:5173" > "$ADDR_FILE"

echo "  ✓ Dummy addr file created at: $ADDR_FILE"
echo "  - Note: File required by xcode-script bug, content not used (frontend is bundled)"

# ==========================================
# 9. Debug: Environment and Network Info
# ==========================================
echo ""
echo "Step 9: Collecting debug information for hostname resolution..."
echo ""

# Hostname and network information
echo "--- System Hostname Information ---"
echo "  hostname command: $(hostname 2>&1 || echo 'FAILED')"
echo "  uname -n: $(uname -n 2>&1 || echo 'FAILED')"
echo "  scutil --get ComputerName: $(scutil --get ComputerName 2>&1 || echo 'FAILED')"
echo "  scutil --get LocalHostName: $(scutil --get LocalHostName 2>&1 || echo 'FAILED')"
echo ""

# Environment variables that might affect hostname resolution
echo "--- Environment Variables ---"
echo "  HOSTNAME: ${HOSTNAME:-not set}"
echo "  HOST: ${HOST:-not set}"
echo "  COMPUTERNAME: ${COMPUTERNAME:-not set}"
echo "  CI: ${CI:-not set}"
echo "  CI_WORKSPACE: ${CI_WORKSPACE:-not set}"
echo "  CI_XCODE_PROJECT: ${CI_XCODE_PROJECT:-not set}"
echo "  CI_XCODEBUILD_ACTION: ${CI_XCODEBUILD_ACTION:-not set}"
echo "  TAURI_DEV_HOST: ${TAURI_DEV_HOST:-not set}"
echo "  TAURI_CLI_NO_DEV_SERVER: ${TAURI_CLI_NO_DEV_SERVER:-not set}"
echo ""

# Network interface information
echo "--- Network Interfaces ---"
ifconfig | grep -E "^[a-z]|inet " | head -20 || echo "  ifconfig failed"
echo ""

# DNS configuration
echo "--- DNS Configuration ---"
echo "  /etc/resolv.conf:"
cat /etc/resolv.conf 2>&1 | head -10 || echo "  Cannot read /etc/resolv.conf"
echo ""
echo "  /etc/hosts (first 20 lines):"
cat /etc/hosts 2>&1 | head -20 || echo "  Cannot read /etc/hosts"
echo ""

# Test hostname resolution
echo "--- Hostname Resolution Tests ---"
echo "  Can resolve localhost: $(ping -c 1 -t 1 localhost 2>&1 | head -1 || echo 'FAILED')"
echo "  Can resolve 127.0.0.1: $(ping -c 1 -t 1 127.0.0.1 2>&1 | head -1 || echo 'FAILED')"
echo ""

# Dev server addr file contents
echo "--- Dev Server Address File ---"
echo "  File path: $ADDR_FILE"
echo "  File exists: $(test -f "$ADDR_FILE" && echo 'YES' || echo 'NO')"
echo "  File contents: $(cat "$ADDR_FILE" 2>&1 || echo 'Cannot read file')"
echo "  File permissions: $(ls -la "$ADDR_FILE" 2>&1 || echo 'Cannot stat file')"
echo ""

echo "  ✓ Debug information collected"

# ==========================================
# Final Status
# ==========================================
echo ""
echo "========================================="
if [ "$CHECKS_PASSED" = true ]; then
    echo "✓ Xcode Cloud Setup Complete!"
    echo "========================================="
    echo ""
    echo "Build environment is ready for iOS compilation."
    exit 0
else
    echo "✗ Xcode Cloud Setup Failed"
    echo "========================================="
    echo ""
    echo "Some checks failed. Review the output above."
    exit 1
fi
