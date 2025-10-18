# Manual Task: iCloud Container Setup

**Agent:** ios-deployment-manager
**Time Required:** 20-30 minutes
**Prerequisite:** Apple Developer Account (paid $99/year account required)
**Status:** REQUIRED for file sharing and document browser

---

## Overview

This guide walks you through setting up an iCloud container for Unicel iOS. This allows users to:
- Store .usheet files in iCloud Drive
- Access files across multiple devices
- Open files from the Files app
- Share files via iCloud

**Why Required:**
- iOS document browser requires iCloud entitlement
- File persistence across app reinstalls
- Seamless file sharing between iOS and macOS versions

---

## Prerequisites

Before starting, ensure you have:
- [ ] Active Apple Developer Program membership ($99/year)
- [ ] Access to developer.apple.com with login credentials
- [ ] Development team ID (found in Account settings)

**To find your Team ID:**
1. Log in to https://developer.apple.com/account
2. Click "Membership" in sidebar
3. Note your "Team ID" (10-character alphanumeric, e.g., `Z3L3V842L2`)

---

## Step 1: Create iCloud Container

**Estimated time:** 5 minutes

### 1.1 Navigate to Certificates, Identifiers & Profiles

1. Open browser to https://developer.apple.com/account
2. Click "Certificates, Identifiers & Profiles" in sidebar
3. Click "Identifiers" in left menu
4. Click the "+" button (top right) to add new identifier

### 1.2 Select iCloud Container

1. Select "iCloud Containers" from the list
2. Click "Continue"

### 1.3 Configure Container

1. **Description:** Enter `Unicel App iCloud Container`
2. **Identifier:** Enter `iCloud.com.unicel.app`
   - Must start with `iCloud.`
   - Must match bundle ID pattern (without `iCloud.` prefix)
   - Case-sensitive - use exactly: `iCloud.com.unicel.app`
3. Click "Continue"
4. Review details
5. Click "Register"

**Screenshot location after creation:**
- You should see "iCloud.com.unicel.app" in the Identifiers list
- Status: "Active"

---

## Step 2: Associate Container with App ID

**Estimated time:** 5 minutes

### 2.1 Find App Identifier

