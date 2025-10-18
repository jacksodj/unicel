# App Store Submission Guide

This guide covers submitting Unicel Viewer to the App Store for public release.

## Prerequisites

Before submitting to the App Store:

1. **TestFlight Testing Complete**
   - Build tested by internal testers
   - Major bugs fixed
   - App is stable and functional

2. **All Metadata Prepared**
   - App description written (see APP_STORE_METADATA.md)
   - Screenshots captured for all required sizes
   - App icon (1024x1024) ready
   - Privacy policy published
   - Support page published

3. **Legal Requirements**
   - Privacy policy URL accessible
   - Export compliance answered
   - Age rating completed

4. **Build Ready**
   - Latest build uploaded to TestFlight
   - Export compliance completed for that build
   - No known critical bugs

## Step 1: Prepare Screenshots

Screenshots are REQUIRED for submission. You need screenshots for:

### Required Sizes:

1. **iPhone 6.7" Display (1290 × 2796 pixels)**
   - iPhone 15 Pro Max, 14 Pro Max
   - Use iOS Simulator or real device

2. **iPhone 6.5" Display (1242 × 2688 pixels)**
   - iPhone 11 Pro Max, XS Max
   - Use iOS Simulator or real device

3. **iPad Pro 12.9" Display (2048 × 2732 pixels)**
   - iPad Pro 12.9" (2nd gen and later)
   - Use iOS Simulator or real device

### How to Capture Screenshots:

#### Method 1: Using iOS Simulator

```bash
# List available simulators
xcrun simctl list devices

# Boot specific simulator
xcrun simctl boot "iPhone 15 Pro Max"

# Open Simulator app
open -a Simulator

# Run app in simulator
cd /Users/dennisjackson/Code/unicel
npm run tauri ios dev

# In Simulator:
# 1. Navigate to screen you want to capture
# 2. Use: File > New Screen Shot
# Or: Command + S
# Or via command line:

# Take screenshot
xcrun simctl io booted screenshot ~/Desktop/screenshot-01-home.png

# To get exact dimensions:
# The simulator saves at the correct resolution automatically
```

#### Method 2: Using Real Device

1. Connect iPhone/iPad via USB
2. Open Xcode
3. Window > Devices and Simulators
4. Select your device
5. Click "Take Screenshot" button
6. Screenshot saved to Desktop

#### Method 3: Using Xcode's Screenshot Feature

1. Run app on device or simulator
2. Xcode > Debug > View Debugging > Take Screenshot
3. Screenshot captured at native resolution

### Screenshot Recommendations:

**Screenshot 1: Home Screen**
- Show app icon and "Open File" button
- Clean, professional first impression
- Include example workbooks list if visible

**Screenshot 2: Grid View with Data**
- Open AWS Cost Estimator or Construction Estimator
- Show unit-aware cells with values and units
- Visible formulas in some cells
- Grid should look populated and useful

**Screenshot 3: Unit Conversion Toggle**
- Before/after of Metric ↔ Imperial toggle
- Clear visual difference (e.g., 100ft → 30.48m)
- Highlight the toggle button
- Show multiple cells converting simultaneously

**Screenshot 4: Multi-Sheet Navigation**
- Sheet tabs visible at bottom
- Multiple sheets shown
- Active sheet highlighted
- Professional layout

**Screenshot 5: Formula and Detail View**
- Cell selected showing formula bar
- Formula visible with units
- Calculated value displayed
- Shows app's intelligence

### Screenshot Tools:

To add captions, device frames, or backgrounds:

- **Fastlane Frameit:** https://docs.fastlane.tools/actions/frameit/
- **Screenshot Studio:** https://screenshots.pro/
- **Figma/Sketch:** For custom designs
- **Preview App:** For basic annotations

### Screenshot Storage:

Save screenshots in organized folders:

```bash
mkdir -p /Users/dennisjackson/Code/unicel/docs/app-store/screenshots/iphone-6.7
mkdir -p /Users/dennisjackson/Code/unicel/docs/app-store/screenshots/iphone-6.5
mkdir -p /Users/dennisjackson/Code/unicel/docs/app-store/screenshots/ipad-12.9
```

## Step 2: Publish Privacy Policy

The privacy policy MUST be publicly accessible before submission.

### Option A: GitHub Pages (Recommended)

1. **Enable GitHub Pages:**
   - Go to repository Settings > Pages
   - Source: Deploy from branch
   - Branch: main
   - Folder: /docs
   - Click Save

2. **Privacy Policy URL:**
   ```
   https://jacksodj.github.io/unicel/app-store/PRIVACY_POLICY.html
   ```

3. **Convert Markdown to HTML:**
   ```bash
   # Using pandoc (install via: brew install pandoc)
   cd /Users/dennisjackson/Code/unicel/docs/app-store
   pandoc PRIVACY_POLICY.md -o PRIVACY_POLICY.html

   # Or create simple HTML manually
   ```

