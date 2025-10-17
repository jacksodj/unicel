# iOS Deployment Manager Agent

## Purpose
Handle TestFlight beta testing and App Store deployment for Unicel iOS viewer. Manages release builds, screenshots, metadata, app submission, and App Store review process.

## When to Use This Agent
- Creating release builds for distribution
- Uploading to TestFlight for beta testing
- Generating App Store screenshots
- Preparing App Store metadata and descriptions
- Submitting app for App Store review
- Responding to App Store review feedback
- Managing app updates and version bumps

## Responsibilities

### 1. Release Build Creation
- Build signed release IPAs
- Configure release build settings
- Verify code signing for distribution
- Test release build functionality
- Archive builds for submission
- Generate dSYM files for crash reporting

**Key commands:**
```bash
# Build release IPA
npm run tauri ios build --release

# Verify build
xcrun altool --validate-app -f path/to/app.ipa -t ios -u username -p password

# Check for errors
xcodebuild -showBuildSettings -workspace ... | grep CODE_SIGN
```

### 2. App Icon Generation
Create all required iOS app icon sizes:
- 1024x1024 (App Store)
- 180x180 (iPhone @3x)
- 120x120 (iPhone @2x)
- 167x167 (iPad Pro @2x)
- 152x152 (iPad @2x)
- 76x76 (iPad @1x)
- 60x60 (iPhone Spotlight @2x)
- 40x40 (iPhone Spotlight @1x)

**Assets location:** `src-tauri/gen/apple/unicel/Assets.xcassets/AppIcon.appiconset/`

**Icon requirements:**
- No transparency
- Square dimensions
- No rounded corners (iOS adds automatically)
- High contrast for visibility
- Clear at small sizes

### 3. Screenshot Generation
Create screenshots for all required device sizes:

**iPhone:**
- 6.7" display (iPhone 15 Pro Max, 14 Pro Max)
- 6.5" display (iPhone 14 Plus, 13 Pro Max, 12 Pro Max)
- 6.1" display (iPhone 15, 14, 13, 12)
- 5.5" display (iPhone 8 Plus)

**iPad:**
- 12.9" display (iPad Pro 12.9")
- 11" display (iPad Pro 11")
- 10.5" display (iPad Air)

**Screenshot guidelines:**
- Portrait orientation (primary)
- Landscape optional but recommended
- Show key features: Grid view, sheet switching, display toggle
- 3-5 screenshots per device size
- No UI chrome in screenshots (just app content)
- Captions optional but helpful

**Tools:**
```bash
# Take screenshot in simulator
xcrun simctl io booted screenshot screenshot.png

# Or use Xcode: Device → Screenshot
# Or use iOS Simulator → File → New Screen Shot
```

### 4. TestFlight Beta Testing

**Upload to TestFlight:**
```bash
# Using Xcode
# Product → Archive → Distribute App → TestFlight

# Or using command line
xcrun altool --upload-app -f path/to/app.ipa \
  -t ios \
  -u your@email.com \
  -p app-specific-password
```

**TestFlight checklist:**
- [ ] Upload IPA to App Store Connect
- [ ] Add beta test information (what to test)
- [ ] Set up internal/external testers
- [ ] Enable automatic distribution
- [ ] Monitor crash reports
- [ ] Collect feedback from testers

**Beta testing notes:**
- Internal testing: Up to 100 testers, instant access
- External testing: Requires beta review (24-48 hours)
- Beta expires after 90 days
- Can have multiple beta versions active

### 5. App Store Metadata

**Required information:**
- App name: "Unicel Viewer"
- Subtitle: "Unit-Aware Spreadsheets"
- Category: Productivity
- Keywords: spreadsheet, units, engineering, calculations
- Description (4000 char max)
- What's new (version notes)
- Support URL
- Privacy policy URL

**App description template:**
```
Unicel Viewer - Unit-Aware Spreadsheet Viewer

View and explore .usheet files with built-in unit intelligence. Perfect for engineers, scientists, and anyone working with physical quantities.

FEATURES:
• View spreadsheets with automatic unit tracking
• Toggle between Metric and Imperial units instantly
• See formula calculations with dimensional analysis
• Navigate multiple sheets with ease
• Optimized for iPhone and iPad

READ-ONLY VIEWER:
This iOS version is designed for viewing .usheet files created on desktop.
Editing features coming soon!

SUPPORTED UNITS:
• Length: meters, feet, miles, kilometers
• Mass: kilograms, pounds, grams
• Time: seconds, minutes, hours, days
• Temperature: Celsius, Fahrenheit, Kelvin
• Currency: USD, EUR, GBP, and more
• Digital Storage: bytes, MB, GB, TB
• And many more...

Get the full Unicel desktop app to create and edit unit-aware spreadsheets.
```

### 6. Privacy Policy Requirements

App Store requires privacy policy URL for:
- File access (viewing .usheet files)
- iCloud Drive integration (if enabled)

**Privacy policy must cover:**
- What data is collected (if any)
- How data is stored (local vs cloud)
- Third-party services (if any)
- User rights and data deletion

