# iOS Device Testing Matrix

This document provides a comprehensive testing matrix for Unicel iOS across all supported devices, orientations, and configurations.

## Supported Devices

### iPhone Models

| Device | Screen Size | Resolution (pts) | Year | Status |
|--------|-------------|------------------|------|--------|
| iPhone SE (3rd gen) | 4.7" | 375 × 667 | 2022 | ✅ Supported (minimum) |
| iPhone 13 | 6.1" | 390 × 844 | 2021 | ✅ Supported |
| iPhone 14 | 6.1" | 390 × 844 | 2022 | ✅ Supported |
| iPhone 14 Pro | 6.1" | 393 × 852 | 2022 | ✅ Supported |
| iPhone 14 Pro Max | 6.7" | 430 × 932 | 2022 | ✅ Supported |
| iPhone 15 | 6.1" | 393 × 852 | 2023 | ✅ Supported |
| iPhone 15 Pro Max | 6.7" | 430 × 932 | 2023 | ✅ Supported |

### iPad Models

| Device | Screen Size | Resolution (pts) | Year | Status |
|--------|-------------|------------------|------|--------|
| iPad Mini (6th gen) | 8.3" | 744 × 1133 | 2021 | ✅ Supported |
| iPad (10th gen) | 10.9" | 820 × 1180 | 2022 | ✅ Supported |
| iPad Air (5th gen) | 10.9" | 820 × 1180 | 2022 | ✅ Supported (recommended) |
| iPad Pro 11" (4th gen) | 11" | 834 × 1194 | 2022 | ✅ Supported |
| iPad Pro 12.9" (6th gen) | 12.9" | 1024 × 1366 | 2022 | ✅ Supported (best experience) |

## Testing Matrix

### Basic Functionality Tests

| Test Case | iPhone SE | iPhone 14 | iPhone 14 Pro Max | iPad Mini | iPad Air | iPad Pro 12.9" |
|-----------|:---------:|:---------:|:-----------------:|:---------:|:--------:|:--------------:|
| **File Opening** |
| Open .usheet file | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Open via Files app | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Open via drag & drop (iPad) | N/A | N/A | N/A | ⬜ | ⬜ | ⬜ |
| **Grid Interaction** |
| Tap to select cell | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Long-press for details | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Swipe to scroll | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Pinch to zoom | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| **Sheet Management** |
| Switch sheets | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Sheet dropdown | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| **Display Preference** |
| Toggle Metric/Imperial | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Unit conversion display | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |

### Orientation Tests

| Test Case | iPhone SE | iPhone 14 | iPhone 14 Pro Max | iPad Mini | iPad Air | iPad Pro 12.9" |
|-----------|:---------:|:---------:|:-----------------:|:---------:|:--------:|:--------------:|
| **Portrait Mode** |
| Grid layout | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Toolbar layout | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Safe areas | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| **Landscape Mode** |
| Grid layout | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Toolbar layout | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| More visible columns | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Split view (iPad) | N/A | N/A | N/A | ⬜ | ⬜ | ⬜ |
| **Rotation Transitions** |
| Portrait → Landscape | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Landscape → Portrait | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Rapid rotation | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| State preservation | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |

### iPad-Specific Features

| Test Case | iPad Mini | iPad Air | iPad Pro 12.9" |
|-----------|:---------:|:--------:|:--------------:|
| **Split View** |
| Grid + Details panel | ⬜ | ⬜ | ⬜ |
| Resize split panes | ⬜ | ⬜ | ⬜ |
| Cell details display | ⬜ | ⬜ | ⬜ |
| **Keyboard Shortcuts** |
| Arrow key navigation | ⬜ | ⬜ | ⬜ |
| Tab/Shift+Tab | ⬜ | ⬜ | ⬜ |
| Cmd+Left/Right (sheets) | ⬜ | ⬜ | ⬜ |
| Esc to deselect | ⬜ | ⬜ | ⬜ |
| **External Keyboard** |
| Magic Keyboard | ⬜ | ⬜ | ⬜ |
| Smart Keyboard Folio | ⬜ | ⬜ | ⬜ |
| Bluetooth keyboard | ⬜ | ⬜ | ⬜ |
| **Multitasking** |
| Split View with other apps | ⬜ | ⬜ | ⬜ |
| Slide Over | ⬜ | ⬜ | ⬜ |
| Picture in Picture | ⬜ | ⬜ | ⬜ |
| **Apple Pencil** |
| Tap to select (if supported) | ⬜ | ⬜ | ⬜ |
| Scroll with pencil | ⬜ | ⬜ | ⬜ |

### Accessibility Tests

| Test Case | iPhone SE | iPhone 14 | iPhone 14 Pro Max | iPad Mini | iPad Air | iPad Pro 12.9" |
|-----------|:---------:|:---------:|:-----------------:|:---------:|:--------:|:--------------:|
| **VoiceOver** |
| Grid navigation | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Cell selection | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Button activation | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Value announcements | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| **Dynamic Type** |
| Small text | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Medium text (default) | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Large text | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Extra Large text | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| **Reduce Motion** |
| Disabled animations | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Instant transitions | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| **High Contrast** |
| Increased borders | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Enhanced colors | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| **Full Keyboard Access** |
| Tab navigation | N/A | N/A | N/A | ⬜ | ⬜ | ⬜ |
| Focus indicators | N/A | N/A | N/A | ⬜ | ⬜ | ⬜ |
| **Voice Control** |
| Voice commands | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Grid overlay | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |

### Performance Tests

