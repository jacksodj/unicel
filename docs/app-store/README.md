# Unicel iOS App Store Deployment

This directory contains all documentation and materials needed for deploying Unicel Viewer to TestFlight and the App Store.

## Quick Start

**Prerequisites:**
- Apple Developer account ($99/year)
- Xcode 15.0+ installed
- iOS implementation complete (Phase 10)
- App builds successfully on iOS

**Deployment Path:**
1. TestFlight Beta → Internal Testing → External Testing → App Store Release

## Documentation

### Core Guides

| Guide | Purpose | When to Use |
|-------|---------|-------------|
| [TESTFLIGHT_GUIDE.md](TESTFLIGHT_GUIDE.md) | Upload to TestFlight for beta testing | First step after iOS development complete |
| [SCREENSHOT_GUIDE.md](SCREENSHOT_GUIDE.md) | Capture App Store screenshots | Before App Store submission |
| [APP_STORE_SUBMISSION_GUIDE.md](APP_STORE_SUBMISSION_GUIDE.md) | Submit to App Store for public release | After successful TestFlight testing |

### Supporting Materials

| File | Purpose |
|------|---------|
| [APP_STORE_METADATA.md](APP_STORE_METADATA.md) | Complete App Store listing details |
| [PRIVACY_POLICY.md](PRIVACY_POLICY.md) | Privacy policy (required by Apple) |
| [SUPPORT.md](SUPPORT.md) | User support documentation |

## Deployment Timeline

### Week 1: TestFlight Beta
**Goal:** Get app into testers' hands

**Tasks:**
- [ ] Build release IPA
- [ ] Upload to TestFlight
- [ ] Add test information
- [ ] Invite internal testers (up to 100)
- [ ] Monitor crash reports
- [ ] Collect feedback

**Time Required:** 1-2 days
**Guide:** [TESTFLIGHT_GUIDE.md](TESTFLIGHT_GUIDE.md)

### Week 2: Beta Testing & Iteration
**Goal:** Fix bugs, gather feedback

**Tasks:**
- [ ] Test core functionality
- [ ] Fix critical bugs
- [ ] Address tester feedback
- [ ] Upload new builds as needed
- [ ] Expand to external testers (optional)

**Time Required:** 1-2 weeks
**Resources:** TestFlight crash logs, tester feedback

### Week 3: App Store Preparation
**Goal:** Prepare all App Store materials

**Tasks:**
- [ ] Capture screenshots for all device sizes
- [ ] Publish privacy policy online
- [ ] Publish support page
- [ ] Write App Store description
- [ ] Prepare app review notes
- [ ] Complete export compliance

**Time Required:** 2-3 days
**Guides:**
- [SCREENSHOT_GUIDE.md](SCREENSHOT_GUIDE.md)
- [APP_STORE_METADATA.md](APP_STORE_METADATA.md)

### Week 4: App Store Submission
**Goal:** Submit for App Store review

**Tasks:**
- [ ] Upload all screenshots
- [ ] Complete all metadata
- [ ] Select final build
- [ ] Submit for review
- [ ] Wait for review (1-3 days)
- [ ] Release to App Store

**Time Required:** 1-2 days + review wait
**Guide:** [APP_STORE_SUBMISSION_GUIDE.md](APP_STORE_SUBMISSION_GUIDE.md)

## Current Status

**App Information:**
- **Name:** Unicel
- **Bundle ID:** com.unicel.app
- **Version:** 0.5.1
- **Platform:** iOS 13.0+
- **Category:** Productivity
- **Price:** Free

**Completed Tasks:**
✅ App icons generated (all required sizes)
✅ Xcode project configured
✅ Bundle identifier set
✅ Privacy policy drafted
✅ Support page drafted
✅ App Store metadata prepared

**Remaining Tasks:**
- [ ] Build release IPA
- [ ] Capture App Store screenshots
- [ ] Publish privacy policy (GitHub Pages or web host)
- [ ] Upload to TestFlight
- [ ] Conduct beta testing
- [ ] Submit to App Store

## Quick Reference

### Essential URLs

**Apple Developer:**
- Developer Portal: https://developer.apple.com/account
- App Store Connect: https://appstoreconnect.apple.com
- TestFlight: https://developer.apple.com/testflight

**Project Resources:**
- GitHub Repo: https://github.com/jacksodj/unicel
- Privacy Policy: [PRIVACY_POLICY.md](PRIVACY_POLICY.md)
- Support Page: [SUPPORT.md](SUPPORT.md)

### Important Paths

**In Project:**
```
/Users/dennisjackson/Code/unicel/
├── src-tauri/
│   ├── tauri.conf.json           # App configuration
│   ├── icons/ios/                # App icons (all sizes)
│   ├── examples/*.usheet         # Example workbooks
│   └── gen/apple/                # Xcode project
│       ├── unicel.xcodeproj      # Open in Xcode
│       └── Assets.xcassets/
│           └── AppIcon.appiconset/  # Icon assets
└── docs/app-store/               # This directory
    ├── screenshots/              # Screenshots for App Store
    └── [all guides]              # Documentation
```

**Build Artifacts:**
```
src-tauri/gen/apple/build/
├── arm64-apple-ios/release/
│   └── bundle/ios/Unicel.ipa    # Release IPA
└── Release-iphoneos/
    └── Unicel.app.dSYM          # Debug symbols
```

### Essential Commands

