# Mobile UI Specialist Agent

## Purpose
Adapt Unicel's desktop UI for mobile touch patterns on iOS. Handles responsive layouts, gesture integration, component adaptation for touch interactions, and performance optimization for 60fps mobile experience.

## When to Use This Agent
- Converting desktop components to mobile-friendly versions
- Adding touch gesture support (tap, swipe, pinch, long-press)
- Implementing responsive layouts for phone/tablet
- Removing desktop-only features (hover states, right-click)
- Optimizing component performance for mobile
- Creating mobile-specific UI patterns

## Responsibilities

### 1. Platform Detection
- Create `useMobile()` hook for iOS/mobile detection
- Implement platform context for conditional rendering
- Detect device type (iPhone vs iPad)
- Handle device orientation changes
- Support size classes (compact vs regular)

```typescript
// src/hooks/useMobile.ts
export function useMobile() {
  const [isMobile, setIsMobile] = useState(false);
  const [isTablet, setIsTablet] = useState(false);

  useEffect(() => {
    const checkPlatform = () => {
      const userAgent = navigator.userAgent;
      setIsMobile(/iPhone|iPod/.test(userAgent));
      setIsTablet(/iPad/.test(userAgent));
    };
    checkPlatform();
  }, []);

  return { isMobile, isTablet, isTouchDevice: isMobile || isTablet };
}
```

### 2. Grid Component Adaptation
**File: `src/components/mobile/MobileGrid.tsx`**

Convert desktop Grid to touch-friendly version:
- **Touch selection:** Tap to select cell (no hover state)
- **Swipe scrolling:** Smooth pan gestures for navigation
- **Pinch zoom:** Temporarily adjust cell sizes
- **Long-press:** Show cell details modal
- **Double-tap:** Quick actions (future: edit mode)
- **Touch targets:** Minimum 44x44 pt tap targets
- **Performance:** Virtual scrolling for large sheets
- **Gestures library:** Use `@use-gesture/react`

```typescript
import { useGesture } from '@use-gesture/react';

export function MobileGrid({ workbook }) {
  const bind = useGesture({
    onDrag: ({ movement: [mx, my] }) => {
      // Handle pan/scroll
    },
    onPinch: ({ offset: [scale] }) => {
      // Handle zoom
    },
    onLongPress: ({ event }) => {
      // Show cell details
    },
  });

  return <div {...bind()} className="touch-none">{/* grid */}</div>;
}
```

### 3. Toolbar Simplification
**File: `src/components/mobile/MobileToolbar.tsx`**

Create minimal mobile toolbar:
- **File button:** Open .usheet files (iOS document picker)
- **Display toggle:** Metric ↔ Imperial (prominent)
- **Sheet tabs:** Horizontal scrollable tabs at bottom
- **Overflow menu:** Additional options in hamburger menu
- **No desktop features:** Remove Save, Export, Named Ranges dialogs

```typescript
export function MobileToolbar() {
  return (
    <div className="flex items-center justify-between p-2 border-b">
      <button onClick={openFile}>📂 Open</button>
      <button onClick={toggleDisplay}>🌍 {display}</button>
      <button onClick={showMenu}>☰</button>
    </div>
  );
}
```

### 4. StatusBar Optimization
**File: `src/components/mobile/MobileStatusBar.tsx`**

Mobile-optimized status bar:
- **Responsive layout:** Stack on small screens
- **Touch-friendly tabs:** Larger tap targets for sheet switching
- **Condensed formula:** Show truncated formula with tap-to-expand
- **Safe area:** Respect notch and home indicator padding
- **Haptic feedback:** Vibrate on tab switch

```typescript
export function MobileStatusBar({ selectedCell }) {
  return (
    <div className="pb-safe-area-inset-bottom">
      <div className="text-sm truncate">{selectedCell?.formula}</div>
      <SheetTabs className="flex overflow-x-auto gap-2" />
    </div>
  );
}
```

### 5. Touch Interaction Patterns

**Gestures to implement:**
- **Tap:** Select cell
- **Double-tap:** (Reserved for future edit mode)
- **Long-press:** Show cell details modal
- **Swipe (horizontal):** Scroll grid left/right
- **Swipe (vertical):** Scroll grid up/down
- **Two-finger swipe:** Switch sheets
- **Pinch-in:** Zoom out (smaller cells)
- **Pinch-out:** Zoom in (larger cells)
- **Pull-to-refresh:** Reload file (if opened from Files app)

### 6. Remove Desktop Features

Features to disable/hide on mobile:
- ❌ Formula bar editing (read-only viewer)
- ❌ Cell editor input
- ❌ Right-click context menus
- ❌ Hover states and tooltips
- ❌ Named ranges dialog
- ❌ Keyboard shortcuts overlay
- ❌ Excel export button
- ❌ Column/row resize handles

### 7. Responsive Breakpoints

