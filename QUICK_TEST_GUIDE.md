# Quick Test Guide - iOS File Handling

**Purpose:** Verify all 4 file handling features work on physical iOS device.

**Time Required:** 15 minutes

## Setup (2 minutes)

1. Connect iPhone to Mac via USB
2. Trust computer if prompted on iPhone
3. Install app:
   ```bash
   cd /Users/dennisjackson/Code/unicel
   npm run tauri ios dev
   ```
   Or install pre-built IPA:
   ```bash
   # Located at: src-tauri/gen/apple/build/arm64/Unicel.ipa
   # Install via Xcode or Apple Configurator
   ```

## Test 1: Example Spreadsheets (3 minutes) ✅

**Most Important - Test This First!**

1. Launch Unicel app
2. Tap "Open Example" button
3. Select "AWS Cost Estimator"
4. **Verify:** Spreadsheet loads and displays correctly
5. Try other examples to confirm all work

**Expected:** All 4 examples load without errors.

## Test 2: File Picker (3 minutes) ✅

1. Close spreadsheet (if open)
2. Tap "Open Spreadsheet" button
3. **Verify:** Document picker opens (NOT photo library)
4. **Verify:** Can browse "On My iPhone", "iCloud Drive"
5. Navigate to Downloads or Documents
6. **Verify:** Only .usheet files are selectable

**Expected:** Document browser appears, not camera roll.

## Test 3: File Association (5 minutes) ✅

### Test 3a: From Files App

1. Open Files app on iPhone
2. Navigate to any folder
3. Download or copy a .usheet file there
4. Tap the .usheet file
5. **Verify:** File opens in Unicel directly

### Test 3b: From Messages (if time permits)

1. AirDrop a .usheet file to iPhone
2. Or send via Messages to yourself
3. Tap the file attachment
4. **Verify:** Option to "Open in Unicel" appears
5. Tap it
6. **Verify:** File opens in Unicel

**Expected:** .usheet files open in Unicel when tapped.

## Test 4: iCloud Drive (2 minutes - SKIP if not set up) ⚠️

**Note:** Requires Apple Developer Portal setup (see ICLOUD_SETUP.md)

1. Open Files app
2. Navigate to iCloud Drive
3. Look for "Unicel" folder
4. **Expected:** Folder appears (may take time)

**If folder doesn't appear:** iCloud setup not complete (expected for initial testing).

## Quick Success Checklist

After 15 minutes, you should confirm:

- [x] ✅ Example spreadsheets load
- [x] ✅ File picker shows documents (not photos)
- [x] ✅ .usheet files open in Unicel from Files app
- [ ] ⚠️ iCloud folder appears (optional - requires setup)

## If Something Doesn't Work

### Examples don't load
**Error:** "Example file not found"

**Quick Fix:**
```bash
# Rebuild app
npm run tauri ios build
# Reinstall on device
```

### Photo picker appears instead of file browser
**Issue:** Seeing camera roll instead of documents

**Quick Fix:**
- Delete app from device
- Clean build: `npm run tauri ios build`
- Reinstall

### File association doesn't work
**Issue:** Tapping .usheet shows "No app to open"

**Quick Fix:**
1. Delete Unicel app from device
2. Restart iPhone
3. Reinstall app
4. Try again

## Console Debugging

If issues occur, check logs:

1. Keep iPhone connected to Mac
2. Open Console.app on Mac
3. Filter to "Unicel"
4. Reproduce the issue
5. Look for errors in console

Common errors to watch for:
- "Example file not found" → Examples not bundled
- "Permission denied" → Capabilities not configured
- "iCloud container not found" → iCloud not set up

## Performance Check

Verify app is responsive:

- App launch: < 2 seconds
- Example load: < 1 second
- File picker open: < 0.5 seconds
- Sheet switching: < 0.3 seconds

If slow, check:
- Release build (not debug)
- No excessive logging
- Example files not too large

## Next Steps

After quick test passes:

1. ✅ All features working → Proceed to full testing (see TESTING_GUIDE.md)
2. ⚠️ Minor issues → Check troubleshooting above
3. ❌ Major issues → Review implementation in iOS_FILE_HANDLING_IMPLEMENTATION.md

## Contact

If issues persist after troubleshooting:
- Review detailed guides: TESTING_GUIDE.md, ICLOUD_SETUP.md
- Check iOS console logs
- Verify Xcode project configuration

---

**Summary:**
- ✅ 3 out of 4 features can be tested immediately (no setup required)
- ⚠️ iCloud requires Apple Developer Portal setup first
- 🎯 Focus on examples, file picker, and file association for quick validation
