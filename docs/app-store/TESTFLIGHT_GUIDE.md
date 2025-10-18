# TestFlight Deployment Guide

This guide covers uploading Unicel Viewer to TestFlight for beta testing.

## Prerequisites

Before starting, ensure you have:

1. **Apple Developer Account** ($99/year)
   - Enrolled at https://developer.apple.com/programs/
   - Account must be in good standing

2. **App Store Connect Access**
   - Log in at https://appstoreconnect.apple.com
   - Create app listing if not already done

3. **Certificates & Provisioning**
   - Distribution certificate installed in Keychain
   - App Store provisioning profile configured
   - Can be managed via Xcode or Apple Developer portal

4. **Xcode Installed**
   - Xcode 15.0 or later
   - Command line tools installed: `xcode-select --install`

5. **Tauri iOS Build**
   - iOS implementation complete (Phase 10)
   - App builds successfully in development mode

## Step 1: Create App in App Store Connect

If the app doesn't exist yet in App Store Connect:

1. Log in to https://appstoreconnect.apple.com
2. Click "My Apps"
3. Click the "+" button and select "New App"
4. Fill in the form:
   - **Platforms:** iOS
   - **Name:** Unicel
   - **Primary Language:** English (U.S.)
   - **Bundle ID:** com.unicel.app (must match tauri.conf.json)
   - **SKU:** unicel-ios-viewer (unique identifier)
   - **User Access:** Full Access
5. Click "Create"

## Step 2: Build Release IPA

### Option A: Using Xcode (Recommended)

1. **Open Xcode Project:**
   ```bash
   cd /Users/dennisjackson/Code/unicel
   open src-tauri/gen/apple/unicel.xcodeproj
   ```

2. **Select Release Scheme:**
   - In Xcode: Product > Scheme > Edit Scheme
   - Set Build Configuration to "Release"
   - Close scheme editor

3. **Select Generic iOS Device:**
   - In the toolbar, click the device selector
   - Choose "Any iOS Device (arm64)"

4. **Archive the App:**
   - Product > Archive
   - Wait for the build to complete (2-5 minutes)
   - The Organizer window will open automatically

5. **Verify Archive:**
   - Check the archive appears in the Organizer
   - Version should be 0.5.1
   - Bundle ID should be com.unicel.app

### Option B: Using Command Line

```bash
# Navigate to project
cd /Users/dennisjackson/Code/unicel

# Build release IPA using Tauri
npm run tauri ios build --release

# IPA will be at:
# src-tauri/gen/apple/build/arm64-apple-ios/release/bundle/ios/Unicel.ipa
```

**Note:** Command line builds may require additional configuration for code signing.

## Step 3: Distribute to TestFlight

### From Xcode Organizer:

1. **Distribute App:**
   - In the Organizer, with your archive selected
   - Click "Distribute App" button

2. **Select Distribution Method:**
   - Choose "App Store Connect"
   - Click "Next"

3. **Select Destination:**
   - Choose "Upload"
   - Click "Next"

4. **Distribution Options:**
   - **App Thinning:** All compatible device variants
   - **Rebuild from Bitcode:** Yes (if available)
   - **Include symbols:** Yes
   - **Manage Version and Build Number:** Automatically manage
   - Click "Next"

5. **Re-sign Options:**
   - **Automatically manage signing:** Recommended
   - Or select your Distribution certificate and provisioning profile
   - Click "Next"

6. **Review Archive:**
   - Verify app name, version, bundle ID
   - Click "Upload"

7. **Wait for Upload:**
   - Progress bar will show upload status
   - Takes 2-10 minutes depending on IPA size
   - Click "Done" when complete

### Using Command Line (Alternative):

If you prefer command line or Xcode upload fails:

1. **Generate App-Specific Password:**
   - Go to https://appleid.apple.com/account/manage
   - Sign in with your Apple ID
   - In Security section, click "Generate Password"
   - Label it "Unicel TestFlight Upload"
   - Save the password securely

