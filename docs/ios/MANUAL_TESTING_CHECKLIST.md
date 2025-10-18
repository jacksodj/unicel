# iOS Manual Testing Checklist

**Quick reference for completing Week 25 tasks**
**Date:** October 18, 2025

---

## Prerequisites (One-Time Setup)

Run these commands once to set up your environment:

```bash
# 1. Switch to Xcode developer tools
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer

# 2. Accept Xcode license
sudo xcodebuild -license accept

# 3. Verify setup
xcodebuild -version
# Should show: Xcode 26.0.1

# 4. Check iOS simulators available
xcrun simctl list devices available | grep -i "iphone\|ipad"
```

---

## Task 10.4: Code Signing Configuration

**Time Required:** 5-10 minutes
**Manual Steps:** YES

### Option A: Sign to Run Locally (No Apple Account Needed)

1. Open Xcode project:
   ```bash
   open /Users/dennisjackson/Code/unicel/src-tauri/gen/apple/unicel.xcodeproj
   ```

2. In Xcode:
   - Select project "unicel" in left sidebar
   - Select target "unicel_iOS"
   - Click "Signing & Capabilities" tab
   - Uncheck "Automatically manage signing"
   - Set Team to: "None (Sign to Run Locally)"

3. Verify: Green checkmark appears, no red errors

### Option B: Automatic Signing (Free Apple Account)

1. Add Apple ID to Xcode:
   - Xcode → Settings → Accounts
   - Click "+" → Sign in with Apple ID

2. Configure in project:
   - Open project (same as Option A)
   - Check "Automatically manage signing"
   - Select your Apple ID from Team dropdown

3. Fix bundle ID if needed:
   - Change `com.unicel.app` to `com.yourname.unicel.app`

**Full details:** See `/Users/dennisjackson/Code/unicel/docs/ios/CODE_SIGNING_GUIDE.md`

---

## Task 10.5: iOS Simulator Testing

**Time Required:** 10-15 minutes (first build)
**Manual Steps:** YES

### Quick Method: Use Testing Script

```bash
# Run automated testing script
/Users/dennisjackson/Code/unicel/scripts/test-ios-simulator.sh

# This will:
# - Verify Xcode setup
# - Build frontend
# - Select simulator
# - Build and launch iOS app
```

### Manual Method: Step-by-Step

```bash
# 1. Build frontend
npm run build

# 2. Build and run in simulator (5-10 minutes first time)
npm run tauri:ios:dev

# 3. Wait for simulator to launch and app to install
```

### Testing Checklist (iPhone Simulator)

In the running simulator, verify:

- [ ] App launches without crashing
- [ ] Main screen renders (grid visible)
- [ ] Status bar shows app name
- [ ] Toolbar displays controls
- [ ] Touch tap selects cells
- [ ] Swipe scrolls grid
- [ ] Example workbooks listed
- [ ] Can open example file

### Testing Checklist (iPad Simulator)

```bash
# Stop current build (Ctrl+C)
# Launch for iPad:
npm run tauri:ios:dev
# (Tauri should auto-detect available simulator)
```

In iPad simulator, verify:

- [ ] App launches in landscape
- [ ] Larger grid layout shown
- [ ] Toolbar adapted for larger screen
- [ ] Touch gestures work
- [ ] Safe area insets respected
- [ ] Rotation works (portrait/landscape)

---

## Task 10.7: Tauri Commands Testing

**Time Required:** 15-20 minutes
**Manual Steps:** YES

### Prerequisites
- Task 10.5 completed
- App running in simulator
- Safari Developer menu enabled

### Setup Safari Web Inspector

1. On Mac:
   - Safari → Settings → Advanced
   - Check "Show Develop menu in menu bar"

2. In Simulator (while app running):
   - Safari → Develop → [Your Mac Name] → iOS Simulator → unicel → localhost

3. Open JavaScript console

### Test Commands

Run each command in the console:

#### 1. Load Example Workbook
```javascript
// Get example workbook path
const path = await window.__TAURI__.invoke('get_example_workbook_path', {
  filename: 'construction_estimator.usheet'
});

// Load it
await window.__TAURI__.invoke('load_workbook', { path });

// Verify
const info = await window.__TAURI__.invoke('get_workbook_info');
console.log('Workbook loaded:', info);
```

**Expected:** Workbook info object with sheets, cells, etc.

#### 2. Get Cell Data
```javascript
const cells = await window.__TAURI__.invoke('get_sheet_cells');
console.log('Cells:', cells);
```

**Expected:** Array of cell objects with values and units

#### 3. Display Preference Toggle
```javascript
// Switch to Metric
await window.__TAURI__.invoke('set_display_mode', { mode: 'Metric' });

// Get cells again (should show metric units)
const metricCells = await window.__TAURI__.invoke('get_sheet_cells');
console.log('Metric cells:', metricCells);

// Switch to Imperial
await window.__TAURI__.invoke('set_display_mode', { mode: 'Imperial' });

const imperialCells = await window.__TAURI__.invoke('get_sheet_cells');
console.log('Imperial cells:', imperialCells);
```

**Expected:** Cell values convert between metric/imperial

#### 4. Unit Preferences
```javascript
const prefs = await window.__TAURI__.invoke('get_unit_preferences');
console.log('Unit preferences:', prefs);
```

**Expected:** Preferences object with unit settings