### Option B: Custom Domain

If you own a domain:
- Upload to https://unicel.app/privacy
- Ensure it's publicly accessible (no login required)
- Must be HTTPS

### Option C: Simple Web Host

Use any web hosting service:
- Netlify
- Vercel
- AWS S3 + CloudFront
- Google Cloud Storage

**Verify Accessibility:**
```bash
# Test URL is publicly accessible
curl -I https://your-privacy-policy-url
# Should return 200 OK
```

## Step 3: Complete App Information

Log in to App Store Connect and fill in all required fields:

### 3.1: General App Information

Navigate to: My Apps > Unicel > App Information

**Name:** Unicel

**Subtitle:** Unit-Aware Spreadsheet Viewer

**Primary Language:** English (U.S.)

**Category:**
- Primary: Productivity
- Secondary: Business (optional)

**Content Rights:**
- [ ] Contains third-party content
  (Check only if you use third-party libraries that require attribution)

**Age Rating:** 4+
- Click "Edit" to complete questionnaire
- Answer "No" to all content questions
- Result: 4+ (All Ages)

**License Agreement:** Standard Apple EULA

### 3.2: Pricing and Availability

Navigate to: My Apps > Unicel > Pricing and Availability

**Price:** Free (0.00 USD)

**Availability:**
- [ ] Available on all territories
  Or select specific countries

**Pre-Order:**
- Not recommended for v1.0
- Can enable for future versions

### 3.3: Privacy Policy

Navigate to: My Apps > Unicel > App Privacy

**Privacy Policy URL:**
```
https://jacksodj.github.io/unicel/app-store/PRIVACY_POLICY.html
```

**Data Collection:**
Click "Get Started" and answer:

1. **Do you or your third-party partners collect data from this app?**
   - Answer: **No**
   - Explanation: App processes everything locally, no data collection

2. Complete the form confirming no data types are collected

3. Click "Publish"

### 3.4: App Review Information

Navigate to: Version > App Review Information

**Contact Information:**
- First Name: Dennis
- Last Name: Jackson
- Phone: [YOUR PHONE NUMBER]
- Email: dennisjackson@unicel.app

**Sign-In Required:** No

**Notes for Reviewer:**
```
Thank you for reviewing Unicel Viewer!

WHAT THIS APP DOES:
Unicel Viewer is a read-only spreadsheet viewer for .usheet files created
with the Unicel desktop application. It's similar to a PDF viewer, but for
unit-aware spreadsheet files.

HOW TO TEST THE APP:

1. Launch Unicel Viewer
2. Tap "Open Example" to load sample workbooks
3. Select "AWS Cost Estimator" or "Construction Estimator"
4. Explore the spreadsheet by scrolling and zooming
5. Toggle between Metric and Imperial units using the toggle button
   - Notice how 100 feet converts to 30.48 meters
   - Currency, temperature, and other units also convert
6. Navigate between sheets using the sheet tabs at the bottom
7. Tap any cell to view its formula and calculated value

KEY FEATURES TO VERIFY:
- Opening example workbooks works
- Unit conversion toggle works smoothly
- Multi-sheet navigation functions
- Spreadsheet grid is responsive
- Cell detail view shows formulas
- App works offline (no network required)

SAMPLE DATA:
The app includes several example workbooks:
- AWS Cost Estimator: Cloud infrastructure cost calculations
- Construction Estimator: Building materials with quantities and costs
- Unit Conversion Tutorial: Demonstrates unit system capabilities

PRIVACY & SECURITY:
- No user data is collected
- All file processing is local to the device
- No network connections for data collection
- Files are only accessed when user explicitly opens them

EXPORT COMPLIANCE:
This app does not use encryption beyond standard iOS data protection APIs.

If you have any questions, please contact: dennisjackson@unicel.app

Thank you!
```

**Attachment:** (Optional)
- If you have a demo video, upload it here

## Step 4: Add Version Information

Navigate to: My Apps > Unicel > iOS App > [Version 0.5.1]

### 4.1: Screenshots

Upload screenshots for each device size:

**iPhone 6.7" Display:**
- Click "+" to add screenshots
- Upload 3-10 screenshots (5 recommended)
- Drag to reorder
- First screenshot is most important (shown in search results)

**iPhone 6.5" Display:**
- Repeat with 6.5" screenshots

**iPad Pro 12.9" Display:**
- Upload iPad screenshots
- Can be same content as iPhone but optimized for tablet

**App Preview Video:** (Optional)
- 15-30 second video
- Same resolution as screenshots
- Auto-plays in App Store

### 4.2: Promotional Text (Optional)

This appears above the description and can be updated anytime without review:

