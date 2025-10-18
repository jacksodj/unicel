# iOS Deployment Quick Start

**Get Unicel Viewer on the App Store in 4 weeks.**

## Prerequisites Checklist

Before you begin, ensure you have:

- [ ] **Apple Developer account** ($99/year) - [Enroll here](https://developer.apple.com/programs/)
- [ ] **Xcode 15.0+** installed on Mac
- [ ] **iOS device** for testing (optional but recommended)
- [ ] **2-5 people** available for TestFlight beta testing

**Time commitment:** 2-3 hours setup + 1-2 weeks testing

## The 4-Week Plan

### Week 1: TestFlight Beta (2-3 hours)

**Day 1-2: Code Signing Setup**
1. Log in to [Apple Developer Portal](https://developer.apple.com/account)
2. Create distribution certificate
3. Download and install in Keychain
4. Create App Store provisioning profile
5. Open Xcode project and configure signing

**Day 3: Build and Upload (30 mins)**
```bash
cd /Users/dennisjackson/Code/unicel
./scripts/ios-build-release.sh
```

Then follow prompts to upload via Xcode or Transporter.

**Day 3-7: TestFlight Testing**
- Wait for Apple processing (10-30 mins)
- Invite 2-5 internal testers
- Collect feedback
- Fix critical bugs (if any)

📚 **Detailed guide:** [TESTFLIGHT_GUIDE.md](TESTFLIGHT_GUIDE.md)

### Week 2: Beta Testing & Iteration (1-2 hours)

**Continue testing:**
- Monitor crash reports
- Respond to tester feedback
- Fix bugs if needed
- Upload new build if changes made

**What testers should verify:**
- App launches successfully
- File opening works
- Unit conversion toggle works
- Multi-sheet navigation works
- No crashes or major bugs

### Week 3: App Store Preparation (3-4 hours)

**Day 15-16: Capture Screenshots (1 hour)**
```bash
# Follow screenshot guide
open docs/app-store/SCREENSHOT_GUIDE.md
```

Capture 5 screenshots for each device size:
- iPhone 15 Pro Max (6.7")
- iPhone 11 Pro Max (6.5")
- iPad Pro 12.9"

**Day 17: Publish Privacy Policy (30 mins)**

**Option 1: GitHub Pages (Recommended)**
```bash
# Enable GitHub Pages in repo settings
# Your privacy policy will be at:
# https://yourusername.github.io/unicel/app-store/PRIVACY_POLICY.html
```

**Option 2: Custom Domain**
Upload `PRIVACY_POLICY.md` (as HTML) to your website.

**Day 18: Complete Metadata (1 hour)**
- Log in to [App Store Connect](https://appstoreconnect.apple.com)
- Fill in all fields from [APP_STORE_METADATA.md](APP_STORE_METADATA.md)
- Upload screenshots
- Enter privacy policy URL
- Save everything

📚 **Detailed guide:** [APP_STORE_SUBMISSION_GUIDE.md](APP_STORE_SUBMISSION_GUIDE.md)

### Week 4: Submission & Launch (1 hour + wait)

**Day 22: Submit for Review (15 mins)**
1. Review all metadata one last time
2. Select your TestFlight build
3. Click "Submit for Review"

**Day 23-25: Wait for Review**
- Check email daily
- Apple typically reviews within 1-3 days
- Monitor App Store Connect for status

**Day 26: Release! (5 mins)**
1. Receive approval notification
2. Click "Release this Version"
3. App goes live within 2-3 hours

**Post-Launch:**
- Monitor reviews and respond
- Track metrics in App Store Connect
- Plan first update

## Quick Commands

```bash
# Build release IPA
cd /Users/dennisjackson/Code/unicel
./scripts/ios-build-release.sh

# Open Xcode project
open src-tauri/gen/apple/unicel.xcodeproj

# Capture screenshot (in iOS Simulator)
xcrun simctl io booted screenshot ~/Desktop/screenshot.png

# List available simulators
xcrun simctl list devices
```

## Essential Files

**Documentation:**
- [README.md](README.md) - Complete overview
- [TESTFLIGHT_GUIDE.md](TESTFLIGHT_GUIDE.md) - TestFlight deployment
- [SCREENSHOT_GUIDE.md](SCREENSHOT_GUIDE.md) - Screenshot capture
- [APP_STORE_SUBMISSION_GUIDE.md](APP_STORE_SUBMISSION_GUIDE.md) - Final submission

**Metadata:**
- [APP_STORE_METADATA.md](APP_STORE_METADATA.md) - Copy/paste ready
- [PRIVACY_POLICY.md](PRIVACY_POLICY.md) - Publish online
- [SUPPORT.md](SUPPORT.md) - Publish online

**Tracking:**
- [DEPLOYMENT_CHECKLIST.md](DEPLOYMENT_CHECKLIST.md) - Track progress
- [DEPLOYMENT_SUMMARY.md](DEPLOYMENT_SUMMARY.md) - Complete overview

## Need Help?

**Common Issues:**

**"Code signing failed"**
→ Check distribution certificate is installed
→ Open Xcode → Preferences → Accounts → View Details
→ Download Manual Profiles

**"TestFlight processing stuck"**
→ Wait up to 1 hour
→ Check email for issues
→ Contact Apple Support if > 2 hours

**"App Store rejected"**
→ Read rejection reason in Resolution Center
→ Fix specific issues mentioned
→ See [APP_STORE_SUBMISSION_GUIDE.md](APP_STORE_SUBMISSION_GUIDE.md) - "If Rejected" section
→ Resubmit

**"Screenshots wrong size"**
→ Use exact simulators listed in [SCREENSHOT_GUIDE.md](SCREENSHOT_GUIDE.md)
→ Don't resize manually
→ Verify with: `sips -g pixelWidth -g pixelHeight file.png`

## Progress Tracking

Use [DEPLOYMENT_CHECKLIST.md](DEPLOYMENT_CHECKLIST.md) to track your progress:

**Phase 1:** Pre-Deployment Setup
**Phase 2:** Build and TestFlight
**Phase 3:** App Store Preparation
**Phase 4:** App Store Submission
**Phase 5:** App Review Process
**Phase 6:** Post-Launch
**Phase 7:** First Update Planning

Print it out and check off items as you complete them!

## Support

**Apple Support:**
- Developer Support: https://developer.apple.com/support/
- App Store Connect: https://appstoreconnect.apple.com

**Project Issues:**
- GitHub: https://github.com/jacksodj/unicel/issues
- Email: support@unicel.app

## What's Already Done

✅ All app icons generated (1024x1024 + all device sizes)
✅ Example workbooks ready (AWS, Construction, etc.)
✅ Bundle ID configured (com.unicel.app)
✅ Privacy policy written
✅ Support page written
✅ App Store description written
✅ Keywords optimized
✅ Build script ready
✅ Comprehensive documentation complete

## What You Need to Do

**Week 1:**
1. Set up code signing (1-2 hours)
2. Build release IPA (30 mins)
3. Upload to TestFlight (15 mins)
4. Invite testers (10 mins)

**Week 2:**
5. Monitor testing (daily check-ins)
6. Fix bugs if needed

**Week 3:**
7. Capture screenshots (1 hour)
8. Publish privacy policy (30 mins)
9. Complete App Store metadata (1 hour)

**Week 4:**
10. Submit for review (15 mins)
11. Wait for approval (1-3 days)
12. Release to App Store (5 mins)

**Total active work: ~6-8 hours over 4 weeks**

---

**Ready to start?** Begin with setting up your Apple Developer account and code signing, then follow [TESTFLIGHT_GUIDE.md](TESTFLIGHT_GUIDE.md).

**Questions?** All answers are in the comprehensive guides in this directory.

**Good luck! 🚀**
