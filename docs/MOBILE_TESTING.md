# Mobile iOS Testing Guide

This guide outlines testing procedures for the iOS mobile viewer implementation.

## Testing Environment

- **Platform**: iOS (iPhone and iPad)
- **Test Method**: iOS Simulator via Xcode
- **Build Command**: `npm run tauri:ios:dev`
- **Release Build**: `npm run tauri:ios:build`

## Test Workbooks

All test files are located in `/Users/dennisjackson/Code/unicel/examples/`

### 1. Construction Estimator (`construction_estimator.usheet`)
- **Size**: 16KB
- **Test Focus**: Imperial units, formulas, cell warnings
- **Expected Behavior**:
  - Opens without errors
  - Displays construction materials with costs per ft
  - Formulas calculate correctly
  - Unit display toggles between ft/m properly
  - Warnings show for formula errors

**Test Cases**:
- [ ] Open file via iOS file picker
- [ ] Verify all cells render correctly
- [ ] Test tap gesture for cell selection
- [ ] Test long-press for cell details popover
- [ ] Toggle Metric/Imperial display
- [ ] Verify formulas display in status bar
- [ ] Check warning indicators for error cells
- [ ] Test pinch-to-zoom (0.5x to 2x)
- [ ] Test smooth scrolling (60fps)

### 2. AWS Cost Estimator (`aws_cost_estimator.usheet`)
- **Size**: 28KB
- **Test Focus**: Currency units, rate calculations, multiple sheets
- **Expected Behavior**:
  - Multiple sheets accessible via dropdown
  - Cost calculations in USD
  - Rate formulas ($/hr, $/month) work correctly
  - Sheet switching is smooth

**Test Cases**:
- [ ] Open file successfully
- [ ] Switch between sheets via toolbar dropdown
- [ ] Verify currency displays correctly
- [ ] Test rate unit conversions ($/hr -> $/month)
- [ ] Check formula recalculation
- [ ] Test virtual scrolling with larger dataset
- [ ] Verify haptic feedback on sheet change

### 3. Investment Portfolio Tracker (`investment_portfolio.usheet`)
- **Size**: 31KB
- **Test Focus**: Financial calculations, percentage units, mixed units
- **Expected Behavior**:
  - Percentage values display correctly (15% not 0.15)
  - Stock prices in USD
  - Portfolio calculations accurate
  - Multiple sheets for different portfolios

**Test Cases**:
- [ ] Open file without errors
- [ ] Verify percentage formatting
- [ ] Test currency unit display
- [ ] Check calculation accuracy
- [ ] Test sheet navigation
- [ ] Verify selected cell details in status bar

### 4. Formula Functions Showcase (`formula_functions_showcase.usheet`)
- **Size**: 48KB (largest test file)
- **Test Focus**: Performance, complex formulas, all function types
- **Expected Behavior**:
  - All formula functions evaluate correctly
  - Large file loads in <2 seconds
  - Smooth scrolling maintained (60fps)
  - Virtual scrolling handles 10,000+ cells efficiently

**Test Cases**:
- [ ] Open large file (measure load time)
- [ ] Scroll through all cells smoothly
- [ ] Verify virtual scrolling only renders visible cells
- [ ] Test pinch zoom performance
- [ ] Check memory usage doesn't grow excessively
- [ ] Verify all formula types display correctly
- [ ] Test cell selection responsiveness

## Performance Benchmarks

### Target Metrics
- **File Open**: <2 seconds (for 50KB file)
- **Sheet Switch**: <500ms
- **Scroll Performance**: 60fps (16ms per frame)
- **Touch Response**: <100ms
- **Zoom Performance**: Smooth at all levels (0.5x-2x)
- **Memory Usage**: <150MB for 10,000 cells

### Measuring Performance

Use iOS Instruments to measure:
```bash
# Profile with Xcode Instruments
1. Build release: npm run tauri:ios:build --release
2. Open in Xcode: open src-tauri/gen/apple/*.xcodeproj
3. Product -> Profile (Cmd+I)
4. Select instrument:
   - Time Profiler (CPU usage)
   - Allocations (memory usage)
   - Core Animation (FPS)
```

