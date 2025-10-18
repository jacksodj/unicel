# Manual Steps Required for App Store Submission

**For User Action:** These steps require an Apple Developer account and manual interaction with App Store Connect.

---

## Prerequisites

Before proceeding, ensure you have:

- [ ] **Apple Developer Account** ($99/year)
  - Sign up at: https://developer.apple.com/programs/
  - Status: Active, paid, in good standing

- [ ] **App Store Connect Access**
  - Log in at: https://appstoreconnect.apple.com
  - Verify you can access "My Apps"

- [ ] **Xcode Configured**
  - Xcode > Preferences > Accounts
  - Apple ID signed in
  - Team selected

- [ ] **Code Signing Certificates**
  - iOS Distribution certificate installed
  - App ID created: `com.unicel.app`
  - Provisioning profile: App Store

- [ ] **Screenshots Captured**
  - Run: `./scripts/capture_ios_screenshots.sh`
  - Location: `~/Desktop/unicel-screenshots/`

- [ ] **Release IPA Built**
  - Run: `./scripts/build_ios_release.sh`
  - Location: `src-tauri/gen/apple/build/Release-iphoneos/unicel.ipa`

---

## Step 1: Create App in App Store Connect

**Only needed if this is the first submission for this app.**

1. **Log in to App Store Connect**
   - Visit: https://appstoreconnect.apple.com
   - Click "My Apps"

2. **Click the "+" button**
   - Select "New App"

3. **Fill in App Information:**
   - **Platform:** iOS
   - **Name:** Unicel
   - **Primary Language:** English (U.S.)
   - **Bundle ID:** Select `com.unicel.app`
   - **SKU:** unicel-ios (unique identifier)
   - **User Access:** Full Access

4. **Click "Create"**

---

## Step 2: Configure App Information

Navigate to: My Apps > Unicel > App Information

### Basic Information

| Field | Value |
|-------|-------|
| **Name** | Unicel |
| **Subtitle** | Unit-Aware Spreadsheet Viewer |
| **Category** | Productivity (Primary) |
| **Secondary Category** | Business (Optional) |

### General Information

| Field | Value |
|-------|-------|
| **Bundle ID** | com.unicel.app |
| **SKU** | unicel-ios |
| **Apple ID** | (Auto-generated after creation) |

### Age Rating

Click "Edit" next to Age Rating and answer:

- All content questions: **None** or **No**
- Unrestricted Web Access: **No**
- Gambling and Contests: **No**

Result: **4+**

Click "Done"

### Copyright

- **Copyright:** 2025 Dennis Jackson

---

## Step 3: Configure Pricing and Availability

Navigate to: My Apps > Unicel > Pricing and Availability

### Price Schedule

- **Price:** Free (or select paid tier)
- **Availability:** All countries/regions
- **Pre-order:** No (not needed for v1)

### App Distribution

- **Available on the App Store:** Yes
- **Automatically distribute updates:** Yes (recommended)

Click "Save"

---

## Step 4: Upload Build to TestFlight

### Option A: Using Xcode (Recommended)

1. **Open Xcode**

2. **Open Organizer**
   - Window > Organizer (or Cmd+Shift+O)

3. **Add Archive**
   - If you have an .ipa file, you can drag it to the Archives list
   - Or build directly in Xcode: Product > Archive

4. **Distribute App**
   - Select your archive
   - Click "Distribute App"
   - Choose "App Store Connect"
   - Click "Upload"
   - Select team and signing options
   - Review content and click "Upload"

5. **Wait for Processing**
   - Processing takes 5-15 minutes
   - You'll receive email when complete

### Option B: Using Command Line

1. **Generate App-Specific Password**
   - Visit: https://appleid.apple.com/account/manage
   - Security > App-Specific Passwords
   - Generate new password
   - Save it securely

2. **Run Upload Command**

```bash
xcrun altool --upload-app \
  -f src-tauri/gen/apple/build/Release-iphoneos/unicel.ipa \
  -t ios \
  -u your@email.com \
  -p "xxxx-xxxx-xxxx-xxxx"
```

Replace:
- `your@email.com` - Your Apple ID email
- `xxxx-xxxx-xxxx-xxxx` - App-specific password

