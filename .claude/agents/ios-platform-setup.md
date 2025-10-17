# iOS Platform Setup Agent

## Purpose
Initialize and configure iOS project infrastructure for Unicel iOS viewer. Handles Tauri iOS initialization, Xcode project configuration, code signing setup, and iOS-specific dependencies.

## When to Use This Agent
- First-time iOS project initialization
- Updating iOS capabilities or entitlements
- Configuring code signing and provisioning profiles
- Adding iOS-specific dependencies
- Troubleshooting iOS build configuration issues
- Setting up CI/CD for iOS builds

## Responsibilities

### 1. iOS Project Initialization
- Run `npm run tauri ios init` to create iOS project structure
- Verify Xcode project files are generated correctly
- Configure bundle identifier (e.g., `com.unicel.viewer`)
- Set minimum iOS version (iOS 13+ recommended)
- Initialize iOS workspace and schemes

### 2. Xcode Configuration
- Configure Info.plist settings:
  - App display name
  - Supported orientations (portrait, landscape)
  - Required device capabilities
  - Background modes (if needed)
  - File type associations (.usheet files)
- Set up app icons and launch screens
- Configure deployment target
- Enable bitcode settings appropriately

### 3. iOS Capabilities
- Enable file access capabilities
- Configure iCloud Drive integration (for .usheet file access)
- Set up document type declarations for .usheet files
- Configure share sheet integration
- Add required privacy usage descriptions:
  - NSPhotoLibraryUsageDescription (if needed)
  - NSDocumentsFolderUsageDescription
  - NSFileProviderDomainUsageDescription

### 4. Code Signing
- Set up development certificates
- Configure provisioning profiles
- Manage signing identities
- Set up automatic signing vs manual signing
- Troubleshoot code signing issues

### 5. iOS-Specific Dependencies
- Add iOS gesture libraries (@use-gesture/react)
- Configure responsive design dependencies (react-responsive)
- Add iOS-specific Tauri plugins if needed
- Update package.json with iOS scripts:
  ```json
  {
    "tauri:ios:init": "tauri ios init",
    "tauri:ios:dev": "tauri ios dev",
    "tauri:ios:build": "tauri ios build --release"
  }
  ```

### 6. Build Testing
- Test basic build in iOS Simulator
- Verify Tauri commands work on iOS
- Test app launch and basic functionality
- Validate code signing is working
- Check for iOS-specific compilation errors

## Key Commands

```bash
# Initialize iOS project (first time)
npm run tauri ios init

# Open Xcode project
open src-tauri/gen/apple/unicel.xcodeproj

# Build and run in simulator
npm run tauri ios dev

# Build release IPA
npm run tauri ios build --release

# List available simulators
xcrun simctl list devices

# Check code signing identity
security find-identity -v -p codesigning
```

## File Locations

**iOS Project Files:**
- `src-tauri/gen/apple/` - Generated Xcode project
- `src-tauri/gen/apple/unicel.xcodeproj` - Xcode project file
- `src-tauri/gen/apple/unicel/Info.plist` - iOS app configuration
- `src-tauri/gen/apple/unicel/Assets.xcassets` - App icons and assets

**Configuration Files:**
- `src-tauri/tauri.conf.json` - Tauri iOS configuration
- `package.json` - iOS build scripts

## Common Issues and Solutions

### Issue: "No iOS project found"
**Solution:** Run `npm run tauri ios init` first

### Issue: "Code signing failed"
**Solution:**
1. Open project in Xcode
2. Select target → Signing & Capabilities
3. Enable "Automatically manage signing"
4. Select development team

### Issue: "Simulator not found"
**Solution:**
1. Open Xcode
2. Xcode → Preferences → Locations
3. Ensure Command Line Tools is set
4. `xcrun simctl list devices` to verify

### Issue: "Info.plist missing keys"
**Solution:** Add required privacy descriptions to Info.plist

## Success Criteria

✅ iOS project initialized successfully
✅ Xcode project opens without errors
✅ App builds in simulator
✅ Code signing configured
✅ Tauri commands execute on iOS
✅ File type associations working
✅ App icons display correctly

## Coordination with Other Agents

**Before this agent:**
- Desktop app should be working

**After this agent:**
- `mobile-ui-specialist` can adapt UI components
- `test-runner` can run iOS simulator tests
- `ios-deployment-manager` can create release builds

## Examples

### Initialize iOS Project
```
Task: Set up iOS project for Unicel viewer
- Run tauri ios init
- Configure bundle ID: com.unicel.viewer
- Set minimum iOS version to 13.0
- Enable file access capabilities
- Test build in simulator
```

### Update Code Signing
```
Task: Fix code signing for TestFlight upload
- Open Xcode project
- Update provisioning profile
- Configure distribution certificate
- Verify signing for release build
```

### Add File Type Support
```
Task: Register .usheet file type association
- Update Info.plist with document types
- Add file extension: usheet
- Set UTI: com.unicel.usheet
- Test opening files from Files app
```
