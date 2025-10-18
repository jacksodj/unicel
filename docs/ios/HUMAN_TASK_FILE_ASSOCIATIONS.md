# Manual Task: Testing File Associations

**Agent:** ios-deployment-manager
**Time Required:** 15-20 minutes
**Prerequisite:** TestFlight build installed, test .usheet files available
**Status:** CRITICAL - File associations only work in production builds

---

## Overview

File associations allow iOS to recognize .usheet files and automatically open them in Unicel Viewer. This is a CRITICAL feature for usability.

**Why This Is Important:**
- Users can tap .usheet files from Messages, Mail, Files app
- iOS recognizes Unicel as the owner of .usheet file type
- No need to manually select app each time
- Professional user experience

**Why Production Build Required:**
- Debug/simulator builds don't register file types with iOS
- Only App Store and TestFlight builds register properly
- File type registration happens during app installation
- Can't be tested in Xcode simulator or local device debug builds

---

## Prerequisites

Before testing, ensure:
- [ ] TestFlight build installed on iOS device (see `HUMAN_TASK_TESTFLIGHT.md`)
- [ ] App has orange "BETA" badge on home screen (confirms TestFlight install)
- [ ] Test .usheet files available (see Preparation section)
- [ ] iCloud Drive enabled and signed in

**Verify TestFlight Installation:**

On iOS device:
1. Open TestFlight app
2. Look for "Unicel Viewer" under installed apps
3. Should show "INSTALLED" status
4. Tap to see build version (e.g., 0.5.1)

If not installed, follow `HUMAN_TASK_TESTFLIGHT.md` first.

---

## Part 1: Prepare Test Files

### 1.1 Create Test Files on Mac

```bash
cd /Users/dennisjackson/Code/unicel

# Use existing example files
ls -lh examples/*.usheet

# Should see:
# construction-project.usheet
# aws-cost-analysis.usheet
# investment-portfolio.usheet
```

### 1.2 Create Minimal Test File (Optional)

If you want a small test file:

```bash
cd /Users/dennisjackson/Code/unicel

# Create minimal .usheet file
cat > /tmp/test-minimal.usheet << 'EOF'
{
  "version": "1.0",
  "sheets": [
    {
      "name": "Test Sheet",
      "cells": {
        "A1": {
          "value": 100,
          "unit": "USD",
          "formula": null
        },
        "A2": {
          "value": 50,
          "unit": "USD",
          "formula": null
        },
        "A3": {
          "value": 150,
          "unit": "USD",
          "formula": "=A1+A2"
        }
      }
    }
  ]
}
EOF

# Verify file is valid JSON
cat /tmp/test-minimal.usheet | python3 -m json.tool
```

### 1.3 Verify File Extension

**CRITICAL:** File must have exactly `.usheet` extension (not `.usheet.json`, not `.json`)

```bash
# Check files
ls -1 examples/*.usheet

# Should show:
# examples/construction-project.usheet
# examples/aws-cost-analysis.usheet

# NOT:
# examples/construction-project.usheet.json  ❌ Wrong!
# examples/construction-project.json         ❌ Wrong!
```

**If files have wrong extension:**
```bash
# Rename files
mv examples/construction-project.usheet.json examples/construction-project.usheet
```

---

## Part 2: Test File Opening Methods

### Method 1: Open from iCloud Drive (Files App)

**RECOMMENDED - Most reliable test**

#### 2.1 Upload File to iCloud Drive

**On Mac:**

