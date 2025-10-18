/**
 * Keyboard shortcuts for external keyboards (iPad)
 *
 * Shortcuts:
 * - Arrow keys: Navigate cells
 * - Tab/Shift+Tab: Move between cells horizontally
 * - Cmd+Left/Right: Switch sheets
 * - Esc: Deselect cell
 * - Cmd+F: Search (placeholder for future)
 */

import { useEffect } from 'react';

export interface KeyboardShortcutsProps {
  onNavigateCell?: (direction: 'up' | 'down' | 'left' | 'right') => void;
  onNextCell?: () => void;
  onPreviousCell?: () => void;
  onNextSheet?: () => void;
  onPreviousSheet?: () => void;
  onDeselect?: () => void;
  onSearch?: () => void;
  enabled?: boolean;
}

export function useKeyboardShortcuts({
  onNavigateCell,
  onNextCell,
  onPreviousCell,
  onNextSheet,
  onPreviousSheet,
  onDeselect,
  onSearch,
  enabled = true,
}: KeyboardShortcutsProps) {
  useEffect(() => {
    if (!enabled) return;

    const handleKeyDown = (event: KeyboardEvent) => {
      const { key, metaKey, shiftKey } = event;

      // Arrow key navigation
      if (key === 'ArrowUp' && onNavigateCell) {
        event.preventDefault();
        onNavigateCell('up');
        return;
      }

      if (key === 'ArrowDown' && onNavigateCell) {
        event.preventDefault();
        onNavigateCell('down');
        return;
      }

      if (key === 'ArrowLeft' && !metaKey && onNavigateCell) {
        event.preventDefault();
        onNavigateCell('left');
        return;
      }

      if (key === 'ArrowRight' && !metaKey && onNavigateCell) {
        event.preventDefault();
        onNavigateCell('right');
        return;
      }

      // Tab navigation
      if (key === 'Tab' && !shiftKey && onNextCell) {
        event.preventDefault();
        onNextCell();
        return;
      }

      if (key === 'Tab' && shiftKey && onPreviousCell) {
        event.preventDefault();
        onPreviousCell();
        return;
      }

      // Cmd+Left/Right for sheet switching
      if (key === 'ArrowLeft' && metaKey && onPreviousSheet) {
        event.preventDefault();
        onPreviousSheet();
        return;
      }

      if (key === 'ArrowRight' && metaKey && onNextSheet) {
        event.preventDefault();
        onNextSheet();
        return;
      }

      // Escape to deselect
      if (key === 'Escape' && onDeselect) {
        event.preventDefault();
        onDeselect();
        return;
      }

      // Cmd+F for search (future feature)
      if (key === 'f' && metaKey && onSearch) {
        event.preventDefault();
        onSearch();
        return;
      }
    };

    window.addEventListener('keydown', handleKeyDown);

    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [enabled, onNavigateCell, onNextCell, onPreviousCell, onNextSheet, onPreviousSheet, onDeselect, onSearch]);
}

/**
 * Keyboard shortcuts help overlay component
 */
export function KeyboardShortcutsHelp({ onClose }: { onClose: () => void }) {
  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg shadow-xl max-w-md w-full mx-4">
        <div className="p-6">
          <div className="flex justify-between items-center mb-4">
            <h2 className="text-xl font-bold">Keyboard Shortcuts</h2>
            <button
              onClick={onClose}
              className="text-gray-500 hover:text-gray-700"
              aria-label="Close"
            >
              ✕
            </button>
          </div>

          <div className="space-y-4">
            <ShortcutSection title="Navigation">
              <Shortcut keys={['↑', '↓', '←', '→']} description="Navigate between cells" />
              <Shortcut keys={['Tab']} description="Next cell (horizontal)" />
              <Shortcut keys={['Shift', 'Tab']} description="Previous cell" />
            </ShortcutSection>

            <ShortcutSection title="Sheets">
              <Shortcut keys={['⌘', '←']} description="Previous sheet" />
              <Shortcut keys={['⌘', '→']} description="Next sheet" />
            </ShortcutSection>

            <ShortcutSection title="Actions">
              <Shortcut keys={['Esc']} description="Deselect cell" />
              <Shortcut keys={['⌘', 'F']} description="Search (coming soon)" />
            </ShortcutSection>
          </div>
        </div>
      </div>
    </div>
  );
}

function ShortcutSection({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <div>
      <h3 className="text-sm font-semibold text-gray-700 mb-2">{title}</h3>
      <div className="space-y-2">{children}</div>
    </div>
  );
}

function Shortcut({ keys, description }: { keys: string[]; description: string }) {
  return (
    <div className="flex justify-between items-center">
      <div className="flex gap-1">
        {keys.map((key, index) => (
          <kbd
            key={index}
            className="px-2 py-1 text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-300 rounded"
          >
            {key}
          </kbd>
        ))}
      </div>
      <span className="text-sm text-gray-600">{description}</span>
    </div>
  );
}