```
View unit-aware spreadsheets on your iPhone and iPad. Toggle between
Metric and Imperial units instantly. Perfect for engineers and data
professionals.
```

### 4.3: Description

Copy from APP_STORE_METADATA.md:

```
[Full description - see APP_STORE_METADATA.md]
```

**Character limit:** 4000 characters

### 4.4: Keywords

```
spreadsheet,units,calculator,viewer,engineering,metric,imperial,conversion,productivity,formula
```

**Character limit:** 100 characters (including commas)

### 4.5: Support URL

```
https://jacksodj.github.io/unicel/app-store/SUPPORT.html
```

Or:
```
https://github.com/jacksodj/unicel/blob/main/docs/app-store/SUPPORT.md
```

### 4.6: Marketing URL (Optional)

```
https://github.com/jacksodj/unicel
```

### 4.7: Version Number

```
0.5.1
```

### 4.8: Copyright

```
Copyright 2025 Dennis Jackson. All rights reserved.
```

### 4.9: What's New in This Version

```
Welcome to Unicel Viewer for iOS!

This is the initial release of Unicel Viewer, bringing unit-aware
spreadsheets to iPhone and iPad.

Features:
• Read-only viewer for .usheet files
• Toggle between Metric and Imperial display modes
• Multi-sheet navigation
• Formula and value display
• Support for 100+ units across all major domains
• Touch-optimized spreadsheet grid
• Works completely offline

Get the desktop version at github.com/jacksodj/unicel for full editing
capabilities.

Thank you for being an early adopter! Please share your feedback.
```

## Step 5: Select Build

1. In the version page, scroll to "Build" section
2. Click "Add Build" or "Select a Build"
3. Choose your TestFlight build (0.5.1)
4. Confirm export compliance is completed

If build is not available:
- Ensure it's processed in TestFlight
- Export compliance must be answered
- May take a few minutes to appear

## Step 6: Submit for Review

### Pre-Submission Checklist:

