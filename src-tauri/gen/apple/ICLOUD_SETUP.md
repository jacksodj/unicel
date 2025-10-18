# iCloud Drive Setup for Unicel

This document explains how to enable iCloud Drive integration for Unicel on iOS.

## Prerequisites

- Apple Developer Account
- Xcode 15.0+
- Physical iOS device (iCloud doesn't work in simulator)

## Steps to Enable iCloud

### 1. Configure iCloud Capability in Xcode

1. Open `unicel.xcodeproj` in Xcode
2. Select the `unicel_iOS` target
3. Go to "Signing & Capabilities" tab
4. Click "+ Capability"
5. Add "iCloud"
6. Enable "iCloud Documents"
7. Set the container identifier to: `iCloud.com.unicel.app`

### 2. Verify Entitlements

The entitlements file (`unicel_iOS.entitlements`) should contain:

```xml
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
```

### 3. Create iCloud Container in Apple Developer Portal

1. Go to https://developer.apple.com/account/resources/identifiers/list/cloudContainer
2. Click "+" to create a new iCloud Container
3. Description: "Unicel Spreadsheets"
4. Identifier: `iCloud.com.unicel.app`
5. Click "Continue" and "Register"

### 4. Update App ID

1. Go to https://developer.apple.com/account/resources/identifiers/list
2. Select your app identifier (com.unicel.app)
3. Enable "iCloud" capability
4. Configure to use the `iCloud.com.unicel.app` container
5. Save changes

### 5. Regenerate Provisioning Profiles

After enabling iCloud:
1. Go to Provisioning Profiles in Apple Developer Portal
2. Delete old profiles for com.unicel.app
3. Create new profiles with iCloud capability enabled

### 6. Update Xcode Signing

1. In Xcode, go to "Signing & Capabilities"
2. Click "Download Manual Profiles" (if using manual signing)
3. Or let Xcode automatically manage signing

## Testing iCloud

### On Device

1. Build and install app on physical device
2. Open Settings > [Your Name] > iCloud
3. Ensure "iCloud Drive" is enabled
4. Open Files app
5. Look for "Unicel" folder under "iCloud Drive"
6. Save a .usheet file there
7. It should sync across devices

### Troubleshooting

**iCloud folder not appearing:**
- Check that you're signed into iCloud on the device
- Verify iCloud Drive is enabled in Settings
- Restart the Files app
- Check Xcode console for iCloud errors

**Files not syncing:**
- Check network connection
- Verify iCloud storage isn't full
- Check that both devices are signed into same iCloud account
- Wait a few minutes (sync can be slow)

**Build errors:**
- Verify container ID matches in entitlements and developer portal
- Ensure provisioning profile includes iCloud capability
- Clean build folder (Cmd+Shift+K) and rebuild

## File Location

With iCloud enabled, .usheet files can be stored in:

- **iCloud Drive**: `iCloud Drive/Unicel/`
- **Local**: App's Documents directory (not synced)

## Notes

- iCloud containers must be created in Apple Developer Portal before use
- Container identifiers are case-sensitive
- Changes to entitlements require new provisioning profiles
- iCloud sync requires internet connection
- Files may take time to sync between devices
- Free iCloud accounts have 5GB storage limit

## Support

For issues with iCloud setup, see:
- [Apple's iCloud Documentation](https://developer.apple.com/icloud/)
- [Tauri iOS Documentation](https://beta.tauri.app/guides/build/ios/)
