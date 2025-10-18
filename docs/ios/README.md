# Unicel iOS Documentation

This directory contains documentation for the iOS viewer implementation (Phase 10 - Weeks 25-29).

---

## Quick Start

To complete Week 25 iOS platform setup:

1. **Read the main setup guide:**
   - [`WEEK_25_iOS_PLATFORM_SETUP.md`](./WEEK_25_iOS_PLATFORM_SETUP.md) - Complete overview and status

2. **Configure code signing:**
   - [`CODE_SIGNING_GUIDE.md`](./CODE_SIGNING_GUIDE.md) - Step-by-step signing instructions

3. **Run manual tests:**
   - [`MANUAL_TESTING_CHECKLIST.md`](./MANUAL_TESTING_CHECKLIST.md) - Testing checklist
   - Or run: `/Users/dennisjackson/Code/unicel/scripts/test-ios-simulator.sh`

---

## Document Index

### Week 25: Platform Setup
- **WEEK_25_iOS_PLATFORM_SETUP.md** - Main completion report
  - Automated setup status
  - Manual testing requirements
  - Xcode configuration
  - Troubleshooting guide

- **CODE_SIGNING_GUIDE.md** - Code signing configuration
  - Sign to Run Locally (no account)
  - Automatic signing (free account)
  - Manual signing (advanced)
  - Account comparison table

- **MANUAL_TESTING_CHECKLIST.md** - Testing procedures
  - Task 10.4 checklist (code signing)
  - Task 10.5 checklist (simulator testing)
  - Task 10.7 checklist (Tauri commands)
  - Screenshot requirements

### Week 29: App Store Deployment (Manual Tasks)
- **HUMAN_TASK_ICLOUD_SETUP.md** - iCloud container setup
  - Apple Developer Portal configuration
  - iCloud container creation
  - App ID association
  - Xcode project setup
  - Testing iCloud integration
  - Time: 20-30 minutes

- **HUMAN_TASK_TESTFLIGHT.md** - TestFlight beta distribution
  - App Store Connect configuration
  - Building release IPA
  - Uploading to TestFlight
  - Adding beta testers
  - Collecting feedback
  - Time: 30-60 minutes

- **HUMAN_TASK_FILE_ASSOCIATIONS.md** - Testing file associations
  - Why production builds required
  - Testing from Files/Messages/Mail
  - Verification procedures
  - Troubleshooting guide
  - Time: 15-20 minutes

### Previous Weeks
- **iOS_SETUP_COMPLETE.md** - Tasks 10.1-10.3 completion (Oct 17)
- **WEEK_28_IPAD_OPTIMIZATION_COMPLETE.md** - iPad UI optimization (Oct 17)
- **DEVICE_TESTING_MATRIX.md** - Device compatibility matrix
- **PERFORMANCE_TESTING.md** - Performance benchmarks and targets
- **ACCESSIBILITY.md** - VoiceOver and accessibility features
- **KEYBOARD_SHORTCUTS.md** - External keyboard support
- **BUILD_SELF_CONTAINED_APP.md** - Self-contained app bundle setup

---

## Current Status

### Completed
- ✅ iOS project initialization (10.1)
- ✅ Xcode project configuration (10.2)
- ✅ Info.plist file associations (10.3)
- ✅ iOS dependencies installed (10.6)
- ✅ Mobile UI implementation (Week 26)
- ✅ File handling & polish (Week 27)
- ✅ iPad optimization (Week 28)

### Requires Manual Testing
- ⚠️ Code signing configuration (10.4)
- ⚠️ iOS simulator testing (10.5)
- ⚠️ Tauri command verification (10.7)

### Week 29 Tasks (App Store Deployment)
- ⏳ iCloud container setup - See `HUMAN_TASK_ICLOUD_SETUP.md`
- ⏳ TestFlight beta testing - See `HUMAN_TASK_TESTFLIGHT.md`
- ⏳ File association testing - See `HUMAN_TASK_FILE_ASSOCIATIONS.md`
- ⏳ App icons generation (all required sizes)
- ⏳ App Store screenshots (iPhone + iPad)
- ⏳ Privacy policy creation
- ⏳ App Store metadata
- ⏳ Final submission

---

## Tools & Scripts

### Testing Script
Location: `/Users/dennisjackson/Code/unicel/scripts/test-ios-simulator.sh`

