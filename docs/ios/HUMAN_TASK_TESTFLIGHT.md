# Manual Task: TestFlight Beta Testing

**Agent:** ios-deployment-manager
**Time Required:** 30-60 minutes (including Apple processing time)
**Prerequisite:** Apple Developer Program membership, iCloud setup complete
**Status:** REQUIRED for testing file associations and real-world usage

---

## Overview

TestFlight allows you to distribute beta builds of Unicel iOS to testers before App Store submission. This is CRITICAL for testing file associations because:

- File associations only work in production builds (not debug/simulator builds)
- You can test opening .usheet files from Messages, Mail, Files app
- Real-world testing on multiple devices and iOS versions
- Collect crash reports and feedback

**Why TestFlight (not just local device testing):**
- Production signing required for file associations
- Simulates App Store environment
- Proper entitlements and capabilities enabled
- Can invite up to 10,000 external testers

---

## Prerequisites

Before starting, ensure you have:
- [ ] Apple Developer Program membership (active)
- [ ] iCloud container setup complete (see `HUMAN_TASK_ICLOUD_SETUP.md`)
- [ ] Xcode installed with development team configured
- [ ] App Store Connect access (same Apple ID as Developer account)

---

## Part 1: Create App in App Store Connect

**Estimated time:** 10 minutes

### 1.1 Log in to App Store Connect

1. Open https://appstoreconnect.apple.com
2. Sign in with Apple ID used for Developer Program
3. Click "My Apps"

### 1.2 Create New App

1. Click "+" button (top left, next to "Apps")
2. Select "New App"

**Fill in details:**

**Platforms:**
- [x] iOS
- [ ] macOS (add later if needed)

**Name:**
```
Unicel Viewer
```
(This is the public-facing App Store name - can be changed later)

**Primary Language:**
```
English (U.S.)
```

**Bundle ID:**
- Select from dropdown: `com.unicel.app` (should appear if iCloud setup is complete)
- If not in dropdown, click "Register a new bundle ID" link
  - Prefix: Your Team ID (auto-filled)
  - Bundle ID: `com.unicel.app`
  - Description: `Unicel Unit-Aware Spreadsheet`

**SKU:**
```
com.unicel.app.ios
```
(Internal identifier - never shown to users, cannot be changed)

**User Access:**
- Select "Full Access" (allows all team members to access)
- Or: "Limited Access" if you want to restrict

Click "Create"

### 1.3 Initial App Information

After creating, you'll see the app page. Don't worry about filling everything now - we'll complete this before App Store submission.

**Required now (minimal):**
1. Click "App Information" in left sidebar
2. Verify:
   - Bundle ID: `com.unicel.app`
   - SKU: `com.unicel.app.ios`
3. Select "Category":
   - Primary: Productivity
   - Secondary: Business (optional)
4. Click "Save"

---

## Part 2: Create Version in App Store Connect

**Estimated time:** 5 minutes

### 2.1 Start New Version

1. In App Store Connect, click your app "Unicel Viewer"
2. Click "+" next to "iOS App" in left sidebar
3. Select version number: `0.5.1` (matches tauri.conf.json)
4. Click "Create"

### 2.2 Minimal Version Information

You'll need to provide minimal information for TestFlight (full details needed for App Store submission):

**What's New in This Version:**
```
Initial TestFlight beta release.

Testing:
- .usheet file associations
- iCloud Drive integration
- Read-only spreadsheet viewing
- Unit display toggle (Metric/Imperial)
```

**Promotional Text (optional for TestFlight):**
Leave blank for now

**Description (required for App Store, optional for TestFlight):**
```
Unicel Viewer is a read-only spreadsheet viewer for .usheet files with built-in unit intelligence.

This TestFlight beta allows testing of:
- File opening from Messages/Mail/Files
- iCloud Drive sync
- Basic viewing functionality

Full editing features coming in future releases.
```

**Keywords (optional for TestFlight):**
Leave blank for now

**Support URL (required):**
```
https://github.com/jacksodj/unicel
```

**Marketing URL (optional):**
Leave blank

Click "Save" (top right)

### 2.3 Build Information

1. Scroll to "Build" section
2. You'll see "No builds are available"
3. This is expected - we'll upload build next

**Leave this page open - we'll return after uploading build**

---

## Part 3: Build Release IPA

**Estimated time:** 10-15 minutes

### 3.1 Update Version Numbers

First, ensure version consistency:

