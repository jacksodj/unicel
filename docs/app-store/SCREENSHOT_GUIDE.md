# Screenshot Generation Guide

This guide provides detailed instructions for capturing App Store screenshots for Unicel Viewer.

## Required Screenshot Sizes

Apple requires screenshots for the following device sizes:

| Device | Size (pixels) | Device Examples |
|--------|---------------|-----------------|
| iPhone 6.7" | 1290 × 2796 | iPhone 15 Pro Max, 14 Pro Max |
| iPhone 6.5" | 1242 × 2688 | iPhone 11 Pro Max, XS Max |
| iPad Pro 12.9" | 2048 × 2732 | iPad Pro 12.9" (2nd gen+) |
| iPad Pro 11" | 1668 × 2388 | iPad Pro 11", iPad Air (optional) |

**Minimum Required:** First 3 sizes
**Recommended:** All 4 sizes for maximum compatibility

## Screenshot Content Plan

We need 5 screenshots for each device size showcasing:

### Screenshot 1: Home Screen / File Browser
**Purpose:** First impression, show clean UI
**Content:**
- App opened to home screen
- "Open File" or "Open Example" button prominent
- List of example workbooks if visible
- Clean, professional appearance

**Key message:** "Easy to get started"

### Screenshot 2: Grid View with Data
**Purpose:** Show the core spreadsheet functionality
**Content:**
- Open the AWS Cost Estimator or Construction Estimator
- Grid populated with data showing values and units
- Multiple cells visible with different unit types
- Units clearly visible next to values (e.g., "$1000", "100 ft", "50 hours")
- Professional, data-rich appearance

**Key message:** "Unit-aware spreadsheet viewing"

### Screenshot 3: Unit Conversion in Action
**Purpose:** Highlight the unique unit conversion feature
**Content:**
- Split-screen or before/after showing Metric ↔ Imperial toggle
- Clear visual difference in values
- Examples: "100 ft" → "30.48 m", "50 mph" → "80.47 km/h"
- Toggle button highlighted or visible
- Multiple cells showing conversion simultaneously

**Key message:** "Toggle units with one tap"

### Screenshot 4: Multi-Sheet Navigation
**Purpose:** Show professional spreadsheet capabilities
**Content:**
- Sheet tabs visible at bottom of screen
- Multiple sheets shown (Sheet1, Sheet2, Sheet3, etc.)
- Active sheet highlighted
- Populated grid in background
- Clean navigation interface

**Key message:** "Navigate multiple sheets easily"

### Screenshot 5: Formula Detail View
**Purpose:** Show intelligence and formula capabilities
**Content:**
- A cell selected with formula bar or detail panel visible
- Formula displayed (e.g., "=B2*C2")
- Calculated value shown with unit
- Clear, readable formula syntax
- Shows depth and capability

**Key message:** "View formulas and calculations"

## Setup Instructions

### Step 1: Prepare Example Workbooks

Ensure example workbooks are ready:

```bash
cd /Users/dennisjackson/Code/unicel/src-tauri/examples

# Verify example files exist:
ls -lh *.usheet
```

**Best workbooks for screenshots:**
1. **aws_cost_estimator.usheet** - Professional, real-world use case
2. **construction_estimator.usheet** - Visual, lots of quantities
3. **unit_conversion_tutorial.usheet** - Clear demonstration of units

### Step 2: Install and Configure iOS Simulators

```bash
# List available simulators
xcrun simctl list devices available

# Install additional simulators if needed (via Xcode)
# Xcode > Window > Devices and Simulators > Simulators > +
```

