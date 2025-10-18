# iOS Testing Checklist - Critical Fixes
## Date: October 18, 2025

## Pre-Testing Setup

### 1. Verify Fixes Applied
- [x] Info.plist patch script created
- [x] Rust code updated for example path resolution
- [x] npm scripts updated
- [x] Info.plist successfully patched

### 2. Build Application
```bash
# Clean build
npm run build

# Start iOS simulator
npm run tauri:ios:dev
```

---

## Test Case 1: File Picker Opens Files App (Not Photos)

### Objective
Verify that tapping "Open Spreadsheet" opens the Files app document picker, not the Photos app image picker.

### Steps
1. Launch Unicel app on iOS simulator
2. On the welcome screen, tap "Open Spreadsheet" button
3. Observe which picker appears

### Expected Result
- ✓ Files app document picker appears
- ✓ Can browse folders (On My iPhone, iCloud Drive, etc.)
- ✓ Can see .usheet files (if any exist)
- ✓ Picker shows "Select Spreadsheet" title

### Failure Indicators
- ✗ Photos app appears
- ✗ Camera roll/albums visible
- ✗ Cannot navigate to Files/Documents
- ✗ No .usheet file filter

### Debugging
If test fails, check:
```bash
# Verify Info.plist has correct UTI
/usr/libexec/PlistBuddy -c "Print :UTExportedTypeDeclarations:0:UTTypeConformsTo" \
  src-tauri/gen/apple/unicel_iOS/Info.plist

# Should output: public.json, public.data, public.content
```

---

## Test Case 2: Example Files Load Successfully

### Objective
Verify all bundled example spreadsheets open without errors.

### Steps for Each Example

#### 2.1: AWS Cost Estimator
1. Tap "Open Example"
2. Select "AWS Cost Estimator"
3. Wait for loading

**Expected:**
- ✓ Example loads within 2 seconds
- ✓ Grid displays with data
- ✓ Sheet name shows "US East" or similar
- ✓ No error messages

**If Fails:**
Check console for error message:
```
Safari Web Inspector → Console
Look for: "Failed to load example: AWS_Cost_Estimator.usheet"
```

#### 2.2: Construction Estimator
1. Tap "Open Example"
2. Select "Construction Estimator"
3. Wait for loading

**Expected:**
- ✓ Example loads within 2 seconds
- ✓ Grid displays with construction data
- ✓ No error messages

#### 2.3: Investment Portfolio
1. Tap "Open Example"
2. Select "Investment Portfolio Tracker"
3. Wait for loading

**Expected:**
- ✓ Example loads within 2 seconds
- ✓ Grid displays with investment data
- ✓ No error messages

#### 2.4: Formula Functions Showcase
1. Tap "Open Example"
2. Select "Formula Functions Showcase"
3. Wait for loading

**Expected:**
- ✓ Example loads within 2 seconds
- ✓ Grid displays with formula examples
- ✓ No error messages

---

## Test Case 3: Example Picker UI

### Objective
Verify the example picker modal functions correctly.

### Steps
1. From welcome screen, tap "Open Example"
2. Observe modal appearance
3. Scroll through examples
4. Tap close button
5. Reopen and select an example

### Expected Result
- ✓ Modal slides up from bottom
- ✓ Shows all 4 examples with proper names
- ✓ Each example has icon and description
- ✓ Close button works
- ✓ Tapping example loads it
- ✓ Modal dismisses after selection

---

## Test Case 4: Error Handling

### Objective
Verify proper error messages if something goes wrong.

### Steps
1. Rename an example file to break path:
   ```bash
   mv src-tauri/gen/apple/ExampleSpreadsheets/AWS_Cost_Estimator.usheet \
      src-tauri/gen/apple/ExampleSpreadsheets/AWS_Cost_Estimator.broken
   ```
2. Try to open AWS Cost Estimator example
3. Observe error message

### Expected Result
- ✓ Error message displays
- ✓ Message includes filename
- ✓ User can return to example picker
- ✓ Other examples still work

### Cleanup
```bash
mv src-tauri/gen/apple/ExampleSpreadsheets/AWS_Cost_Estimator.broken \
   src-tauri/gen/apple/ExampleSpreadsheets/AWS_Cost_Estimator.usheet
```

---

## Test Case 5: Console Logging

### Objective
Verify debug logging works for troubleshooting.

### Steps
1. Open Safari Web Inspector
2. Connect to iOS simulator
3. Open Unicel app
4. Select an example
5. Check console output

### Expected Logs
```
Opening file picker...
Loading example list...
Example list received: [...]
Getting path for example: AWS_Cost_Estimator.usheet
Example path received: /path/to/AWS_Cost_Estimator.usheet
Example loaded successfully
```

### Rust Backend Logs (if accessible)
```
get_example_workbook_path called with filename: AWS_Cost_Estimator.usheet
Trying iOS resource path: ExampleSpreadsheets/AWS_Cost_Estimator.usheet
Found iOS example at: /path/to/AWS_Cost_Estimator.usheet
```

---

## Test Case 6: Real Device Testing (if available)

### Objective
Verify fixes work on physical iPhone/iPad, not just simulator.

### Steps
1. Build for device: `npm run tauri:ios:build`
2. Install on device via Xcode
3. Run same tests as above

### Critical Checks
- ✓ File picker behavior on real device
- ✓ Examples load from device bundle
- ✓ Performance is acceptable
- ✓ No crashes or warnings

---

## Regression Testing

### Objective
Ensure fixes don't break existing functionality.

### Tests
1. **Grid Interaction**
   - ✓ Can scroll grid
   - ✓ Can select cells
   - ✓ Cell values display correctly

2. **Display Toggle**
   - ✓ Can toggle Metric/Imperial
   - ✓ Units convert properly
   - ✓ No crashes

3. **Sheet Navigation**
   - ✓ Can switch sheets
   - ✓ Sheet names display
   - ✓ Data loads for each sheet

---

## Performance Testing

### Metrics to Check
- File picker opens: < 500ms
- Example loads: < 2 seconds
- Grid renders: < 1 second
- No dropped frames during scrolling

---

## Final Checklist

Before marking as COMPLETE:
- [ ] File picker opens Files app (not Photos)
- [ ] All 4 examples load successfully
- [ ] No error messages in console
- [ ] Error handling works when files missing
- [ ] Logging provides useful debug info
- [ ] Performance is acceptable
- [ ] No regressions in existing features

---

## Test Results

### Test Case 1: File Picker
- Status: ⏳ PENDING
- Tester: _______
- Date: _______
- Result: _______

### Test Case 2.1: AWS Cost Estimator
- Status: ⏳ PENDING
- Result: _______

### Test Case 2.2: Construction Estimator
- Status: ⏳ PENDING
- Result: _______

### Test Case 2.3: Investment Portfolio
- Status: ⏳ PENDING
- Result: _______

### Test Case 2.4: Formula Functions
- Status: ⏳ PENDING
- Result: _______

### Test Case 3: Example Picker UI
- Status: ⏳ PENDING
- Result: _______

### Test Case 4: Error Handling
- Status: ⏳ PENDING
- Result: _______

### Test Case 5: Console Logging
- Status: ⏳ PENDING
- Result: _______

---

## Notes

_Add any observations, issues, or additional findings here._
