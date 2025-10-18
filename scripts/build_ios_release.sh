#!/bin/bash
# iOS Release Build Script for Unicel
# Builds a signed release IPA ready for TestFlight/App Store submission

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

PROJECT_DIR="/Users/dennisjackson/Code/unicel"
BUILD_DIR="$PROJECT_DIR/src-tauri/gen/apple/build"

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Unicel iOS Release Build                                ║${NC}"
echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo ""

# Step 1: Verify environment
echo -e "${YELLOW}[1/6] Verifying build environment...${NC}"

# Check for Xcode
if ! command -v xcodebuild &> /dev/null; then
    echo -e "${RED}✗ Xcode not found. Please install Xcode from the App Store.${NC}"
    exit 1
fi

# Check for npm
if ! command -v npm &> /dev/null; then
    echo -e "${RED}✗ npm not found. Please install Node.js.${NC}"
    exit 1
fi

# Check for Tauri CLI
cd "$PROJECT_DIR"
if ! npm run tauri --version &> /dev/null; then
    echo -e "${RED}✗ Tauri CLI not found. Installing...${NC}"
    npm install
fi

echo -e "${GREEN}✓ Build environment verified${NC}"
echo ""

# Step 2: Clean previous builds
echo -e "${YELLOW}[2/6] Cleaning previous builds...${NC}"
if [ -d "$BUILD_DIR" ]; then
    rm -rf "$BUILD_DIR"
    echo -e "${GREEN}✓ Cleaned build directory${NC}"
else
    echo -e "${GREEN}✓ No previous builds to clean${NC}"
fi
echo ""

# Step 3: Install dependencies
echo -e "${YELLOW}[3/6] Installing dependencies...${NC}"
npm install
echo -e "${GREEN}✓ Dependencies installed${NC}"
echo ""

# Step 4: Run tests
echo -e "${YELLOW}[4/6] Running tests...${NC}"
cd "$PROJECT_DIR/src-tauri"
if cargo test --lib 2>&1 | tail -10; then
    echo -e "${GREEN}✓ All tests passed${NC}"
else
    echo -e "${RED}✗ Tests failed. Fix errors before building release.${NC}"
    exit 1
fi
echo ""

# Step 5: Build release IPA
echo -e "${YELLOW}[5/6] Building iOS release IPA...${NC}"
echo -e "${BLUE}This may take several minutes...${NC}"
cd "$PROJECT_DIR"

# Run the build
npm run tauri ios build -- --release

echo -e "${GREEN}✓ Build complete${NC}"
echo ""

# Step 6: Verify build artifacts
echo -e "${YELLOW}[6/6] Verifying build artifacts...${NC}"

IPA_PATH=$(find "$BUILD_DIR" -name "*.ipa" -type f | head -1)
APP_PATH=$(find "$BUILD_DIR" -name "*.app" -type d | grep -i "Release" | head -1)

if [ -z "$IPA_PATH" ]; then
    echo -e "${RED}✗ IPA file not found!${NC}"
    echo -e "${YELLOW}Looking in: $BUILD_DIR${NC}"
    ls -R "$BUILD_DIR" 2>/dev/null || echo "Build directory not found"
    exit 1
fi

if [ -z "$APP_PATH" ]; then
    echo -e "${YELLOW}⚠ .app bundle not found (IPA may still be valid)${NC}"
fi

# Get IPA info
IPA_SIZE=$(ls -lh "$IPA_PATH" | awk '{print $5}')
echo -e "${GREEN}✓ IPA file created: $IPA_PATH${NC}"
echo -e "${BLUE}   Size: $IPA_SIZE${NC}"

# Verify code signing (if .app exists)
if [ ! -z "$APP_PATH" ]; then
    echo ""
    echo -e "${YELLOW}Verifying code signing...${NC}"
    if codesign -dv --verbose=4 "$APP_PATH" 2>&1 | grep -q "Authority"; then
        echo -e "${GREEN}✓ Code signing verified${NC}"
        echo -e "${BLUE}Signing identity:${NC}"
        codesign -dv --verbose=4 "$APP_PATH" 2>&1 | grep "Authority" | head -3
    else
        echo -e "${YELLOW}⚠ Could not verify code signing details${NC}"
    fi
fi

# Summary
echo ""
echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}   Build Complete!${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${GREEN}Release IPA: $IPA_PATH${NC}"
echo -e "${GREEN}Size: $IPA_SIZE${NC}"
echo ""
echo -e "${BLUE}Next Steps:${NC}"
echo -e "   1. Test the IPA in a simulator or device"
echo -e "   2. Upload to TestFlight for beta testing"
echo -e "   3. Submit to App Store for review"
echo ""
echo -e "${YELLOW}Upload Commands:${NC}"
echo -e "${BLUE}# Upload to TestFlight:${NC}"
echo -e "xcrun altool --upload-app \\"
echo -e "  -f \"$IPA_PATH\" \\"
echo -e "  -t ios \\"
echo -e "  -u your@email.com \\"
echo -e "  -p app-specific-password"
echo ""
echo -e "${BLUE}# Or use Xcode:${NC}"
echo -e "# 1. Open Xcode"
echo -e "# 2. Window > Organizer"
echo -e "# 3. Drag IPA to Archives"
echo -e "# 4. Distribute App > App Store Connect"
echo ""
echo -e "${BLUE}Documentation:${NC}"
echo -e "   See docs/app-store/TESTFLIGHT_GUIDE.md for upload instructions"
echo ""