**Required simulators:**
- iPhone 15 Pro Max (for 6.7" screenshots)
- iPhone 11 Pro Max (for 6.5" screenshots)
- iPad Pro 12.9" (3rd gen or later)

### Step 3: Build and Run App

```bash
cd /Users/dennisjackson/Code/unicel

# Build for iOS simulator
npm run tauri ios dev
```

## Capturing Screenshots

### Method 1: Using iOS Simulator (Recommended)

#### For iPhone 6.7" (iPhone 15 Pro Max):

```bash
# Boot simulator
xcrun simctl boot "iPhone 15 Pro Max"

# Open Simulator app
open -a Simulator

# Wait for Unicel app to launch, then navigate to desired screen

# Take screenshot (repeat for each screen)
xcrun simctl io booted screenshot ~/Desktop/iphone67-01-home.png
xcrun simctl io booted screenshot ~/Desktop/iphone67-02-grid.png
xcrun simctl io booted screenshot ~/Desktop/iphone67-03-conversion.png
xcrun simctl io booted screenshot ~/Desktop/iphone67-04-sheets.png
xcrun simctl io booted screenshot ~/Desktop/iphone67-05-formula.png
```

#### For iPhone 6.5" (iPhone 11 Pro Max):

```bash
# Shutdown current simulator
xcrun simctl shutdown booted

# Boot 6.5" simulator
xcrun simctl boot "iPhone 11 Pro Max"

# Repeat screenshot process
xcrun simctl io booted screenshot ~/Desktop/iphone65-01-home.png
# ... etc
```

#### For iPad Pro 12.9":

```bash
# Shutdown current simulator
xcrun simctl shutdown booted

# Boot iPad simulator
xcrun simctl boot "iPad Pro (12.9-inch) (6th generation)"

# Repeat screenshot process
xcrun simctl io booted screenshot ~/Desktop/ipad129-01-home.png
# ... etc
```

### Method 2: Using Xcode Simulator UI

1. Open Simulator app
2. Navigate to desired screen in Unicel app
3. File > New Screen Shot (or Cmd+S)
4. Screenshot saved to Desktop

### Method 3: Using Real Device

1. Connect iPhone/iPad via USB
2. Open Xcode
3. Window > Devices and Simulators
4. Select device
5. Click "Take Screenshot"
6. Screenshot saved to Desktop

**Advantages:**
- True device performance
- Real-world appearance
- Better for video capture

**Disadvantages:**
- Need physical devices
- More time-consuming
- Harder to get exact timing

## Screenshot Workflow Script

Create a helper script to automate screenshot capture:

```bash
#!/bin/bash
# save as: scripts/capture_screenshots.sh

set -e

DEVICES=("iPhone 15 Pro Max" "iPhone 11 Pro Max" "iPad Pro (12.9-inch) (6th generation)")
OUTPUT_DIR="$HOME/Desktop/unicel-screenshots"

mkdir -p "$OUTPUT_DIR/iphone67"
mkdir -p "$OUTPUT_DIR/iphone65"
mkdir -p "$OUTPUT_DIR/ipad129"

echo "📸 Starting screenshot capture..."

# Capture for each device
for device in "${DEVICES[@]}"; do
    echo "📱 Capturing for: $device"

    # Determine output folder
    case "$device" in
        "iPhone 15 Pro Max")
            folder="iphone67"
            ;;
        "iPhone 11 Pro Max")
            folder="iphone65"
            ;;
        "iPad Pro"*)
            folder="ipad129"
            ;;
    esac

    # Boot device
    echo "   Booting simulator..."
    xcrun simctl boot "$device" 2>/dev/null || true

    # Wait for boot
    sleep 5

    # Open Simulator
    open -a Simulator

    echo "   Ready to capture!"
    echo "   Navigate to each screen and press ENTER to capture..."

    for i in {1..5}; do
        read -p "   Screenshot $i ready? (Press ENTER) "
        xcrun simctl io booted screenshot "$OUTPUT_DIR/$folder/screenshot-$i.png"
        echo "   ✓ Captured screenshot $i"
    done

    # Shutdown
    xcrun simctl shutdown booted
    echo "   ✓ Complete for $device"
    echo ""
done

echo "✅ All screenshots captured!"
echo "📁 Location: $OUTPUT_DIR"
```

**Usage:**
```bash
chmod +x scripts/capture_screenshots.sh
./scripts/capture_screenshots.sh
```

## Manual Capture Process

If you prefer manual capture:

### For Each Device Size:

1. **Launch App in Simulator**
   ```bash
   # Boot correct simulator
   xcrun simctl boot "iPhone 15 Pro Max"
   open -a Simulator

   # Launch Unicel (via Tauri dev)
   npm run tauri ios dev
   ```

2. **Navigate to Screen 1 (Home)**
   - App opens to home screen
   - Capture: `Cmd+S` or File > New Screen Shot

3. **Open Example Workbook**
   - Tap "Open Example"
   - Select "AWS Cost Estimator"

4. **Navigate to Screen 2 (Grid View)**
   - Workbook opens showing grid
   - Scroll to show populated cells with units
   - Capture screenshot

5. **Toggle Unit Display**
   - Tap toggle button to switch Metric/Imperial
   - Capture screenshot showing converted values

6. **Navigate to Screen 4 (Multi-Sheet)**
   - Swipe to show sheet tabs
   - Ensure multiple sheets visible
   - Capture screenshot

7. **Navigate to Screen 5 (Formula)**
   - Tap a cell with a formula
   - Formula bar or detail view appears
   - Capture screenshot

8. **Repeat for Other Devices**
   - Shutdown simulator
   - Boot next device
   - Repeat process

## Screenshot Post-Processing

### Option 1: No Processing (Clean)

Use screenshots as-is from simulator:
- Advantages: Quick, authentic
- Disadvantages: No device frame, no captions

### Option 2: Add Device Frames

Use tools to add device bezels around screenshots:

**Fastlane Frameit:**
```bash
# Install
brew install fastlane

# Add frames
fastlane frameit
```

**Screenshot Studio:**
- Visit https://screenshots.pro/
- Upload screenshots
- Select device frames
- Download framed versions

### Option 3: Add Captions and Graphics

Use design tools to enhance screenshots:

**Tools:**
- Figma (free, web-based)
- Sketch (macOS app)
- Adobe Photoshop
- Canva (simple, online)

**What to add:**
- Descriptive captions
- Highlighted features (arrows, circles)
- App logo or branding
- Background colors or gradients

**Best Practices:**
- Keep it simple and clean
- Don't obscure app content
- Use consistent styling
- Ensure text is legible
- Follow Apple's design guidelines

## Organizing Screenshots

Create organized folder structure:

```bash
mkdir -p /Users/dennisjackson/Code/unicel/docs/app-store/screenshots

cd /Users/dennisjackson/Code/unicel/docs/app-store/screenshots

# Create device folders
mkdir -p iphone-6.7-inch
mkdir -p iphone-6.5-inch
mkdir -p ipad-12.9-inch

# Optional: create subfolders for versions
mkdir -p iphone-6.7-inch/raw
mkdir -p iphone-6.7-inch/framed
mkdir -p iphone-6.7-inch/final
```

**Naming convention:**
```
01-home.png
02-grid-view.png
03-unit-conversion.png
04-multi-sheet.png
05-formula-detail.png
```

## Uploading to App Store Connect

Once screenshots are ready:

1. **Log in to App Store Connect**
   - https://appstoreconnect.apple.com

2. **Navigate to App**
   - My Apps > Unicel > iOS App > [Version]

3. **Upload Screenshots**
   - Scroll to "App Previews and Screenshots"
   - Select device size (iPhone 6.7", etc.)
   - Click "+" to add screenshot
   - Upload 3-10 screenshots per device
   - Drag to reorder (first screenshot is most important)

4. **Verify**
   - Check screenshots display correctly
   - Ensure correct aspect ratio
   - No distortion or black bars
   - Text is legible

5. **Save**
   - Click "Save" at top of page

## Screenshot Checklist

Before finalizing screenshots:

### Technical Requirements:
- [ ] Correct dimensions for each device
- [ ] PNG or JPG format
- [ ] File size under 500KB each
- [ ] No transparency (solid background)
- [ ] High resolution (not blurry)
- [ ] Correct aspect ratio (no black bars)

### Content Requirements:
- [ ] Shows actual app interface (not mockups)
- [ ] Real data visible (not lorem ipsum)
- [ ] Professional appearance
- [ ] Accurate representation of app
- [ ] No misleading content
- [ ] Status bar clean (time 9:41, full battery)

### User Experience:
- [ ] First screenshot is compelling
- [ ] Screenshots tell a story
- [ ] Key features highlighted
- [ ] Easy to understand
- [ ] Professional and polished
- [ ] Consistent styling across all screenshots

## Tips for Great Screenshots

1. **Show Real Use Cases:**
   - Use AWS Cost Estimator or Construction Estimator
   - Populate with realistic data
   - Show professional scenarios

2. **Highlight Unique Features:**
   - Unit conversion is your differentiator
   - Show before/after of toggle
   - Emphasize unit intelligence

3. **Keep it Clean:**
   - Remove clutter
   - Hide debug overlays
   - Clean status bar
   - No placeholder content

4. **Tell a Story:**
   - Screenshot 1: Hook (home screen)
   - Screenshots 2-4: Features (core functionality)
   - Screenshot 5: Depth (advanced capability)

5. **Optimize for Thumbnails:**
   - First screenshot shows well at small sizes
   - Important content centered
   - High contrast for readability

6. **Test on Device:**
   - View in App Store on real iPhone
   - Check how thumbnails look
   - Ensure text is legible

## Troubleshooting

### Screenshot Wrong Size:
- Verify simulator is correct model
- Use `sips -g pixelWidth -g pixelHeight file.png` to check
- Simulator should save at exact required dimensions

### Status Bar Shows Wrong Time:
- Set simulator time to 9:41 AM (Apple convention)
- Settings > General > Date & Time
- Or use Xcode's status bar overrides

### App Doesn't Look Good:
- Ensure running release build (not debug)
- Check for debug overlays or logging
- Use example workbooks (look professional)
- Consider light/dark mode

### Screenshots Look Blurry:
- Ensure using @3x or @2x resolution
- Don't resize screenshots manually
- Capture at native resolution
- PNG format preferred over JPG

## Resources

- **App Store Screenshot Specs:** https://developer.apple.com/help/app-store-connect/reference/screenshot-specifications
- **Fastlane Frameit:** https://docs.fastlane.tools/actions/frameit/
- **Human Interface Guidelines:** https://developer.apple.com/design/human-interface-guidelines/screenshots
- **Marketing Resources:** https://developer.apple.com/app-store/marketing/

---

**Next Steps:** After capturing screenshots, proceed to uploading them in APP_STORE_SUBMISSION_GUIDE.md
