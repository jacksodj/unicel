# iOS Platform Setup Status

## Current Status: READY FOR BUILD (Pending xcode-select Fix)

### Completed Configuration

#### 1. Code Signing (DONE)
- **Status**: Configured
- **Method**: Automatic signing with development team
- **Configuration**: `src-tauri/tauri.conf.json`
- **Development Team**: Z3L3V842L2
- **Certificate**: Apple Development: Dennis Jackson (887GTC3PSW)

```json
"iOS": {
  "minimumSystemVersion": "13.0",
  "developmentTeam": "Z3L3V842L2"
}
```

#### 2. iOS Project Initialization (DONE)
- **Status**: Complete
- **Location**: `/Users/dennisjackson/Code/unicel/src-tauri/gen/apple/`
- **Xcode Project**: `unicel.xcodeproj`
- **Bundle ID**: com.unicel.app
- **Version**: 0.5.1

#### 3. File Type Associations (DONE)
- **Status**: Configured in Info.plist
- **File type**: .usheet files
- **UTI**: com.unicel.usheet
- **Role**: Viewer
- **Rank**: Owner

The Info.plist includes:
- CFBundleDocumentTypes for .usheet files
- UTExportedTypeDeclarations for com.unicel.usheet
- Proper orientation support (portrait and landscape)

### Remaining Issue: xcode-select Path

**Problem**:
- Current path: `/Library/Developer/CommandLineTools`
- Required path: `/Applications/Xcode.app/Contents/Developer`

**Impact**:
- `xcrun simctl` commands fail with exit code 72
- Cannot list or boot iOS simulators
- Build process cannot find Xcode tools

**Solution**:
Run the setup script I created:

```bash
./fix-xcode-setup.sh
```

Or manually:

```bash
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
sudo xcodebuild -license accept
```

**Verification**:
```bash
xcode-select -p
# Should output: /Applications/Xcode.app/Contents/Developer

xcrun simctl list devices available
# Should list iOS simulators without errors
```

## Next Steps (After Fixing xcode-select)

### 1. Run Development Build

```bash
npm run tauri:ios:dev
```

**Expected behavior**:
- Rust compilation for iOS targets (first time: 5-10 minutes)
- Xcode may open automatically
- iOS Simulator launches
- App appears in simulator

### 2. Test Basic Functionality

Once app launches:
- Verify app opens without crashes
- Test basic UI navigation
- Try opening a .usheet file (if file picker works)
- Test basic Tauri commands

### 3. Test Different Simulators

```bash
# iPhone SE (smallest screen)
xcrun simctl list devices | grep "iPhone SE"

# iPhone 15 Pro (standard)
xcrun simctl list devices | grep "iPhone 15"

# iPad Air (tablet)
xcrun simctl list devices | grep "iPad Air"
```

## Build Commands Reference

```bash
# Development build (opens simulator)
npm run tauri:ios:dev

# Production build (creates IPA)
npm run tauri:ios:build --release

# Open Xcode project manually
open src-tauri/gen/apple/unicel.xcodeproj

# List available simulators
xcrun simctl list devices available

# Boot specific simulator
xcrun simctl boot "iPhone 15 Pro"

# Open Simulator app
open -a Simulator
```

## Troubleshooting

### Build Fails in Xcode

If Xcode opens with build errors:
1. Select "unicel_iOS" target
2. Select an iOS Simulator device
3. Go to Signing & Capabilities
4. Verify team is selected: "Dennis Jackson (Z3L3V842L2)"
5. Enable "Automatically manage signing"
6. Clean build folder: Product → Clean Build Folder
7. Build: Product → Build (Cmd+B)

### Simulator Issues

```bash
# Kill all simulators
killall Simulator

# List booted simulators
xcrun simctl list devices | grep Booted

# Shutdown all simulators
xcrun simctl shutdown all

# Reset specific simulator
xcrun simctl erase "iPhone 15 Pro"
```

### Xcode Cache Issues

```bash
# Clear derived data
rm -rf ~/Library/Developer/Xcode/DerivedData/*

# Clear Tauri build
rm -rf src-tauri/target/
rm -rf src-tauri/gen/apple/build/

# Rebuild
npm run tauri:ios:dev
```

## Configuration Files

### Modified Files
- `src-tauri/tauri.conf.json`: Added iOS configuration
  - Development team ID
  - Minimum iOS version (13.0)

### Generated Files (Do Not Modify Manually)
- `src-tauri/gen/apple/unicel_iOS/Info.plist`: Auto-generated
- `src-tauri/gen/apple/project.yml`: Xcode project configuration
- `src-tauri/gen/apple/unicel.xcodeproj`: Xcode project

### Helper Scripts
- `fix-xcode-setup.sh`: Fixes xcode-select path (run once)
- `IOS_BUILD_FIX_GUIDE.md`: Detailed troubleshooting guide

## Success Criteria

- [x] iOS project initialized
- [x] Code signing configured
- [x] File type associations added
- [x] Development team set
- [ ] **xcode-select path fixed** (USER ACTION REQUIRED)
- [ ] Build completes without errors
- [ ] App launches in simulator
- [ ] No crashes on launch
- [ ] Basic UI renders correctly

## System Information

**Development Environment**:
- macOS: Darwin 24.6.0
- Xcode: Installed at /Applications/Xcode.app
- Node.js: (check with `node --version`)
- Rust: (check with `rustc --version`)

**Code Signing**:
- Team ID: Z3L3V842L2
- Certificate: Apple Development: Dennis Jackson (887GTC3PSW)
- Signing Method: Automatic

**iOS Targets**:
- Minimum iOS Version: 13.0
- Supported Devices: iPhone, iPad
- Supported Orientations: All

## Notes

- This is development configuration only
- Production builds require App Store provisioning profiles
- First iOS build takes 5-10 minutes (Rust cross-compilation)
- Simulator builds are debug builds (larger, slower than release)
- The `gen/apple/` directory is auto-generated; don't manually edit Xcode project files there

## Contact & Support

If issues persist after fixing xcode-select:
1. Check Xcode version: `xcodebuild -version`
2. Verify Rust iOS targets: `rustup target list | grep ios`
3. Check Tauri CLI version: `npm list @tauri-apps/cli`
4. Review Xcode build logs in Xcode's Report Navigator
