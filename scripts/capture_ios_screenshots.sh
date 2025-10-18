#!/bin/bash
# Screenshot Capture Script for Unicel iOS App Store Submission
# This script automates the process of capturing screenshots for all required device sizes

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
OUTPUT_DIR="$HOME/Desktop/unicel-screenshots"
PROJECT_DIR="/Users/dennisjackson/Code/unicel"

# Device configurations
declare -A DEVICES=(
    ["iPhone 15 Pro Max"]="iphone-6.7-inch"
    ["iPhone 11 Pro Max"]="iphone-6.5-inch"
    ["iPad Pro (12.9-inch) (6th generation)"]="ipad-12.9-inch"
)

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Unicel iOS App Store Screenshot Capture Script          ║${NC}"
echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo ""

# Create output directories
mkdir -p "$OUTPUT_DIR"
for folder in "${DEVICES[@]}"; do
    mkdir -p "$OUTPUT_DIR/$folder"
done

echo -e "${GREEN}✓ Created output directories at: $OUTPUT_DIR${NC}"
echo ""

# Function to wait for user
wait_for_user() {
    local message=$1
    echo -e "${YELLOW}⏸  $message${NC}"
    read -p "   Press ENTER when ready... "
}

# Function to capture screenshot
capture_screenshot() {
    local device=$1
    local folder=$2
    local number=$3
    local description=$4

    local filename="$OUTPUT_DIR/$folder/screenshot-$number-$description.png"

    echo -e "${BLUE}📸 Capturing screenshot $number: $description${NC}"
    xcrun simctl io booted screenshot "$filename"

    if [ -f "$filename" ]; then
        # Get file size
        local size=$(ls -lh "$filename" | awk '{print $5}')
        # Get dimensions
        local dims=$(sips -g pixelWidth -g pixelHeight "$filename" 2>/dev/null | grep -E "pixelWidth|pixelHeight" | awk '{print $2}' | tr '\n' 'x' | sed 's/x$//')
        echo -e "${GREEN}   ✓ Saved: $filename ($dims, $size)${NC}"
        return 0
    else
        echo -e "${RED}   ✗ Failed to capture screenshot${NC}"
        return 1
    fi
}

# Function to process device
process_device() {
    local device=$1
    local folder=$2

    echo ""
    echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}   Device: $device${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
    echo ""

    # Shutdown any running simulators
    echo -e "${YELLOW}Shutting down existing simulators...${NC}"
    xcrun simctl shutdown all 2>/dev/null || true
    sleep 2

    # Boot the device
    echo -e "${YELLOW}Booting $device...${NC}"
    xcrun simctl boot "$device" 2>/dev/null || true
    sleep 3

    # Open Simulator app
    echo -e "${YELLOW}Opening Simulator...${NC}"
    open -a Simulator
    sleep 2

    echo ""
    echo -e "${GREEN}✓ Simulator ready!${NC}"
    echo ""
    echo -e "${YELLOW}Now you need to:${NC}"
    echo -e "   1. Launch Unicel app in the simulator"
    echo -e "   2. Navigate to the required screens"
    echo -e "   3. Press ENTER for each screenshot when ready"
    echo ""
    echo -e "${BLUE}Required Screenshots:${NC}"
    echo -e "   1. Home screen with 'Open File' button"
    echo -e "   2. Grid view with AWS/Construction Estimator loaded"
    echo -e "   3. Unit conversion toggle in action"
    echo -e "   4. Multi-sheet navigation (sheet tabs visible)"
    echo -e "   5. Formula detail view (cell selected, formula visible)"
    echo ""

    # Capture each screenshot
    wait_for_user "Navigate to SCREEN 1: Home screen"
    capture_screenshot "$device" "$folder" "01" "home"
    echo ""

    wait_for_user "Navigate to SCREEN 2: Grid view with data"
    capture_screenshot "$device" "$folder" "02" "grid-view"
    echo ""

    wait_for_user "Navigate to SCREEN 3: Unit conversion toggle"
    capture_screenshot "$device" "$folder" "03" "unit-conversion"
    echo ""

    wait_for_user "Navigate to SCREEN 4: Multi-sheet navigation"
    capture_screenshot "$device" "$folder" "04" "multi-sheet"
    echo ""

    wait_for_user "Navigate to SCREEN 5: Formula detail view"
    capture_screenshot "$device" "$folder" "05" "formula-detail"
    echo ""

    echo -e "${GREEN}✓ All screenshots captured for $device${NC}"

    # Shutdown simulator
    xcrun simctl shutdown booted
}

# Main execution
echo -e "${YELLOW}This script will guide you through capturing screenshots for:${NC}"
for device in "${!DEVICES[@]}"; do
    echo -e "   • $device"
done
echo ""

read -p "Ready to begin? (y/n) " -n 1 -r
echo ""
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${RED}Cancelled.${NC}"
    exit 1
fi

# Process each device
for device in "${!DEVICES[@]}"; do
    folder="${DEVICES[$device]}"
    process_device "$device" "$folder"
done

# Shutdown all simulators
echo ""
echo -e "${YELLOW}Shutting down all simulators...${NC}"
xcrun simctl shutdown all 2>/dev/null || true

# Generate summary
echo ""
echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}   Screenshot Capture Complete!${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${GREEN}✓ All screenshots saved to: $OUTPUT_DIR${NC}"
echo ""
echo -e "${BLUE}Summary:${NC}"
for folder in "${DEVICES[@]}"; do
    count=$(ls -1 "$OUTPUT_DIR/$folder"/*.png 2>/dev/null | wc -l | tr -d ' ')
    echo -e "   $folder: $count screenshots"
done
echo ""
echo -e "${BLUE}Next Steps:${NC}"
echo -e "   1. Review screenshots in: $OUTPUT_DIR"
echo -e "   2. Optional: Add device frames using fastlane frameit"
echo -e "   3. Upload to App Store Connect"
echo -e "   4. See docs/app-store/APP_STORE_SUBMISSION_GUIDE.md for details"
echo ""