**Check tauri.conf.json:**
```bash
cd /Users/dennisjackson/Code/unicel
cat src-tauri/tauri.conf.json | grep version
```

Should show:
```json
"version": "0.5.1",
```

**If different, update to match App Store Connect version (0.5.1)**

### 3.2 Clean Previous Builds

```bash
cd /Users/dennisjackson/Code/unicel

# Clean Rust build artifacts
cargo clean

# Clean iOS build artifacts
rm -rf src-tauri/gen/apple/build
rm -rf src-tauri/gen/apple/DerivedData

# Clean npm artifacts
rm -rf dist
```

### 3.3 Build Release IPA

**Option A: Using Xcode (RECOMMENDED)**

1. Open Xcode project:
```bash
open src-tauri/gen/apple/unicel.xcodeproj
```

2. In Xcode:
   - Select "Any iOS Device (arm64)" from device dropdown (top toolbar)
   - Product > Scheme > Edit Scheme
   - Change "Run" > Build Configuration to "Release"
   - Click "Close"

3. Create archive:
   - Product > Archive
   - Wait for build to complete (5-10 minutes)
   - Archive Organizer window opens automatically

4. Verify archive:
   - Should see "unicel iOS" with version 0.5.1
   - Date and build number shown
   - Click "Distribute App"

**Option B: Using Tauri CLI**

```bash
cd /Users/dennisjackson/Code/unicel

# Build release IPA
npm run tauri ios build --release

# This runs:
# 1. Frontend build (npm run build)
# 2. Rust compilation for iOS (cargo build --release --target aarch64-apple-ios)
# 3. Xcode archive and export
```

**Expected output:**
```
Building frontend...
Compiling Rust for iOS...
Creating Xcode archive...
Exporting IPA...

Build complete:
- IPA: src-tauri/gen/apple/build/Payload/Unicel.ipa
- Size: ~50-100 MB
```

### 3.4 Verify Build

```bash
# Check IPA exists
ls -lh /Users/dennisjackson/Code/unicel/src-tauri/gen/apple/build/Payload/Unicel.ipa

# Check code signing
codesign -dv --verbose=4 /Users/dennisjackson/Code/unicel/src-tauri/gen/apple/build/Payload/Unicel.app

# Should show:
# Authority=Apple Distribution: [Your Name] ([Team ID])
# or
# Authority=iPhone Distribution: [Your Name]
```

**Common issues:**
- "No signing certificates found": Configure in Xcode > Signing & Capabilities
- "No provisioning profile": Download from developer.apple.com/account
- "Build failed": Check build log in Xcode for specific errors

---

## Part 4: Upload to App Store Connect

**Estimated time:** 10-20 minutes (includes Apple processing time)

### 4.1 Upload Using Xcode (RECOMMENDED)

**If you used Xcode to build (Option A):**

