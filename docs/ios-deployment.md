# iOS Deployment Guide

## Creating Release Builds for TestFlight/App Store

### IMPORTANT: Do NOT Use Xcode's Product → Archive

Tauri v2 iOS projects require using the Tauri CLI for creating archives, **not** Xcode's built-in "Product → Archive" menu option.

**Why?** The Xcode pre-build script (`tauri ios xcode-script`) communicates with a Tauri CLI server that only runs during `tauri ios dev` or `tauri ios build` commands. Running Xcode's Archive directly will fail with:

```
failed to read CLI options: Error when opening the TCP socket: Connection refused (os error 61)
```

### Correct Workflow

#### For TestFlight (Beta Testing)

```bash
npm run tauri ios build --export-method release-testing
```

This will:
1. Build the frontend (`npm run build`)
2. Compile the Rust backend
3. Build the iOS app in release mode
4. Create an archive
5. Export an IPA ready for TestFlight

**Output:** `/Users/dennisjackson/Code/unicel/src-tauri/gen/apple/build/arm64/Unicel.ipa`

#### For App Store Submission

```bash
npm run tauri ios build --export-method app-store-connect
```

This creates an IPA ready for App Store Connect submission.

### Upload to TestFlight

After building, upload using Xcode's Application Loader or command line:

```bash
xcrun altool --upload-app \
  --type ios \
  --file src-tauri/gen/apple/build/arm64/Unicel.ipa \
  --username "your-apple-id@example.com" \
  --password "@keychain:AC_PASSWORD"
```

Or use the Transporter app (recommended):
1. Open **Transporter.app**
2. Sign in with your Apple ID
3. Drag and drop the `.ipa` file
4. Click "Deliver"

### Development Builds

For testing on simulators during development:

```bash
npm run tauri ios dev
```

This launches the app in the iOS Simulator with hot-reload enabled.

### Building for Different Architectures

```bash
# iPhone (arm64 - default)
npm run tauri ios build --target aarch64

# iPhone Simulator on Apple Silicon (arm64)
npm run tauri ios build --target aarch64-sim

# iPhone Simulator on Intel (x86_64)
npm run tauri ios build --target x86_64
```

### Debug Builds

For debugging release issues:

```bash
npm run tauri ios build --export-method debugging --debug
```

### CI/CD Builds

For automated builds, use the `--ci` flag to skip prompts:

```bash
npm run tauri ios build --export-method release-testing --ci
```

### Troubleshooting

#### "No provisioning profile found"

1. Open Xcode
2. Open `src-tauri/gen/apple/unicel.xcodeproj`
3. Select the `unicel_iOS` target
4. Go to "Signing & Capabilities"
5. Enable "Automatically manage signing"
6. Select your development team
7. Close Xcode and retry the build

#### "Build number already exists in TestFlight"

Increment the build number:

```bash
npm run tauri ios build --export-method release-testing --build-number 2
```

#### "Bundle identifier ends with .app"

Edit `src-tauri/tauri.conf.json` and change:
```json
"identifier": "com.unicel.viewer"
```

(Remove the `.app` suffix)

### Version Management

Version numbers are managed in `src-tauri/tauri.conf.json`:

```json
{
  "version": "0.5.1",
  "identifier": "com.unicel.app"
}
```

The version format is `MAJOR.MINOR.PATCH` (following semantic versioning).

### Pre-Release Checklist

- [ ] Update version in `src-tauri/tauri.conf.json`
- [ ] Update `CHANGELOG.md` with release notes
- [ ] Test app on iPhone Simulator
- [ ] Test app on real device (if available)
- [ ] Verify file associations work (open .usheet files)
- [ ] Run `npm run tauri ios build --export-method release-testing`
- [ ] Upload to TestFlight
- [ ] Test on TestFlight before promoting to production

### Resources

- [Tauri iOS Guide](https://tauri.app/develop/build/ios/)
- [Apple App Store Connect](https://appstoreconnect.apple.com/)
- [TestFlight Documentation](https://developer.apple.com/testflight/)
