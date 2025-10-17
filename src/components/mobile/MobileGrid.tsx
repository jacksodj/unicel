/**
 * Touch-enabled grid component for iOS
 *
 * Handles:
 * - Touch gestures (tap, swipe, pinch, long-press)
 * - Virtual scrolling for performance
 * - Cell selection and details display
 * - Safe area insets
 */

import { useState } from 'react';
// import { useGesture } from '@use-gesture/react';

interface MobileGridProps {
  workbookPath: string;
  sheetName: string;
  displayPreference: 'Metric' | 'Imperial';
}

export function MobileGrid({ workbookPath, sheetName, displayPreference }: MobileGridProps) {
  const [selectedCell, _setSelectedCell] = useState<string | null>(null);
  const [_scrollPosition, _setScrollPosition] = useState({ x: 0, y: 0 });

  // TODO: Implement gesture handling with @use-gesture/react
  // const bind = useGesture({
  //   onDrag: ({ movement: [mx, my] }) => {
  //     // Pan/scroll the grid
  //     setScrollPosition({ x: mx, y: my });
  //   },
  //   onPinch: ({ offset: [scale] }) => {
  //     // Zoom in/out (optional)
  //   },
  //   onTap: ({ event }) => {
  //     // Select cell
  //     setSelectedCell(cellAddress);
  //   },
  //   onLongPress: ({ event }) => {
  //     // Show cell details popover
  //   },
  // });

  // TODO: Implement virtual scrolling for performance
  // TODO: Load cell data from Tauri backend
  // TODO: Apply display preference (Metric/Imperial)

  return (
    <div
      className="flex-1 overflow-hidden bg-white touch-none"
      style={{
        paddingTop: 'env(safe-area-inset-top)',
        paddingLeft: 'env(safe-area-inset-left)',
        paddingRight: 'env(safe-area-inset-right)',
      }}
    >
      <div className="p-4 text-center text-gray-500">
        <p>Mobile Grid Component</p>
        <p className="text-sm">Workbook: {workbookPath}</p>
        <p className="text-sm">Sheet: {sheetName}</p>
        <p className="text-sm">Display: {displayPreference}</p>
        {selectedCell && <p className="text-sm">Selected: {selectedCell}</p>}
        <p className="text-xs mt-4">Touch gestures will be implemented here</p>
      </div>
    </div>
  );
}