3. **Wait for Processing**
   - Command completes when upload finishes
   - Processing continues in App Store Connect

### Verify Upload

1. Go to App Store Connect
2. My Apps > Unicel > TestFlight tab
3. Check "iOS Builds" section
4. Build should appear (may take 5-15 minutes)
5. Status: "Processing" → "Testing" → "Ready to Submit"

---

## Step 5: Configure TestFlight (Optional but Recommended)

Navigate to: My Apps > Unicel > TestFlight

### Add Test Information

1. Click on your build (version 0.5.1)

2. **Test Details** section:
   - **What to Test:**
     ```
     Thank you for testing Unicel Viewer!

     Please test the following features:
     1. Opening example workbooks
     2. Toggling between Metric and Imperial units
     3. Navigating between multiple sheets
     4. Viewing formulas and values
     5. Scrolling and zooming the spreadsheet grid

     Report any issues to: support@unicel.app
     ```

   - **Test Notes:** (Optional)
     ```
     This is the initial iOS release. The app is a read-only
     viewer for .usheet files created with the desktop version.
     ```

3. Click "Save"

### Add Internal Testers (Optional)

1. Click "Internal Testing" in left sidebar
2. Click "+" to add group
3. Name: "Unicel Team"
4. Add testers by email (up to 100)
5. Click "Add"

Internal testers receive TestFlight invite immediately.

### Add External Testers (Optional)

**Note:** External beta requires Apple review (1-2 days).

1. Click "External Testing" in left sidebar
2. Click "+" to add group
3. Name: "Public Beta"
4. Add testers by email or public link
5. Submit for Beta Review
6. Wait for approval

---

## Step 6: Prepare App Store Listing

Navigate to: My Apps > Unicel > [Version 0.5.1]

### 1. App Store Information

**Promotional Text** (170 characters):
```
View unit-aware spreadsheets on your iPhone and iPad. Toggle between Metric and Imperial units instantly. Perfect for engineers and data professionals.
```

**Description** (4000 characters):
Copy from: `docs/app-store/APP_STORE_METADATA.md` (Description section)

**Keywords** (100 characters):
```
spreadsheet,units,calculator,viewer,engineering,metric,imperial,conversion,productivity,formula
```

**Support URL:**
```
https://github.com/jacksodj/unicel/blob/main/docs/app-store/SUPPORT.md
```

**Marketing URL** (Optional):
```
https://github.com/jacksodj/unicel
```

### 2. App Privacy

Click "Edit" next to App Privacy

**Privacy Policy URL:**
```
https://github.com/jacksodj/unicel/blob/main/docs/app-store/PRIVACY_POLICY.md
```

**Data Collection:**
- Do you collect data from this app? **No**

Click "Save"

### 3. App Previews and Screenshots

**Upload screenshots for each device size:**

#### iPhone 6.7" Display (Required)
- Source: `~/Desktop/unicel-screenshots/iphone-6.7-inch/`
- Upload 5 screenshots in order:
  1. `screenshot-01-home.png`
  2. `screenshot-02-grid-view.png`
  3. `screenshot-03-unit-conversion.png`
  4. `screenshot-04-multi-sheet.png`
  5. `screenshot-05-formula-detail.png`

**Drag to reorder if needed (first screenshot is most important)**

#### iPhone 6.5" Display (Required)
- Source: `~/Desktop/unicel-screenshots/iphone-6.5-inch/`
- Upload same 5 screenshots

#### iPad Pro 12.9" Display (Required)
- Source: `~/Desktop/unicel-screenshots/ipad-12.9-inch/`
- Upload same 5 screenshots

**Verify:** All screenshots display correctly with no distortion or black bars.

Click "Save"

### 4. General App Information

**App Icon:**
- This should auto-populate from your IPA
- If not, upload: `src-tauri/gen/apple/Assets.xcassets/AppIcon.appiconset/AppIcon-512@2x.png`

**Version** (Read-only):
- 0.5.1 (from IPA)

**Copyright:**
```
2025 Dennis Jackson
```

**Routing App Coverage File:**
- Not applicable (leave empty)

**Build:**
- Select build 0.5.1 from dropdown

Click "Save"

### 5. What's New in This Version