1. Open Finder
2. Navigate to iCloud Drive (in Favorites sidebar)
3. Create folder "Unicel Test" (if doesn't exist)
4. Copy test file into folder:

```bash
# Copy via command line
cp examples/construction-project.usheet ~/Library/Mobile\ Documents/com~apple~CloudDocs/Unicel\ Test/
```

OR:

Drag `examples/construction-project.usheet` into iCloud Drive folder in Finder

**Wait for sync:**
- File should show cloud icon (uploading)
- Then checkmark icon (synced)
- Usually takes 5-30 seconds

#### 2.2 Open File on iOS Device

**On iOS device:**

1. Open **Files** app (built-in iOS app, blue folder icon)
2. Tap "Browse" (bottom right)
3. Under "Locations," tap "iCloud Drive"
4. Navigate to "Unicel Test" folder
5. You should see `construction-project.usheet`

**Check file icon:**
- If file association is working: File shows Unicel app icon
- If NOT working: File shows generic document icon

6. **Tap on the file**

**Expected Behavior (SUCCESS):**
- File opens immediately in Unicel Viewer
- No "How do you want to open this file?" dialog
- App launches to grid view showing spreadsheet
- You can navigate sheets, see cell values
- Display toggle works (Metric/Imperial)

**Unexpected Behavior (FAILURE):**
- iOS shows "No app to open this file"
- OR: Shows app picker: "Open with..."
- OR: Nothing happens
- See Troubleshooting section

#### 2.3 Verify File Persistence

1. After opening file, press Home button
2. Re-open Unicel Viewer from home screen
3. App should remember last opened file
4. OR: Show file picker to select file again

---

### Method 2: Open from Messages

**Tests file associations via attachment**

#### 2.4 Send File via Messages

**On Mac:**

1. Open **Messages** app
2. Start conversation with yourself (iMessage to your phone number)
3. Drag `construction-project.usheet` into message text field
4. OR: Click attachment button (paperclip) and select file
5. Send message

**Alternative: Send to another person**
- Send to friend/colleague with iPhone
- Ask them to tap file and report results

#### 2.5 Open File on iOS Device

**On iOS device:**

1. Open **Messages** app
2. Open conversation with test message
3. You should see file attachment

**Check attachment:**
- File name: "construction-project.usheet"
- If file association working: Shows Unicel icon
- If NOT working: Shows generic file icon

4. **Tap on attachment**

**Expected Behavior (SUCCESS):**
- File downloads (if not already downloaded)
- Taps open file in Unicel Viewer
- No app picker shown
- Spreadsheet displays correctly

**Alternative tap methods if direct tap doesn't work:**
1. Tap and hold attachment
2. Should see menu: "Quick Look", "Share", "Save to Files"
3. If "Open in Unicel Viewer" appears in menu, tap it
4. If not in menu, file association may not be working

---

### Method 3: Open from Mail

**Tests file associations via email attachment**

#### 2.6 Send File via Email

**On Mac or webmail:**

1. Open **Mail** app or Gmail/Outlook web
2. Compose new email to yourself
3. Attach `construction-project.usheet`
4. Send email

**Email should arrive within seconds**

#### 2.7 Open File on iOS Device

**On iOS device:**

1. Open **Mail** app
2. Open email with attachment
3. You should see attachment preview

**Check attachment:**
- Shows file name: "construction-project.usheet"
- Shows file size (e.g., "45 KB")
- If file association working: Unicel icon may appear

4. **Tap on attachment preview**

**Expected Behavior (SUCCESS):**
- Attachment downloads (if not already)
- File opens in Unicel Viewer
- Spreadsheet displays correctly

**Alternative methods:**
1. Tap attachment preview > Tap share icon (square with arrow)
2. Scroll to "Copy to Unicel Viewer" or "Open in Unicel Viewer"
3. OR: Save to Files first, then open from Files app

---

### Method 4: Open from Safari Download

**Tests file associations via web download**

#### 2.8 Upload File to Cloud Storage

**Using Dropbox:**
1. Go to https://www.dropbox.com
2. Upload `construction-project.usheet`
3. Click "Share" > "Create link"
4. Copy link

**Using Google Drive:**
1. Go to https://drive.google.com
2. Upload `construction-project.usheet`
3. Right-click > "Get link" > "Anyone with link"
4. Copy link

**Using GitHub Gist:**
1. Go to https://gist.github.com
2. Create new gist
3. Filename: `test.usheet`
4. Paste .usheet file content
5. Create public gist
6. Click "Raw" button
7. Copy URL

#### 2.9 Download and Open on iOS

**On iOS device:**

1. Open **Safari** browser
2. Navigate to shared link (paste from clipboard)
3. File should begin downloading
4. Tap "Downloads" icon (arrow pointing down in search bar)
5. OR: Notification banner appears: "Download complete"

**Tap on downloaded file**

**Expected Behavior (SUCCESS):**
- File opens in Unicel Viewer
- Spreadsheet displays correctly
- File may be saved to Downloads folder

**Alternative:**
1. After download, open **Files** app
2. Navigate to "Downloads" folder
3. Find `construction-project.usheet`
4. Tap to open

---

### Method 5: AirDrop (Bonus Test)

**Tests file associations via AirDrop**

#### 2.10 AirDrop File from Mac

**On Mac:**

1. Open Finder
2. Right-click on `construction-project.usheet`
3. Select "Share" > "AirDrop"
4. Select your iPhone from list
5. Click "Done"

**On iOS device:**

1. AirDrop notification appears
2. Tap "Accept"
3. File downloads

**Tap notification or find file in:**
- Downloads folder (Files app)
- OR: File may open automatically

**Expected Behavior (SUCCESS):**
- File opens in Unicel Viewer
- Spreadsheet displays correctly

---

## Part 3: Advanced File Association Tests

### 3.1 Set Unicel as Default App (iOS 14+)

**On iOS device:**

1. Open **Files** app
2. Navigate to .usheet file
3. Tap and hold on file
4. Select "Get Info" or "Info"
5. Look for "Open with" section
6. Should default to "Unicel Viewer"
7. If shows other app or "None":
   - Tap "Open with"
   - Select "Unicel Viewer"
   - Tap "Change All" (sets as default for all .usheet files)

**If "Unicel Viewer" not in list:**
- File association not working
- See Troubleshooting section

### 3.2 Share Sheet Integration

**Test sharing from other apps:**

1. Open **Files** app
2. Find .usheet file
3. Tap share button (square with arrow)
4. Scroll through share sheet
5. Look for "Unicel Viewer" in app list

**Expected:**
- Unicel Viewer appears in share sheet
- Can share file to Unicel directly
- Tapping opens file in app

### 3.3 Quick Look Preview

**Test file preview (iOS 13+):**

1. Open **Files** app
2. Find .usheet file
3. Tap and hold (don't release)
4. Quick Look preview should appear

**Expected:**
- Shows file name and size
- May show content preview (if Quick Look plugin exists)
- Swipe up to see full file
- Tap "Open in Unicel Viewer" button

**Note:** Quick Look preview is optional - most important is direct opening

### 3.4 Spotlight Search

**Test file search:**

1. Swipe down on home screen (iOS search)
2. Type: "construction" or ".usheet"
3. Files should appear in results
4. Tap file

**Expected:**
- File opens in Unicel Viewer
- No app picker shown

---

## Part 4: Verify UTI Registration

### 4.1 Check Installed App UTI

**On Mac (if you have device connected):**

```bash
# Connect iOS device via USB
# Trust computer if prompted

# List installed apps and their UTIs
ios-deploy --list-bundle-id

# Or use Xcode
# Window > Devices and Simulators
# Select device > Installed Apps
# Find "Unicel Viewer"
# Click gear icon > Show Container
```

### 4.2 Check File Type Registration

**On iOS device:**

**Method A: Via Settings**
1. Settings > General > iPhone Storage
2. Scroll to "Unicel Viewer"
3. Tap on it
4. Look for "Documents & Data" section
5. Should show .usheet file support

**Method B: Via Files App**
1. Files app > Browse > On My iPhone
2. Look for "Unicel Viewer" folder
3. Should exist if app claims document support
4. Can store files here offline

---

## Part 5: Test Edge Cases

### 5.1 Multiple File Extensions

Test if app only handles `.usheet` and not other extensions:

1. Rename file to `test.json`
2. Try to open
3. Should NOT open in Unicel (not a .usheet file)
4. Rename back to `test.usheet`
5. Should open correctly

### 5.2 Capitalization Variations

Test case sensitivity:

1. Create files with different cases:
   - `test.USHEET` (uppercase)
   - `test.USheet` (mixed case)
2. Try to open each

**Expected:**
- All variations should work (UTI is case-insensitive)
- If only lowercase works, may need to add UTI tag variations

### 5.3 Corrupted File

Test app handles invalid files gracefully:

1. Create text file: `test.usheet` with content "not valid JSON"
2. Try to open in Unicel

**Expected:**
- App attempts to open
- Shows error message: "Invalid file format" or similar
- Doesn't crash
- Returns to file picker or previous screen

### 5.4 Large File

Test performance with larger files:

1. Create or use large .usheet file (>1 MB)
2. Open from Files app

**Expected:**
- Loading indicator appears
- File loads within reasonable time (<5 seconds)
- UI remains responsive
- No crashes or freezes

---

## Troubleshooting

### File Opens in Wrong App

**Symptom:** .usheet file opens in Notes, Files viewer, or other app

**Causes:**
1. Multiple apps claim .usheet file type
2. User previously selected different app as default
3. File association priority incorrect

**Solutions:**

**Reset default app:**
1. Files app > Find .usheet file
2. Tap and hold > Get Info
3. "Open with" > Select "Unicel Viewer"
4. Tap "Change All"

**Verify no other apps claim .usheet:**
1. Search App Store for other apps using .usheet
2. Uninstall if testing
3. Reinstall Unicel from TestFlight

### File Doesn't Open at All

**Symptom:** Tapping file does nothing, or shows "No app available"

**Causes:**
1. File type not registered (most common)
2. Info.plist missing CFBundleDocumentTypes
3. Not using production/TestFlight build
4. iOS didn't refresh file type database

**Solutions:**

**Verify TestFlight installation:**
```bash
# On iOS device, check:
- Open TestFlight app
- "Unicel Viewer" shows "INSTALLED"
- App icon has orange "BETA" badge
```

**Force file type re-registration:**
1. Delete Unicel Viewer app
2. Restart iPhone (hold power button, slide to power off)
3. Reinstall from TestFlight
4. Try opening .usheet file again

**Check Info.plist configuration:**
```bash
# On Mac, verify configuration
cd /Users/dennisjackson/Code/unicel
cat src-tauri/gen/apple/build/Payload/Unicel.app/Info.plist | grep -A 30 CFBundleDocumentTypes
```

Should show:
```xml
<key>CFBundleDocumentTypes</key>
<array>
    <dict>
        <key>CFBundleTypeName</key>
        <string>Unicel Spreadsheet</string>
        <key>CFBundleTypeRole</key>
        <string>Viewer</string>
        <key>LSHandlerRank</key>
        <string>Owner</string>
        <key>LSItemContentTypes</key>
        <array>
            <string>com.unicel.usheet</string>
        </array>
    </dict>
</array>
```

If missing, regenerate Xcode project and rebuild.

### Shows "Open With..." Dialog

**Symptom:** iOS shows app picker even though Unicel is installed

**Causes:**
1. LSHandlerRank not set to "Owner"
2. Another app has higher priority
3. File type registration incomplete

**Solutions:**

**Check LSHandlerRank:**
```bash
# Should be "Owner" not "Viewer" or "Editor"
cat src-tauri/gen/apple/unicel_iOS/Info.plist | grep LSHandlerRank
```

**Update project.yml if needed:**
```bash
cd /Users/dennisjackson/Code/unicel/src-tauri/gen/apple
# Edit project.yml, ensure LSHandlerRank: Owner
xcodegen generate
```

**Rebuild and re-upload to TestFlight**

### File Icon Doesn't Show Unicel Logo

**Symptom:** File shows generic document icon instead of Unicel icon

**Note:** This is cosmetic - file associations can work without custom icon

**Causes:**
1. Document icon not provided (expected for initial release)
2. Icon not in correct asset catalog
3. Icon not associated with UTI

**Solutions (OPTIONAL - not critical):**

**Add document icon:**
1. Create icon: 64x64 px PNG with Unicel branding
2. Add to Xcode: Assets.xcassets > New Icon Set > "Document Icon"
3. Update Info.plist:
```xml
<key>CFBundleDocumentTypes</key>
<array>
    <dict>
        <key>CFBundleTypeIconFile</key>
        <string>DocumentIcon</string>
        ...
    </dict>
</array>
```

**This is LOW PRIORITY - focus on functionality first**

### File Opens but Shows Error

**Symptom:** Unicel opens but shows "Cannot load file" or crashes

**Causes:**
1. File parsing error in app
2. Incompatible .usheet format
3. Missing required fields

**Solutions:**

**Verify file is valid JSON:**
```bash
cat test.usheet | python3 -m json.tool
# Should output formatted JSON without errors
```

**Check app logs:**
1. Connect device to Mac
2. Xcode > Window > Devices and Simulators
3. Select device > Open Console
4. Filter: "Unicel"
5. Try to open file
6. Look for error messages

**Test with known-good file:**
```bash
# Use example file that definitely works
cp examples/construction-project.usheet /tmp/test-known-good.usheet
# Send to device and test
```

**Report bug:**
- TestFlight > Send Beta Feedback
- Include: file that fails, device model, iOS version

---

## Success Checklist

After completing all tests, verify:

### Basic Functionality
- [ ] .usheet files have Unicel icon in Files app
- [ ] Tapping .usheet file opens Unicel Viewer (no prompts)
- [ ] Files open from iCloud Drive
- [ ] Files open from Messages attachments
- [ ] Files open from Mail attachments
- [ ] Files open from Safari downloads

### Advanced Features
- [ ] Unicel set as default app for .usheet files
- [ ] Files show in Spotlight search
- [ ] Share sheet includes Unicel Viewer
- [ ] Quick Look preview works (or shows file info)
- [ ] Can AirDrop .usheet files to device

### Edge Cases
- [ ] Invalid .usheet files show error (don't crash)
- [ ] Large files load without freezing
- [ ] File extensions are case-insensitive (.USHEET works)
- [ ] Other file types (.json, .txt) don't open in Unicel

### Production Build Verification
- [ ] App installed via TestFlight (orange BETA badge)
- [ ] NOT installed via Xcode debug build
- [ ] Build version matches TestFlight (Settings > Unicel Viewer)

---

## Why Production Builds Are Required

**Technical Explanation:**

### Debug vs Production Builds

**Debug builds (Xcode simulator/device debugging):**
- Code signed with Development certificate
- Entitlements are more permissive (for debugging)
- Launch Services doesn't register app as file handler
- File type UTIs not published to iOS system database
- App ID may be different (e.g., `com.unicel.app.debug`)

**Production builds (TestFlight/App Store):**
- Code signed with Distribution certificate
- Entitlements match production requirements
- Launch Services registers app during installation
- UTIs published to `lsd` (Launch Services Database)
- App ID must match bundle identifier exactly

### File Type Registration Process

**During app installation (production only):**

1. iOS reads `Info.plist` from app bundle
2. Finds `UTExportedTypeDeclarations` and `CFBundleDocumentTypes`
3. Registers UTI `com.unicel.usheet` with Launch Services
4. Associates .usheet extension with Unicel app
5. Sets Unicel as "Owner" (LSHandlerRank)
6. Updates system file handler database

**In debug builds:**
- Steps 3-6 are SKIPPED
- File associations not registered
- Tapping .usheet file shows "No app to open"

### How to Verify Build Type

**On iOS device:**

```
Production build indicators:
✓ Orange "BETA" badge on app icon (TestFlight)
✓ Listed in TestFlight app as "INSTALLED"
✓ Settings > General > iPhone Storage > shows app size/data
✓ .usheet files open in app from Files/Messages/Mail

Debug build indicators:
✗ No "BETA" badge
✗ Xcode console shows when device is connected
✗ Can be launched from Xcode (Product > Run)
✗ .usheet files DON'T open automatically
```

---

## Reference Documentation

### Apple Documentation

**File Type Associations:**
- Declaring file types: https://developer.apple.com/documentation/uikit/view_controllers/adding_a_document_browser_to_your_app
- UTI reference: https://developer.apple.com/library/archive/documentation/FileManagement/Conceptual/understanding_utis/
- Info.plist keys: https://developer.apple.com/documentation/bundleresources/information_property_list

**Launch Services:**
- Launch Services Programming Guide: https://developer.apple.com/documentation/coreservices/launch_services

**TestFlight:**
- TestFlight guide: https://developer.apple.com/testflight/
- Beta testing: https://help.apple.com/app-store-connect/#/dev3b56ce97c

### Tauri iOS

**iOS configuration:**
- Tauri iOS: https://tauri.app/v2/guides/building/ios
- Info.plist setup: https://tauri.app/v2/reference/config/#bundle-ios

### Useful Commands

```bash
# Verify file type on Mac
mdls -name kMDItemContentType file.usheet

# Check UTI registration on Mac
/System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/LaunchServices.framework/Versions/A/Support/lsregister -dump | grep -i usheet

# List iOS simulators
xcrun simctl list devices available

# Install app on simulator
xcrun simctl install booted path/to/app.app

# Check app bundle
codesign -dv --verbose=4 path/to/Unicel.app
```

---

## Next Steps

### If File Associations Work

**Congratulations!** You've successfully configured and tested file associations.

**Next:**
1. Continue beta testing with TestFlight users
2. Gather feedback on file opening experience
3. Test with various file sources (cloud storage, chat apps, etc.)
4. Monitor crash reports for file-opening issues
5. Prepare for App Store submission

### If File Associations Don't Work

**Don't panic!** This is a common issue with specific solutions.

**Debug checklist:**
1. Verify using TestFlight build (not Xcode debug)
2. Check Info.plist has CFBundleDocumentTypes
3. Confirm UTExportedTypeDeclarations present
4. Delete app, restart device, reinstall
5. Test with known-good .usheet file
6. Check Troubleshooting section above

**If still not working:**
1. Review Xcode project configuration
2. Verify project.yml has correct document types
3. Regenerate Xcode project: `xcodegen generate`
4. Build new IPA and upload to TestFlight
5. Test again with fresh installation

---

## File Locations

**Test files:**
- Examples: `/Users/dennisjackson/Code/unicel/examples/*.usheet`
- Minimal test: `/tmp/test-minimal.usheet`

**Configuration:**
- Info.plist: `/Users/dennisjackson/Code/unicel/src-tauri/gen/apple/unicel_iOS/Info.plist`
- project.yml: `/Users/dennisjackson/Code/unicel/src-tauri/gen/apple/project.yml`
- Built IPA: `/Users/dennisjackson/Code/unicel/src-tauri/gen/apple/build/Payload/Unicel.ipa`

**On iOS device:**
- App location: /var/containers/Bundle/Application/[UUID]/Unicel.app
- Documents: /var/mobile/Containers/Data/Application/[UUID]/Documents/
- iCloud: Synced with iCloud Drive

---

**Last updated:** 2025-10-18
**Agent:** ios-deployment-manager