#### 5. Units in Use
```javascript
const units = await window.__TAURI__.invoke('get_units_in_use');
console.log('Units currently in use:', units);
```

**Expected:** Array of unit strings (e.g., ['USD', 'ft', 'sqft'])

### Testing Matrix

Test each workbook:

| Workbook | Load | Display | Toggle | Units |
|----------|------|---------|--------|-------|
| construction_estimator.usheet | [ ] | [ ] | [ ] | [ ] |
| aws_cost_estimator.usheet | [ ] | [ ] | [ ] | [ ] |
| investment_portfolio.usheet | [ ] | [ ] | [ ] | [ ] |
| formula_functions_showcase.usheet | [ ] | [ ] | [ ] | [ ] |

---

## Mobile UI Verification

### Touch Gestures (Week 26 Implementation)

Test these gestures in simulator:

- [ ] **Tap:** Select cell, button interaction
- [ ] **Swipe:** Scroll grid horizontally/vertically
- [ ] **Pinch:** Zoom in/out (if implemented)
- [ ] **Long Press:** Cell details (if implemented)
- [ ] **Double Tap:** Edit cell (if enabled on mobile)

### MobileGrid Component

Verify:
- [ ] Cells render correctly
- [ ] Cell borders visible
- [ ] Unit labels display
- [ ] Formula results calculate
- [ ] Warnings show (orange cells)
- [ ] Virtual scrolling smooth

### MobileToolbar

Verify:
- [ ] File name displays
- [ ] Open button works
- [ ] Metric/Imperial toggle visible
- [ ] Settings button accessible
- [ ] Layout responsive to orientation

### MobileStatusBar

Verify:
- [ ] Selected cell address shows
- [ ] Cell value displays
- [ ] Unit displays
- [ ] Safe area insets correct (notch/home indicator)

---

## Performance Testing

### Load Times

Measure with browser DevTools Performance tab:

- [ ] App launch: < 1 second
- [ ] Workbook load (10KB): < 500ms
- [ ] Workbook load (50KB): < 1 second
- [ ] Cell render (100 cells): < 100ms
- [ ] Metric/Imperial toggle: < 100ms

### Frame Rate

Use Simulator → Debug → Color Blended Layers:

- [ ] Scroll: 60fps sustained
- [ ] Pinch zoom: 60fps
- [ ] Toggle display: no visible lag
- [ ] Cell selection: instant feedback

### Memory Usage

Use Xcode → Debug Navigator → Memory:

- [ ] Initial load: < 100MB
- [ ] After loading large workbook: < 200MB
- [ ] No memory leaks during navigation
- [ ] Memory stable during scrolling

---

## Screenshot Documentation

Take screenshots for documentation:

### iPhone Screenshots
- [ ] App launch (home screen)
- [ ] Empty workbook
- [ ] Workbook with data
- [ ] Example workbook open
- [ ] Metric display mode
- [ ] Imperial display mode
- [ ] Cell selected
- [ ] Touch gesture in action

### iPad Screenshots
- [ ] Landscape mode
- [ ] Portrait mode
- [ ] Split view (if applicable)
- [ ] Larger grid layout
- [ ] Toolbar in landscape
- [ ] Example workbook on iPad

### Error States
- [ ] File not found
- [ ] Load error
- [ ] Formula error
- [ ] Unit incompatibility warning

---

## Issues to Document

If you encounter any issues, document:

1. **Issue description:**
2. **Steps to reproduce:**
3. **Expected behavior:**
4. **Actual behavior:**
5. **Screenshot:**
6. **Console errors:**
7. **Device/simulator:**

---

## Success Criteria

All tasks complete when:

- ✅ Code signing configured (Option A or B)
- ✅ App builds successfully for simulator
- ✅ iPhone simulator test passed
- ✅ iPad simulator test passed
- ✅ All Tauri commands work
- ✅ Mobile UI renders correctly
- ✅ Touch gestures functional
- ✅ Performance meets targets
- ✅ Screenshots captured

---

## Next Steps

After completing manual tests:

1. **Update TASKS.md:**
   ```markdown
   - [x] 10.4: Configure code signing and provisioning profiles
   - [x] 10.5: Test basic build in iOS simulator (iPhone and iPad)
   - [x] 10.7: Verify Tauri commands work on iOS
   ```

2. **Document results:**
   - Add screenshots to `/Users/dennisjackson/Code/unicel/docs/ios/screenshots/`
   - Note any issues in GitHub Issues
   - Update performance metrics

3. **Prepare for Week 29:**
   - Generate app icons (all sizes)
   - Create App Store screenshots
   - Write privacy policy
   - Plan TestFlight beta

---

## Quick Commands Reference

```bash
# Open Xcode project
open /Users/dennisjackson/Code/unicel/src-tauri/gen/apple/unicel.xcodeproj

# Run automated test
/Users/dennisjackson/Code/unicel/scripts/test-ios-simulator.sh

# Build for simulator
npm run tauri:ios:dev

# Build release
npm run tauri:ios:build

# List simulators
xcrun simctl list devices available

# Kill simulator
killall Simulator

# Check Xcode path
xcode-select -p

# View build logs
tail -f ~/Library/Logs/iOS\ Simulator/*/system.log
```

---

**Generated:** October 18, 2025
**File:** `/Users/dennisjackson/Code/unicel/docs/ios/MANUAL_TESTING_CHECKLIST.md`