**Version 0.5.1 Release Notes:**
```
Welcome to Unicel Viewer for iOS!

This is the initial release of Unicel Viewer, bringing unit-aware spreadsheets to iPhone and iPad.

Features:
• Read-only viewer for .usheet files
• Toggle between Metric and Imperial display modes
• Multi-sheet navigation
• Formula and value display
• Support for 100+ units across all major domains
• Touch-optimized spreadsheet grid
• Works completely offline

Get the desktop version at github.com/jacksodj/unicel for full editing capabilities.

Thank you for being an early adopter! Please share your feedback.
```

Click "Save"

---

## Step 7: App Review Information

Navigate to: Version 0.5.1 > App Review Information

### Contact Information

**First Name:** Dennis

**Last Name:** Jackson

**Phone Number:** [Your phone number]

**Email:** dennisjackson@unicel.app

### Demo Account

- **Sign-in required:** No
- **Demo account:** Not needed

### Notes

**Notes for Reviewer:**
```
Thank you for reviewing Unicel Viewer!

WHAT THIS APP DOES:
Unicel Viewer is a read-only spreadsheet viewer for .usheet files. It's like a PDF reader, but for unit-aware spreadsheets created with the Unicel desktop application.

HOW TO TEST:
1. Launch the app
2. Sample workbooks are included - tap "Open Example"
3. Try the "AWS Cost Estimator" or "Construction Estimator" examples
4. Toggle between Metric and Imperial units using the toggle button
5. Navigate between sheets using the sheet tabs
6. Tap cells to view formulas and values

KEY FEATURES TO VERIFY:
• File opening works (from Files app or examples)
• Unit conversion toggle works smoothly
• Multi-sheet navigation works
• Spreadsheet grid is responsive and scrollable
• App works offline (no network required)

PRIVACY:
This app does not collect any user data. All file processing happens locally on the device. No network connections are made for data collection.

EXPORT COMPLIANCE:
The app does not use encryption beyond standard iOS data protection. All data processing is local.

If you have any questions, please contact dennisjackson@unicel.app
```

### Attachment (Optional)

You can attach a demo video or additional screenshots if needed.

Click "Save"

---

## Step 8: Export Compliance

Navigate to: Version 0.5.1 > Export Compliance

### Question: Does your app use encryption?

**Answer:** No

**Explanation:** The app uses only standard iOS data protection APIs. It does not implement custom cryptographic algorithms.

If prompted for self-classification:
- Select: "No" - The app doesn't use encryption

Click "Save"

---

## Step 9: Submit for Review

### Pre-Submission Checklist

Verify all items are complete:

- [ ] App Information configured
- [ ] Pricing set (Free or Paid)
- [ ] Build uploaded and processed
- [ ] Screenshots uploaded (all 3 sizes)
- [ ] App description complete
- [ ] Keywords entered
- [ ] Support URL added
- [ ] Privacy Policy URL added
- [ ] Age rating completed
- [ ] What's New text added
- [ ] App Review Information complete
- [ ] Export Compliance answered
- [ ] No errors or warnings

### Submit

1. **Review Everything**
   - Check all sections have green checkmarks
   - No red errors or yellow warnings

2. **Click "Submit for Review"**
   - Button is at the top-right of the version page
   - If disabled, check for incomplete sections

3. **Confirm Submission**
   - Review automatic release settings
   - Choose: "Automatically release this version" or "Manually release"
   - Click "Submit"

4. **Success!**
   - Status changes to "Waiting for Review"
   - You'll receive email confirmation

---

## Step 10: Monitor Review Status

### Review Timeline

| Stage | Duration | Actions |
|-------|----------|---------|
| **Waiting for Review** | 1-2 days | No action needed |
| **In Review** | 1-2 days | App is being tested by Apple |
| **Pending Developer Release** | - | Approved! Click "Release" |
| **Ready for Sale** | - | App is live! |

### Check Status

1. **App Store Connect Dashboard**
   - https://appstoreconnect.apple.com
   - My Apps > Unicel > Activity tab

2. **Email Notifications**
   - Status changes trigger emails
   - Review approvals or rejections

3. **Resolution Center**
   - If rejected, details appear here
   - Respond to reviewer questions

### If Approved

