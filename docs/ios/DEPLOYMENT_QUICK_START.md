# iOS Deployment Quick Start Guide

**Last updated:** 2025-10-18
**Agent:** ios-deployment-manager

---

## Overview

This quick start guide provides a high-level roadmap for deploying Unicel iOS to the App Store. Follow the three main manual tasks in order.

**Total Time Required:** ~1-2 hours (excluding Apple review time)

**Prerequisites:**
- Apple Developer Program membership ($99/year) - REQUIRED
- Xcode installed and configured
- Mac with macOS 13.0+ and Xcode 15.0+
- Test .usheet files available

---

## Deployment Roadmap

### Phase 1: iCloud Container Setup
**Time:** 20-30 minutes
**Document:** `HUMAN_TASK_ICLOUD_SETUP.md`
**Required for:** File storage and document browser

**Steps:**
1. Log in to https://developer.apple.com/account
2. Create iCloud Container: `iCloud.com.unicel.app`
3. Associate with App ID: `com.unicel.app`
4. Download and install provisioning profiles
5. Configure Xcode project with iCloud capability
6. Update `tauri.conf.json` with development team ID
7. Test iCloud Drive integration in simulator

**Success Criteria:**
- [ ] iCloud container created in Developer Portal
- [ ] App ID has iCloud capability enabled
- [ ] Xcode project shows iCloud in Signing & Capabilities
- [ ] Can access iCloud Drive from Files app in simulator

**Troubleshooting:** See Section 7 in `HUMAN_TASK_ICLOUD_SETUP.md`

---

### Phase 2: TestFlight Beta Distribution
**Time:** 30-60 minutes (including processing time)
**Document:** `HUMAN_TASK_TESTFLIGHT.md`
**Required for:** Testing file associations and real-world usage

**Steps:**
1. Create app in App Store Connect
2. Create version 0.5.1 (or current version)
3. Build release IPA: `npm run tauri ios build`
4. Upload to App Store Connect (via Xcode or Transporter)
5. Wait for processing (5-15 minutes)
6. Add "What to Test" notes
7. Invite internal testers (up to 100)
8. Install TestFlight on iOS device
9. Install Unicel Viewer from TestFlight
10. Verify app launches and works correctly

**Success Criteria:**
- [ ] Build uploaded and shows "Ready to Test" status
- [ ] Internal testers invited
- [ ] App installed via TestFlight (has orange "BETA" badge)
- [ ] App launches without crashes
- [ ] Basic functionality works (load .usheet files)

**Troubleshooting:** See Section 9 in `HUMAN_TASK_TESTFLIGHT.md`

---

### Phase 3: File Association Testing
**Time:** 15-20 minutes
**Document:** `HUMAN_TASK_FILE_ASSOCIATIONS.md`
**Required for:** Verifying .usheet files open in Unicel automatically

**CRITICAL:** Must use TestFlight/App Store build (not simulator/debug)

**Steps:**
1. Prepare test .usheet files
2. Upload files to iCloud Drive
3. Test opening from Files app - tap file should open in Unicel
4. Test opening from Messages - send file, tap to open
5. Test opening from Mail - attach file, tap to open
6. Test opening from Safari downloads
7. Test AirDrop (optional)

**Success Criteria:**
- [ ] Tapping .usheet file opens Unicel Viewer (no prompts)
- [ ] Files from iCloud Drive open correctly
- [ ] Files from Messages attachments open correctly
- [ ] Files from Mail attachments open correctly
- [ ] No "How do you want to open this file?" dialog

**Troubleshooting:** See Section 5 in `HUMAN_TASK_FILE_ASSOCIATIONS.md`

---

## Pre-Flight Checklist

Before starting deployment, verify:

### Apple Developer Account
- [ ] Active membership ($99/year paid)
- [ ] Can log in to https://developer.apple.com/account
- [ ] Can log in to https://appstoreconnect.apple.com
- [ ] Development team ID known (e.g., `Z3L3V842L2`)

### Local Development Environment
- [ ] Xcode installed (not just Command Line Tools)
- [ ] Xcode version 15.0+ (`xcodebuild -version`)
- [ ] iOS Simulator available (`xcrun simctl list devices`)
- [ ] Rust iOS targets installed (`rustup target list --installed`)
- [ ] Project builds successfully (`npm run tauri ios dev`)

### Code Signing
- [ ] Development certificate installed
- [ ] Distribution certificate installed (for App Store)
- [ ] Provisioning profiles downloaded
- [ ] Xcode project has "Automatically manage signing" enabled

### Test Files
- [ ] Example .usheet files available (`examples/*.usheet`)
- [ ] Files have correct extension (.usheet not .usheet.json)
- [ ] Files are valid JSON (`cat file.usheet | python3 -m json.tool`)

---

## Common Pitfalls to Avoid

### 1. Using Debug Builds for File Association Testing
**Problem:** File associations ONLY work in production builds (TestFlight/App Store)
**Solution:** Always test file associations using TestFlight build, never simulator

### 2. Incorrect iCloud Container Naming
**Problem:** Container must be exactly `iCloud.com.unicel.app` (case-sensitive)
**Solution:** Double-check capitalization and prefix before creating

### 3. Expired Provisioning Profiles
**Problem:** Build fails with "No profiles found" error
**Solution:** Download fresh profiles from developer.apple.com/account

