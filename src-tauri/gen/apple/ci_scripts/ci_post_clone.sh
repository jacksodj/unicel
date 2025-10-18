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
