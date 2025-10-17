# iOS Development Guide

This guide covers the iOS version of Unicel, a read-only viewer for .usheet spreadsheet files.

## Table of Contents

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Initial Setup](#initial-setup)
4. [Architecture](#architecture)
5. [Development Workflow](#development-workflow)
6. [Testing](#testing)
7. [Deployment](#deployment)
8. [Troubleshooting](#troubleshooting)

---

## Overview

**iOS MVP Scope:**
- Read-only viewer for .usheet files
- Open files from iOS Files app or iCloud Drive
- Render spreadsheet grid with touch interaction
- Display unit-aware cell values
- Toggle Metric/Imperial display
- **NOT included:** Editing, saving, exporting (unless trivial to implement)

**Target Devices:**
- iPhone: iPhone SE (3rd gen) and newer
- iPad: iPad Air (4th gen) and newer
- iOS Version: 15.0+

**Timeline:** 5 weeks (Weeks 25-29)
- Week 25: Platform setup
- Week 26: Mobile UI adaptation
- Week 27: File handling and polish
- Week 28: iPad optimization
- Week 29: App Store submission

---

## Prerequisites

### Required Software

1. **macOS**: macOS 13 Ventura or newer
2. **Xcode**: Version 15.0 or newer
   ```bash
   # Install from App Store or
   xcode-select --install
   ```

3. **iOS Simulator**: Installed with Xcode
4. **Rust**: Already installed (from desktop development)
5. **Node.js**: Already installed (from desktop development)

### Apple Developer Account

- **Free Account**: Sufficient for simulator testing and 7-day device testing
- **Paid Account ($99/year)**: Required for TestFlight and App Store submission

### Recommended Tools

- **Xcode Command Line Tools**: `xcode-select --install`
- **CocoaPods** (if needed for dependencies): `sudo gem install cocoapods`
- **iOS Device** (optional): For real device testing

---

## Initial Setup

### 1. Initialize iOS Project

```bash
# From project root
npm run tauri ios init
```

This creates:
- `src-tauri/gen/apple/` directory
- Xcode project at `src-tauri/gen/apple/unicel.xcodeproj`
- iOS-specific configuration files

### 2. Configure Xcode Project

Open the project:
```bash
open src-tauri/gen/apple/unicel.xcodeproj
```

**Required Settings:**

1. **Bundle Identifier**: `com.unicel.app` (or your organization)
2. **Display Name**: `Unicel`
3. **Version**: Match `tauri.conf.json` version
4. **Deployment Target**: iOS 15.0+

### 3. Configure Info.plist

Add file type association for .usheet files:

```xml
<!-- In src-tauri/gen/apple/Info.plist -->
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

<key>UTExportedTypeDeclarations</key>
<array>
    <dict>
        <key>UTTypeIdentifier</key>
        <string>com.unicel.usheet</string>
        <key>UTTypeDescription</key>
        <string>Unicel Spreadsheet</string>
        <key>UTTypeConformsTo</key>
        <array>
            <string>public.data</string>
            <string>public.content</string>
        </array>
        <key>UTTypeTagSpecification</key>
        <dict>
            <key>public.filename-extension</key>
            <array>
                <string>usheet</string>
            </array>
        </dict>
    </dict>
</array>
```

### 4. Add iOS Capabilities

Enable in Xcode project settings:
- **File Access**: Document Browser
- **iCloud Drive**: For cloud file access (optional)
- **Background Modes**: None needed for MVP

### 5. Install iOS-Specific Dependencies

```bash
npm install @use-gesture/react react-responsive
```

Add to `package.json`:
```json
{
  "dependencies": {
    "@use-gesture/react": "^10.3.0",
    "react-responsive": "^10.0.0"
  }
}
```

---

## Architecture

### Platform Detection

Create `src/hooks/useMobile.ts`:
```typescript
import { useEffect, useState } from 'react';

export function useMobile() {
  const [isMobile, setIsMobile] = useState(false);
  const [isTablet, setIsTablet] = useState(false);

  useEffect(() => {
    const checkPlatform = () => {
      const userAgent = navigator.userAgent;
      const isIOS = /iPhone|iPad|iPod/.test(userAgent);
      setIsMobile(/iPhone|iPod/.test(userAgent));
      setIsTablet(/iPad/.test(userAgent));
    };
    checkPlatform();
  }, []);

  return {
    isMobile,
    isTablet,
    isTouchDevice: isMobile || isTablet,
    isIOS: isMobile || isTablet
  };
}
```

### Conditional Rendering

```typescript
// In src/App.tsx or main component
import { useMobile } from './hooks/useMobile';

function App() {
  const { isMobile, isTablet } = useMobile();

  if (isMobile || isTablet) {
    return <MobileApp />;
  }

  return <DesktopApp />;
}
```

### Component Structure

```
src/
├── components/
│   ├── mobile/              # iOS-specific components
│   │   ├── MobileGrid.tsx   # Touch-enabled grid
│   │   ├── MobileToolbar.tsx # Simplified toolbar
│   │   └── MobileStatusBar.tsx # Safe area aware
│   ├── Grid.tsx             # Desktop grid
│   └── Spreadsheet.tsx      # Desktop spreadsheet
├── hooks/
│   └── useMobile.ts         # Platform detection
└── App.tsx                  # Root component
```

### Touch Gesture Handling

```typescript
// In MobileGrid.tsx
import { useGesture } from '@use-gesture/react';

export function MobileGrid({ workbook, sheetName }) {
  const bind = useGesture({
    onDrag: ({ movement: [mx, my] }) => {
      // Pan/scroll the grid
    },
    onPinch: ({ offset: [scale] }) => {
      // Zoom in/out
    },
    onTap: ({ event }) => {
      // Select cell
    },
    onLongPress: ({ event }) => {
      // Show cell details popover
    },
  });

  return (
    <div {...bind()} className="touch-none overflow-hidden">
      {/* Grid rendering */}
    </div>
  );
}
```

### Safe Area Handling

```typescript
// In MobileStatusBar.tsx
export function MobileStatusBar() {
  return (
    <div
      className="fixed bottom-0 left-0 right-0 bg-white border-t"
      style={{
        paddingBottom: 'env(safe-area-inset-bottom)',
      }}
    >
      {/* Status bar content */}
    </div>
  );
}
```

---

## Development Workflow

### Commands

```bash
# Start development server with iOS simulator
npm run tauri:ios:dev

# Build release IPA
npm run tauri:ios:build --release

# Open Xcode project
open src-tauri/gen/apple/unicel.xcodeproj

# Clean build artifacts
cargo clean
rm -rf src-tauri/gen/apple/build
```

Add to `package.json`:
```json
{
  "scripts": {
    "tauri:ios:init": "tauri ios init",
    "tauri:ios:dev": "tauri ios dev",
    "tauri:ios:build": "tauri ios build --release"
  }
}
```

### Hot Reload

When running `npm run tauri:ios:dev`:
- Frontend changes hot-reload automatically
- Rust changes require rebuild (stop and restart)
- iOS-specific changes (Info.plist, etc.) require rebuild

### Logging

```typescript
// Frontend logging (visible in Xcode console)
console.log('iOS Debug:', data);

// Rust logging (visible in Xcode console)
// In Rust code:
use log::{info, warn, error};
info!("Loading workbook on iOS");
```

View logs in Xcode:
1. Run app in simulator or device
2. Open Xcode → Window → Devices and Simulators
3. Select device → View Device Logs

---

## Testing

### iOS Simulator Testing

```bash
# Run in iPhone simulator
npm run tauri:ios:dev

# Select different simulators in Xcode
# Xcode → Product → Destination → Choose simulator
```

**Test Matrix:**
- iPhone SE (3rd gen) - smallest screen (4.7")
- iPhone 15 Pro - standard size (6.1")
- iPhone 15 Pro Max - largest phone (6.7")
- iPad Air (11") - smallest tablet
- iPad Pro (12.9") - largest tablet

### Real Device Testing

1. **Connect iOS device via USB**
2. **Trust computer on device**
3. **Select device in Xcode**: Product → Destination → Your Device
4. **Build and run**: `npm run tauri:ios:dev`

**Note:** Free Apple Developer accounts can test on devices for 7 days before re-signing required.

### Testing Checklist

**File Operations:**
- [ ] Open .usheet file from Files app
- [ ] Open from iCloud Drive
- [ ] Handle corrupted files gracefully
- [ ] Display error messages clearly

**Grid Interaction:**
- [ ] Tap to select cell
- [ ] Swipe to scroll
- [ ] Pinch to zoom (if implemented)
- [ ] Long-press for details

**Display:**
- [ ] Units render correctly
- [ ] Formulas display in cell details
- [ ] Metric/Imperial toggle works
- [ ] Layout adapts to orientation

**Performance:**
- [ ] 60fps scrolling on iPhone 15
- [ ] Smooth on older devices (iPhone SE)
- [ ] Large workbooks (10,000+ cells) load < 2s
- [ ] No memory leaks (test with Xcode Instruments)

**Edge Cases:**
- [ ] Rotate device (portrait ↔ landscape)
- [ ] Backgrounding and foregrounding
- [ ] Low memory warnings
- [ ] Airplane mode / offline behavior

---

## Deployment

### Code Signing

**For TestFlight and App Store:**

1. **Create App ID** in Apple Developer Portal:
   - Identifier: `com.unicel.app`
   - Capabilities: None needed for MVP

2. **Create Provisioning Profile**:
   - Distribution profile for App Store
   - Download and install in Xcode

3. **Configure in Xcode**:
   - Select project → Signing & Capabilities
   - Team: Your Apple Developer account
   - Provisioning Profile: Automatic or Manual

### Building Release IPA

```bash
# Build release version
npm run tauri:ios:build --release

# IPA location:
# src-tauri/gen/apple/build/arm64-apple-ios/release/bundle/ios/Unicel.ipa
```

### App Icons

Generate all required sizes:

| Size | Usage |
|------|-------|
| 1024x1024 | App Store |
| 180x180 | iPhone @3x |
| 120x120 | iPhone @2x |
| 167x167 | iPad Pro @2x |
| 152x152 | iPad @2x |
| 76x76 | iPad @1x |

**Tool**: Use Xcode Asset Catalog or online generators

Place in `src-tauri/gen/apple/Assets.xcassets/AppIcon.appiconset/`

### Screenshots

**Required Sizes** (for App Store):

**iPhone:**
- 6.7" display (1290 × 2796 px) - iPhone 15 Pro Max
- 6.5" display (1242 × 2688 px) - iPhone 11 Pro Max
- 5.5" display (1242 × 2208 px) - iPhone 8 Plus

**iPad:**
- 12.9" display (2048 × 2732 px) - iPad Pro
- 11" display (1668 × 2388 px) - iPad Pro

**Content Ideas:**
1. Opening a .usheet file
2. Grid view with cells and units
3. Cell details popover
4. Metric/Imperial toggle
5. Large workbook scrolling

### TestFlight Beta Testing

1. **Archive build in Xcode**:
   - Product → Archive
   - Wait for build to complete

2. **Upload to App Store Connect**:
   - Xcode Organizer → Distribute App
   - App Store Connect → Upload

3. **Configure TestFlight**:
   - Add testers (email addresses)
   - Add test information and privacy policy URL

4. **Invite testers**:
   - Internal testers: Up to 100 (instant access)
   - External testers: Unlimited (requires Apple review)

### App Store Submission

**Preparation:**

1. **App Store Connect**:
   - Create app entry
   - Bundle ID: `com.unicel.app`
   - Name: `Unicel`
   - Category: Productivity

2. **Metadata**:
   - Description (4000 char max)
   - Keywords (100 char max)
   - Support URL
   - Privacy Policy URL

3. **Privacy Policy**:
   - Required for all App Store apps
   - Host at: https://unicel.app/privacy (or GitHub Pages)
   - Content: "Unicel does not collect any personal data. All spreadsheet files are stored locally on your device."

4. **App Review Information**:
   - Demo account (if needed): N/A
   - Notes: "This is a read-only viewer for .usheet spreadsheet files. Users can open files from Files app or iCloud Drive."

**Submission Process:**

1. Upload final build via Xcode
2. Select build in App Store Connect
3. Submit for review
4. Wait 1-3 days for review
5. Monitor status in App Store Connect

**Review Tips:**
- Provide clear instructions
- Include sample .usheet file link
- Respond promptly to review feedback
- Test on latest iOS version before submitting

---

## Troubleshooting

### Build Errors

**"No provisioning profiles found"**
- Solution: Create provisioning profile in Apple Developer Portal
- Or: Use Xcode automatic signing (Xcode → Signing & Capabilities)

**"Rust target not found"**
```bash
# Add iOS targets
rustup target add aarch64-apple-ios      # iPhone/iPad (ARM64)
rustup target add x86_64-apple-ios       # Simulator (Intel)
rustup target add aarch64-apple-ios-sim  # Simulator (Apple Silicon)
```

**"Tauri command not found"**
```bash
# Install Tauri CLI
npm install -g @tauri-apps/cli
# Or use npm run tauri
```

### Simulator Issues

**"Simulator not booting"**
- Solution: Restart Xcode and try again
- Or: Delete simulator and recreate (Xcode → Window → Devices and Simulators)

**"Hot reload not working"**
- Solution: Restart dev server (`npm run tauri:ios:dev`)
- Check that WKWebView can access localhost

**"Black screen on launch"**
- Solution: Check frontend builds successfully (`npm run build`)
- Check Xcode console for JavaScript errors

### File Handling Issues

**".usheet files not opening"**
- Solution: Verify Info.plist has correct UTI declarations
- Reinstall app after Info.plist changes
- Check File app → On My iPhone → Unicel folder

**"Permission denied reading files"**
- Solution: Check iOS entitlements for file access
- Verify app has "Document Browser" capability

### Performance Issues

**"Slow scrolling on device"**
- Solution: Enable production mode (`npm run tauri:ios:build --release`)
- Profile with Xcode Instruments (Time Profiler)
- Implement virtual scrolling for large grids

**"Memory warnings"**
- Solution: Reduce grid cell count rendered at once
- Lazy load cell values
- Profile with Xcode Instruments (Allocations)

### Deployment Issues

**"Upload failed to App Store Connect"**
- Solution: Verify bundle ID matches App Store Connect
- Check provisioning profile is valid
- Ensure all icons and metadata are present

**"App rejected for missing privacy policy"**
- Solution: Add privacy policy URL in App Store Connect
- Ensure policy is publicly accessible

**"TestFlight build not appearing"**
- Solution: Wait 5-10 minutes for processing
- Check email for processing errors
- Verify build passed automated checks

---

## Resources

### Official Documentation

- **Tauri iOS Guide**: https://v2.tauri.app/develop/mobile/
- **Apple Developer**: https://developer.apple.com/
- **App Store Review Guidelines**: https://developer.apple.com/app-store/review/guidelines/
- **Human Interface Guidelines**: https://developer.apple.com/design/human-interface-guidelines/ios

### Community Resources

- **Tauri Discord**: https://discord.com/invite/tauri
- **Stack Overflow**: Tag `tauri`, `ios`, `react-native-webview`

### Tools

- **Xcode**: https://developer.apple.com/xcode/
- **CocoaPods**: https://cocoapods.org/
- **TestFlight**: https://developer.apple.com/testflight/

---

## Next Steps

After completing iOS MVP:

1. **Gather user feedback** from TestFlight beta
2. **Consider adding editing** if demand is high
3. **Android version** (if iOS is successful)
4. **Sync across devices** (iCloud or custom backend)
5. **Apple Pencil support** for iPad (annotations, sketching)

---

**Last Updated**: 2025-10-17
**Phase**: 10 - iOS Viewer MVP
**Status**: Documentation Complete
