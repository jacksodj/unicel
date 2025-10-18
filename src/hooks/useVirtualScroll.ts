/**
 * Virtual scrolling hook for efficient rendering of large grids
 * Only renders cells in the visible viewport plus a buffer
 */

import { useCallback, useMemo, useRef, useState } from 'react';

export interface VirtualScrollConfig {
  totalRows: number;
  totalCols: number;
  rowHeight: number;
  colWidth: number;
  containerHeight: number;
  containerWidth: number;
  overscanRows?: number;
  overscanCols?: number;
}

export interface VirtualScrollResult {
  startRow: number;
  endRow: number;
  startCol: number;
  endCol: number;
  visibleRows: number;
  visibleCols: number;
  scrollTop: number;
  scrollLeft: number;
  onScroll: (e: React.UIEvent<HTMLDivElement>) => void;
  containerRef: React.RefObject<HTMLDivElement>;
}

export function useVirtualScroll(config: VirtualScrollConfig): VirtualScrollResult {
  const {
    totalRows,
    totalCols,
    rowHeight,
    colWidth,
    containerHeight,
    containerWidth,
    overscanRows = 5,
    overscanCols = 3,
  } = config;

  const [scrollTop, setScrollTop] = useState(0);
  const [scrollLeft, setScrollLeft] = useState(0);
  const containerRef = useRef<HTMLDivElement>(null);

  // Calculate visible range
  const visibleRows = Math.ceil(containerHeight / rowHeight);
  const visibleCols = Math.ceil(containerWidth / colWidth);

  const startRow = Math.max(0, Math.floor(scrollTop / rowHeight) - overscanRows);
  const endRow = Math.min(totalRows, Math.ceil((scrollTop + containerHeight) / rowHeight) + overscanRows);

  const startCol = Math.max(0, Math.floor(scrollLeft / colWidth) - overscanCols);
  const endCol = Math.min(totalCols, Math.ceil((scrollLeft + containerWidth) / colWidth) + overscanCols);

  // Debounced scroll handler for better performance
  const handleScroll = useCallback((e: React.UIEvent<HTMLDivElement>) => {
    const target = e.currentTarget;
    setScrollTop(target.scrollTop);
    setScrollLeft(target.scrollLeft);
  }, []);

  return useMemo(
    () => ({
      startRow,
      endRow,
      startCol,
      endCol,
      visibleRows,
      visibleCols,
      scrollTop,
      scrollLeft,
      onScroll: handleScroll,
      containerRef,
    }),
    [startRow, endRow, startCol, endCol, visibleRows, visibleCols, scrollTop, scrollLeft, handleScroll]
  );
}

/**
 * Calculate optimal cell sizes for mobile viewing
 */
export function useCellSizes(containerWidth: number, containerHeight: number) {
  return useMemo(() => {
    // Calculate cell sizes based on container size
    const minCellWidth = 80; // Minimum for touch targets
    const minCellHeight = 44; // iOS minimum touch target

    // Default cell sizes
    const cellWidth = Math.max(minCellWidth, Math.floor(containerWidth / 6));
    const cellHeight = minCellHeight;

    return {
      cellWidth,
      cellHeight,
      minCellWidth,
      minCellHeight,
    };
  }, [containerWidth, containerHeight]);
}
