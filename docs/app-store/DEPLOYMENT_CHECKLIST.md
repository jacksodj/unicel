# Unicel iOS Deployment Checklist

Use this checklist to track your progress deploying Unicel Viewer to TestFlight and the App Store.

## Phase 1: Pre-Deployment Setup

### Apple Developer Account
- [ ] Enrolled in Apple Developer Program ($99/year)
- [ ] Account in good standing
- [ ] Payment method on file
- [ ] Two-factor authentication enabled

### Development Environment
- [x] Xcode 15.0+ installed
- [x] Command line tools installed
- [x] iOS implementation complete (Phase 10)
- [ ] App builds successfully in development mode
- [ ] App tested on real iOS device
- [ ] No critical bugs in current build

### Code Signing
- [ ] Distribution certificate created
- [ ] Distribution certificate installed in Keychain
- [ ] App Store provisioning profile created
- [ ] Provisioning profile downloaded
- [ ] Bundle identifier matches: com.unicel.app
- [ ] Code signing verified with codesign command

### App Store Connect Setup
- [ ] Logged in to App Store Connect
- [ ] App created with bundle ID: com.unicel.app
- [ ] App name reserved: "Unicel"
- [ ] Primary language set: English (U.S.)
- [ ] SKU created: unicel-ios-viewer

## Phase 2: Build and TestFlight

### Release Build
- [ ] tauri.conf.json version is 0.5.1
- [ ] Release build configuration verified
- [ ] Build for "Any iOS Device (arm64)"
- [ ] Release IPA created successfully
- [ ] IPA size is reasonable (< 50 MB recommended)
- [ ] dSYM files generated for crash reporting

### TestFlight Upload
- [ ] IPA uploaded to App Store Connect
- [ ] Upload completed without errors
- [ ] Processing status: Complete (not "Processing")
- [ ] Build appears in TestFlight tab
- [ ] Build number auto-incremented or set correctly

### Export Compliance
- [ ] Export compliance questionnaire answered
- [ ] Cryptography question: NO
- [ ] Exemption: Standard iOS encryption only
- [ ] Export compliance documentation saved

### TestFlight Configuration
- [ ] "What to Test" notes added
- [ ] Test instructions clear and detailed
- [ ] Known issues listed (if any)
- [ ] Contact email provided for feedback

### Internal Testing
- [ ] Internal testing group created
- [ ] Internal testers invited (up to 100)
- [ ] Testers received invitation emails
- [ ] At least 3 testers have installed
- [ ] Core functionality verified by testers
- [ ] No critical crashes reported in crash logs
- [ ] Tester feedback collected

### Bug Fixes (If Needed)
- [ ] Critical bugs identified
- [ ] Bugs fixed in codebase
- [ ] New build uploaded to TestFlight
- [ ] Updated "What's Changed" notes
- [ ] Re-tested by internal testers

### External Testing (Optional)
- [ ] External testing group created
- [ ] Build submitted for review
- [ ] Apple review approved (24-48 hours)
- [ ] External testers invited
- [ ] Public link shared (if using)
- [ ] Wider feedback collected

## Phase 3: App Store Preparation

### Privacy Policy
- [ ] Privacy policy written (see PRIVACY_POLICY.md)
- [ ] Published at public URL (HTTPS required)
- [ ] URL tested in incognito browser
- [ ] URL responds with 200 OK status
- [ ] Content clearly states no data collection
- [ ] Contact information included

### Support Page
- [ ] Support page written (see SUPPORT.md)
- [ ] Published at public URL (HTTPS required)
- [ ] URL tested and accessible
- [ ] FAQ section complete
- [ ] Contact information provided
- [ ] Bug reporting process explained

### App Store Description
- [ ] App name finalized: "Unicel"
- [ ] Subtitle: "Unit-Aware Spreadsheet Viewer"
- [ ] Promotional text written (170 chars max)
- [ ] Full description written (4000 chars max)
- [ ] Description highlights unique features
- [ ] Description includes use cases
- [ ] Description mentions desktop version
- [ ] Keywords optimized (100 chars max)
- [ ] Copyright text provided

