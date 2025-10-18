# iOS File Handling Testing Guide

This guide covers testing all 4 critical file handling features implemented for Unicel iOS.

## Prerequisites

- Physical iOS device (iPhone or iPad)
- Device connected via USB or Wi-Fi
- Xcode 15.0+
- Latest build installed on device

## Feature 1: File Association (.usheet files)

### Test 1.1: Open from Messages

1. Send yourself a .usheet file via Messages (use AirDrop from Mac)
2. Open Messages on iOS device
3. Tap the .usheet file attachment
4. **Expected:** iOS shows "Open in Unicel" option
5. Tap "Open in Unicel"
6. **Expected:** Unicel launches and displays the spreadsheet

### Test 1.2: Open from Email

1. Email yourself a .usheet file
2. Open Mail app on iOS device
3. Tap the .usheet attachment
4. **Expected:** Share sheet appears with Unicel icon
5. Select "Open in Unicel"
6. **Expected:** File opens in Unicel

### Test 1.3: Open from Files App

1. Save a .usheet file to Files app (On My iPhone or iCloud Drive)
2. Navigate to the file in Files app
3. Tap the .usheet file
4. **Expected:** File opens directly in Unicel

**If file association doesn't work:**
- Check Info.plist has UTExportedTypeDeclarations
- Verify CFBundleDocumentTypes is configured
- Reinstall the app (delete and install fresh)
- Restart iOS device

## Feature 2: Example Spreadsheets

### Test 2.1: Access Examples

1. Launch Unicel app
2. On the welcome screen, tap "Open Example" button
3. **Expected:** Modal appears showing 4 example spreadsheets:
   - AWS Cost Estimator
   - Construction Estimator
   - Investment Portfolio Tracker
   - Formula Functions Showcase

### Test 2.2: Load Example

1. Tap any example from the list
2. **Expected:** Example loads and displays correctly
3. Verify you can:
   - View all cells with data
   - Toggle between Metric/Imperial
   - Switch between sheets (if multi-sheet example)
   - Select cells and see values in status bar

### Test 2.3: Example Files Bundled

1. Close any open spreadsheet
2. Tap "Open Example" again
3. Try loading each example one by one
4. **Expected:** All 4 examples load successfully without errors

**If examples don't load:**
- Check Xcode project includes ExampleSpreadsheets folder
- Verify files are in "Copy Bundle Resources" build phase
- Check console for path errors

## Feature 3: File Picker (Documents, not Photos)

### Test 3.1: File Picker Shows Documents

1. Launch Unicel
2. Tap "Open Spreadsheet" button
3. **Expected:** System file picker appears showing:
   - "Browse" tab active (not "Photos")
   - Folders like "On My iPhone", "iCloud Drive", "Downloads"
   - .usheet files visible and selectable
   - NO photo library or camera roll

### Test 3.2: Filter Works

1. In file picker, navigate to a folder with mixed files
2. **Expected:** Only .usheet files are selectable (or highlighted)
3. Other file types (.pdf, .xlsx, .txt) should be grayed out

### Test 3.3: Open from Different Locations

Test opening .usheet files from:
- iCloud Drive
- On My iPhone > Downloads
- On My iPhone > Unicel (if exists)
- Recent files

**Expected:** All locations work correctly

**If photo picker appears instead:**
- Check tauri-plugin-dialog configuration
- Verify filters array includes only 'usheet' extension
- Check iOS capabilities don't include photo library access

## Feature 4: iCloud Drive Integration

**Note:** This feature requires additional setup in Apple Developer Portal.
See `ICLOUD_SETUP.md` for complete instructions.

### Test 4.1: iCloud Capability Enabled

1. Open Xcode project
2. Select unicel_iOS target
3. Go to "Signing & Capabilities"
4. **Expected:** "iCloud" capability is listed
5. Verify "iCloud Documents" is checked
6. Container should be: `iCloud.com.unicel.app`

### Test 4.2: Unicel Folder in iCloud Drive

**After completing iCloud setup:**

1. Open Files app on iOS
2. Navigate to "iCloud Drive"
3. **Expected:** "Unicel" folder appears (may take first launch)
4. Folder should have Unicel app icon

### Test 4.3: Save to iCloud

