# Xcode Cloud Setup Guide for Unicel iOS

This guide explains how to configure Xcode Cloud for automated iOS builds and TestFlight deployment of the Unicel iOS app.

## Overview

**Xcode Cloud** is Apple's CI/CD service that automatically builds, tests, and distributes your iOS app. It's integrated directly into App Store Connect and Xcode.

**Why we need custom scripts:**
- Unicel iOS is built with **Tauri v2** (Rust + Vite/React)
- Xcode Cloud's default environment doesn't include Node.js or Rust
- The `ci_post_clone.sh` script installs these dependencies before building

## Prerequisites

1. **Apple Developer Account** with Admin or App Manager role
2. **App created in App Store Connect** (Bundle ID: `com.unicel.app`)
3. **GitHub repository access** (or other Git provider)
4. **Xcode 15+** installed locally for initial setup

## File Structure

The Xcode Cloud scripts are located in:
```
src-tauri/gen/apple/ci_scripts/
└── ci_post_clone.sh          # Installs Node.js, Rust, and dependencies
```

**What the script does:**
1. Installs Node.js 20 (LTS) via Homebrew
2. Installs npm dependencies (`npm ci`)
3. Installs Rust toolchain
4. Adds iOS compilation targets (`aarch64-apple-ios`)
5. Builds frontend assets (`npm run build`)
6. Verifies all tools are installed correctly

## Initial Setup in App Store Connect

### 1. Enable Xcode Cloud