### Screenshots - iPhone 6.7" (REQUIRED)
- [ ] Simulator configured: iPhone 15 Pro Max
- [ ] Screenshot 1: Home screen captured
- [ ] Screenshot 2: Grid view with data captured
- [ ] Screenshot 3: Unit conversion captured
- [ ] Screenshot 4: Multi-sheet navigation captured
- [ ] Screenshot 5: Formula detail captured
- [ ] All screenshots are 1290 × 2796 pixels
- [ ] Screenshots show real app content
- [ ] No debug overlays visible
- [ ] Professional appearance verified

### Screenshots - iPhone 6.5" (REQUIRED)
- [ ] Simulator configured: iPhone 11 Pro Max
- [ ] All 5 screenshots captured
- [ ] Screenshots are 1242 × 2688 pixels
- [ ] Content matches 6.7" screenshots
- [ ] Quality verified

### Screenshots - iPad 12.9" (REQUIRED)
- [ ] Simulator configured: iPad Pro 12.9"
- [ ] All 5 screenshots captured
- [ ] Screenshots are 2048 × 2732 pixels
- [ ] Layout optimized for tablet
- [ ] Quality verified

### App Icon
- [x] 1024x1024 App Store icon exists
- [x] Icon is PNG format
- [x] Icon has no transparency
- [x] Icon has no rounded corners (iOS adds them)
- [x] Icon is clear and professional
- [x] Icon located at: src-tauri/icons/ios/AppIcon-512@2x.png

### App Review Information
- [ ] Contact first name provided
- [ ] Contact last name provided
- [ ] Contact email provided: dennisjackson@unicel.app
- [ ] Contact phone number provided
- [ ] Demo account: N/A (no account needed)
- [ ] Detailed notes for reviewer written
- [ ] Review notes explain how to test app
- [ ] Review notes list example workbooks
- [ ] Review notes explain offline operation

### Age Rating
- [ ] Age rating questionnaire completed
- [ ] All content questions answered: None
- [ ] Final rating: 4+ (All Ages)
- [ ] Rating appropriate for app content

### Pricing and Availability
- [ ] Price set: Free ($0.00)
- [ ] All territories selected (or specific ones)
- [ ] Pre-order: Disabled (for v1.0)
- [ ] Availability date: Immediate upon approval

## Phase 4: App Store Submission

### Metadata Upload
- [ ] Logged in to App Store Connect
- [ ] Navigated to: My Apps > Unicel > iOS App
- [ ] Version 0.5.1 selected
- [ ] All required fields marked complete
- [ ] Privacy policy URL entered and verified
- [ ] Support URL entered and verified
- [ ] Marketing URL entered (optional)
- [ ] Age rating completed

### Screenshot Upload
- [ ] iPhone 6.7" screenshots uploaded (5 images)
- [ ] iPhone 6.5" screenshots uploaded (5 images)
- [ ] iPad 12.9" screenshots uploaded (5 images)
- [ ] Screenshots in correct order
- [ ] First screenshot is most compelling
- [ ] All screenshots display correctly in preview

### App Information
- [ ] App description copy-pasted from metadata doc
- [ ] Promotional text added
- [ ] Keywords entered (under 100 chars)
- [ ] "What's New" text for version 0.5.1 added
- [ ] Copyright information entered
- [ ] All text proofread for typos

### Build Selection
- [ ] TestFlight build selected from dropdown
- [ ] Build version matches: 0.5.1
- [ ] Export compliance shows complete
- [ ] Build is correct one (latest tested version)

### Final Verification
- [ ] All sections marked as complete (green checkmarks)
- [ ] No red error indicators visible
- [ ] Privacy policy URL loads correctly
- [ ] Support URL loads correctly
- [ ] Screenshots look professional
- [ ] Description is accurate and compelling
- [ ] All metadata is factually correct

### Submission
- [ ] Clicked "Save" to save all changes
- [ ] Clicked "Submit for Review"
- [ ] Confirmed submission in popup dialog
- [ ] Status changed to "Waiting for Review"
- [ ] Confirmation email received from Apple
- [ ] Submission timestamp noted: ___________

## Phase 5: App Review Process