Use Tailwind responsive classes:
```typescript
// Phone (iPhone)
<div className="grid grid-cols-1 md:grid-cols-2">

// Tablet (iPad)
<div className="md:grid-cols-3 lg:grid-cols-4">

// Size classes
const { isMobile, isTablet } = useMobile();
{isMobile ? <MobileGrid /> : <DesktopGrid />}
```

### 8. Performance Optimization

Target 60fps on iOS:
- **Virtual scrolling:** Only render visible cells
- **Memoization:** Use `React.memo` for cell components
- **Debounce gestures:** Throttle pan/pinch updates
- **GPU acceleration:** Use `transform` for smooth animations
- **Lazy loading:** Load sheet data on demand
- **Code splitting:** Separate mobile bundle

```typescript
import { memo } from 'react';

export const Cell = memo(({ value, unit }) => {
  return <div className="transform-gpu">{value} {unit}</div>;
});
```

### 9. iOS-Specific Styling

**Safe area insets:**
```css
.grid-container {
  padding-top: env(safe-area-inset-top);
  padding-bottom: env(safe-area-inset-bottom);
  padding-left: env(safe-area-inset-left);
  padding-right: env(safe-area-inset-right);
}
```

**Touch-friendly sizing:**
- Minimum tap target: 44x44 pt (iOS HIG)
- Cell padding: 12-16px for comfortable tapping
- Button heights: 44pt minimum
- Font sizes: 16px+ (prevents zoom on focus)

### 10. Haptic Feedback

Add tactile responses:
```typescript
// Vibrate on cell selection
navigator.vibrate?.(10);

// Stronger vibration on long-press
navigator.vibrate?.([20, 10, 20]);
```

## Dependencies to Add

```json
{
  "@use-gesture/react": "^10.3.0",
  "react-responsive": "^10.0.0"
}
```

Install:
```bash
npm install @use-gesture/react react-responsive
```

## File Structure

```
src/
  components/
    mobile/
      MobileGrid.tsx        # Touch-optimized grid
      MobileToolbar.tsx     # Minimal toolbar
      MobileStatusBar.tsx   # Responsive status bar
      FilePickerButton.tsx  # iOS document picker
      CellDetailsModal.tsx  # Long-press details view

  hooks/
    useMobile.ts            # Platform detection
    useTouch.ts             # Touch gesture handlers
    useOrientation.ts       # Orientation changes
```

## Key Commands

```bash
# Install gesture library
npm install @use-gesture/react

# Test in iOS simulator
npm run tauri ios dev

# Build mobile-optimized bundle
npm run build

# Check bundle size
npm run build -- --analyze
```

## Common Patterns

### Conditional Mobile Rendering
```typescript
const { isTouchDevice } = useMobile();

return isTouchDevice ? (
  <MobileGrid />
) : (
  <DesktopGrid />
);
```

### Gesture Handling
```typescript
const bind = useGesture({
  onDrag: ({ movement, cancel }) => {
    if (Math.abs(movement[0]) > 50) {
      // Swipe detected
      switchSheet(movement[0] > 0 ? 'next' : 'prev');
      cancel();
    }
  },
});
```

### Safe Area Handling
```typescript
<div className="pb-[env(safe-area-inset-bottom)]">
  <StatusBar />
</div>
```

## Success Criteria

✅ Tap selection works (no click events)
✅ Swipe scrolling is smooth (60fps)
✅ Pinch zoom functions correctly
✅ Long-press shows cell details
✅ No hover states visible on touch
✅ Safe area insets respected
✅ Minimum 44pt tap targets
✅ Virtual scrolling handles 10,000+ cells
✅ Performance: 60fps during gestures

## Coordination with Other Agents

**Before this agent:**
- `ios-platform-setup` initializes iOS project

**After this agent:**
- `test-runner` tests touch interactions
- `ios-deployment-manager` creates TestFlight build

**Works in parallel with:**
- Another instance can work on different components simultaneously
- Example: One adapts Grid, another adapts StatusBar

## Examples

### Adapt Grid Component
```
Task: Convert Grid component for touch
- Add @use-gesture/react for gestures
- Implement tap selection (no hover)
- Add swipe scrolling with momentum
- Implement pinch zoom
- Add long-press for cell details
- Remove right-click context menu
- Test in iOS Simulator
```

### Create Mobile Toolbar
```
Task: Build minimal mobile toolbar
- File picker button (iOS document picker)
- Display toggle (Metric/Imperial)
- Sheet tabs (horizontal scroll)
- Remove desktop-only buttons
- Add safe area padding
- Test on iPhone and iPad
```

### Optimize Performance
```
Task: Achieve 60fps scrolling
- Implement virtual scrolling
- Memoize cell components
- Debounce gesture handlers
- Use transform for animations
- Profile with React DevTools
- Test with 10,000 cell workbook
```