1. Go to [App Store Connect](https://appstoreconnect.apple.com)
2. Navigate to **Apps** → **Unicel**
3. Click on **Xcode Cloud** tab (or **TestFlight** → **Xcode Cloud**)
4. Click **Get Started** or **Enable Xcode Cloud**

### 2. Connect Your Repository

1. Click **Connect Repository**
2. Select your Git provider:
   - **GitHub** (recommended)
   - GitLab
   - Bitbucket
   - Or use App Store Connect's Git hosting
3. Authorize Xcode Cloud to access your repository
4. Select the `jacksodj/unicel` repository
5. Grant necessary permissions (read repository, read pull requests)

### 3. Create a Workflow

Xcode Cloud workflows define when and how to build your app.

#### Basic Workflow Configuration

1. Click **Create Workflow** or **Add Workflow**
2. Configure the workflow:

   **General:**
   - **Name**: `Release Build (TestFlight)`
   - **Description**: Builds release IPA and uploads to TestFlight

   **Environment:**
   - **Xcode Version**: Latest Release (or specify 15.x+)
   - **macOS Version**: Latest (Ventura or Sonoma)
   - **Clean Build**: Disabled (for faster builds)

   **Start Conditions (Triggers):**
   - **Branch**: `main`
   - **Start Condition**: On every push (or manual start)
   - Optional: Only on tags matching `v*` (e.g., `v1.0.0`)

   **Actions:**
   1. **Archive - iOS**
      - **Scheme**: `unicel_iOS`
      - **Platform**: iOS
      - **Configuration**: Release

   **Post-Actions:**
   1. **TestFlight (External Testing)** or **TestFlight (Internal Testing)**
      - Automatically uploads to TestFlight after successful build
      - Select testing group (e.g., "Internal Testers")

   **Environment Variables:**
   - None required (scripts handle everything)

3. Click **Save** or **Create Workflow**

### 4. Verify Custom Scripts Are Detected

Xcode Cloud automatically detects scripts in `ci_scripts/`:
- `ci_post_clone.sh` - Runs after cloning repository
- `ci_pre_xcodebuild.sh` - Runs before xcodebuild (optional, not used)
- `ci_post_xcodebuild.sh` - Runs after xcodebuild (optional, not used)

You should see a confirmation that custom scripts were found.

## Triggering a Build

### Manual Trigger

1. Go to **Xcode Cloud** in App Store Connect
2. Select your workflow
3. Click **Start Build**
4. Select branch/commit
5. Click **Start**

### Automatic Triggers

Builds start automatically based on your workflow configuration:
- On every push to `main`
- On pull requests (if configured)
- On tags matching pattern (e.g., `v*`)

## Monitoring Builds

### In App Store Connect

1. Go to **Xcode Cloud** tab
2. Select your workflow
3. View build list with statuses:
   - 🔵 **Running** - Build in progress
   - ✅ **Succeeded** - Build completed successfully
   - ❌ **Failed** - Build failed (click to see logs)
   - ⏸️ **Canceled** - Manually canceled

4. Click on a build to see:
   - **Build Timeline** - Each step with duration
   - **Logs** - Full console output
   - **Artifacts** - IPA files, dSYMs, logs
   - **TestFlight** - Upload status

### In Xcode (macOS)

1. Open Xcode
2. Go to **Report Navigator** (⌘9)
3. Select **Cloud** tab
4. View builds from all workflows

## Build Timeline

Typical Xcode Cloud build takes **15-25 minutes**:

1. **Clone Repository** (1-2 min)
   - Checks out code from Git

2. **Post-Clone Script** (5-8 min)
   - Installs Node.js (~2 min)
   - Installs npm dependencies (~2 min)
   - Installs Rust (~2 min)
   - Adds iOS targets (~30 sec)
   - Builds frontend (~1 min)

3. **Xcode Build** (8-12 min)
   - Pre-build script runs `tauri ios xcode-script`
   - Compiles Rust to ARM64 library (~5 min)
   - Compiles Swift wrapper (~1 min)
   - Links binary (~1 min)
   - Creates IPA (~1 min)

4. **Archive & Export** (1-2 min)
   - Signs IPA with distribution certificate
   - Creates dSYM for crash reporting

5. **TestFlight Upload** (2-3 min)
   - Uploads IPA to App Store Connect
   - Starts TestFlight processing

6. **Post-Build** (1 min)
   - Cleanup and artifact storage

## Troubleshooting

### Common Issues

#### 1. "npm: command not found"

**Symptom:**
```
/Volumes/workspace/.../Script-XXX.sh: line 2: npm: command not found
```

**Solution:**
- Ensure `ci_post_clone.sh` exists in `src-tauri/gen/apple/ci_scripts/`
- Ensure script is executable (`chmod +x`)
- Verify script is committed to Git
- Check logs to see if post-clone script ran

#### 2. "rustc: command not found"

**Symptom:**
```
error: cannot find `rustc` in PATH
```

**Solution:**
- Ensure `ci_post_clone.sh` installs Rust
- Check if Rust installation succeeded in logs
- Verify `$HOME/.cargo/env` is sourced

#### 3. "Frontend dist/ directory not found"

**Symptom:**
```
Error: Frontend dist directory not found
```

**Solution:**
- Ensure `npm run build` runs in `ci_post_clone.sh`
- Check Vite build logs for errors
- Verify `dist/` directory exists after frontend build

#### 4. Build Times Out (>120 minutes)

**Symptom:**
```
Build exceeded maximum duration
```

**Solution:**
- Enable incremental builds (cache cargo dependencies)
- Reduce frontend bundle size
- Use `npm ci` instead of `npm install` for faster installs

#### 5. Code Signing Failed

**Symptom:**
```
Code signing failed: No valid signing identity found
```

**Solution:**
- Verify development team ID in `tauri.conf.json` (`Z3L3V842L2`)
- Check Apple Developer account has valid certificates
- Ensure provisioning profiles are up to date
- Try "Automatic" code signing in project settings

### Viewing Detailed Logs

1. Go to **Xcode Cloud** → Select build
2. Click **View Logs**
3. Expand each phase:
   - **Clone** - Git checkout
   - **Post-Clone** - Custom script output
   - **Build** - Xcode compilation
   - **Archive** - IPA creation
4. Download full logs: **Actions** → **Download Logs**

### Testing Scripts Locally

You can simulate the Xcode Cloud environment locally:

```bash
cd /path/to/unicel

# Run post-clone script
src-tauri/gen/apple/ci_scripts/ci_post_clone.sh

# Check if it succeeds
echo $?  # Should print 0

# Build iOS manually
npm run tauri:ios:build
```

## Environment Variables

Currently, **no custom environment variables are required**. The `ci_post_clone.sh` script handles all setup.

**Optional variables** (if needed in future):
- `NODE_VERSION` - Override Node.js version (default: 20)
- `RUST_BACKTRACE` - Set to `1` or `full` for debugging
- `CARGO_TERM_COLOR` - Set to `always` for colored output

To add environment variables:
1. Go to workflow settings
2. Scroll to **Environment Variables**
3. Click **Add Variable**
4. Enter name and value
5. Save

## Best Practices

### 1. Use Git Tags for Releases

Tag releases in Git to trigger builds:
```bash
git tag v1.0.0
git push origin v1.0.0
```

Configure workflow to only build on tags matching `v*`.

### 2. Separate Workflows for Different Purposes

Create multiple workflows:
- **Pull Request Builds** - Build on PRs (no TestFlight upload)
- **Release Builds** - Build on `main` (upload to TestFlight)
- **App Store Submissions** - Manual trigger (upload to App Store)

### 3. Enable Build Notifications

1. Go to **Account Settings** in App Store Connect
2. Enable **Xcode Cloud Notifications**
3. Get emails for build successes/failures

### 4. Monitor Build Minutes

Apple provides **25 hours/month free** for Xcode Cloud.
- Check usage: **App Store Connect** → **Xcode Cloud** → **Usage**
- Each build takes ~20 minutes (1.5-3 builds/hour free tier)

### 5. Cache Dependencies (Future Optimization)

To speed up builds, consider caching:
- Cargo dependencies (`~/.cargo/registry`, `~/.cargo/git`)
- npm dependencies (`node_modules/`, though `package-lock.json` helps)
- Frontend build artifacts

Xcode Cloud supports basic caching via incremental builds.

## TestFlight Integration

After successful build, IPA is uploaded to TestFlight:

1. **Processing** (5-15 minutes)
   - App Store Connect processes IPA
   - Generates download assets
   - Prepares for testing

2. **Ready for Testing**
   - Status changes to "Ready to Submit" or "Testing"
   - Internal testers can install via TestFlight app

3. **Add Test Information** (Required)
   - Go to **TestFlight** → Select build
   - Add "What to Test" notes
   - Describe new features, changes, or known issues

4. **Distribute to Testers**
   - **Internal Testing**: Up to 100 testers (no review required)
   - **External Testing**: Unlimited testers (requires App Review)

See [TestFlight Deployment Guide](./testflight-deployment.md) for details.

## Next Steps After Setup

1. **Trigger first build** - Push to `main` or start manually
2. **Monitor build logs** - Ensure `ci_post_clone.sh` succeeds
3. **Wait for TestFlight upload** - Takes ~20 minutes total
4. **Install on device** - Use TestFlight app
5. **Test core functionality**:
   - Open .usheet files
   - View spreadsheets
   - Toggle Metric ↔ Imperial
   - Navigate sheets
6. **Iterate** - Fix issues, push changes, auto-build

## Resources

**Apple Documentation:**
- [Xcode Cloud Overview](https://developer.apple.com/xcode-cloud/)
- [Xcode Cloud Workflows](https://developer.apple.com/documentation/xcode/xcode-cloud-workflow-reference)
- [Custom Build Scripts](https://developer.apple.com/documentation/xcode/writing-custom-build-scripts)
- [TestFlight Guide](https://developer.apple.com/testflight/)

**Tauri Documentation:**
- [Tauri iOS Guide](https://tauri.app/v1/guides/building/ios)
- [Tauri Configuration](https://tauri.app/v1/api/config)

**Project Files:**
- `src-tauri/tauri.conf.json` - Tauri configuration
- `src-tauri/gen/apple/project.yml` - Xcode project configuration
- `src-tauri/gen/apple/ci_scripts/ci_post_clone.sh` - Build script

## Troubleshooting Contacts

**For build issues:**
1. Check build logs in App Store Connect
2. Review this documentation
3. Test scripts locally
4. Check Tauri GitHub issues
5. Contact Apple Developer Support (if Xcode Cloud specific)

**For TestFlight issues:**
1. Check App Store Connect → TestFlight tab
2. Review Apple's TestFlight documentation
3. Contact Apple Developer Support

---

**Last Updated:** 2025-10-18
**Unicel Version:** 0.5.1
**Tauri Version:** 2.1.0