**Option A: Automatic Release** (if configured)
- App goes live automatically
- No action needed

**Option B: Manual Release**
1. Status: "Pending Developer Release"
2. Click "Release This Version"
3. App goes live within hours

### If Rejected

1. **Read Rejection Reason**
   - Resolution Center has details
   - Common reasons:
     - Crashes on launch
     - Missing features mentioned in description
     - Privacy policy issues
     - Metadata inaccuracies

2. **Fix Issues**
   - Address all points mentioned
   - Update build if needed
   - Revise metadata if needed

3. **Resubmit**
   - Upload new build (if code changes)
   - Update version info
   - Click "Submit for Review" again

4. **Respond to Reviewer**
   - Use Resolution Center
   - Explain what was fixed
   - Provide clarifications

---

## Step 11: Post-Launch Actions

### Monitor Performance

**App Analytics:**
- App Store Connect > Analytics
- Track: Downloads, sessions, crashes

**User Reviews:**
- App Store Connect > Ratings and Reviews
- Respond within 24-48 hours

**Crash Reports:**
- TestFlight > Crashes
- Xcode > Organizer > Crashes

### Respond to Reviews

**Best Practices:**
- Respond to all reviews (especially negative ones)
- Thank users for feedback
- Address issues mentioned
- Explain upcoming fixes
- Be professional and helpful

**Example Responses:**

*Positive Review:*
```
Thank you for the positive feedback! We're glad you're enjoying
Unicel Viewer. Stay tuned for updates with new features!
```

*Negative Review:*
```
Thank you for the feedback. We're sorry you experienced [issue].
We're working on a fix for the next update. If you have more
details, please contact support@unicel.app
```

### Plan Updates

**Version 0.5.2 (Bug Fixes):**
- Address user-reported issues
- Fix crashes
- Performance improvements

**Version 0.6.0 (Features):**
- Based on user feedback
- New capabilities
- UI enhancements

---

## Troubleshooting

### Build Upload Fails

**Error: "Archive not found"**
- Build release IPA first: `./scripts/build_ios_release.sh`
- Verify IPA exists: `ls src-tauri/gen/apple/build/Release-iphoneos/*.ipa`

**Error: "Invalid code signature"**
- Check Xcode > Preferences > Accounts
- Verify Distribution certificate is installed
- Regenerate provisioning profile

**Error: "Bundle ID mismatch"**
- Ensure `com.unicel.app` matches App Store Connect
- Check `tauri.conf.json`: `identifier: "com.unicel.app"`

### Screenshots Rejected

**Issue: "Screenshots show incorrect content"**
- Ensure screenshots are from actual app
- Remove any placeholder content
- Show real features, not mockups

**Issue: "Wrong dimensions"**
- Verify with: `sips -g pixelWidth -g pixelHeight screenshot.png`
- Recapture with correct simulator

### App Rejected

**Common Reasons:**
1. **Crashes:** Fix bugs, test thoroughly
2. **Incomplete features:** Ensure description matches functionality
3. **Privacy issues:** Update privacy policy
4. **Metadata:** Fix misleading descriptions

**How to Fix:**
- Read rejection details carefully
- Fix all mentioned issues
- Respond to reviewer explaining fixes
- Resubmit

---

## Support Resources

**Apple Documentation:**
- App Store Connect Help: https://help.apple.com/app-store-connect/
- Review Guidelines: https://developer.apple.com/app-store/review/guidelines/
- TestFlight Guide: https://developer.apple.com/testflight/

**Unicel Documentation:**
- `docs/app-store/TESTFLIGHT_GUIDE.md`
- `docs/app-store/APP_STORE_SUBMISSION_GUIDE.md`
- `docs/app-store/DEPLOYMENT_CHECKLIST.md`

**Contact:**
- GitHub Issues: https://github.com/jacksodj/unicel/issues
- Email: dennisjackson@unicel.app

---

## Summary

**Estimated Time:**
- Initial setup: 2-3 hours
- Screenshot capture: 30 minutes
- Build and upload: 30 minutes
- Metadata entry: 1 hour
- Review wait time: 2-4 days

**Total:** ~5 hours of work + 2-4 days waiting for Apple review

**Good luck with your App Store submission!**