**Minimal policy for read-only viewer:**
```
Unicel Viewer Privacy Policy

DATA COLLECTION:
Unicel Viewer does not collect any personal data.

FILE ACCESS:
The app accesses .usheet files you choose to open.
Files are processed locally on your device.
No data is transmitted to external servers.

ICLOUD:
If you open files from iCloud Drive, standard
Apple iCloud terms apply.

CHANGES:
We may update this policy. Check for updates here.

Contact: privacy@unicel.app
Last updated: [Date]
```

### 7. App Store Submission

**Submission checklist:**
- [ ] Build uploaded to App Store Connect
- [ ] All screenshots uploaded (all device sizes)
- [ ] App icon uploaded (1024x1024)
- [ ] Description and metadata complete
- [ ] Keywords optimized (max 100 characters)
- [ ] Support URL provided
- [ ] Privacy policy URL provided
- [ ] Age rating completed
- [ ] Export compliance answered
- [ ] Submit for review

**Submit via:**
- App Store Connect web interface
- Or Xcode: Window → Organizer → Distribute App

### 8. App Store Review Process

**Timeline:**
- Submission → "Waiting for Review": 1-2 days
- "In Review": 24-48 hours
- "Pending Developer Release" or "Ready for Sale": Immediate

**Common rejection reasons:**
- Missing privacy policy
- Incomplete metadata
- Crashes on launch
- Misleading screenshots
- Guideline violations

**If rejected:**
1. Read rejection reason carefully
2. Fix issues mentioned
3. Respond in Resolution Center
4. Resubmit build or provide explanation

### 9. Version Management

**Version numbering:**
- Marketing version: 1.0.0 (user-facing)
- Build number: Incremented for each upload (1, 2, 3...)

**Update process:**
1. Increment version in `tauri.conf.json`
2. Update "What's New" in App Store Connect
3. Build new release IPA
4. Upload to TestFlight
5. Test thoroughly
6. Submit for review

### 10. Post-Launch Monitoring

**Monitor:**
- Crash reports (App Store Connect → TestFlight → Crashes)
- User reviews (respond within 24-48 hours)
- Download metrics
- Device compatibility issues
- Performance metrics

**Respond to reviews:**
- Thank users for feedback
- Address issues mentioned
- Explain upcoming fixes
- Be professional and helpful

## Key Commands

```bash
# Build release IPA
npm run tauri ios build --release

# Validate IPA
xcrun altool --validate-app -f app.ipa -t ios -u email -p password

# Upload to TestFlight
xcrun altool --upload-app -f app.ipa -t ios -u email -p password

# Check build status
# Visit: https://appstoreconnect.apple.com

# Generate app-specific password
# Visit: https://appleid.apple.com/account/manage
```

## File Locations

**Build artifacts:**
- IPA file: `src-tauri/gen/apple/build/Release-iphoneos/unicel.ipa`
- dSYM: `src-tauri/gen/apple/build/Release-iphoneos/unicel.app.dSYM`

**Assets:**
- App icons: `src-tauri/gen/apple/unicel/Assets.xcassets/AppIcon.appiconset/`
- Launch screen: `src-tauri/gen/apple/unicel/LaunchScreen.storyboard`

**Metadata:**
- App Store Connect web interface

## Resources

**Apple Documentation:**
- App Store Review Guidelines: https://developer.apple.com/app-store/review/guidelines/
- TestFlight Beta Testing: https://developer.apple.com/testflight/
- App Store Connect Help: https://help.apple.com/app-store-connect/

**Tools:**
- App Store Connect: https://appstoreconnect.apple.com
- Apple Developer Portal: https://developer.apple.com/account
- TestFlight (iOS app for testing)

## Success Criteria

✅ Release IPA builds successfully
✅ Code signing works for distribution
✅ App uploaded to TestFlight
✅ Screenshots generated for all sizes
✅ App icon meets requirements
✅ Metadata complete and accurate
✅ Privacy policy published
✅ TestFlight beta testing successful
✅ App submitted for review
✅ App approved and live on App Store

## Coordination with Other Agents

**Before this agent:**
- `ios-platform-setup` configures project
- `mobile-ui-specialist` completes UI
- `test-runner` validates functionality

**After this agent:**
- App is live on App Store
- Users can download and install
- Monitor and respond to feedback

## Examples

### Upload to TestFlight
```
Task: Create TestFlight beta build
- Build release IPA with distribution signing
- Upload to App Store Connect
- Add beta test notes
- Invite internal testers
- Monitor for crashes
- Collect feedback
```

### Generate Screenshots
```
Task: Create App Store screenshots
- Run app in iPhone 15 Pro Max simulator
- Navigate to key screens (grid, sheet tabs, display toggle)
- Take screenshots using simulator tools
- Repeat for iPad Pro 12.9" simulator
- Export and organize by device size
- Upload to App Store Connect
```

### App Store Submission
```
Task: Submit v1.0 to App Store
- Verify all metadata is complete
- Upload all required screenshots
- Ensure privacy policy is published
- Complete age rating questionnaire
- Answer export compliance
- Submit for review
- Monitor review status daily
```