2. **Upload Using altool:**
   ```bash
   xcrun altool --upload-app \
     -f src-tauri/gen/apple/build/arm64-apple-ios/release/bundle/ios/Unicel.ipa \
     -t ios \
     -u your.apple.id@email.com \
     -p app-specific-password
   ```

3. **Or use Transporter app:**
   - Download Transporter from Mac App Store
   - Open Transporter
   - Drag and drop IPA file
   - Sign in with Apple ID
   - Click "Deliver"

## Step 4: Wait for Processing

After upload:

1. **App Store Connect Processing:**
   - Log in to https://appstoreconnect.apple.com
   - Go to My Apps > Unicel
   - Click on "TestFlight" tab
   - You'll see "Processing" next to your build

2. **Processing Time:**
   - Usually 5-15 minutes
   - Can take up to 60 minutes in rare cases
   - You'll receive email when processing completes

3. **Check Status:**
   - Refresh the TestFlight page periodically
   - Status will change from "Processing" to "Ready to Submit" or "Testing"

## Step 5: Configure TestFlight Build

Once processing is complete:

1. **Add Test Information:**
   - In App Store Connect, click the build number
   - Under "Test Details", click "Add"
   - Fill in "What to Test":

   ```
   Welcome to Unicel Viewer v0.5.1!

   This is the initial iOS release. Please test:

   CORE FUNCTIONALITY:
   - Open the included example workbooks
   - Try AWS Cost Estimator and Construction Estimator examples
   - Verify all values display correctly

   UNIT CONVERSION:
   - Toggle between Metric and Imperial using the toggle button
   - Verify conversions are accurate (e.g., 100ft = 30.48m)
   - Check that different unit types convert properly

   NAVIGATION:
   - Swipe between sheets or use sheet tabs
   - Zoom and scroll the spreadsheet grid
   - Tap cells to view formulas and values

   FILE HANDLING:
   - Try opening .usheet files from Files app
   - Test with files from iCloud Drive (if available)
   - Verify error handling for invalid files

   KNOWN ISSUES:
   - [Add any known issues here]

   Please report bugs or feedback to: dennisjackson@unicel.app
   ```

2. **Export Compliance:**
   - Click "Provide Export Compliance Information"
   - Answer "No" to cryptography questions (see APP_STORE_METADATA.md)
   - Click "Start Internal Testing" when done

## Step 6: Add Internal Testers

Internal testers can start testing immediately (no review required):

1. **Internal Testing:**
   - In TestFlight tab, go to "Internal Testing" section
   - Click "App Store Connect Users" or create a new group
   - Click "+" to add testers

2. **Add Testers:**
   - Select up to 100 internal testers
   - These must be App Store Connect users with roles:
     - Admin
     - App Manager
     - Developer
     - Marketing
   - Click "Add"

3. **Testers Receive Invite:**
   - Invited testers receive email notification
   - They install TestFlight app from App Store
   - Accept invite and install Unicel Viewer
   - Can start testing immediately

## Step 7: Add External Testers (Optional)

External testing requires App Review approval:

1. **Create External Group:**
   - In TestFlight, go to "External Testing"
   - Click "+" to create new group
   - Name it (e.g., "Public Beta")
   - Enable "Public Link" if you want shareable link

2. **Add Build:**
   - Select your build (0.5.1)
   - Click "Add Build to Group"

3. **Add Testers:**
   - Enter email addresses (up to 10,000 testers)
   - Or share public link for automatic enrollment

4. **Submit for Review:**
   - Fill in "Test Information" (same as internal)
   - Add "App Review Information"
   - Click "Submit for Review"
   - Wait 24-48 hours for approval

5. **After Approval:**
   - External testers receive invite emails
   - They can install and test via TestFlight app

## Step 8: Monitor Feedback

### View Crash Reports:

1. **In App Store Connect:**
   - TestFlight > iOS Builds > [Your Build]
   - Click "Crashes" tab
   - View crash logs and stack traces

