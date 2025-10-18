# iOS Code Signing Configuration Guide

**For:** Unicel iOS Viewer MVP
**Date:** October 18, 2025
**Audience:** Developers setting up iOS builds for first time

---

## Overview

iOS code signing is required to run apps on simulators and devices. This guide covers three approaches:

1. **Sign to Run Locally** (Simulator only, no Apple account needed)
2. **Automatic Signing** (Free Apple Developer account, simulator + device)
3. **Manual Signing** (Advanced, full control over provisioning)

For MVP testing, **Option 1 (Sign to Run Locally)** is sufficient.

---

## Prerequisites

Before starting, ensure:
- ✅ Xcode 26.0.1 installed at `/Applications/Xcode.app`
- ✅ Xcode developer path set correctly (run `sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer`)
- ✅ Xcode license accepted (run `sudo xcodebuild -license accept`)

---

## Option 1: Sign to Run Locally (Recommended for MVP)

**Use this for:** Simulator testing only, no Apple Developer account needed.

### Steps:

1. **Open Xcode Project:**
   ```bash
   open /Users/dennisjackson/Code/unicel/src-tauri/gen/apple/unicel.xcodeproj
   ```

2. **Select Target:**
   - In left sidebar, click on project icon (blue "unicel")
   - Under "TARGETS", select **"unicel_iOS"**

3. **Open Signing Tab:**
   - Click **"Signing & Capabilities"** tab at top

4. **Configure Local Signing:**
   - Uncheck **"Automatically manage signing"** (if checked)
   - In **"Team"** dropdown, select: **"None (Sign to Run Locally)"**
   - Provisioning Profile should show: **"iOS Team Provisioning Profile: *"**

5. **Verify Configuration:**
   - Status should show green checkmark: **"Valid signing identity"**
   - No red errors should appear

6. **Build for Simulator:**
   - Select a simulator from device dropdown (e.g., "iPhone 17 Pro")
   - Click ▶ (Play) button, OR use command line:
     ```bash
     npm run tauri:ios:dev
     ```

### What This Allows:
- ✅ Run on iOS Simulator (all device types)
- ✅ Debug in Xcode
- ✅ Test all app features
- ❌ Cannot run on physical iPhone/iPad
- ❌ Cannot distribute via TestFlight
- ❌ Cannot submit to App Store

---

## Option 2: Automatic Signing (For Device Testing)

**Use this for:** Testing on real iPhone/iPad, requires free Apple Developer account.

### Steps:

1. **Create Apple Developer Account (if needed):**
   - Go to: https://developer.apple.com/
   - Click "Account" → Sign in with your Apple ID
   - Accept Developer Agreement
   - **Cost:** FREE (for development/testing)

2. **Add Account to Xcode:**
   - Xcode → Settings (Cmd+,)
   - Click "Accounts" tab
   - Click "+" → "Apple ID"
   - Sign in with your Apple ID
   - Wait for account to sync

3. **Open Xcode Project:**
   ```bash
   open /Users/dennisjackson/Code/unicel/src-tauri/gen/apple/unicel.xcodeproj
   ```

4. **Select Target:**
   - In left sidebar, click project icon → select "unicel_iOS" target

5. **Enable Automatic Signing:**
   - Go to **"Signing & Capabilities"** tab
   - Check **"Automatically manage signing"**
   - In **"Team"** dropdown, select your Apple ID (e.g., "Your Name (Personal Team)")

6. **Fix Bundle Identifier (if needed):**
   - If you see error "Failed to register bundle identifier", change it:
   - Click on bundle identifier: `com.unicel.app`
   - Change to something unique: `com.yourname.unicel.app`
   - Xcode will automatically create provisioning profile

7. **Connect iPhone/iPad:**
   - Plug in device via USB
   - Unlock device
   - Trust computer when prompted
   - Select device from Xcode's device dropdown

8. **Build and Run:**
   - Click ▶ (Play) button
   - First time: Device will show "Untrusted Developer" warning
   - On device: Settings → General → VPN & Device Management
   - Tap your Apple ID → "Trust"
   - Return to Xcode and click ▶ again

### What This Allows:
- ✅ Run on iOS Simulator
- ✅ Run on your personal iPhone/iPad (up to 3 devices)
- ✅ Debug on real hardware
- ✅ Test camera, GPS, other device-specific features
- ❌ Cannot distribute via TestFlight (requires paid account)
- ❌ Cannot submit to App Store (requires paid account)

---

## Option 3: Manual Signing (Advanced)

**Use this for:** Custom provisioning profiles, enterprise distribution, full control.

**Note:** Not recommended for MVP. Use Option 1 or 2 instead.

### When You Need This:
- Enterprise distribution (in-house apps)
- Custom entitlements not supported by automatic signing
- Multiple team members sharing certificates
- CI/CD pipelines

### Steps:

1. **Create Provisioning Profile:**
   - Go to: https://developer.apple.com/account/resources/profiles/
   - Click "+" to create new profile
   - Select type (Development, App Store, etc.)
   - Select App ID (create if needed)
   - Select certificates
   - Select devices (for development profiles)
   - Download .mobileprovision file

2. **Install Certificate:**
   - Download .p12 certificate file
   - Double-click to install in Keychain
   - Verify in Keychain Access under "My Certificates"

3. **Configure in Xcode:**
   - Open project → "Signing & Capabilities"
   - Uncheck "Automatically manage signing"
   - Import provisioning profile (drag .mobileprovision file to Xcode)
   - Select profile from dropdown

4. **Build:**
   ```bash
   xcodebuild -project unicel.xcodeproj \
              -scheme unicel_iOS \
              -configuration Release \
              -archivePath unicel.xcarchive \
              archive
   ```

---

## Apple Developer Account Comparison

| Feature | No Account | Free Account | Paid Account ($99/yr) |
|---------|------------|--------------|------------------------|
| Simulator Testing | ✅ Yes | ✅ Yes | ✅ Yes |
| Device Testing | ❌ No | ✅ Yes (3 devices) | ✅ Yes (100 devices) |
| TestFlight Distribution | ❌ No | ❌ No | ✅ Yes |
| App Store Release | ❌ No | ❌ No | ✅ Yes |
| Push Notifications | ❌ No | ❌ No | ✅ Yes |
| iCloud Integration | ❌ No | ❌ No | ✅ Yes |
| Advanced Capabilities | ❌ No | ❌ No | ✅ Yes |

**Recommendation for MVP:** Start with "Sign to Run Locally" (no account). Upgrade to free account if you need device testing. Only purchase paid account when ready for TestFlight/App Store.

---

## Troubleshooting

### Error: "No signing certificate found"
**Solution:**
- Option 1: Use "Sign to Run Locally"
- Option 2: Add Apple ID in Xcode → Settings → Accounts
- Option 3: Download certificate from developer.apple.com

### Error: "Failed to register bundle identifier"
**Solution:**
- Change bundle ID to something unique: `com.yourname.unicel.app`
- Update in both Xcode AND `tauri.conf.json`

### Error: "Provisioning profile doesn't match entitlements"
**Solution:**
- Go to Signing & Capabilities tab
- Click "+" to add missing capability, OR
- Remove entitlement from `unicel_iOS.entitlements` file

### Error: "Device not registered"
**Solution:**
- Free account: Limited to 3 devices, remove old device from developer.apple.com
- Paid account: Register device in developer.apple.com → Devices

### Error: "The maximum number of apps...has been reached"
**Solution:**
- Free account: Limited to 10 App IDs per 7 days
- Wait 7 days OR delete unused App IDs OR upgrade to paid account

---

## Verifying Code Signing Configuration

### Check from Command Line:
```bash
# Show signing settings
xcodebuild -project /Users/dennisjackson/Code/unicel/src-tauri/gen/apple/unicel.xcodeproj \
           -target unicel_iOS \
           -showBuildSettings | grep CODE_SIGN

# Expected output:
# CODE_SIGN_IDENTITY = - (for local signing)
# CODE_SIGN_IDENTITY = Apple Development (for automatic signing)
```

### Check Available Signing Identities:
```bash
security find-identity -v -p codesigning

# Expected output (if using local signing):
# 1) [long hash] "Apple Development: Your Name (XXXXXXXXXX)"
# Or: "No identities found" (OK for simulator-only)
```

### Check Provisioning Profiles:
```bash
ls ~/Library/MobileDevice/Provisioning\ Profiles/

# Should show .mobileprovision files if using automatic signing
# Empty directory is OK for "Sign to Run Locally"
```

---

## What's Next

After configuring code signing:

1. **Run iOS build:**
   ```bash
   /Users/dennisjackson/Code/unicel/scripts/test-ios-simulator.sh
   ```

2. **Test in simulator:**
   - Verify app launches
   - Open example workbook
   - Test touch gestures
   - Toggle Metric/Imperial

3. **Document results:**
   - Take screenshots
   - Note any errors
   - Record performance

4. **Proceed to Week 29:**
   - Generate app icons
   - Create screenshots
   - Prepare for App Store

---

## Quick Reference

### Open Xcode Project
```bash
open /Users/dennisjackson/Code/unicel/src-tauri/gen/apple/unicel.xcodeproj
```

### Build for Simulator
```bash
npm run tauri:ios:dev
```

### Build Release IPA (requires paid account)
```bash
npm run tauri:ios:build
```

### List Simulators
```bash
xcrun simctl list devices available
```

### Kill Stuck Simulator
```bash
killall Simulator
```

---

**Generated:** October 18, 2025
**File:** `/Users/dennisjackson/Code/unicel/docs/ios/CODE_SIGNING_GUIDE.md`