- [ ] All screenshots uploaded (iPhone 6.7", 6.5", iPad 12.9")
- [ ] App icon 1024x1024 uploaded
- [ ] Description complete (under 4000 characters)
- [ ] Keywords optimized (under 100 characters)
- [ ] Privacy policy URL accessible and working
- [ ] Support URL accessible and working
- [ ] App review contact information complete
- [ ] Notes for reviewer detailed and helpful
- [ ] Build selected and export compliance complete
- [ ] Age rating completed (4+)
- [ ] Pricing set (Free)
- [ ] Tested build works on real devices
- [ ] No critical bugs known

### Submit:

1. Review all information one last time
2. Click "Save" to save all changes
3. Click "Submit for Review" button (top right)
4. Confirm submission in dialog
5. Status changes to "Waiting for Review"

## Step 7: Review Process

### Timeline:

- **Waiting for Review:** 1-3 days typically
- **In Review:** 24-48 hours
- **Approved or Rejected:** Immediate notification via email

### Status Meanings:

- **Waiting for Review:** In queue, not yet reviewed
- **In Review:** Apple reviewer is testing your app
- **Pending Developer Release:** Approved! Waiting for you to release
- **Ready for Sale:** Live in the App Store
- **Rejected:** Not approved, see Resolution Center for details

### What Apple Reviews:

1. **Functionality:**
   - App works as described
   - No crashes or major bugs
   - All features are accessible

2. **Design:**
   - UI follows iOS Human Interface Guidelines
   - No placeholder content
   - Professional appearance

3. **Safety:**
   - No objectionable content
   - Privacy policy present
   - Data handling transparent

4. **Legal:**
   - Export compliance completed
   - Age rating appropriate
   - No misleading claims

5. **Business:**
   - No hidden features
   - Pricing clear
   - In-app purchases disclosed (N/A for this app)

## Step 8: Handle Review Outcome

### If Approved:

1. **Notification:**
   - You'll receive email: "Your app status is Pending Developer Release"
   - Log in to App Store Connect to release

2. **Release Options:**

   **Option A: Manual Release**
   - Click "Release this Version"
   - App goes live within 1-2 hours

   **Option B: Automatic Release**
   - Pre-configure in version settings
   - App releases automatically upon approval

   **Option C: Scheduled Release**
   - Set specific date/time for release
   - Useful for coordinated launches

3. **Going Live:**
   - Status changes to "Ready for Sale"
   - App appears in App Store within 2-3 hours
   - Search indexing takes 24-48 hours

### If Rejected:

1. **Review Rejection:**
   - Check email for rejection notice
   - Log in to App Store Connect
   - Go to Resolution Center

2. **Common Rejection Reasons:**

   **Guideline 2.1 - App Completeness:**
   - App crashes on launch
   - Features don't work as described
   - Placeholder content present

   **Fix:** Test thoroughly, fix bugs, resubmit

   **Guideline 2.3 - Accurate Metadata:**
   - Screenshots don't match app
   - Description is misleading
   - Keywords are spam

   **Fix:** Update metadata to accurately reflect app

   **Guideline 4.0 - Design:**
   - UI doesn't follow iOS guidelines
   - App looks unfinished
   - Poor user experience

   **Fix:** Improve design, polish UI

   **Guideline 5.1.1 - Privacy:**
   - Privacy policy missing or inaccessible
   - Data collection not disclosed

   **Fix:** Ensure privacy policy URL works, update privacy disclosure

3. **Respond to Rejection:**
   - Read rejection reason carefully
   - Fix the issues mentioned
   - Respond in Resolution Center if needed
   - Upload new build if required
   - Resubmit for review

4. **Appeal (If Necessary):**
   - If you believe rejection is incorrect
   - Use App Review Board contact
   - Provide detailed explanation
   - Reference specific guidelines

## Step 9: Post-Launch Activities

### Launch Day:

1. **Verify App is Live:**
   - Search for "Unicel" in App Store
   - Check app page loads correctly
   - Test download and installation

2. **Share the News:**
   - Post on social media
   - Update GitHub README with App Store link
   - Email beta testers
   - Add badge to website

### First Week:

1. **Monitor Metrics:**
   - Downloads and installs
   - Crash reports (should be near zero)
   - Ratings and reviews

2. **Respond to Reviews:**
   - Reply to reviews within 24-48 hours
   - Thank users for positive feedback
   - Address concerns in negative reviews
   - Be professional and helpful

3. **Track Issues:**
   - Check crash logs daily
   - Monitor support email
   - Log bugs in GitHub Issues
   - Prioritize fixes

### Ongoing:

1. **Regular Updates:**
   - Fix bugs promptly
   - Add requested features
   - Improve performance
   - Keep app current with iOS updates

2. **Version Updates:**
   - Increment version for each release
   - Write detailed "What's New" notes
   - Test in TestFlight first
   - Submit updates for review

3. **App Store Optimization:**
   - Update keywords based on search trends
   - Refresh screenshots periodically
   - Update description for new features
   - Monitor conversion rate

## Useful App Store Links

**App Store Badge:**
```html
<a href="https://apps.apple.com/app/idYOUR_APP_ID">
  <img src="https://tools.applemediaservices.com/api/badges/download-on-the-app-store/black/en-us?size=250x83"
       alt="Download on the App Store">
</a>
```

**Direct App Store URL:**
```
https://apps.apple.com/app/idYOUR_APP_ID
```

**App Store Search:**
```
https://apps.apple.com/search?term=unicel
```

## Troubleshooting

### Screenshot Upload Fails:
- Verify exact dimensions match requirements
- Use PNG format (not JPG)
- File size under 500KB per screenshot
- No transparency

### Build Not Appearing:
- Check TestFlight processing status
- Complete export compliance
- Wait 10-15 minutes after processing
- Try refreshing the page

### Privacy Policy URL Rejected:
- Must be HTTPS
- Must be publicly accessible (no login)
- Must load quickly
- Must contain relevant privacy information

### App Rejected for Crashes:
- Review crash logs in App Store Connect
- Test on multiple devices and iOS versions
- Use real devices, not just simulator
- Check for memory issues

### Keywords Not Saving:
- Must be under 100 characters including commas
- No spaces after commas
- No special characters
- No duplicate words

## Resources

- **App Store Connect:** https://appstoreconnect.apple.com
- **App Store Review Guidelines:** https://developer.apple.com/app-store/review/guidelines/
- **Resolution Center:** https://appstoreconnect.apple.com/resolution-center
- **Marketing Resources:** https://developer.apple.com/app-store/marketing/
- **App Store Badges:** https://developer.apple.com/app-store/marketing/guidelines/

## Final Checklist

### Before Submission:
- [ ] All metadata complete and proofread
- [ ] Screenshots for all required sizes uploaded
- [ ] Privacy policy published and accessible
- [ ] Support page published and accessible
- [ ] Build tested thoroughly on real devices
- [ ] Export compliance completed
- [ ] Age rating accurate
- [ ] Contact information correct
- [ ] Detailed notes for reviewer
- [ ] No known critical bugs

### After Submission:
- [ ] Monitor email for review updates
- [ ] Check Resolution Center daily
- [ ] Prepare for launch activities
- [ ] Social media posts drafted
- [ ] Support email monitored
- [ ] Crash reporting configured

### After Approval:
- [ ] Release app to App Store
- [ ] Verify app is live
- [ ] Share news on social media
- [ ] Update GitHub README
- [ ] Thank beta testers
- [ ] Monitor metrics and reviews
- [ ] Plan first update

---

**Congratulations on submitting to the App Store! Good luck with your review!**
