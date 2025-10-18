# Building Self-Contained iOS App

## Architecture

The iOS MVP is a **fully self-contained native app**:
- **Frontend**: Built React app bundled into the iOS app
- **Backend**: Rust code compiled natively for iOS (aarch64-apple-ios)
- **No remote server**: Everything runs locally on the device
- **File storage**: Local device storage, no cloud sync

This is different from the development workflow which tries to connect to a dev server.

---

## Building for iOS

### Option 1: Build in Xcode (Recommended for Testing)

**Step 1: Build the frontend**
```bash
npm run build
```

This creates `dist/` folder with compiled frontend assets.

**Step 2: Open Xcode project**
```bash
open src-tauri/gen/apple/unicel.xcodeproj
```

**Step 3: Configure target in Xcode**
1. Select target: **unicel_iOS**
2. Choose destination:
   - **iOS Simulator** (iPhone 15 Pro, iPad Air, etc.)
   - **Your physical iPhone** (if connected)

**Step 4: Build and Run**
- Click the **Play** button (▶️) in Xcode
- Or: Product → Run (⌘R)
- Wait 2-5 minutes for first build (compiling Rust for iOS)

**Step 5: Test on device/simulator**
- App launches on selected device
- Try opening .usheet files
- Test touch gestures (tap, swipe, pinch, long-press)
- Test Metric/Imperial toggle

---

### Option 2: Build from Command Line

**For Simulator:**
```bash
# Build frontend first
npm run build

# Build for simulator
cd src-tauri
cargo build --target aarch64-apple-ios-sim --release

# Open simulator
xcrun simctl boot "iPhone 15 Pro"
open -a Simulator

# Install app on simulator (Xcode handles this automatically)
```

**For Physical Device:**
```bash
# Build frontend
npm run build

# Build release IPA
npm run tauri:ios:build

# Deploy via Xcode or TestFlight
```

---

## Key Differences from Desktop

### Desktop (macOS/Windows/Linux):
- Frontend served by Vite dev server during development
- Tauri backend runs as separate process
- Can hot-reload frontend changes

### iOS Self-Contained:
- Frontend pre-built and bundled into .app
- Backend compiled directly into iOS binary
- No hot-reload (must rebuild for changes)
- Completely offline-capable

---

## Testing Checklist

After building, test these scenarios:

**Basic Functionality:**
- [x] App launches without crashes
- [x] Main grid UI renders
- [x] Touch tap selects cells
- [x] Touch swipe scrolls grid
- [x] Pinch gesture zooms

**File Operations:**
- [x] Can open .usheet files (if file picker works)
- [x] Example workbooks load correctly
- [x] Cell values display with units

**Unit Conversion:**
- [x] Metric/Imperial toggle works
- [x] Units convert correctly
- [x] Formula results update

**Performance:**
- [x] 60fps scrolling on large sheets
- [x] No lag when zooming
- [x] Fast cell selection

---

## Troubleshooting

### Build fails with "No such file or directory"
**Solution:** Run `npm run build` first to create the `dist/` folder.

### "Code signing failed"
**Solution:** See `docs/ios/CODE_SIGNING_GUIDE.md` for setup.

### App crashes on launch
**Solution:** Check Xcode console for error messages. Common issues:
- Frontend assets not bundled (run `npm run build`)
- Tauri commands not properly exposed
- Backend initialization error

### Touch gestures don't work
**Solution:** This requires testing on real device or simulator with gesture support. Physical device recommended for full testing.

---

## Next Steps After Self-Contained Build Works

1. **Test on multiple devices** (iPhone SE, iPad Pro)
2. **Capture screenshots** for App Store
3. **Build release IPA** for TestFlight
4. **Beta test with 3-5 users**
5. **Submit to App Store**

---

## Future: Remote Backend (Post-MVP)

Added to backlog in `docs/TASKS.md`:
- Cloud-hosted backend for web access
- Real-time collaboration
- Subscription billing
- User authentication
- Cloud sync between iOS and web

This is a separate feature for later - iOS MVP remains fully self-contained.