1. Archive Organizer should still be open (or: Window > Organizer > Archives)
2. Select your archive (unicel iOS 0.5.1)
3. Click "Distribute App"
4. Select "App Store Connect"
5. Click "Next"
6. Select "Upload"
7. Click "Next"
8. **Distribution options:**
   - App Thinning: All compatible device variants
   - Rebuild from Bitcode: No (Tauri doesn't use bitcode)
   - Include symbols: Yes (RECOMMENDED - for crash reports)
9. Click "Next"
10. **Automatic signing:**
    - Select your distribution certificate
    - Xcode auto-selects provisioning profile
11. Click "Upload"
12. Review summary
13. Click "Upload"

**Wait for upload (5-10 minutes depending on connection speed)**

You'll see:
```
Uploading Unicel.ipa...
Upload progress: [=====>    ] 45%
```

When complete:
```
Upload Successful
Unicel.ipa has been uploaded to App Store Connect.
```

### 4.2 Upload Using Transporter App (Alternative)

1. Download "Transporter" from Mac App Store
2. Open Transporter
3. Sign in with Apple ID
4. Drag and drop IPA file into Transporter window
   - File: `/Users/dennisjackson/Code/unicel/src-tauri/gen/apple/build/Payload/Unicel.ipa`
5. Click "Deliver"
6. Wait for upload and validation

### 4.3 Upload Using Command Line (Alternative)

**First-time setup:**
1. Generate app-specific password:
   - Go to https://appleid.apple.com/account/manage
   - Sign In
   - Security > App-Specific Passwords
   - Generate new password
   - Name: "Xcode Upload"
   - Save password securely

**Upload command:**
```bash
cd /Users/dennisjackson/Code/unicel

xcrun altool --upload-app \
  -f src-tauri/gen/apple/build/Payload/Unicel.ipa \
  -t ios \
  -u your-apple-id@example.com \
  -p "xxxx-xxxx-xxxx-xxxx"  # App-specific password

# Or store password in keychain:
xcrun altool --upload-app \
  -f src-tauri/gen/apple/build/Payload/Unicel.ipa \
  -t ios \
  -u your-apple-id@example.com \
  -p "@keychain:Xcode Upload"
```

**Expected output:**
```
Uploading Unicel.ipa to App Store Connect...
[                    ] 0%
[======>             ] 30%
[============>       ] 60%
[===================>] 95%
[====================] 100%

Upload successful. Build is being processed.
```

---

## Part 5: Wait for Processing

**Estimated time:** 5-15 minutes

### 5.1 Monitor Processing

1. In App Store Connect, go to your app
2. Click "TestFlight" tab (top navigation)
3. Click "iOS" in left sidebar under "Builds"
4. You'll see your build with status: "Processing"

**Status progression:**
1. **Processing** (5-15 minutes) - Apple is scanning for malware, checking entitlements
2. **Ready to Submit** - Build is ready, but not yet available to testers
3. **Waiting for Review** (external beta only)
4. **Testing** - Available to testers

**While waiting, you can:**
- Add internal testers (see below)
- Create beta groups
- Prepare "What to Test" notes

### 5.2 Check Email

You'll receive emails from App Store Connect:
1. "Your build has been uploaded" - Immediate
2. "Your build is processing" - Within 1 minute
3. "Your build is ready for testing" - After 5-15 minutes
4. OR: "Your build has processing issues" - If errors found

**Common processing issues:**
- Missing entitlements (usually auto-fixed)
- Code signing issues (re-export with correct certificate)
- Binary rejected (check email for details)

---

## Part 6: Configure TestFlight Beta

**Estimated time:** 10 minutes

### 6.1 Add Build to Version

1. Once build shows "Ready to Submit" (NOT "Processing")
2. Go to App Store Connect > Your App > TestFlight tab
3. Click build number (e.g., "0.5.1 (1)")
4. Under "Test Information" add:

**What to Test:**
```
CRITICAL: Testing file associations!

Please test:
1. Opening .usheet files from Messages/Mail attachments
2. Files should open directly in Unicel app
3. iCloud Drive file sync
4. Display toggle (Metric <-> Imperial)
5. Basic navigation between sheets

Known limitations:
- Read-only mode (editing not implemented)
- Performance may vary on older devices

Please report:
- Any crashes or freezes
- Files that fail to open
- UI layout issues on your device
```

**Test Details:**
- Leave other fields default
- Click "Save"

### 6.2 Export Compliance

1. Scroll to "Export Compliance Information"
2. Question: "Does your app use encryption?"
   - Select "No" (if app doesn't use custom encryption beyond HTTPS)
   - OR: "Yes" and answer follow-up questions about encryption type
3. Click "Save"

**For most apps:**
- Standard HTTPS: Answer "No"
- Custom encryption: Answer "Yes" and provide details

### 6.3 Create Internal Testing Group

1. Still in TestFlight tab
2. Left sidebar > "Internal Testing" (under "Internal Group")
3. Should see default "App Store Connect Users" group
4. Or click "+" to create new group:
   - Group Name: "Core Team"
   - Add testers (see next step)

### 6.4 Add Internal Testers

**Internal testers:**
- Must have App Store Connect account role (Admin, Developer, Marketing, Sales)
- Up to 100 internal testers allowed
- No review required by Apple
- Instant access once build is ready

**Add tester:**
1. TestFlight tab > Internal Testing
2. Click "+" next to Testers
3. Enter email addresses (must match App Store Connect user)
4. Select users from list
5. Click "Add"
6. Testers receive email invitation immediately

**If tester not in list:**
1. Go to App Store Connect > Users and Access
2. Click "+"
3. Add user:
   - First Name, Last Name, Email
   - Role: "App Manager" or "Developer" (for testing)
   - Access to your app
4. User receives invitation to join App Store Connect
5. Once accepted, they appear in TestFlight tester list

### 6.5 Create External Testing Group (Optional)

**External testers:**
- Anyone with email address (no App Store Connect account needed)
- Up to 10,000 external testers allowed
- Requires Apple review (1-2 days)
- Good for wider beta testing

**To add external testers:**
1. TestFlight tab > External Testing (in left sidebar)
2. Click "+" to create group
3. Group Name: "Public Beta"
4. Click "Create"
5. Click "+ Add Testers"
6. Enter email addresses (one per line)
7. Click "Add"
8. Add build to group:
   - Click on group name
   - Click "+ Add Build"
   - Select build (0.5.1)
   - Add "What to Test" notes (same as above)
   - Click "Submit for Review"
9. Wait for Apple review (1-2 days)

**Skip external testing for now - use internal testers first**

---

## Part 7: Install TestFlight on Devices

**Estimated time:** 5 minutes per device

### 7.1 Tester Receives Email

Each invited tester receives email:
```
Subject: You're invited to test Unicel Viewer

[Your Name] has invited you to test Unicel Viewer using TestFlight.

Start Testing:
- Install TestFlight from the App Store
- Tap the "Start Testing" button
- Follow instructions to install Unicel Viewer
```

### 7.2 Install TestFlight App

1. On iOS device, open App Store
2. Search "TestFlight"
3. Install TestFlight (free, by Apple)
4. Open TestFlight app
5. Sign in with Apple ID (same email used for invitation)

### 7.3 Accept Invitation

**Method A: From email**
1. Open invitation email on iOS device
2. Tap "View in TestFlight" button
3. TestFlight app opens
4. Tap "Accept" to accept invitation
5. Tap "Install" to install Unicel Viewer

**Method B: From TestFlight app**
1. Open TestFlight app
2. Look under "Apps Available to Test"
3. See "Unicel Viewer"
4. Tap on it
5. Tap "Install"

**Method C: Using redemption code (if provided)**
1. TestFlight app > Redeem
2. Enter code (you generate in App Store Connect)
3. Tap "Redeem"
4. Tap "Install"

### 7.4 Launch App

1. After installation, TestFlight shows "INSTALLED"
2. Tap "OPEN" to launch
3. Or: Find "Unicel Viewer" on home screen (has orange "BETA" badge)
4. Launch from home screen

**First launch:**
- App may show splash screen
- May request permissions (Files access, etc.)
- Accept required permissions
- App should load to main screen

---

## Part 8: Test File Associations

**Estimated time:** 10 minutes
**THIS IS THE CRITICAL PART - File associations only work in TestFlight/production builds**

### 8.1 Prepare Test Files

**On your Mac:**

1. Create test .usheet files:
```bash
cd /Users/dennisjackson/Code/unicel

# Copy example files
cp examples/construction-project.usheet ~/Desktop/test-construction.usheet
cp examples/aws-cost-analysis.usheet ~/Desktop/test-aws.usheet
```

2. Upload to iCloud Drive:
   - Open Finder
   - Navigate to iCloud Drive
   - Create folder: "Unicel Test Files"
   - Drag test files into folder

### 8.2 Test: Open from iCloud Drive (Files App)

**On iOS device with TestFlight build installed:**

1. Open Files app (built-in iOS app)
2. Navigate to iCloud Drive > Unicel Test Files
3. Tap on `test-construction.usheet`

**Expected behavior:**
- File should open directly in Unicel Viewer app
- No "How do you want to open this file?" dialog
- Unicel app launches and displays spreadsheet

**If file association works:**
- File opens automatically
- Unicel app shows grid with data
- You can navigate sheets

**If file association DOESN'T work:**
- iOS asks "How do you want to open this file?"
- Or: "No app available to open this file"
- **SOLUTION:** See Troubleshooting section below

### 8.3 Test: Open from Messages

**Send test file via Messages:**

1. On Mac: Open Messages app
2. Start conversation with yourself or test contact
3. Drag `test-construction.usheet` into message
4. Send message

**On iOS device:**

1. Open Messages app
2. Open conversation with test message
3. Tap on .usheet file attachment
4. Should see preview or download
5. Tap file to open

**Expected behavior:**
- File downloads
- Tapping file opens Unicel Viewer
- File displays correctly

### 8.4 Test: Open from Mail

**Send test file via email:**

1. Open Mail app (Mac or web)
2. Compose new email to yourself
3. Attach `test-construction.usheet`
4. Send email

**On iOS device:**

1. Open Mail app
2. Open email with attachment
3. Tap attachment to preview
4. Tap share button (square with arrow)
5. Select "Unicel Viewer" from app list
6. OR: Tap-and-hold attachment > Open in Unicel Viewer

**Expected behavior:**
- Attachment shows with Unicel icon (if file association working)
- Opens in Unicel Viewer
- File displays correctly

### 8.5 Test: Open from Safari/Website

**Upload file to cloud storage (Dropbox, Google Drive, etc.):**

1. Upload `test-construction.usheet` to Dropbox/Drive
2. Generate share link
3. Open link in Safari on iOS device
4. Download file
5. Tap to open

**Expected behavior:**
- Downloads folder shows file
- Tapping file opens Unicel Viewer
- File displays correctly

---

## Part 9: Collect Feedback

### 9.1 View Crash Reports

**In App Store Connect:**
1. Your App > TestFlight tab
2. Select build (0.5.1)
3. Click "Crash Reports" (if any crashes occurred)
4. View crash logs
5. Filter by device type, iOS version

**In Xcode:**
1. Window > Organizer
2. Select "Crashes" tab
3. Select your app
4. View symbolicated crash logs (if dSYM uploaded)

### 9.2 Collect Tester Feedback

**Testers can provide feedback:**

**Method A: In TestFlight app**
1. TestFlight app > Unicel Viewer
2. Tap "Send Beta Feedback"
3. Screenshot attached automatically (if app was open)
4. Type feedback
5. Submit

**Method B: Email**
- Testers reply to invitation email
- Or email directly to your support address

**You receive feedback:**
1. App Store Connect > Your App > TestFlight
2. Select build
3. Click "Feedback" tab
4. View all feedback with screenshots

### 9.3 Monitor Metrics

**In App Store Connect > TestFlight:**

**Testers tab:**
- Number of invitations sent
- Number of testers accepted
- Number of active testers

**Builds tab:**
- Installs per build
- Sessions (app opens)
- Crashes per session
- Average session length

**Export data:**
- Click "Export" to download CSV
- Analyze in spreadsheet

---

## Part 10: Iterate and Update

### 10.1 Upload New Build

When you fix issues or add features:

1. Update version or build number:
   - Increment version: `0.5.1` → `0.5.2` (for new features)
   - OR: Keep version, increment build: `0.5.1 (1)` → `0.5.1 (2)` (for bug fixes)

2. Rebuild IPA (same as Part 3)

3. Upload to App Store Connect (same as Part 4)

4. Wait for processing (same as Part 5)

5. In TestFlight tab:
   - New build appears automatically
   - Existing testers get notification: "New build available"
   - Update "What to Test" notes

6. Testers update:
   - Open TestFlight app
   - Tap "Update" next to Unicel Viewer
   - OR: Enable "Automatic Updates" for auto-install

### 10.2 Build Number Management

**Version vs Build Number:**
- **Marketing Version** (e.g., 0.5.1): User-facing version
- **Build Number** (e.g., 1, 2, 3): Internal build identifier

**In tauri.conf.json:**
```json
{
  "version": "0.5.1"  // Marketing version AND build number
}
```

**Tauri uses same number for both. To increment:**
```json
{
  "version": "0.5.2"  // Next version
}
```

**Or use Git commit as build number:**
```bash
# Get commit count
git rev-list --count HEAD
# Example: 147

# Update tauri.conf.json:
{
  "version": "0.5.1.147"  // Version.build
}
```

---

## Troubleshooting

### Build Upload Failed: "Invalid Bundle"

**Cause:** Code signing, entitlements, or Info.plist issue

**Solution:**
1. Open Xcode > Organizer > Archives
2. Right-click archive > Show in Finder
3. Right-click .xcarchive > Show Package Contents
4. Check Products/Applications/Unicel.app/Info.plist
5. Verify:
   - CFBundleIdentifier: `com.unicel.app`
   - CFBundleDocumentTypes present
   - UTExportedTypeDeclarations present
6. Rebuild with Xcode (not CLI)

### File Associations Not Working

**Symptoms:**
- Tapping .usheet file shows "No app to open this file"
- OR: iOS asks which app to use
- Unicel not in list of apps

**Causes:**
1. **File type declaration missing from Info.plist**
2. **Not using production build** (simulator/debug builds don't register file types)
3. **App not installed via TestFlight/App Store**

**Solutions:**

**Verify Info.plist:**
```bash
cd /Users/dennisjackson/Code/unicel
cat src-tauri/gen/apple/unicel_iOS/Info.plist | grep -A 20 CFBundleDocumentTypes
```

Should show `com.unicel.usheet` in LSItemContentTypes

**Verify you're using TestFlight build:**
- App has orange "BETA" badge on home screen
- TestFlight app lists it as "INSTALLED"
- Opening from Files app should work

**Force re-registration:**
1. Delete Unicel app from device
2. Restart device
3. Reinstall from TestFlight
4. Try opening .usheet file again

**Check file UTI:**
```bash
# On Mac, check file UTI
mdls ~/Desktop/test-construction.usheet | grep kMDItemContentType

# Should show: com.unicel.usheet
# If shows "dyn.xxx" or "public.data", file type not recognized
```

### Build Processing Stuck

**Symptom:** Build shows "Processing" for >30 minutes

**Solution:**
1. Refresh page (sometimes UI doesn't update)
2. Check email for processing failure notice
3. If truly stuck:
   - Contact Apple Developer Support
   - Or: Upload new build with incremented version

### Tester Can't Accept Invitation

**Symptoms:**
- "Invitation expired" error
- "You're not authorized" error

**Solutions:**

**If invitation expired:**
1. App Store Connect > TestFlight > Testers
2. Find tester
3. Click "..." > Resend Invitation

**If not authorized:**
1. Verify tester email matches Apple ID
2. For internal testers: Ensure they have App Store Connect access
3. For external testers: Check they haven't exceeded 100 TestFlight apps limit

**If tester doesn't receive email:**
1. Check spam folder
2. Use public link instead:
   - App Store Connect > TestFlight > Public Link
   - Enable public link
   - Share link directly

### Crash Reports Not Appearing

**Cause:** dSYM (debug symbols) not uploaded

**Solution:**
1. Find dSYM file:
```bash
cd /Users/dennisjackson/Code/unicel
ls src-tauri/gen/apple/build/*.dSYM
```

2. Upload to App Store Connect:
   - Xcode > Window > Organizer
   - Select Archives
   - Select your archive
   - Click "Download Debug Symbols" (if missing)
   - OR: Upload manually in App Store Connect > Build > Activity

3. Wait 24-48 hours for symbolication

### "Missing Compliance" Warning

**Symptom:** Build shows warning about export compliance

**Solution:**
1. TestFlight tab > Select build
2. Scroll to "Export Compliance Information"
3. Answer encryption question
4. Save

**For Unicel (no custom encryption):**
- Uses encryption: No
- OR: Uses encryption: Yes > Standard encryption only (HTTPS)

---

## Success Checklist

After completing this guide, verify:

- [ ] App created in App Store Connect
- [ ] Version 0.5.1 created
- [ ] Release IPA built successfully
- [ ] IPA uploaded to App Store Connect
- [ ] Build shows "Ready to Test" status
- [ ] Internal testers added and invited
- [ ] TestFlight installed on test device
- [ ] Unicel Viewer installed via TestFlight
- [ ] App launches successfully
- [ ] .usheet files open from Files app
- [ ] .usheet files open from Messages
- [ ] .usheet files open from Mail
- [ ] File associations work without prompts
- [ ] Crash reports visible (if any crashes)
- [ ] Feedback mechanism tested

---

## Next Steps

Once TestFlight is working:
1. Continue testing with beta users
2. Gather feedback and fix critical bugs
3. Upload updated builds as needed
4. Prepare for App Store submission:
   - Generate screenshots for all device sizes
   - Write App Store description
   - Create App Store preview video (optional)
   - Add privacy policy URL
5. See `HUMAN_TASK_APP_STORE_SUBMISSION.md` (coming soon)

---

## Reference Links

**App Store Connect:**
- Dashboard: https://appstoreconnect.apple.com
- TestFlight guide: https://developer.apple.com/testflight/
- App Store Connect help: https://help.apple.com/app-store-connect/

**TestFlight:**
- Tester guide: https://testflight.apple.com/
- Beta testing guide: https://developer.apple.com/testflight/testers/

**Documentation:**
- Upload builds: https://help.apple.com/xcode/mac/current/#/dev442d7f2ca
- Export compliance: https://help.apple.com/app-store-connect/#/dev88f5c7bf9

---

## File Locations

**Build artifacts:**
- IPA: `/Users/dennisjackson/Code/unicel/src-tauri/gen/apple/build/Payload/Unicel.ipa`
- Archive: `~/Library/Developer/Xcode/Archives/`
- dSYM: `/Users/dennisjackson/Code/unicel/src-tauri/gen/apple/build/*.dSYM`

**Configuration:**
- Version: `/Users/dennisjackson/Code/unicel/src-tauri/tauri.conf.json`
- Xcode project: `/Users/dennisjackson/Code/unicel/src-tauri/gen/apple/unicel.xcodeproj`

---

**Last updated:** 2025-10-18
**Agent:** ios-deployment-manager
