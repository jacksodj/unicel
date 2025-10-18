# Xcode Cloud Setup Checklist

Quick reference for enabling Xcode Cloud builds for Unicel iOS.

## Pre-Flight Checklist

- [ ] Apple Developer Account (Admin or App Manager role)
- [ ] App created in App Store Connect (Bundle ID: `com.unicel.app`)
- [ ] Repository connected to GitHub (or other Git provider)
- [ ] Local builds working (`npm run tauri:ios:build`)

## File Verification

- [ ] `ci_scripts/ci_post_clone.sh` exists
- [ ] Script is executable (`chmod +x`)
- [ ] Script is committed to Git repository
- [ ] Changes pushed to remote (`main` branch)

## App Store Connect Configuration

### 1. Enable Xcode Cloud
- [ ] Navigate to App Store Connect → Apps → Unicel → Xcode Cloud
- [ ] Click "Enable Xcode Cloud"

### 2. Connect Repository
- [ ] Click "Connect Repository"
- [ ] Select GitHub (or your Git provider)
- [ ] Authorize Xcode Cloud access
- [ ] Select `jacksodj/unicel` repository
- [ ] Grant read permissions

### 3. Create Workflow
- [ ] Click "Create Workflow"
- [ ] Name: "Release Build (TestFlight)"
- [ ] Xcode Version: Latest Release
- [ ] Start Condition: Branch = `main`, On every push
- [ ] Action: Archive - iOS (Scheme: `unicel_iOS`)
- [ ] Post-Action: TestFlight (Internal Testing)
- [ ] Verify custom scripts detected: `ci_post_clone.sh`
- [ ] Save workflow

## First Build

### Trigger Build
- [ ] Push to `main` branch (or click "Start Build" manually)
- [ ] Build status shows "Running"

### Monitor Progress (15-25 minutes total)
- [ ] Clone Repository (1-2 min)
- [ ] Post-Clone Script (5-8 min) - **Check logs for Node.js/Rust install**
- [ ] Xcode Build (8-12 min)
- [ ] Archive & Export (1-2 min)
- [ ] TestFlight Upload (2-3 min)

### Verify Success
- [ ] Build status shows "Succeeded" (green checkmark)
- [ ] IPA uploaded to TestFlight
- [ ] TestFlight processing started

## Post-Build Verification

### TestFlight
- [ ] Go to App Store Connect → TestFlight
- [ ] See new build (status: "Processing" or "Ready to Test")
- [ ] Add "What to Test" notes
- [ ] Invite internal testers
- [ ] Install on device via TestFlight app
- [ ] Test core functionality

### Next Steps
- [ ] Test opening .usheet files
- [ ] Verify Metric ↔ Imperial toggle
- [ ] Test sheet navigation
- [ ] Report any issues
- [ ] Iterate and improve

## Troubleshooting

### If Build Fails

**Check logs:**
- [ ] Click on failed build in App Store Connect
- [ ] Expand "Post-Clone" logs
- [ ] Look for errors in Node.js/Rust installation
- [ ] Download full logs (Actions → Download Logs)

**Common issues:**
- [ ] `npm: command not found` → Ensure `ci_post_clone.sh` ran
- [ ] `rustc: command not found` → Check Rust installation in logs
- [ ] Code signing failed → Verify Team ID in `tauri.conf.json`
- [ ] Frontend build failed → Check Vite build logs

**Test locally:**
```bash
# Run post-clone script locally
./src-tauri/gen/apple/ci_scripts/ci_post_clone.sh

# Build iOS locally
npm run tauri:ios:build
```

## Quick Commands

```bash
# View script
cat src-tauri/gen/apple/ci_scripts/ci_post_clone.sh

# Test script locally
./src-tauri/gen/apple/ci_scripts/ci_post_clone.sh

# Make executable (if needed)
chmod +x src-tauri/gen/apple/ci_scripts/ci_post_clone.sh

# Commit and push
git add src-tauri/gen/apple/ci_scripts/
git commit -m "Add Xcode Cloud build scripts"
git push origin main
```

## Resources

- Full setup guide: `/docs/xcode-cloud-setup.md`
- Tauri iOS guide: https://tauri.app/v1/guides/building/ios
- Xcode Cloud docs: https://developer.apple.com/xcode-cloud/

---

**Status:** Ready for first build
**Last Updated:** 2025-10-18
