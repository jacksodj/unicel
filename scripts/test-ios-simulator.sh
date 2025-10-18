#!/bin/bash
# iOS Simulator Testing Script
# Run this after completing manual setup in Task 10.4
# Location: /Users/dennisjackson/Code/unicel/scripts/test-ios-simulator.sh

set -e  # Exit on error

echo "========================================"
echo "Unicel iOS Simulator Testing Script"
echo "========================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if xcode-select is set correctly
echo "Step 1: Checking Xcode configuration..."
XCODE_PATH=$(xcode-select -p)
if [[ "$XCODE_PATH" == "/Applications/Xcode.app/Contents/Developer" ]]; then
    echo -e "${GREEN}✓${NC} Xcode path is correct: $XCODE_PATH"
else
    echo -e "${RED}✗${NC} Xcode path is incorrect: $XCODE_PATH"
    echo -e "${YELLOW}→${NC} Please run: sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer"
    exit 1
fi

# Verify Xcode version
echo ""
echo "Step 2: Verifying Xcode version..."
XCODE_VERSION=$(xcodebuild -version | head -1)
echo -e "${GREEN}✓${NC} $XCODE_VERSION"

# Check iOS targets
echo ""
echo "Step 3: Checking Rust iOS targets..."
if rustup target list | grep -q "aarch64-apple-ios (installed)"; then
    echo -e "${GREEN}✓${NC} aarch64-apple-ios target installed"
else
    echo -e "${RED}✗${NC} aarch64-apple-ios target missing"
    echo "   Installing..."
    rustup target add aarch64-apple-ios
fi

if rustup target list | grep -q "aarch64-apple-ios-sim (installed)"; then
    echo -e "${GREEN}✓${NC} aarch64-apple-ios-sim target installed"
else
    echo -e "${RED}✗${NC} aarch64-apple-ios-sim target missing"
    echo "   Installing..."
    rustup target add aarch64-apple-ios-sim
fi

# List available simulators
echo ""
echo "Step 4: Available iOS Simulators:"
echo "------------------------------------"
xcrun simctl list devices available | grep -E "iPhone|iPad" | head -10

# Build frontend
echo ""
echo "Step 5: Building frontend..."
npm run build
echo -e "${GREEN}✓${NC} Frontend built successfully"

# Ask user which simulator to use
echo ""
echo "Step 6: Select simulator type:"
echo "  1) iPhone 17 Pro (default)"
echo "  2) iPad Pro 11-inch"
echo "  3) List all simulators"
read -p "Enter choice (1-3) [1]: " CHOICE
CHOICE=${CHOICE:-1}

case $CHOICE in
    1)
        SIMULATOR="iPhone 17 Pro"
        ;;
    2)
        SIMULATOR="iPad Pro 11-inch (M4)"
        ;;
    3)
        echo ""
        xcrun simctl list devices available
        read -p "Enter simulator name: " SIMULATOR
        ;;
    *)
        SIMULATOR="iPhone 17 Pro"
        ;;
esac

echo -e "${YELLOW}→${NC} Will use simulator: $SIMULATOR"

# Boot simulator
echo ""
echo "Step 7: Booting simulator..."
DEVICE_ID=$(xcrun simctl list devices available | grep "$SIMULATOR" | head -1 | grep -o "\([A-F0-9-]*\)" | head -1 || echo "")

if [[ -n "$DEVICE_ID" ]]; then
    echo "   Device ID: $DEVICE_ID"
    xcrun simctl boot "$DEVICE_ID" 2>/dev/null || echo "   (Simulator may already be booted)"
    open -a Simulator
    sleep 3
    echo -e "${GREEN}✓${NC} Simulator booted"
else
    echo -e "${YELLOW}⚠${NC} Could not find device ID, simulator may launch automatically"
fi

# Build and run iOS app
echo ""
echo "Step 8: Building iOS app (this may take 5-10 minutes on first run)..."
echo -e "${YELLOW}→${NC} Press Ctrl+C to cancel"
echo ""

npm run tauri:ios:dev

echo ""
echo "========================================"
echo -e "${GREEN}iOS app launched successfully!${NC}"
echo "========================================"
echo ""
echo "Testing checklist:"
echo "  [ ] App launches without crashes"
echo "  [ ] Main screen renders correctly"
echo "  [ ] Touch gestures work (tap, swipe)"
echo "  [ ] Open example workbook"
echo "  [ ] Cells display with units"
echo "  [ ] Metric/Imperial toggle works"
echo "  [ ] Pinch-to-zoom works"
echo "  [ ] Scroll is smooth (60fps)"
echo ""
echo "To test Tauri commands, open Safari Developer menu"
echo "and connect to the iOS Simulator's web inspector."
echo ""