## Gesture Testing

### Touch Gestures
| Gesture | Expected Behavior | Test Result |
|---------|-------------------|-------------|
| Tap | Select cell, show in status bar | ☐ Pass ☐ Fail |
| Long-press | Show cell details popover | ☐ Pass ☐ Fail |
| Swipe horizontal | Scroll grid left/right | ☐ Pass ☐ Fail |
| Swipe vertical | Scroll grid up/down | ☐ Pass ☐ Fail |
| Pinch in | Zoom out (0.5x min) | ☐ Pass ☐ Fail |
| Pinch out | Zoom in (2x max) | ☐ Pass ☐ Fail |
| Double-tap | (Reserved for future) | N/A |

### Haptic Feedback
| Action | Expected Haptic | Test Result |
|--------|-----------------|-------------|
| Cell tap | Light (10ms) | ☐ Pass ☐ Fail |
| Sheet change | Medium (20ms) | ☐ Pass ☐ Fail |
| Display toggle | Light (10ms) | ☐ Pass ☐ Fail |
| File open success | Success pattern | ☐ Pass ☐ Fail |
| Error | Error pattern | ☐ Pass ☐ Fail |

## UI/UX Testing

### Safe Area Insets
Test on devices with notch (iPhone X and later):
- [ ] Toolbar respects top safe area (notch)
- [ ] Status bar respects bottom safe area (home indicator)
- [ ] Content doesn't get hidden by notch
- [ ] Landscape mode safe areas correct

### Orientation
- [ ] Portrait mode layouts correctly
- [ ] Landscape mode layouts correctly
- [ ] Orientation change doesn't lose state
- [ ] Grid adjusts to new dimensions

### Dark Mode (Future)
- [ ] Light mode displays correctly
- [ ] (Dark mode not yet implemented)

## Error Handling

### Test Error Scenarios
1. **Corrupt File**:
   - Create invalid .usheet file
   - Verify error message displays
   - Check "Try Another File" button works

2. **Missing File**:
   - Open non-existent file path
   - Verify graceful error handling

3. **Backend Error**:
   - Simulate backend crash
   - Verify error boundary catches it
   - Check error UI displays

4. **Network Issues** (if applicable):
   - Test with airplane mode
   - Verify offline functionality

## Accessibility

### VoiceOver (Screen Reader)
- [ ] File picker button has aria-label
- [ ] Grid cells are accessible
- [ ] Status bar content is announced
- [ ] Toolbar buttons have proper labels

### Dynamic Type
- [ ] Text scales with system font size
- [ ] Layouts remain usable at larger sizes
- [ ] Touch targets maintain 44x44pt minimum

## Device Coverage

### iPhone Models
- [ ] iPhone SE (small screen, 4.7")
- [ ] iPhone 15 (standard screen, 6.1")
- [ ] iPhone 15 Pro Max (large screen, 6.7")

### iPad Models
- [ ] iPad Air (10.9")
- [ ] iPad Pro 11"
- [ ] iPad Pro 12.9"

## Known Issues

Track issues found during testing:

1. **Issue**: [Description]
   - **Severity**: Critical/High/Medium/Low
   - **Device**: [Device model]
   - **Steps**: [Reproduction steps]
   - **Status**: Open/Fixed

## Test Automation (Future)

Future testing can be automated with:
- XCTest for UI testing
- Detox for React Native-style E2E tests
- Appium for cross-platform mobile testing

## Sign-off

### Test Results Summary

| Test Category | Pass Rate | Notes |
|---------------|-----------|-------|
| File Opening | __/4 | |
| Gestures | __/6 | |
| Performance | __/6 | |
| Error Handling | __/4 | |
| Accessibility | __/4 | |
| Device Coverage | __/6 | |

**Tested By**: _________________
**Date**: _________________
**Build Version**: _________________
**Overall Status**: ☐ Pass ☐ Fail ☐ Needs Work

## Next Steps

After testing:
1. Document all issues in GitHub Issues
2. Prioritize fixes based on severity
3. Re-test after fixes implemented
4. Proceed to TestFlight beta testing with real devices
