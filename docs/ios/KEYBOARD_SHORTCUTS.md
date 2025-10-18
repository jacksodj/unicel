# Keyboard Shortcuts (External Keyboard - iPad)

Unicel supports external keyboards on iPad for enhanced productivity. All shortcuts are disabled on iPhone to avoid conflicts with on-screen keyboard.

## Navigation

| Shortcut | Action | Description |
|----------|--------|-------------|
| `↑` | Navigate Up | Move selection one cell up |
| `↓` | Navigate Down | Move selection one cell down |
| `←` | Navigate Left | Move selection one cell left |
| `→` | Navigate Right | Move selection one cell right |
| `Tab` | Next Cell | Move selection to next cell horizontally |
| `Shift + Tab` | Previous Cell | Move selection to previous cell horizontally |

## Sheet Management

| Shortcut | Action | Description |
|----------|--------|-------------|
| `⌘ + ←` | Previous Sheet | Switch to previous sheet in workbook |
| `⌘ + →` | Next Sheet | Switch to next sheet in workbook |

## Actions

| Shortcut | Action | Description |
|----------|--------|-------------|
| `Esc` | Deselect | Clear current cell selection |
| `⌘ + F` | Search | Open search dialog (coming soon) |

## Implementation Details

### Detection

Keyboard shortcuts are automatically enabled when:
1. Running on iPad (detected via user agent)
2. External keyboard is connected (assumed if iPad)
3. The app is in foreground

### Priority

Keyboard shortcuts take precedence over:
- Touch gestures (when keyboard is active)
- On-screen keyboard input

But yield to:
- System shortcuts (⌘ + Tab, etc.)
- Accessibility features (VoiceOver commands)

### Accessibility

All shortcuts work with:
- VoiceOver enabled
- Full Keyboard Access enabled
- Switch Control enabled

### Testing

Test keyboard shortcuts on:
- iPad with Magic Keyboard
- iPad with Smart Keyboard Folio
- iPad with Bluetooth keyboard
- iPad Pro with external keyboard

### Future Enhancements

Planned shortcuts for future releases:
- `⌘ + O` - Open file
- `⌘ + W` - Close file
- `⌘ + D` - Toggle display preference
- `⌘ + K` - Show keyboard shortcuts help
- `Space` - Toggle cell expansion (long-press equivalent)
- `Enter` - Confirm selection

## Code Reference

Implementation: `src/components/mobile/KeyboardShortcuts.tsx`

Hook usage:
```typescript
import { useKeyboardShortcuts } from './KeyboardShortcuts';

useKeyboardShortcuts({
  onNavigateCell: (direction) => { /* ... */ },
  onNextCell: () => { /* ... */ },
  onPreviousCell: () => { /* ... */ },
  onNextSheet: () => { /* ... */ },
  onPreviousSheet: () => { /* ... */ },
  onDeselect: () => { /* ... */ },
  onSearch: () => { /* ... */ },
  enabled: isTablet, // Only on iPad
});
```

Help component:
```typescript
import { KeyboardShortcutsHelp } from './KeyboardShortcuts';

<KeyboardShortcutsHelp onClose={() => setShowHelp(false)} />
```
