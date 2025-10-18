#!/bin/bash
# iOS Release Build Script for Unicel Viewer
# This script builds a signed release IPA for TestFlight/App Store submission

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🍎 Unicel iOS Release Build"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Change to project root
cd "$PROJECT_ROOT"

# Check prerequisites
echo "📋 Checking prerequisites..."

if ! command -v npm &> /dev/null; then
    echo -e "${RED}❌ npm not found. Please install Node.js.${NC}"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ cargo not found. Please install Rust.${NC}"
    exit 1
fi

if ! command -v xcodebuild &> /dev/null; then
    echo -e "${RED}❌ xcodebuild not found. Please install Xcode.${NC}"
    exit 1
fi

echo -e "${GREEN}✓${NC} Prerequisites found"
echo ""

# Check tauri.conf.json
if [ ! -f "src-tauri/tauri.conf.json" ]; then
    echo -e "${RED}❌ tauri.conf.json not found${NC}"
    exit 1
fi

# Extract version from tauri.conf.json
VERSION=$(grep -o '"version": *"[^"]*"' src-tauri/tauri.conf.json | grep -o '"[^"]*"$' | tr -d '"')
echo "📦 Building version: ${GREEN}${VERSION}${NC}"
echo ""

# Check bundle identifier
BUNDLE_ID=$(grep -o '"identifier": *"[^"]*"' src-tauri/tauri.conf.json | grep -o '"[^"]*"$' | tr -d '"')
echo "🏷️  Bundle ID: ${BUNDLE_ID}"
echo ""

# Verify code signing setup
echo "🔐 Checking code signing..."

if ! security find-identity -v -p codesigning | grep -q "Apple Distribution"; then
    echo -e "${YELLOW}⚠️  Warning: No Distribution certificate found in Keychain${NC}"
    echo "   You may need to install your distribution certificate"
    echo "   Download from: https://developer.apple.com/account/resources/certificates/"
    echo ""
else
    echo -e "${GREEN}✓${NC} Distribution certificate found"
    echo ""
fi

# Clean previous builds
echo "🧹 Cleaning previous builds..."
rm -rf src-tauri/gen/apple/build
rm -rf src-tauri/target/release
echo -e "${GREEN}✓${NC} Cleaned"
echo ""

# Install dependencies
echo "📦 Installing dependencies..."
npm install
echo -e "${GREEN}✓${NC} Dependencies installed"
echo ""

# Build frontend
echo "🏗️  Building frontend..."
npm run build
if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Frontend build failed${NC}"
    exit 1
fi
echo -e "${GREEN}✓${NC} Frontend built"
echo ""

# Build iOS release
echo "🚀 Building iOS release IPA..."
echo "   This may take 5-10 minutes..."
echo ""

npm run tauri ios build --release

if [ $? -ne 0 ]; then
    echo ""
    echo -e "${RED}❌ iOS build failed${NC}"
    echo ""
    echo "Common issues:"
    echo "  - Code signing not configured"
    echo "  - Provisioning profile missing"
    echo "  - Bundle ID doesn't match certificate"
    echo ""
    echo "Try opening Xcode and building from there:"
    echo "  open src-tauri/gen/apple/unicel.xcodeproj"
    exit 1
fi

echo ""
echo -e "${GREEN}✓${NC} iOS build completed"
echo ""

# Find the IPA
echo "🔍 Locating IPA..."

# Tauri iOS build outputs to different locations depending on configuration
IPA_PATHS=(
    "src-tauri/gen/apple/build/arm64-apple-ios/release/bundle/ios/Unicel.ipa"
    "src-tauri/gen/apple/build/Release-iphoneos/Unicel.ipa"
    "src-tauri/target/aarch64-apple-ios/release/bundle/ios/Unicel.ipa"
)

IPA_PATH=""
for path in "${IPA_PATHS[@]}"; do
    if [ -f "$path" ]; then
        IPA_PATH="$path"
        break
    fi
done

if [ -z "$IPA_PATH" ]; then
    echo -e "${YELLOW}⚠️  Could not locate IPA file automatically${NC}"
    echo ""
    echo "Searching for IPA files..."
    find src-tauri/gen/apple/build -name "*.ipa" 2>/dev/null || true
    find src-tauri/target -name "*.ipa" 2>/dev/null || true
    echo ""
    echo "If build succeeded, the IPA should be in one of these locations:"
    for path in "${IPA_PATHS[@]}"; do
        echo "  - $path"
    done
    exit 1
fi

echo -e "${GREEN}✓${NC} Found IPA: $IPA_PATH"
echo ""

# Get IPA info
IPA_SIZE=$(du -h "$IPA_PATH" | cut -f1)
echo "📊 IPA Information:"
echo "   Location: $IPA_PATH"
echo "   Size: $IPA_SIZE"
echo ""

# Verify code signing
echo "🔐 Verifying code signing..."

# Extract IPA to temp directory
TEMP_DIR=$(mktemp -d)
unzip -q "$IPA_PATH" -d "$TEMP_DIR"

# Find .app bundle
APP_BUNDLE=$(find "$TEMP_DIR" -name "*.app" -type d | head -1)

if [ -z "$APP_BUNDLE" ]; then
    echo -e "${RED}❌ Could not find .app bundle in IPA${NC}"
    rm -rf "$TEMP_DIR"
    exit 1
fi

# Check code signing
codesign -dv "$APP_BUNDLE" 2>&1 | grep -E "(Authority|Identifier|TeamIdentifier)"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓${NC} Code signing verified"
else
    echo -e "${YELLOW}⚠️  Could not verify code signing${NC}"
fi

# Clean up temp directory
rm -rf "$TEMP_DIR"
echo ""

# Find dSYM files
echo "🐛 Checking for debug symbols..."
DSYM_PATHS=$(find src-tauri/gen/apple/build -name "*.dSYM" -type d)
if [ -n "$DSYM_PATHS" ]; then
    echo -e "${GREEN}✓${NC} Debug symbols found:"
    echo "$DSYM_PATHS" | while read -r dsym; do
        echo "   - $dsym"
    done
else
    echo -e "${YELLOW}⚠️  No dSYM files found${NC}"
    echo "   Debug symbols are needed for crash reporting in TestFlight"
fi
echo ""

# Success summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${GREEN}✅ Build completed successfully!${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📦 Release IPA: $IPA_PATH"
echo "📊 Size: $IPA_SIZE"
echo "🏷️  Version: $VERSION"
echo "🆔 Bundle ID: $BUNDLE_ID"
echo ""
echo "Next steps:"
echo ""
echo "1. Upload to TestFlight:"
echo "   - Open Xcode"
echo "   - Window > Organizer"
echo "   - Select 'Archives' tab"
echo "   - Click 'Distribute App'"
echo "   - Choose 'App Store Connect'"
echo "   - Follow the upload wizard"
echo ""
echo "   Or use command line (requires app-specific password):"
echo "   xcrun altool --upload-app \\"
echo "     -f \"$IPA_PATH\" \\"
echo "     -t ios \\"
echo "     -u your@email.com \\"
echo "     -p app-specific-password"
echo ""
echo "2. Or use Transporter app:"
echo "   - Download from Mac App Store"
echo "   - Drag and drop IPA file"
echo "   - Sign in and click 'Deliver'"
echo ""
echo "📚 For detailed instructions, see:"
echo "   docs/app-store/TESTFLIGHT_GUIDE.md"
echo ""