Automated script that:
- Verifies Xcode configuration
- Checks Rust iOS targets
- Builds frontend
- Launches iOS simulator
- Runs app build

Usage:
```bash
/Users/dennisjackson/Code/unicel/scripts/test-ios-simulator.sh
```

### Manual Commands
```bash
# Open Xcode project
open /Users/dennisjackson/Code/unicel/src-tauri/gen/apple/unicel.xcodeproj

# Build for simulator
npm run tauri:ios:dev

# Build release IPA
npm run tauri:ios:build

# List simulators
xcrun simctl list devices available
```

---

## Key Files & Locations

### iOS Project Structure
```
/Users/dennisjackson/Code/unicel/src-tauri/gen/apple/
├── unicel.xcodeproj/              # Xcode project
├── unicel_iOS/
│   ├── Info.plist                # iOS configuration
│   └── unicel_iOS.entitlements   # App capabilities
├── Sources/                       # iOS source code
├── Assets.xcassets/               # App icons
├── project.yml                    # XcodeGen config
└── LaunchScreen.storyboard        # Launch screen
```

### Configuration Files
- `src-tauri/tauri.conf.json` - Tauri iOS settings
- `package.json` - iOS build scripts
- `src-tauri/Cargo.toml` - Rust dependencies

### Example Files
- `examples/*.usheet` - Test workbooks
- Four example workbooks bundled with app

---

## Common Issues

### "xcodebuild requires Xcode"
**Cause:** Developer path points to CommandLineTools
**Fix:**
```bash
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
```

### "Code signing failed"
**Cause:** No signing identity configured
**Fix:** See `CODE_SIGNING_GUIDE.md` → Use "Sign to Run Locally"

### "Simulator doesn't launch"
**Cause:** Previous simulator process stuck
**Fix:**
```bash
killall Simulator
npm run tauri:ios:dev
```

### "Rust compilation failed"
**Cause:** iOS targets not installed
**Fix:**
```bash
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
```

---

## Apple Developer Account Requirements

| Task | Free Account | Paid Account ($99/yr) |
|------|--------------|------------------------|
| Simulator testing | ✅ Not needed | ✅ Works |
| Device testing | ✅ Yes (3 devices) | ✅ Yes (100 devices) |
| TestFlight | ❌ No | ✅ Required |
| App Store | ❌ No | ✅ Required |

**For MVP (Week 25):** No Apple Developer account needed (use "Sign to Run Locally")

**For TestFlight/App Store (Week 29):** Paid account required

---

## Next Steps

### For Current Testing (Week 25)
1. **Complete manual testing:**
   - Follow `MANUAL_TESTING_CHECKLIST.md`
   - Run automated testing script
   - Document results

2. **Update task tracking:**
   - Mark completed tasks in `docs/TASKS.md`
   - Add screenshots to `docs/ios/screenshots/`
   - Note any issues

### For App Store Deployment (Week 29)

**Step 1: iCloud Setup (20-30 minutes)**
- Follow `HUMAN_TASK_ICLOUD_SETUP.md`
- Set up iCloud container in Developer Portal
- Configure Xcode project
- Test iCloud Drive integration

**Step 2: TestFlight Distribution (30-60 minutes)**
- Follow `HUMAN_TASK_TESTFLIGHT.md`
- Create app in App Store Connect
- Build and upload release IPA
- Invite beta testers
- Monitor feedback and crashes

**Step 3: File Association Testing (15-20 minutes)**
- Follow `HUMAN_TASK_FILE_ASSOCIATIONS.md`
- Test opening .usheet files from various sources
- Verify file type registration
- Test on multiple devices/iOS versions

**Step 4: App Store Preparation**
- Generate app icons (all required sizes)
- Create App Store screenshots (iPhone + iPad)
- Write privacy policy
- Complete App Store metadata
- Submit for review

---

## Support & References

### Documentation
- [Tauri iOS Guide](https://tauri.app/v1/guides/building/ios)
- [Apple Developer Documentation](https://developer.apple.com/documentation/)
- [Xcode Help](https://help.apple.com/xcode/)

### Project Documentation
- `docs/PROJECT_PLAN.md` - Full implementation roadmap
- `docs/TASKS.md` - Task tracking
- `CLAUDE.md` - Project overview

---

**Last Updated:** October 18, 2025
**Current Phase:** Week 25 - iOS Platform Setup
**Status:** Requires manual testing (automated setup complete)
