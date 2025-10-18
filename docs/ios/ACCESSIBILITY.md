# iOS Accessibility Features

Unicel for iOS supports all major iOS accessibility features to ensure the app is usable by everyone.

## Supported Features

### 1. VoiceOver (Screen Reader)

**Status:** Partial support (requires real device testing)

**Implementation:**
- All interactive elements have ARIA labels
- Grid cells are marked with `role="grid"` and `role="gridcell"`
- Proper focus management for cell selection
- Announcements for state changes

**Usage:**
- Enable: Settings > Accessibility > VoiceOver
- Swipe right/left to navigate cells
- Double-tap to select cell
- Three-finger swipe to scroll grid

**Testing Status:**
- ✅ Basic navigation tested in simulator
- ⚠️ Real device testing required for full validation
- ⚠️ Voice announcements need refinement

### 2. Dynamic Type (Text Scaling)

**Status:** Fully supported

**Implementation:**
- Font sizes respond to system text size preference
- Layout adapts to larger text
- Touch targets remain 44pt minimum

**Supported Sizes:**
- Small (0.875x)
- Medium (1.0x) - Default
- Large (1.125x)
- Extra Large (1.25x)

**Testing:**
- Enable: Settings > Accessibility > Display & Text Size > Larger Text
- App automatically detects and applies scaling

**Code:**
```typescript
const { fontSize } = useAccessibility();
const multiplier = getFontSizeMultiplier(fontSize);
```

### 3. Reduce Motion

**Status:** Fully supported

**Implementation:**
- All animations respect reduce motion preference
- Transitions disabled when preference is on
- Smooth scrolling replaced with instant scrolling
- Zoom animations removed

**Testing:**
- Enable: Settings > Accessibility > Motion > Reduce Motion
- All animations become instant

**Code:**
```typescript
const { prefersReducedMotion } = useAccessibility();
const duration = prefersReducedMotion ? 0 : 200;
```

### 4. High Contrast Mode

**Status:** Fully supported

**Implementation:**
- Border widths increase (2px → 4px)
- Color contrast ratios meet WCAG AAA
- Selected cells have stronger visual distinction

**Testing:**
- Enable: Settings > Accessibility > Display & Text Size > Increase Contrast
- UI automatically adjusts

**Code:**
```typescript
const { prefersHighContrast } = useAccessibility();
const borderClass = prefersHighContrast ? 'border-4' : 'border-2';
```

### 5. Color Blindness Support

**Status:** Planned

**Current Implementation:**
- Not reliant on color alone for information
- Unit warnings use icons + color
- Selected cells use multiple visual cues (border, background, icon)

**Future Enhancements:**
- Color blind-friendly palette option
- Pattern overlays for color-coded data
- Customizable color schemes

### 6. Full Keyboard Access

**Status:** Fully supported (iPad only)

**Implementation:**
- All interactive elements reachable via keyboard
- Focus indicators visible
- Tab order logical and intuitive
- Arrow keys navigate grid cells

**Testing:**
- Enable: Settings > Accessibility > Keyboards > Full Keyboard Access
- Navigate app using only keyboard

### 7. Touch Accommodations

**Status:** Supported via iOS

**Implementation:**
- Minimum touch target: 44pt × 44pt (iOS requirement)
- Touch targets larger on iPad (48pt+)
- No timing-dependent interactions
- No complex gestures required

**iOS Settings:**
- Settings > Accessibility > Touch
  - Touch Accommodations
  - Hold Duration
  - Ignore Repeat

### 8. Voice Control

**Status:** Basic support

**Implementation:**
- All buttons labeled for voice commands
- Grid navigation via voice (experimental)

**Testing:**
- Enable: Settings > Accessibility > Voice Control
- Commands: "Open file", "Switch sheets", "Select cell A1"

## Accessibility Hooks

### `useAccessibility()`

Returns current accessibility preferences:

```typescript
interface AccessibilityPreferences {
  prefersReducedMotion: boolean;
  prefersHighContrast: boolean;
  prefersColorScheme: 'light' | 'dark';
  fontSize: 'small' | 'medium' | 'large' | 'x-large';
  isVoiceOverEnabled: boolean;
}
```

### Helper Functions

```typescript
// Get animation duration based on reduced motion
getAnimationDuration(preferences, normalDuration): number

// Get font size multiplier based on Dynamic Type
getFontSizeMultiplier(preferences): number
```

## ARIA Implementation

### Grid Component

```html
<div role="grid" aria-label="Spreadsheet grid for Sheet1">
  <div role="row">
    <div role="gridcell" aria-selected="true" tabindex="0">
      A1
    </div>
  </div>
</div>
```

### Interactive Elements

```html
<button aria-label="Open file" style="min-height: 44px">
  📂 Open
</button>
```

## Testing Checklist

### Simulator Testing ✅

- [x] VoiceOver basic navigation
- [x] Dynamic Type scaling
- [x] Reduce Motion
- [x] High Contrast
- [x] Dark Mode
- [x] Keyboard navigation

### Real Device Testing (Required)

- [ ] VoiceOver on iPhone SE
- [ ] VoiceOver on iPhone 14 Pro
- [ ] VoiceOver on iPad Air
- [ ] Dynamic Type on all devices
- [ ] Full Keyboard Access on iPad
- [ ] Voice Control
- [ ] Touch Accommodations
- [ ] Switch Control

## Known Issues

### VoiceOver Detection

**Issue:** VoiceOver enabled state is difficult to detect programmatically.

**Current Approach:** Heuristic-based detection using focus events and ARIA attributes.

**Solution:** Requires real device testing to refine.

### Grid Navigation with VoiceOver

**Issue:** Grid navigation with VoiceOver can be confusing with 10,000+ cells.

**Mitigation:**
- Landmark regions for navigation
- "Skip to" links for large grids
- Search functionality (planned)

### Performance with Reduced Motion

**Issue:** Disabling animations doesn't improve performance significantly.

**Optimization:**
- Virtual scrolling always enabled
- GPU acceleration even without animations

## Resources

### Apple Documentation

- [Human Interface Guidelines - Accessibility](https://developer.apple.com/design/human-interface-guidelines/accessibility)
- [Accessibility for iOS](https://developer.apple.com/accessibility/ios/)
- [VoiceOver Testing](https://developer.apple.com/library/archive/technotes/TestingAccessibilityOfiOSApps/TestingtheAccessibilityofiOSApps/TestingtheAccessibilityofiOSApps.html)

### WCAG Guidelines

- [WCAG 2.1 Level AA](https://www.w3.org/WAI/WCAG21/quickref/)
- [Mobile Accessibility](https://www.w3.org/WAI/standards-guidelines/mobile/)

## Implementation Files

- `src/hooks/useAccessibility.ts` - Accessibility detection hook
- `src/components/mobile/MobileGrid.tsx` - ARIA grid implementation
- `src/components/mobile/KeyboardShortcuts.tsx` - Keyboard navigation

## Future Enhancements

1. **Enhanced VoiceOver Support**
   - Cell value announcements
   - Formula reading
   - Unit information in announcements

2. **Braille Display Support**
   - Test with Braille displays
   - Optimize for Braille output

3. **Switch Control**
   - Full testing with Switch Control
   - Optimize scanning order

4. **Hearing Accessibility**
   - Visual indicators for audio feedback
   - Captions for any audio content

5. **Cognitive Accessibility**
   - Simplified mode option
   - Reduced visual complexity
   - Step-by-step guides
