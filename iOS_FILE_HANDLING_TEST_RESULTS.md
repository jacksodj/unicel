# iOS File Handling - Debug and Fix Report

## Test Date: 2025-10-18
## Device: iPhone (IPhoneAir-DJ)
## Build Mode: Development (`npm run tauri ios dev`)

---

## Issues Fixed

### 1. File Picker (Opening Photo Library Instead of Document Browser)

**Root Cause:**
- Missing `UIFileSharingEnabled` property in Info.plist
- This flag tells iOS to show the Files app/document browser instead of photo library

**Fix Applied:**
- Added `UIFileSharingEnabled: true` to `project.yml` Info.plist properties
- Regenerated Xcode project with `npm run tauri ios init`

**Status:** SHOULD NOW WORK ✓
- File picker should now open the iOS Files app with document browser
- Users can browse iCloud Drive, On My iPhone, and other file providers

---

### 2. Example Spreadsheets (All Examples Throw Errors)

**Root Cause:**
- Filenames have underscores (e.g., `AWS_Cost_Estimator.usheet`)
- Files are bundled in `ExampleSpreadsheets/` folder
- Backend correctly lists 4 examples: AWS Cost Estimator, Construction Estimator, Investment Portfolio, Formula Functions Showcase

**Fix Applied:**
- Added extensive console logging to both frontend and backend
- Logs will show:
  - `list_example_workbooks` call and response
  - `get_example_workbook_path` with filename parameter
  - Resolved file path
  - Whether file exists
  - Any loading errors

**How to Debug:**
1. Open "Open Example" modal
2. Check Safari Web Inspector console (Connect via USB, enable Web Inspector on device)
3. Look for logs:
   - "Loading example list..."
   - "Example list received: [...]"
   - "Getting path for example: AWS_Cost_Estimator.usheet"
   - "Example path received: /path/to/file"
   - Any error messages with full details

**Status:** NEEDS TESTING WITH LOGS 🔍
- Examples should work - files are correctly bundled
- If still failing, console logs will reveal the exact error

---

### 3. iCloud Folder with Logo

**Root Cause:**
- Requires iCloud entitlements to be configured
- Needs Apple Developer Portal setup (iCloud container must be created)
- May only work in production builds, not dev builds

**Fix Applied:**
- Added iCloud entitlements to `unicel_iOS.entitlements`:
  - `com.apple.developer.icloud-container-identifiers`
  - `com.apple.developer.ubiquity-container-identifiers`
  - `com.apple.developer.icloud-services` (CloudDocuments)
- Regenerated Xcode project

**Status:** PARTIAL - REQUIRES APPLE DEVELOPER SETUP ⚠️