```bash
# Build release IPA
cd /Users/dennisjackson/Code/unicel
npm run tauri ios build --release

# Open Xcode project
open src-tauri/gen/apple/unicel.xcodeproj

# Check code signing
codesign -dv --verbose=4 path/to/Unicel.app

# List simulators
xcrun simctl list devices

# Boot simulator
xcrun simctl boot "iPhone 15 Pro Max"

# Take screenshot
xcrun simctl io booted screenshot ~/Desktop/screenshot.png

# Upload to TestFlight (command line)
xcrun altool --upload-app \
  -f path/to/Unicel.ipa \
  -t ios \
  -u your@email.com \
  -p app-specific-password
```

## App Store Checklist

Use this checklist to track your progress:

### Pre-TestFlight
- [x] iOS implementation complete
- [x] App builds successfully
- [x] App icons generated
- [x] Bundle identifier configured
- [ ] Distribution certificate installed
- [ ] Provisioning profile configured
- [ ] Release build tested on device

### TestFlight
- [ ] App created in App Store Connect
- [ ] Release IPA built
- [ ] Uploaded to TestFlight
- [ ] Export compliance completed
- [ ] Test information added
- [ ] Internal testers invited
- [ ] No critical crashes reported
- [ ] Feedback collected and addressed

### App Store Preparation
- [ ] Screenshots captured (iPhone 6.7")
- [ ] Screenshots captured (iPhone 6.5")
- [ ] Screenshots captured (iPad 12.9")
- [ ] Privacy policy published online
- [ ] Support page published online
- [ ] App description finalized
- [ ] Keywords optimized
- [ ] Age rating completed
- [ ] App review notes prepared

### App Store Submission
- [ ] All screenshots uploaded
- [ ] All metadata complete
- [ ] Build selected
- [ ] Privacy policy URL verified
- [ ] Support URL verified
- [ ] Contact information correct
- [ ] Submitted for review
- [ ] Approved by Apple
- [ ] Released to App Store

## Troubleshooting

### Common Issues

**Build fails with code signing error:**
- Check distribution certificate is installed
- Verify provisioning profile is valid
- Ensure bundle ID matches exactly
- Try: Product > Clean Build Folder in Xcode

**TestFlight upload rejected:**
- Ensure Info.plist has all required keys
- Check version number is incremented
- Verify export compliance is answered
- Review rejection email for specific issues

**Screenshots wrong size:**
- Use exact device simulators listed in SCREENSHOT_GUIDE.md
- Don't resize screenshots manually
- Verify with: `sips -g pixelWidth -g pixelHeight file.png`

**Privacy policy URL not accessible:**
- Must be HTTPS
- Must be publicly accessible (no login)
- Test in incognito/private browser
- Verify with: `curl -I https://your-url`

**App Store review rejection:**
- Read rejection reason carefully in Resolution Center
- Reference App Store Review Guidelines
- Fix specific issues mentioned
- Respond professionally
- Resubmit when fixed

### Getting Help

**Apple Support:**
- Developer Technical Support: https://developer.apple.com/support/technical/
- App Review: Contact via Resolution Center in App Store Connect
- TestFlight: https://developer.apple.com/support/testflight/

**Community:**
- Apple Developer Forums: https://developer.apple.com/forums/
- Stack Overflow: Tag with [ios], [testflight], [app-store-connect]

**Project-Specific:**
- GitHub Issues: https://github.com/jacksodj/unicel/issues
- Email: support@unicel.app

## Best Practices

### For Successful TestFlight:
1. Test on real devices before uploading
2. Provide clear, detailed test instructions
3. Respond to feedback within 24 hours
4. Fix critical bugs promptly
5. Keep testers informed of updates

### For App Store Success:
1. Capture high-quality, professional screenshots
2. Write clear, compelling app description
3. Use relevant, searchable keywords
4. Provide detailed notes for reviewers
5. Ensure privacy policy is comprehensive
6. Test thoroughly before submission
7. Monitor reviews and respond professionally

### For Long-Term Success:
1. Release updates regularly
2. Fix bugs quickly
3. Listen to user feedback
4. Improve based on metrics
5. Keep app current with iOS updates
6. Optimize App Store listing periodically

## Version History

### Version 0.5.1 (Current)
- Initial iOS release
- Read-only .usheet viewer
- Unit conversion support
- Multi-sheet navigation
- Touch-optimized grid

### Planned Updates
- v0.6.0: Performance improvements
- v0.7.0: Additional gestures and interactions
- v0.8.0: Enhanced formula display
- v1.0.0: Full feature parity with desktop viewer

## Resources

### Apple Documentation
- [App Store Review Guidelines](https://developer.apple.com/app-store/review/guidelines/)
- [TestFlight Beta Testing Guide](https://developer.apple.com/testflight/)
- [App Store Connect Help](https://help.apple.com/app-store-connect/)
- [iOS Human Interface Guidelines](https://developer.apple.com/design/human-interface-guidelines/ios)
- [App Store Marketing Guidelines](https://developer.apple.com/app-store/marketing/guidelines/)

### Tools
- [Fastlane](https://fastlane.tools/) - Automate deployment
- [Transporter App](https://apps.apple.com/app/transporter/id1450874784) - Upload builds
- [TestFlight App](https://apps.apple.com/app/testflight/id899247664) - Beta testing
- [App Store Connect API](https://developer.apple.com/app-store-connect/api/) - Automation

### Tauri Resources
- [Tauri iOS Guide](https://tauri.app/v2/guides/distribution/ios/)
- [Tauri Configuration](https://tauri.app/v2/reference/config/)

## Contact

For questions about Unicel iOS deployment:

- **Email:** dennisjackson@unicel.app
- **GitHub:** https://github.com/jacksodj/unicel
- **Issues:** https://github.com/jacksodj/unicel/issues

---

**Ready to deploy?** Start with [TESTFLIGHT_GUIDE.md](TESTFLIGHT_GUIDE.md)