### 4. Missing Development Team ID
**Problem:** Build fails with "Signing requires a development team"
**Solution:** Add to `tauri.conf.json`:
```json
{
  "bundle": {
    "iOS": {
      "developmentTeam": "YOUR_TEAM_ID"
    }
  }
}
```

### 5. Wrong File Extension
**Problem:** Files named `.usheet.json` instead of `.usheet`
**Solution:** Rename files to have exactly `.usheet` extension

### 6. Not Waiting for TestFlight Processing
**Problem:** Trying to install build before processing completes
**Solution:** Wait for email: "Your build is ready for testing" (5-15 minutes)

### 7. Testing on Simulator Instead of Device
**Problem:** File associations not working in simulator
**Solution:** Use TestFlight on physical device or production-signed simulator build

---

## Time Estimates by Phase

| Phase | Estimated Time | Wait Time | Total |
|-------|----------------|-----------|-------|
| iCloud Setup | 20-30 min | None | 20-30 min |
| TestFlight Upload | 10-20 min | 5-15 min (processing) | 15-35 min |
| File Association Testing | 15-20 min | None | 15-20 min |
| **Total** | **45-70 min** | **5-15 min** | **50-85 min** |

**Not included:** App Store review (24-72 hours after submission)

---

## Success Indicators

### After Phase 1 (iCloud)
- Xcode shows iCloud capability enabled
- Can see iCloud Drive in Files app (simulator)
- No "iCloud container not found" errors

### After Phase 2 (TestFlight)
- App appears in TestFlight app on device
- App icon has orange "BETA" badge
- App launches and shows main screen
- No immediate crashes

### After Phase 3 (File Associations)
- .usheet files show Unicel icon in Files app
- Tapping file opens app immediately
- No app picker dialog appears
- Files from Messages/Mail open correctly

---

## What Comes Next

After completing all three manual tasks:

### Immediate Next Steps
1. Gather feedback from beta testers
2. Monitor crash reports in App Store Connect
3. Fix critical bugs and upload new builds
4. Test on multiple devices and iOS versions

### App Store Submission Preparation
1. Generate app icons (all required sizes)
   - 1024x1024 for App Store
   - Various sizes for devices (see agent knowledge base)

2. Create screenshots (all device sizes)
   - iPhone 15 Pro Max (6.7")
   - iPhone 14 Plus (6.5")
   - iPhone 15 (6.1")
   - iPad Pro 12.9"
   - iPad Air 11"

3. Write App Store metadata
   - App name: "Unicel Viewer"
   - Subtitle: "Unit-Aware Spreadsheets"
   - Description (see agent knowledge base)
   - Keywords: "spreadsheet,units,engineering,calculations"
   - Privacy policy URL

4. Submit for App Store review
   - Answer export compliance questions
   - Submit for review via App Store Connect
   - Wait 24-72 hours for review

---

## Emergency Contacts & Support

### If iCloud Setup Fails
- Check: `HUMAN_TASK_ICLOUD_SETUP.md` → Troubleshooting (Section 7)
- Apple Developer Support: https://developer.apple.com/contact/

### If TestFlight Upload Fails
- Check: `HUMAN_TASK_TESTFLIGHT.md` → Troubleshooting (Section 9)
- App Store Connect Status: https://developer.apple.com/system-status/

### If File Associations Don't Work
- Check: `HUMAN_TASK_FILE_ASSOCIATIONS.md` → Troubleshooting (Section 5)
- Verify: Using TestFlight build (not debug build)
- Verify: Info.plist has CFBundleDocumentTypes

### General iOS Build Issues
- Check: `docs/ios/CODE_SIGNING_GUIDE.md`
- Check: `docs/ios/MANUAL_TESTING_CHECKLIST.md`

---

## Quick Command Reference

```bash
# Check Xcode installation
xcodebuild -version
xcode-select -p

# List simulators
xcrun simctl list devices available

# Build release IPA
cd /Users/dennisjackson/Code/unicel
npm run tauri ios build

# Verify code signing
codesign -dv --verbose=4 path/to/Unicel.app

# Check provisioning profiles
security find-identity -v -p codesigning
```

---

## Resource Links

### Official Documentation
- **Apple Developer Portal:** https://developer.apple.com/account
- **App Store Connect:** https://appstoreconnect.apple.com
- **TestFlight Guide:** https://developer.apple.com/testflight/
- **Tauri iOS Guide:** https://tauri.app/v2/guides/building/ios

### Project Documentation
- **Full Guides:**
  - `docs/ios/HUMAN_TASK_ICLOUD_SETUP.md`
  - `docs/ios/HUMAN_TASK_TESTFLIGHT.md`
  - `docs/ios/HUMAN_TASK_FILE_ASSOCIATIONS.md`

- **Supporting Docs:**
  - `docs/ios/CODE_SIGNING_GUIDE.md`
  - `docs/ios/MANUAL_TESTING_CHECKLIST.md`
  - `docs/ios/README.md`

---

## Version History

| Date | Version | Changes |
|------|---------|---------|
| 2025-10-18 | 1.0 | Initial deployment guide creation |

---

## Notes

- All three manual tasks require paid Apple Developer Program membership
- File associations ONLY work in production builds (TestFlight/App Store)
- iCloud setup must be completed before TestFlight upload
- TestFlight build must be installed before file association testing
- Allow extra time for Apple processing and review

**Recommended approach:** Complete all three phases in one session (1-2 hours) to avoid context switching and ensure consistent configuration.

---

**Questions?** Refer to the detailed guides linked above or check the Troubleshooting sections in each guide.
