/**
 * Touch-enabled grid component for iOS
 *
 * Handles:
 * - Touch gestures (tap, swipe, pinch, long-press)
 * - Virtual scrolling for performance
 * - Cell selection and details display
 * - Safe area insets
 *
 * Gestures:
 * - Tap: Select cell
 * - Long-press: Show cell details popover
 * - Swipe: Scroll grid
 * - Pinch: Zoom in/out
 */

import { useState, useEffect, useRef, useMemo } from 'react';
import { useGesture } from '@use-gesture/react';
import { tauriApi, convertCellData } from '../../api/tauri';
import { Cell, colNumberToLetter } from '../../types/workbook';
import { haptics } from '../../utils/haptics';

interface MobileGridProps {
  workbookPath: string;
  sheetName: string;
  displayPreference: 'Metric' | 'Imperial';
  onSheetChange?: (direction: 'left' | 'right') => void;
  onCellSelect?: (address: string) => void;
  selectedCell?: string | null;
}

interface CellDetailsPopover {
  address: string;
  cell: Cell;
  position: { x: number; y: number };
}

const CELL_WIDTH = 100;
const CELL_HEIGHT = 44; // iOS minimum touch target
const VISIBLE_COLS = 10;
const VISIBLE_ROWS = 20;

export function MobileGrid({
  workbookPath,
  sheetName,
  displayPreference,
  onSheetChange: _onSheetChange,
  onCellSelect,
  selectedCell: externalSelectedCell,
}: MobileGridProps) {
  const [cells, setCells] = useState<Map<string, Cell>>(new Map());
  const [selectedCell, setSelectedCell] = useState<string | null>(externalSelectedCell || null);
  const [scrollOffset, setScrollOffset] = useState({ x: 0, y: 0 });
  const [zoom, setZoom] = useState(1.0);
  const [cellDetailsPopover, setCellDetailsPopover] = useState<CellDetailsPopover | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  const gridRef = useRef<HTMLDivElement>(null);
  const scrollVelocity = useRef({ x: 0, y: 0 });

  // Load cells from backend
  useEffect(() => {
    const loadCells = async () => {
      setIsLoading(true);
      try {
        const cellsData = await tauriApi.getSheetCells();
        const cellsMap = new Map<string, Cell>();

        cellsData.forEach(([address, cellData]) => {
          cellsMap.set(address, convertCellData(cellData));
        });

        setCells(cellsMap);
      } catch (error) {
        console.error('Failed to load cells:', error);
        haptics.error();
      } finally {
        setIsLoading(false);
      }
    };

    loadCells();
  }, [workbookPath, sheetName]);

  // Calculate visible cell range based on scroll offset and zoom
  const visibleRange = useMemo(() => {
    const startCol = Math.max(0, Math.floor(-scrollOffset.x / (CELL_WIDTH * zoom)));
    const startRow = Math.max(0, Math.floor(-scrollOffset.y / (CELL_HEIGHT * zoom)));
    const endCol = startCol + VISIBLE_COLS;
    const endRow = startRow + VISIBLE_ROWS;

    return { startCol, startRow, endCol, endRow };
  }, [scrollOffset, zoom]);

  // Get cell at touch position
  const getCellAtPosition = (x: number, y: number): string | null => {
    if (!gridRef.current) return null;

    const rect = gridRef.current.getBoundingClientRect();
    const relativeX = x - rect.left;
    const relativeY = y - rect.top;

    const col = Math.floor((relativeX - scrollOffset.x) / (CELL_WIDTH * zoom));
    const row = Math.floor((relativeY - scrollOffset.y - CELL_HEIGHT) / (CELL_HEIGHT * zoom)) + 1;

    if (col < 0 || row < 1) return null;

    const colLetter = colNumberToLetter(col + 1);
    return `${colLetter}${row}`;
  };

  // Handle cell tap
  const handleCellTap = (x: number, y: number) => {
    const address = getCellAtPosition(x, y);
    if (address) {
      setSelectedCell(address);
      setCellDetailsPopover(null);
      onCellSelect?.(address);
      haptics.light();
    }
  };

  // Handle long-press to show cell details
  const handleLongPress = (x: number, y: number) => {
    const address = getCellAtPosition(x, y);
    if (address) {
      const cell = cells.get(address);
      if (cell) {
        setCellDetailsPopover({
          address,
          cell,
          position: { x, y },
        });
        haptics.medium();
      }
    }
  };

  // Gesture handlers
  const bind = useGesture(
    {
      // Drag for scrolling
      onDrag: ({ delta: [dx, dy], velocity: [vx, vy], last }) => {
        setScrollOffset((prev) => ({
          x: Math.min(0, prev.x + dx),
          y: Math.min(0, prev.y + dy),
        }));

        scrollVelocity.current = { x: vx, y: vy };

        // Apply momentum scrolling on release
        if (last && (Math.abs(vx) > 0.1 || Math.abs(vy) > 0.1)) {
          const momentum = setInterval(() => {
            scrollVelocity.current.x *= 0.95;
            scrollVelocity.current.y *= 0.95;

            if (Math.abs(scrollVelocity.current.x) < 0.01 && Math.abs(scrollVelocity.current.y) < 0.01) {
              clearInterval(momentum);
            } else {
              setScrollOffset((prev) => ({
                x: Math.min(0, prev.x + scrollVelocity.current.x * 10),
                y: Math.min(0, prev.y + scrollVelocity.current.y * 10),
              }));
            }
          }, 16);
        }
      },

      // Pinch for zoom
      onPinch: ({ offset: [scale] }) => {
        const newZoom = Math.max(0.5, Math.min(2.0, scale));
        setZoom(newZoom);
      },

      // Tap for cell selection
      onClick: ({ event }) => {
        const touch = event as unknown as TouchEvent;
        if (touch.touches && touch.touches[0]) {
          handleCellTap(touch.touches[0].clientX, touch.touches[0].clientY);
        } else if (event instanceof MouseEvent) {
          handleCellTap(event.clientX, event.clientY);
        }
      },

      // Long-press for cell details
      onContextMenu: ({ event }) => {
        event.preventDefault();
        const touch = event as unknown as TouchEvent;
        if (touch.touches && touch.touches[0]) {
          handleLongPress(touch.touches[0].clientX, touch.touches[0].clientY);
        }
      },
    },
    {
      drag: {
        filterTaps: true,
        rubberband: true,
      },
      pinch: {
        scaleBounds: { min: 0.5, max: 2.0 },
        rubberband: true,
      },
    }
  );

  // Render cell value
  const renderCellValue = (cell: Cell): string => {
    if (cell.value.type === 'number' && cell.value.value !== undefined) {
      const unit = displayPreference === 'Metric' ? cell.displayUnit : cell.storageUnit;
      return `${cell.value.value.toFixed(2)} ${unit || ''}`;
    } else if (cell.value.type === 'text' && cell.value.text) {
      return cell.value.text;
    } else if (cell.value.type === 'error' && cell.value.error) {
      return `#${cell.value.error}`;
    }
    return '';
  };

  if (isLoading) {
    return (
      <div className="flex-1 flex items-center justify-center bg-gray-50">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500 mx-auto mb-4"></div>
          <p className="text-gray-600">Loading spreadsheet...</p>
        </div>
      </div>
    );
  }

  return (
    <div
      ref={gridRef}
      {...bind()}
      className="flex-1 overflow-hidden bg-white touch-none relative"
      style={{
        paddingLeft: 'env(safe-area-inset-left)',
        paddingRight: 'env(safe-area-inset-right)',
      }}
    >
      {/* Grid container with transform for smooth scrolling */}
      <div
        className="absolute inset-0"
        style={{
          transform: `translate(${scrollOffset.x}px, ${scrollOffset.y}px) scale(${zoom})`,
          transformOrigin: 'top left',
          willChange: 'transform',
        }}
      >
        {/* Column headers */}
        <div className="flex sticky top-0 bg-gray-100 z-10 border-b-2 border-gray-300">
          <div className="w-12 h-11 flex-shrink-0 border-r-2 border-gray-300" />
          {Array.from({ length: VISIBLE_COLS }, (_, i) => {
            const col = visibleRange.startCol + i;
            const colLetter = colNumberToLetter(col + 1);
            return (
              <div
                key={colLetter}
                className="flex items-center justify-center border-r border-gray-200 text-sm font-semibold text-gray-700"
                style={{ width: CELL_WIDTH, height: CELL_HEIGHT }}
              >
                {colLetter}
              </div>
            );
          })}
        </div>

        {/* Grid rows */}
        {Array.from({ length: VISIBLE_ROWS }, (_, i) => {
          const row = visibleRange.startRow + i + 1;
          return (
            <div key={row} className="flex">
              {/* Row header */}
              <div
                className="w-12 flex items-center justify-center bg-gray-100 border-r-2 border-b border-gray-300 text-sm font-semibold text-gray-700 flex-shrink-0"
                style={{ height: CELL_HEIGHT }}
              >
                {row}
              </div>

              {/* Row cells */}
              {Array.from({ length: VISIBLE_COLS }, (_, j) => {
                const col = visibleRange.startCol + j;
                const colLetter = colNumberToLetter(col + 1);
                const address = `${colLetter}${row}`;
                const cell = cells.get(address);
                const isSelected = selectedCell === address;

                return (
                  <div
                    key={address}
                    className={`border-r border-b border-gray-200 px-2 flex items-center text-sm ${
                      isSelected ? 'bg-blue-100 border-blue-500' : 'bg-white'
                    } ${cell?.warning ? 'bg-orange-50' : ''}`}
                    style={{ width: CELL_WIDTH, height: CELL_HEIGHT }}
                  >
                    {cell ? renderCellValue(cell) : ''}
                  </div>
                );
              })}
            </div>
          );
        })}
      </div>

      {/* Cell details popover */}
      {cellDetailsPopover && (
        <div
          className="absolute bg-white rounded-lg shadow-2xl p-4 border-2 border-blue-500 z-50 max-w-xs"
          style={{
            left: cellDetailsPopover.position.x + 20,
            top: cellDetailsPopover.position.y + 20,
          }}
        >
          <div className="flex justify-between items-start mb-2">
            <h3 className="font-bold text-lg">{cellDetailsPopover.address}</h3>
            <button
              onClick={() => setCellDetailsPopover(null)}
              className="text-gray-500 hover:text-gray-700 text-xl leading-none"
            >
              ×
            </button>
          </div>
          <div className="space-y-1 text-sm">
            <p>
              <span className="font-semibold">Value:</span> {renderCellValue(cellDetailsPopover.cell)}
            </p>
            {cellDetailsPopover.cell.formula && (
              <p>
                <span className="font-semibold">Formula:</span> {cellDetailsPopover.cell.formula}
              </p>
            )}
            <p>
              <span className="font-semibold">Storage Unit:</span> {cellDetailsPopover.cell.storageUnit || 'None'}
            </p>
            {cellDetailsPopover.cell.displayUnit && (
              <p>
                <span className="font-semibold">Display Unit:</span> {cellDetailsPopover.cell.displayUnit}
              </p>
            )}
            {cellDetailsPopover.cell.warning && (
              <p className="text-orange-600">
                <span className="font-semibold">Warning:</span> {cellDetailsPopover.cell.warning}
              </p>
            )}
          </div>
        </div>
      )}

      {/* Zoom indicator */}
      {zoom !== 1.0 && (
        <div className="absolute bottom-4 right-4 bg-black bg-opacity-75 text-white px-3 py-1 rounded-full text-sm">
          {Math.round(zoom * 100)}%
        </div>
      )}
    </div>
  );
}