| Test Case | iPhone SE | iPhone 14 | iPhone 14 Pro Max | iPad Mini | iPad Air | iPad Pro 12.9" |
|-----------|:---------:|:---------:|:-----------------:|:---------:|:--------:|:--------------:|
| **Small Workbook (100 cells)** |
| Load time < 500ms | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Scroll at 60 FPS | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Memory < 50MB | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| **Medium Workbook (1,000 cells)** |
| Load time < 1s | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Scroll at 60 FPS | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Memory < 100MB | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| **Large Workbook (10,000 cells)** |
| Load time < 2s | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Scroll at 60 FPS | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Memory < 150MB | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| **Stress Test (50,000+ cells)** |
| Load time < 5s | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Scroll at 30+ FPS | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| No crash | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |

### Edge Cases & Error Handling

| Test Case | iPhone SE | iPhone 14 | iPhone 14 Pro Max | iPad Mini | iPad Air | iPad Pro 12.9" |
|-----------|:---------:|:---------:|:-----------------:|:---------:|:--------:|:--------------:|
| **App Lifecycle** |
| Background → Foreground | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Low memory warning | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| App termination | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| State restoration | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| **Network Conditions** |
| Offline mode | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Slow network | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Network lost during use | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| **File Errors** |
| Corrupted .usheet file | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Missing file | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Invalid JSON | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| **Display Issues** |
| Empty cells | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Missing units | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Invalid formulas | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Circular references | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |

## Testing Checklist by Device Category

### iPhone Testing (Minimum: iPhone SE)

**Priority:** HIGH - Smallest screen, lowest memory

- [ ] File opens successfully
- [ ] All gestures work (tap, swipe, pinch, long-press)
- [ ] Text readable at all sizes
- [ ] Touch targets ≥ 44pt
- [ ] Safe areas respected (notch)
- [ ] Portrait mode perfect
- [ ] Landscape mode usable
- [ ] 60 FPS scrolling
- [ ] No memory warnings

### iPad Testing (Recommended: iPad Air)

**Priority:** HIGH - Primary target device

- [ ] All iPhone tests pass
- [ ] Split view working
- [ ] Cell details panel displays
- [ ] External keyboard shortcuts work
- [ ] Larger grid cells
- [ ] More columns visible
- [ ] Multitasking (Split View, Slide Over)
- [ ] 60 FPS with split view

## Test Data Files

### Sample Workbooks for Testing

1. **small-test.usheet** (100 cells)
   - Basic data types
   - Simple formulas
   - Few units

2. **medium-test.usheet** (1,000 cells)
   - Mixed formulas
   - Multiple sheets
   - Various units

3. **large-test.usheet** (10,000 cells)
   - Complex dependencies
   - Currency conversions
   - Table aggregations

4. **stress-test.usheet** (50,000+ cells)
   - Maximum capacity test
   - Deep formula nesting
   - All features enabled

5. **error-test.usheet**
   - Circular references
   - Invalid formulas
   - Missing units
   - Warnings

## Testing Tools

### iOS Simulator

```bash
# List available simulators
xcrun simctl list devices

# Boot specific simulator
xcrun simctl boot "iPhone SE (3rd generation)"
xcrun simctl boot "iPad Air (5th generation)"
xcrun simctl boot "iPad Pro (12.9-inch) (6th generation)"

# Run app in simulator
npm run tauri ios dev
```

### Accessibility Inspector

```bash
# Open Accessibility Inspector in Xcode
# Xcode > Open Developer Tool > Accessibility Inspector
# Use to test VoiceOver, color contrast, etc.
```

### Performance Profiling

```bash
# Profile in Instruments
# Product > Profile (⌘ + I)
# Choose Time Profiler, Allocations, or Core Animation
```

## Sign-Off Checklist

Before submitting for TestFlight:

- [ ] All critical tests pass on iPhone SE
- [ ] All critical tests pass on iPad Air
- [ ] Performance targets met
- [ ] Accessibility features verified
- [ ] No crashes in 30-minute use session
- [ ] Orientation changes smooth
- [ ] Memory usage acceptable
- [ ] VoiceOver usable (basic functionality)

## Bug Reporting Template

```markdown
## Bug Report

**Device:** iPhone 14 Pro Max / iPad Air 5th gen
**iOS Version:** 17.0
**App Version:** 0.5.1
**Orientation:** Portrait / Landscape

**Steps to Reproduce:**
1. Open app
2. Load large-test.usheet
3. Swipe rapidly left
4. ...

**Expected Behavior:**
Smooth scrolling at 60 FPS

**Actual Behavior:**
Frame rate drops to ~30 FPS, stuttering visible

**Screenshots/Video:**
[Attach here]

**Additional Context:**
Happens only with 10k+ cell workbooks
```

## Testing Schedule

### Week 28 (10.23-10.29): Simulator Testing

- [ ] Run all tests in iOS Simulator
- [ ] Test all device sizes
- [ ] Test all orientations
- [ ] Verify responsive breakpoints
- [ ] Document issues found

### Week 29: Real Device Testing

- [ ] Acquire test devices (minimum iPhone SE + iPad Air)
- [ ] Install TestFlight build
- [ ] Run full testing matrix
- [ ] Profile performance with Instruments
- [ ] Test accessibility features

### Week 30: Bug Fixes & Iteration

- [ ] Fix critical bugs from Week 29
- [ ] Re-test on real devices
- [ ] Verify fixes don't regress other tests
- [ ] Update documentation

---

**Legend:**
- ✅ = Test passed
- ⚠️ = Test passed with minor issues
- ❌ = Test failed
- ⬜ = Not yet tested
- N/A = Not applicable to this device

**Last Updated:** 2025-10-17
**Next Review:** After real device testing (Week 29)