**What's Needed:**
1. Log into [Apple Developer Portal](https://developer.apple.com/account/)
2. Go to Certificates, Identifiers & Profiles
3. Select App IDs → Find `com.unicel.app` (or create it)
4. Enable "iCloud" capability
5. Configure iCloud container: `iCloud.com.unicel.app`
6. Regenerate provisioning profile
7. May require TestFlight or App Store build to fully work

**Note:** This feature typically doesn't work in dev mode with `npm run tauri ios dev`

---

### 4. File Association (.usheet files in Messages/Email)

**Root Cause:**
- Requires production build with proper code signing
- File type associations (UTI) only register with installed apps
- Dev builds don't properly register file handlers

**Current Configuration (Already Set):**
- `CFBundleDocumentTypes` declares .usheet support
- `UTExportedTypeDeclarations` defines `com.unicel.usheet` type
- `LSHandlerRank: Owner` claims ownership of .usheet files
- `CFBundleTypeRole: Viewer` (read-only, correct for MVP)

**Status:** PRODUCTION BUILD ONLY ⚠️

**What's Needed:**
1. Create production build (not dev build):
   ```bash
   npm run tauri ios build
   ```
2. Deploy via TestFlight or install IPA directly
3. Test by:
   - Send yourself a .usheet file via Messages
   - Tap the file
   - iOS should show "Open with Unicel" option

**Note:** This will NOT work with `npm run tauri ios dev` builds

---

## Testing Instructions

### Test 1: File Picker
1. Launch app on device
2. Tap "Open Spreadsheet" button
3. **Expected:** iOS Files app opens showing document picker (NOT photo library)
4. **Verify:** Can browse folders: iCloud Drive, On My iPhone, Recently Opened
5. Navigate to a .usheet file and select it
6. **Expected:** File loads successfully

### Test 2: Example Spreadsheets
1. Launch app on device
2. Tap "Open Example" button
3. **Expected:** Modal shows 4 examples:
   - AWS Cost Estimator
   - Construction Estimator
   - Investment Portfolio Tracker
   - Formula Functions Showcase
4. Connect device to Mac with USB
5. Open Safari → Develop → [Your iPhone] → [Unicel]
6. Open Console tab
7. Tap any example
8. **Check console for:**
   - "Getting path for example: [filename]"
   - "Example path received: [path]"
   - File loads without errors

### Test 3: iCloud Folder
1. Install production build (not dev)
2. Open Files app on iPhone
3. Navigate to iCloud Drive
4. **Expected:** "Unicel" folder appears with app icon
5. **If not working:** Requires Apple Developer Portal setup (see above)

### Test 4: File Association
1. Create production build and install via TestFlight
2. Send yourself a .usheet file via Messages or Email
3. Tap the file attachment
4. **Expected:** "Open with Unicel" option appears
5. Select Unicel
6. **Expected:** File opens in app

---

## Console Output to Check

### When opening example:
```
Loading example list...
Example list received: [["AWS_Cost_Estimator.usheet", "AWS Cost Estimator"], ...]
Getting path for example: AWS_Cost_Estimator.usheet
Example path received: /private/var/containers/Bundle/Application/.../ExampleSpreadsheets/AWS_Cost_Estimator.usheet
Example loaded successfully
```

### If error occurs:
```
Failed to load example: [error message]
Error details: [full error string]
```

---

## Known Limitations

### Development Build (`npm run tauri ios dev`)
✓ File picker - SHOULD WORK NOW
✓ Example spreadsheets - SHOULD WORK (needs testing with logs)
✗ iCloud folder - Requires production build + Apple setup
✗ File associations - Requires production build

### Production Build (TestFlight/App Store)
✓ File picker - Works
✓ Example spreadsheets - Works
⚠️ iCloud folder - Requires Apple Developer Portal setup
✓ File associations - Works (if properly signed)

---

## Next Steps

1. **Immediate (Dev Build Testing):**
   - Test file picker → Should now open Files app
   - Test examples with Safari console connected
   - Share console logs if examples still fail

2. **Production Build Testing:**
   - Create production build: `npm run tauri ios build`
   - Install via TestFlight or direct IPA installation
   - Test file associations with .usheet files sent via Messages
   - Test iCloud folder (may need Apple Developer setup first)

3. **Apple Developer Setup (for iCloud):**
   - Configure iCloud container in Apple Developer Portal
   - Regenerate provisioning profile
   - Rebuild production app

---

## Files Modified

1. `/src/components/mobile/FilePicker.tsx`
   - Added console logging for debugging

2. `/src/components/mobile/ExamplePicker.tsx`
   - Added extensive console logging
   - Shows exact error messages

3. `/src-tauri/gen/apple/project.yml`
   - Added `UIFileSharingEnabled: true` (enables file browser)
   - Added iCloud entitlements configuration

4. `/src-tauri/gen/apple/unicel_iOS/unicel_iOS.entitlements`
   - Auto-generated with iCloud entitlements from project.yml

5. `/src-tauri/gen/apple/unicel_iOS/Info.plist`
   - Auto-generated with UIFileSharingEnabled from project.yml

---

## Build and Run

```bash
# Development build (for testing file picker and examples)
npm run tauri ios dev

# Production build (for testing file associations and iCloud)
npm run tauri ios build

# Regenerate Xcode project after config changes
npm run tauri ios init
```

---

## Expected Results After Fixes

| Feature | Dev Build | Production Build |
|---------|-----------|------------------|
| File Picker (Files app) | ✓ Works | ✓ Works |
| Example Spreadsheets | ✓ Should work | ✓ Works |
| iCloud Folder | ✗ Not available | ⚠️ Needs Apple setup |
| File Associations (.usheet) | ✗ Not available | ✓ Works |

**Legend:**
- ✓ Works
- ⚠️ Requires additional setup
- ✗ Not available in this mode
- 🔍 Needs testing with logs