### Monitoring
- [ ] Check email daily for review updates
- [ ] Check Resolution Center in App Store Connect
- [ ] Status tracked in App Store Connect
- [ ] Phone available for Apple reviewer calls

### Status Tracking
- [ ] Waiting for Review (1-3 days typical)
- [ ] In Review (24-48 hours typical)
- [ ] Review outcome received: [ ] Approved [ ] Rejected

### If Approved
- [ ] "Pending Developer Release" status confirmed
- [ ] Release option chosen:
  - [ ] Manual release (click "Release this Version")
  - [ ] Automatic release (released immediately)
  - [ ] Scheduled release (set date/time)
- [ ] App released to App Store
- [ ] Status changed to "Ready for Sale"
- [ ] Verified app appears in App Store search
- [ ] Download and install tested on real device
- [ ] App listing looks correct in App Store

### If Rejected
- [ ] Rejection reason read carefully in Resolution Center
- [ ] Specific guideline violations noted
- [ ] Issues understood and documented
- [ ] Response drafted (if appealing)
- [ ] Code changes made (if fixing issues)
- [ ] New build uploaded to TestFlight (if needed)
- [ ] Metadata corrected (if needed)
- [ ] Resubmitted for review
- [ ] Second submission timestamp: ___________

## Phase 6: Post-Launch

### Launch Day Activities
- [ ] Verified app is live in App Store
- [ ] Tested download and installation
- [ ] App listing appearance verified
- [ ] Direct App Store link obtained
- [ ] App Store badge downloaded

### Communication
- [ ] GitHub README updated with App Store badge
- [ ] Beta testers thanked via email
- [ ] Social media announcement posted (if applicable)
- [ ] Project website updated (if applicable)
- [ ] Support email monitored

### Monitoring
- [ ] App Store Connect metrics checked daily
- [ ] Downloads and installs tracked
- [ ] Crash reports reviewed (should be near zero)
- [ ] Customer reviews monitored
- [ ] Customer ratings tracked
- [ ] Support emails answered within 24 hours

### Response Plan
- [ ] Review response templates prepared
- [ ] Positive reviews thanked
- [ ] Negative reviews addressed professionally
- [ ] Common issues documented
- [ ] FAQ updated based on feedback

### Week 1 Metrics
- [ ] Total downloads: _________
- [ ] Total installs: _________
- [ ] Crash-free rate: _________%
- [ ] Average rating: _________
- [ ] Number of reviews: _________
- [ ] Support emails received: _________
- [ ] Critical bugs found: _________

## Phase 7: First Update Planning

### Feedback Collection
- [ ] User feedback compiled from reviews
- [ ] Support email feedback organized
- [ ] TestFlight feedback reviewed
- [ ] Common feature requests identified
- [ ] Bug reports prioritized

### Update Planning
- [ ] Version 0.6.0 or 0.5.2 planned
- [ ] Bug fixes prioritized
- [ ] Feature additions considered
- [ ] Development work scheduled
- [ ] "What's New" text drafted
- [ ] Update timeline: _________

### Continuous Improvement
- [ ] Analytics reviewed weekly
- [ ] Crash reports monitored daily
- [ ] Keywords optimized based on search data
- [ ] Screenshots refreshed if needed
- [ ] Description updated for clarity
- [ ] Regular updates scheduled (monthly/quarterly)

## Notes and Timestamps

**TestFlight First Upload:** _________

**First Internal Test:** _________

**App Store Submission:** _________

**Review Started:** _________

**Review Completed:** _________

**App Released:** _________

**Key Learnings:**
-
-
-

**Issues Encountered:**
-
-
-

**Time to App Store:** _________ days

---

## Progress Summary

**Phases Completed:**
- [ ] Phase 1: Pre-Deployment Setup
- [ ] Phase 2: Build and TestFlight
- [ ] Phase 3: App Store Preparation
- [ ] Phase 4: App Store Submission
- [ ] Phase 5: App Review Process
- [ ] Phase 6: Post-Launch
- [ ] Phase 7: First Update Planning

**Overall Completion:** _____ / 7 phases

**Next Action:** ________________________________

**Blocked By:** ________________________________

**Target Release Date:** ________________________________

---

**Last Updated:** ___________
**Updated By:** ___________