**Note:** This requires save functionality (read-only viewer doesn't save)

For future implementation:
1. Open a .usheet file in Unicel
2. Save or copy to iCloud Drive > Unicel
3. Verify file appears on other devices signed into same iCloud account

### Test 4.4: iCloud Sync

1. Save .usheet file to iCloud Drive > Unicel on Mac
2. Wait 1-2 minutes for sync
3. Open Files app on iOS device
4. Navigate to iCloud Drive > Unicel
5. **Expected:** File appears and can be opened in Unicel

**If iCloud doesn't work:**
- Verify device is signed into iCloud (Settings > [Your Name])
- Check iCloud Drive is enabled (Settings > [Your Name] > iCloud > iCloud Drive)
- Ensure not in Low Data Mode or Airplane Mode
- Check Apple Developer Portal has iCloud container registered
- Verify provisioning profile includes iCloud capability
- See troubleshooting in ICLOUD_SETUP.md

## Testing Checklist

### File Association
- [ ] Open .usheet from Messages
- [ ] Open .usheet from Email
- [ ] Open .usheet from Files app
- [ ] File opens in Unicel (not generic viewer)

### Example Spreadsheets
- [ ] "Open Example" button appears
- [ ] Modal shows all 4 examples
- [ ] AWS Cost Estimator loads
- [ ] Construction Estimator loads
- [ ] Investment Portfolio loads
- [ ] Formula Functions Showcase loads

### File Picker
- [ ] Opens to documents (not photos)
- [ ] Shows iCloud Drive
- [ ] Shows On My iPhone
- [ ] Filters to .usheet files only
- [ ] Can navigate folders
- [ ] Can select and open files

### iCloud Drive (Setup Required)
- [ ] iCloud capability enabled in Xcode
- [ ] Container ID: iCloud.com.unicel.app
- [ ] Unicel folder appears in Files app
- [ ] Files sync between devices
- [ ] Can open files from iCloud

## Common Issues

### File association not working
**Symptom:** Tapping .usheet shows "No app available"

**Fix:**
1. Delete app from device
2. Clean build (Cmd+Shift+K)
3. Rebuild and reinstall
4. Restart device if needed

### Examples not loading
**Symptom:** "Example file not found" error

**Fix:**
1. Verify ExampleSpreadsheets folder in Xcode navigator
2. Check files are in "Copy Bundle Resources" phase
3. Rebuild app

### Photo picker appears
**Symptom:** Camera roll shows instead of file browser

**Fix:**
1. Check FilePicker.tsx uses correct filter: `extensions: ['usheet']`
2. Verify tauri-plugin-dialog is latest version
3. Check permissions in capabilities/default.json

### iCloud not syncing
**Symptom:** Files don't appear on other devices

**Fix:**
1. Check network connection
2. Wait 5-10 minutes (iCloud can be slow)
3. Force quit Files app and reopen
4. Toggle iCloud Drive off/on in Settings
5. Verify not out of iCloud storage

## Performance Benchmarks

Expected performance on iPhone 15 Pro:

- App launch: < 2s
- Example load: < 1s
- File picker open: < 500ms
- .usheet file open: < 2s (for 10KB file)
- Sheet switch: < 300ms

If performance is slower, check:
- Debug symbols not stripped in Release build
- Example files are compressed
- No excessive logging in production

## Regression Testing

After any changes, verify:

1. **File Opening Still Works**
   - Open from Messages
   - Open from Files app

2. **Examples Still Load**
   - All 4 examples open correctly

3. **File Picker Still Shows Documents**
   - No photo library access

4. **Core Viewer Features Work**
   - Cell selection
   - Metric/Imperial toggle
   - Sheet navigation
   - Status bar display

## Reporting Issues

When reporting issues, include:

1. iOS version (Settings > General > About)
2. Device model (iPhone/iPad)
3. Exact steps to reproduce
4. Expected vs actual behavior
5. Console logs (connect to Mac, open Console.app)
6. Screenshots or screen recording

## Next Steps

After testing, proceed to:

1. TestFlight distribution (for beta testing)
2. App Store submission
3. User acceptance testing
4. Production release

See `ios-deployment-manager` agent for deployment steps.