2. **In Xcode:**
   - Window > Organizer
   - Select "Crashes" tab
   - View detailed crash information

### Collect Feedback:

1. **TestFlight Feedback:**
   - Testers can submit feedback via TestFlight app
   - Screenshot feedback appears in App Store Connect
   - Check regularly for issues

2. **Email Feedback:**
   - Include support email in test notes
   - Respond to tester questions promptly

3. **Metrics:**
   - View install rates, session duration, crashes
   - TestFlight > [Build] > Metrics

## Step 9: Iterate and Update

To upload a new build:

1. **Increment Build Number:**
   - In tauri.conf.json, keep version at 0.5.1
   - Build number auto-increments or set manually
   - Or bump to 0.5.2 if significant changes

2. **Build and Upload:**
   - Repeat Steps 2-5
   - Add "What's Changed" notes for testers

3. **Replace Build:**
   - In TestFlight groups, add new build
   - Old build remains available or can be removed
   - Testers update to new build automatically

## Best Practices

### For Successful TestFlight Builds:

1. **Test Locally First:**
   - Always test on real iOS device before uploading
   - Verify core functionality works
   - Check for obvious crashes

2. **Version Management:**
   - Keep version numbers semantic (0.5.1, 0.5.2, etc.)
   - Use build numbers for minor iterations
   - Document changes between builds

3. **Clear Test Instructions:**
   - Tell testers exactly what to test
   - Highlight new features or fixes
   - List known issues to avoid duplicate reports

4. **Respond Quickly:**
   - Reply to feedback within 24-48 hours
   - Acknowledge bugs and give timelines
   - Thank testers for their time

5. **Collect Meaningful Data:**
   - Ask specific questions
   - Request screenshots when possible
   - Track recurring issues

### Common Issues:

1. **"Invalid Binary" Error:**
   - Check Bundle ID matches App Store Connect
   - Verify provisioning profile is correct
   - Ensure Info.plist has required keys

2. **Processing Takes Too Long:**
   - Usually resolves within 1 hour
   - If longer, contact Apple Support
   - Check for email from App Store Connect

3. **Missing Export Compliance:**
   - Must answer compliance questions
   - See APP_STORE_METADATA.md for answers
   - Can't skip this step

4. **Crashes on Launch:**
   - Check crash logs in Organizer
   - Test on multiple devices/iOS versions
   - Verify all resources are bundled

## Useful Commands

```bash
# Check code signing
codesign -dv --verbose=4 path/to/Unicel.app

# Verify IPA
unzip -l path/to/Unicel.ipa

# List available provisioning profiles
security find-identity -v -p codesigning

# Check build settings
xcodebuild -showBuildSettings -project src-tauri/gen/apple/unicel.xcodeproj
```

## Resources

- **App Store Connect:** https://appstoreconnect.apple.com
- **TestFlight App:** https://apps.apple.com/app/testflight/id899247664
- **TestFlight Help:** https://developer.apple.com/testflight/
- **Beta Testing Guide:** https://developer.apple.com/testflight/testers/

## Checklist

Before uploading to TestFlight:

- [ ] Apple Developer account active
- [ ] App created in App Store Connect
- [ ] Distribution certificate installed
- [ ] App builds successfully in Release mode
- [ ] Code signing configured correctly
- [ ] Version number is correct (0.5.1)
- [ ] Bundle ID matches (com.unicel.app)
- [ ] Example workbooks are bundled
- [ ] App tested on real iOS device
- [ ] Test notes prepared
- [ ] Export compliance information ready
- [ ] Internal testers identified
- [ ] Support email configured

After successful upload:

- [ ] Build processed successfully
- [ ] Export compliance completed
- [ ] Test information added
- [ ] Internal testers invited
- [ ] Crash reports monitored
- [ ] Feedback collected
- [ ] Issues tracked and prioritized

---

**Next Steps:** Once TestFlight testing is successful, proceed to APP_STORE_SUBMISSION_GUIDE.md for final App Store submission.
