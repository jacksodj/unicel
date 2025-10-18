# Quick Reference - Unicel iOS App Store Submission

**Fast reference for Week 29 deliverables and next steps**

---

## Status: READY FOR SUBMISSION ✓

All preparation complete. 3 tasks require Apple Developer account.

---

## Quick Commands

### Capture Screenshots (30 min)
```bash
./scripts/capture_ios_screenshots.sh
```
Output: `~/Desktop/unicel-screenshots/`

### Build Release IPA (10 min)
```bash
./scripts/build_ios_release.sh
```
Output: `src-tauri/gen/apple/build/Release-iphoneos/unicel.ipa`

### Verify Assets
```bash
# Check app icons
ls -lh src-tauri/gen/apple/Assets.xcassets/AppIcon.appiconset/*.png

# Check screenshots
ls -lh ~/Desktop/unicel-screenshots/*/

# Check IPA
ls -lh src-tauri/gen/apple/build/Release-iphoneos/*.ipa
```

---

## Documentation Quick Links

| Need | Read |
|------|------|
| **Overview** | `WEEK_29_COMPLETION_GUIDE.md` |
| **Manual steps** | `MANUAL_STEPS_REQUIRED.md` |
| **Metadata** | `APP_STORE_METADATA.md` |
| **Screenshots** | `SCREENSHOT_GUIDE.md` |
| **TestFlight** | `TESTFLIGHT_GUIDE.md` |
| **Submission** | `APP_STORE_SUBMISSION_GUIDE.md` |

---

## App Store URLs to Use

**Support URL:**
```
https://github.com/jacksodj/unicel/blob/main/docs/app-store/SUPPORT.md
```

**Privacy Policy URL:**
```
https://github.com/jacksodj/unicel/blob/main/docs/app-store/PRIVACY_POLICY.md
```

**Marketing URL:**
```
https://github.com/jacksodj/unicel
```

---

## Key Metadata

**App Name:** Unicel

**Subtitle:** Unit-Aware Spreadsheet Viewer

**Keywords (96 chars):**
```
spreadsheet,units,calculator,viewer,engineering,metric,imperial,conversion,productivity,formula
```

**Promotional Text (170 chars):**
```
View unit-aware spreadsheets on your iPhone and iPad. Toggle between Metric and Imperial units instantly. Perfect for engineers and data professionals.
```

**Category:** Productivity (Primary), Business (Secondary)

**Age Rating:** 4+ (All Ages)

**Price:** Free

---

## Screenshot Requirements

| Device | Size (pixels) | Quantity |
|--------|--------------|----------|
| iPhone 6.7" | 1290 × 2796 | 5 |
| iPhone 6.5" | 1242 × 2688 | 5 |
| iPad 12.9" | 2048 × 2732 | 5 |

**Screenshot Order:**
1. Home screen
2. Grid view with data
3. Unit conversion toggle
4. Multi-sheet navigation
5. Formula detail view

---

## Upload to TestFlight

### Option 1: Xcode
```
1. Open Xcode
2. Window > Organizer
3. Drag IPA to Archives
4. Distribute App > App Store Connect > Upload
```

### Option 2: Command Line
```bash
# Generate app-specific password first:
# https://appleid.apple.com/account/manage

xcrun altool --upload-app \
  -f src-tauri/gen/apple/build/Release-iphoneos/unicel.ipa \
  -t ios \
  -u your@email.com \
  -p "xxxx-xxxx-xxxx-xxxx"
```

---

## App Review Notes

Copy-paste this into "Notes for Reviewer":

```
Thank you for reviewing Unicel Viewer!

WHAT THIS APP DOES:
Unicel Viewer is a read-only spreadsheet viewer for .usheet files.

HOW TO TEST:
1. Launch the app
2. Tap "Open Example"
3. Try "AWS Cost Estimator" or "Construction Estimator"
4. Toggle between Metric and Imperial units
5. Navigate between sheets
6. Tap cells to view formulas

KEY FEATURES TO VERIFY:
• File opening works
• Unit conversion toggle works
• Multi-sheet navigation works
• Spreadsheet grid is responsive
• App works offline

PRIVACY:
No data collection. All processing is local.

EXPORT COMPLIANCE:
Uses only standard iOS encryption.

Contact: dennisjackson@unicel.app
```

---

## Export Compliance

**Question:** Does your app use encryption?

**Answer:** No

**Explanation:** Uses only standard iOS data protection APIs.

---

## Timeline

| Step | Time |
|------|------|
| Capture screenshots | 30 min |
| Build IPA | 10 min |
| Upload to TestFlight | 30 min |
| Processing | 15 min |
| Enter metadata | 1 hour |
| Submit for review | 5 min |
| **User time** | **~2.5 hours** |
| Apple review | 2-4 days |

---

## Checklist

### Before Submission
- [ ] Screenshots captured (run script)
- [ ] IPA built (run script)
- [ ] Privacy policy URL accessible
- [ ] Support URL accessible
- [ ] Review all metadata

### TestFlight
- [ ] IPA uploaded
- [ ] Processing complete
- [ ] Test notes added
- [ ] Internal testers invited
- [ ] No crashes reported

### App Store
- [ ] All screenshots uploaded
- [ ] All metadata entered
- [ ] Build selected
- [ ] URLs verified
- [ ] Review notes added
- [ ] Export compliance answered
- [ ] Submitted for review

### Post-Submission
- [ ] Monitor review status
- [ ] Respond to any questions
- [ ] Release when approved
- [ ] Monitor user reviews

---

## Troubleshooting

**Build fails:**
- Check code signing certificates
- Verify provisioning profile
- Run: `cargo clean && cargo build`

**Upload fails:**
- Verify app-specific password
- Check internet connection
- Try Xcode method instead

**Screenshots wrong size:**
- Use exact simulator models
- Don't resize manually
- Recapture with correct device

**Review rejected:**
- Read rejection details
- Fix specific issues mentioned
- Respond professionally
- Resubmit when fixed

---

## Support

**Documentation:** `docs/app-store/`

**Scripts:** `scripts/`

**Apple:** https://appstoreconnect.apple.com

**Issues:** https://github.com/jacksodj/unicel/issues

**Email:** dennisjackson@unicel.app

---

## Files Created This Week

- `scripts/capture_ios_screenshots.sh` - Screenshot automation
- `scripts/build_ios_release.sh` - Build automation
- `docs/app-store/WEEK_29_COMPLETION_GUIDE.md` - Week overview
- `docs/app-store/MANUAL_STEPS_REQUIRED.md` - App Store guide
- `docs/app-store/WEEK_29_SUMMARY.md` - Executive summary
- `docs/app-store/QUICK_REFERENCE.md` - This document

---

**Updated:** October 18, 2025
**Status:** Ready for submission
**Next:** Run screenshot script, build IPA, follow manual guide