1. Still in "Identifiers" section
2. Click the filter dropdown (shows "iCloud Containers")
3. Select "App IDs"
4. Look for `com.unicel.app` (may need to create if doesn't exist)

**If `com.unicel.app` doesn't exist:**
1. Click "+" button
2. Select "App IDs"
3. Click "Continue"
4. Select "App" (not App Clip)
5. Click "Continue"
6. Description: `Unicel Unit-Aware Spreadsheet`
7. Bundle ID: Explicit - `com.unicel.app`
8. Scroll to "iCloud" capability
9. Check "iCloud"
10. Click "Continue" > "Register"

### 2.2 Edit App ID Capabilities

1. Click on `com.unicel.app` in the list
2. Scroll to "Capabilities" section
3. Find "iCloud" row
4. Click "Edit" or "Configure" button

### 2.3 Enable iCloud Services

1. Check "iCloud"
2. Under "iCloud Services," select:
   - [ ] "Include CloudKit support" - OPTIONAL (not needed for basic file storage)
   - [x] "Use custom containers" - REQUIRED
3. Click "Assign..." under "Containers"
4. Check the box next to `iCloud.com.unicel.app`
5. Click "Continue"
6. Click "Save"
7. Confirm changes by clicking "Confirm"

**What you should see:**
- App ID: `com.unicel.app`
- Capabilities > iCloud: "Enabled"
- Containers: `iCloud.com.unicel.app`

---

## Step 3: Update Provisioning Profiles

**Estimated time:** 5 minutes

### 3.1 Development Profile

1. In left sidebar, click "Profiles"
2. Look for existing development profile for `com.unicel.app`
3. If exists: Click on it, then click "Edit"
4. If doesn't exist: Click "+" to create new profile

**Creating new development profile:**
1. Select "iOS App Development"
2. Click "Continue"
3. Select App ID: `com.unicel.app`
4. Click "Continue"
5. Select certificates (your development certificate)
6. Click "Continue"
7. Select devices (if testing on physical devices)
8. Click "Continue"
9. Profile Name: `Unicel Development`
10. Click "Generate"

**For existing profile:**
1. Click "Edit"
2. Ensure it includes iCloud capability
3. Click "Generate" to refresh

### 3.2 Download Profile

1. Click "Download" button
2. Save to Downloads folder
3. Double-click downloaded file to install in Xcode
4. Or drag-and-drop into Xcode window

**Verify in Xcode:**
```bash
# List installed profiles
ls ~/Library/MobileDevice/Provisioning\ Profiles/
```

### 3.3 Distribution Profile (for App Store)

**OPTIONAL - Only needed when ready to submit to App Store**

1. Click "+" to create new profile
2. Select "App Store" (under Distribution)
3. Click "Continue"
4. Select App ID: `com.unicel.app`
5. Click "Continue"
6. Select distribution certificate
7. Click "Continue"
8. Profile Name: `Unicel App Store Distribution`
9. Click "Generate"
10. Download and install

---

## Step 4: Update Xcode Project

**Estimated time:** 5 minutes

### 4.1 Open Project in Xcode

```bash
cd /Users/dennisjackson/Code/unicel
open src-tauri/gen/apple/unicel.xcodeproj
```

### 4.2 Configure Signing & Capabilities

1. In Xcode, select project "unicel" in left navigator
2. Select target "unicel_iOS"
3. Click "Signing & Capabilities" tab

**Automatic Signing (RECOMMENDED):**
1. Check "Automatically manage signing"
2. Select Team: Your development team (e.g., "Dennis Jackson (Z3L3V842L2)")
3. Provisioning Profile: Should auto-select development profile
4. Status should show: "Profile 'Unicel Development' is valid"

**Manual Signing (alternative):**
1. Uncheck "Automatically manage signing"
2. Provisioning Profile (Debug): Select "Unicel Development"
3. Provisioning Profile (Release): Select "Unicel App Store Distribution"
4. Signing Certificate: Select your certificate

### 4.3 Add iCloud Capability

1. Still in "Signing & Capabilities" tab
2. Click "+ Capability" button (top left)
3. Scroll to find "iCloud"
4. Double-click "iCloud" to add

**Configure iCloud settings:**
1. Under "Services," check:
   - [x] iCloud Documents
   - [ ] CloudKit (optional)
2. Under "Containers," click "+ Container"
3. Select `iCloud.com.unicel.app` from list
4. Or enter custom container: `iCloud.com.unicel.app`

**What you should see:**
```
iCloud
  Services:
    ✓ iCloud Documents
  Containers:
    ✓ iCloud.com.unicel.app
```

### 4.4 Verify Entitlements File

1. In Xcode navigator, open `unicel_iOS/unicel_iOS.entitlements`
2. Should contain:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>com.apple.developer.icloud-container-identifiers</key>
    <array>
        <string>iCloud.com.unicel.app</string>
    </array>
    <key>com.apple.developer.ubiquity-container-identifiers</key>
    <array>
        <string>iCloud.com.unicel.app</string>
    </array>
    <key>com.apple.developer.icloud-services</key>
    <array>
        <string>CloudDocuments</string>
    </array>
</dict>
</plist>
```

**If file is empty or missing keys:**
1. Close Xcode
2. Delete `unicel_iOS.entitlements`
3. Re-open Xcode
4. Add iCloud capability again (Step 4.3)

---

## Step 5: Update Tauri Configuration

**Estimated time:** 3 minutes

### 5.1 Add Team ID to tauri.conf.json

Open `/Users/dennisjackson/Code/unicel/src-tauri/tauri.conf.json`

Find the `iOS` section and add your `developmentTeam`:

```json
{
  "bundle": {
    "iOS": {
      "minimumSystemVersion": "13.0",
      "developmentTeam": "Z3L3V842L2"
    }
  }
}
```

**Replace `Z3L3V842L2` with YOUR team ID from Step Prerequisites.**

### 5.2 Verify Configuration

```bash
cd /Users/dennisjackson/Code/unicel
npm run tauri info
```

Look for:
```
[✔] iOS Environment
  - Development Team: Z3L3V842L2
  - Bundle Identifier: com.unicel.app
```

---

## Step 6: Test iCloud Integration

**Estimated time:** 10 minutes

### 6.1 Build for Simulator

```bash
cd /Users/dennisjackson/Code/unicel
npm run tauri ios dev
```

**Expected output:**
```
Building iOS app...
Code signing: Development
Provisioning profile: Unicel Development
Installing to simulator...
```

### 6.2 Enable iCloud in Simulator

1. Open Simulator (should auto-launch)
2. Settings app > [Your Apple ID] at top
3. Sign in with Apple ID (can use test account)
4. Enable iCloud Drive

**Create test iCloud account if needed:**
1. Go to https://appleid.apple.com/account
2. Create new Apple ID for testing
3. Use in simulator only (not your personal Apple ID)

### 6.3 Test File Storage

**In Unicel app:**
1. Open Files app in simulator
2. Navigate to "iCloud Drive"
3. Create folder "Unicel" (if not auto-created)
4. Place test .usheet file in folder
5. Open Unicel app
6. Tap file picker
7. Navigate to iCloud Drive > Unicel
8. Select .usheet file
9. Verify file loads correctly

**Command to copy test file to simulator:**
```bash
# Find simulator UUID
xcrun simctl list devices | grep Booted

# Copy file (replace SIMULATOR_UUID)
xcrun simctl addmedia SIMULATOR_UUID /Users/dennisjackson/Code/unicel/examples/construction-project.usheet
```

### 6.4 Verify iCloud Sync

1. Open Files app on simulator
2. Check that Unicel folder appears under "iCloud Drive"
3. Files should show cloud sync icon (download/upload arrows)

**If iCloud not working:**
- Check you're signed into iCloud in Settings
- Check iCloud Drive is enabled
- Check Unicel has permission to access iCloud Drive
- Check entitlements file has correct container ID

---

## Step 7: Physical Device Testing (Optional)

**Estimated time:** 10 minutes
**Prerequisite:** iPhone or iPad with iOS 13.0+

### 7.1 Register Device

1. Connect device to Mac via USB
2. Open Xcode > Window > Devices and Simulators
3. Select your device
4. Note "Identifier" (UUID)
5. Go to https://developer.apple.com/account
6. Certificates, Identifiers & Profiles > Devices
7. Click "+" to add new device
8. Device Name: `My iPhone` (or descriptive name)
9. Device ID (UUID): Paste identifier from Xcode
10. Click "Continue" > "Register"

### 7.2 Update Provisioning Profile

1. Go to Profiles section
2. Edit "Unicel Development" profile
3. Add newly registered device
4. Click "Generate"
5. Download and install updated profile

### 7.3 Build and Install

```bash
# Build for device
npm run tauri ios dev --device "My iPhone"

# Or select in Xcode
# Product > Destination > My iPhone
# Product > Run (Cmd+R)
```

**First time setup:**
1. On device: Settings > General > VPN & Device Management
2. Trust developer certificate
3. Tap on your Apple ID
4. Tap "Trust"

### 7.4 Test on Device

1. App should launch on device
2. Open Files app
3. Navigate to iCloud Drive
4. Place .usheet file in iCloud Drive (from macOS or another device)
5. Open Unicel on device
6. Verify file is accessible

---

## Troubleshooting

### Error: "No profiles for 'com.unicel.app' were found"

**Solution:**
1. Xcode > Preferences > Accounts
2. Select your Apple ID
3. Click "Download Manual Profiles" button
4. Or: Create new development profile in Xcode
   - Xcode will auto-generate if "Automatically manage signing" is enabled

### Error: "iCloud container not found"

**Solution:**
1. Verify container ID exactly matches: `iCloud.com.unicel.app`
2. Check App ID has iCloud capability enabled
3. Download fresh provisioning profile
4. Clean build: `rm -rf src-tauri/gen/apple/build`
5. Rebuild

### Error: "Signing requires a development team"

**Solution:**
1. Open Xcode project
2. Select target > Signing & Capabilities
3. Select Team dropdown
4. Choose your team (personal or organization)

**Alternative: Add to tauri.conf.json:**
```json
{
  "bundle": {
    "iOS": {
      "developmentTeam": "YOUR_TEAM_ID_HERE"
    }
  }
}
```

### Error: "The app delegate must implement..."

**Solution:** This is expected - Tauri handles app lifecycle. Ignore warning.

### iCloud files not syncing in simulator

**Solution:**
1. Simulator > Reset Content and Settings
2. Re-sign into iCloud
3. Wait 1-2 minutes for initial sync
4. Check Settings > [Apple ID] > iCloud > iCloud Drive is ON

### Error: "Code signing is required for product type 'Application'"

**Solution:**
1. Install full Xcode (not just Command Line Tools)
2. Open Xcode at least once to install components
3. Accept license agreement
4. Xcode > Preferences > Locations > Command Line Tools = Xcode

### Provisioning profile expired

**Solution:**
1. Go to developer.apple.com > Profiles
2. Select expired profile
3. Click "Edit"
4. Click "Generate" to renew
5. Download and reinstall

---

## Success Checklist

After completing this guide, verify:

- [ ] iCloud container `iCloud.com.unicel.app` created in Developer Portal
- [ ] App ID `com.unicel.app` has iCloud capability enabled
- [ ] App ID associated with iCloud container
- [ ] Development provisioning profile includes iCloud entitlement
- [ ] Xcode project shows iCloud capability in Signing & Capabilities
- [ ] Entitlements file contains correct container ID
- [ ] `tauri.conf.json` contains developmentTeam ID
- [ ] App builds successfully in simulator
- [ ] iCloud Drive is accessible from Files app
- [ ] Can place .usheet files in iCloud Drive
- [ ] Unicel app can open files from iCloud Drive

---

## Next Steps

Once iCloud is working:
1. Proceed to TestFlight deployment (see `HUMAN_TASK_TESTFLIGHT.md`)
2. Test file associations (see `HUMAN_TASK_FILE_ASSOCIATIONS.md`)
3. Prepare App Store metadata and screenshots

---

## Reference Links

**Apple Developer Portal:**
- Certificates, Identifiers & Profiles: https://developer.apple.com/account/resources
- iCloud Container setup: https://developer.apple.com/icloud/

**Documentation:**
- CloudKit and iCloud: https://developer.apple.com/icloud/
- Document-based apps: https://developer.apple.com/documentation/uikit/view_controllers/building_a_document_browser-based_app
- Entitlements reference: https://developer.apple.com/documentation/bundleresources/entitlements

**Tauri iOS:**
- Tauri iOS guide: https://tauri.app/v2/guides/building/ios
- iOS configuration: https://tauri.app/v2/reference/config/#bundle-ios

---

## File Locations

**Configuration:**
- Xcode project: `/Users/dennisjackson/Code/unicel/src-tauri/gen/apple/unicel.xcodeproj`
- Entitlements: `/Users/dennisjackson/Code/unicel/src-tauri/gen/apple/unicel_iOS/unicel_iOS.entitlements`
- Tauri config: `/Users/dennisjackson/Code/unicel/src-tauri/tauri.conf.json`

**Provisioning profiles:**
- System location: `~/Library/MobileDevice/Provisioning Profiles/`
- Download from: https://developer.apple.com/account/resources/profiles/list

---

**Last updated:** 2025-10-18
**Agent:** ios-deployment-manager
