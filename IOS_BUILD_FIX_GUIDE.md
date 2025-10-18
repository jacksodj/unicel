# iOS Build Fix Guide

## Problem Summary

The iOS build was failing with two issues:
1. **xcode-select path error**: Pointing to Command Line Tools instead of Xcode.app
2. **Code signing not configured**: Missing development team ID in tauri.conf.json

## Solutions Applied

### 1. Code Signing Configuration (DONE)

I've updated `src-tauri/tauri.conf.json` with your iOS configuration:

```json
"iOS": {
  "minimumSystemVersion": "13.0",
  "developmentTeam": "Z3L3V842L2"
}
```

This uses your existing Apple Development certificate:
- Identity: "Apple Development: Dennis Jackson (887GTC3PSW)"
- Team ID: Z3L3V842L2

### 2. Fix xcode-select Path (YOU NEED TO RUN THIS)

**Run the setup script I created:**

```bash
./fix-xcode-setup.sh
```

This script will:
1. Switch xcode-select to Xcode.app (requires sudo password)
2. Accept Xcode license if needed
3. Verify xcrun commands work

**Or run manually:**

```bash
# Switch to Xcode.app
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer

# Accept license
sudo xcodebuild -license accept

# Verify it worked
xcode-select -p
# Should output: /Applications/Xcode.app/Contents/Developer

# Test xcrun
xcrun simctl list devices available
# Should list iOS simulators without errors
```

## After Fixing xcode-select

Once you've run the setup script above, try the iOS build:

```bash
npm run tauri:ios:dev
```

### What to Expect

1. **First build takes 5-10 minutes** - Rust needs to compile for iOS targets
2. **Xcode may open automatically** - This is normal
3. **Build output will be verbose** - Lots of compilation messages
4. **Simulator will launch** - If successful, iOS Simulator opens with the app

### If Xcode Opens with Errors

If Xcode opens showing build errors:

1. Select the "unicel_iOS" target (top left)
2. Select an iOS Simulator device (e.g., "iPhone 15 Pro")
3. Go to Signing & Capabilities tab
4. Verify "Automatically manage signing" is checked
5. Verify Team shows "Dennis Jackson (Z3L3V842L2)"
6. Click the Play button to build

### Troubleshooting

**If xcrun still fails after fixing xcode-select:**
```bash
# Kill and restart Xcode
killall Xcode
killall Simulator

# Clear Xcode derived data
rm -rf ~/Library/Developer/Xcode/DerivedData/*

# Try again
npm run tauri:ios:dev
```

**If you get "No iOS project found":**
```bash
# Initialize iOS project (if not done yet)
npm run tauri:ios:init
```

**If simulator doesn't launch:**
```bash
# List available simulators
xcrun simctl list devices available

# Boot a specific simulator manually
xcrun simctl boot "iPhone 15 Pro"

# Open Simulator app
open -a Simulator
```

## Success Criteria

After successful build:
- No exit code 72 errors
- iOS Simulator launches
- App icon appears on simulator home screen
- App opens without crashes
- Main Unicel UI is visible

## Next Steps After Success

1. Test opening a .usheet file
2. Test basic spreadsheet operations
3. Verify Tauri commands work
4. Test on different simulator sizes (iPhone SE, iPad)
5. Take screenshots for documentation

## Files Modified

- `src-tauri/tauri.conf.json`: Added iOS configuration with development team
- `fix-xcode-setup.sh`: Created script to fix xcode-select path
- `IOS_BUILD_FIX_GUIDE.md`: This guide

## Notes

- The xcode-select fix only needs to be done once
- Code signing configuration is now permanent in tauri.conf.json
- Development team ID (Z3L3V842L2) is from your existing Apple Developer account
- This is development signing only - production builds need different configuration
