/**
 * Status bar for mobile iOS app
 *
 * Features:
 * - Selected cell address (e.g., "A1")
 * - Selected cell value with unit
 * - Cell formula display
 * - Safe area inset handling (bottom home indicator)
 */

import { Cell } from '../../types/workbook';

interface MobileStatusBarProps {
  selectedCell: string | null;
  selectedCellData: Cell | null;
  displayPreference: 'Metric' | 'Imperial';
}

export function MobileStatusBar({
  selectedCell,
  selectedCellData,
  displayPreference,
}: MobileStatusBarProps) {
  // Format cell value for display
  const formatCellValue = (cell: Cell): string => {
    if (cell.value.type === 'number' && cell.value.value !== undefined) {
      const unit = displayPreference === 'Metric' ? cell.displayUnit : cell.storageUnit;
      return `${cell.value.value.toFixed(2)} ${unit || ''}`.trim();
    } else if (cell.value.type === 'text' && cell.value.text) {
      return cell.value.text;
    } else if (cell.value.type === 'error' && cell.value.error) {
      return `#${cell.value.error}`;
    } else if (cell.value.type === 'empty') {
      return '(empty)';
    }
    return '';
  };

  return (
    <div
      className="bg-white border-t shadow-sm"
      style={{
        paddingBottom: 'calc(env(safe-area-inset-bottom) + 0.5rem)',
        paddingLeft: 'calc(env(safe-area-inset-left) + 1rem)',
        paddingRight: 'calc(env(safe-area-inset-right) + 1rem)',
      }}
    >
      {selectedCell && selectedCellData ? (
        <div className="px-4 py-3">
          {/* Cell address */}
          <div className="flex items-center justify-between mb-2">
            <div className="flex items-center space-x-2">
              <span className="text-sm font-bold text-blue-600">{selectedCell}</span>
              {selectedCellData.warning && (
                <span
                  className="text-orange-500 text-xs"
                  title={selectedCellData.warning}
                  aria-label="Warning"
                >
                  ⚠️
                </span>
              )}
            </div>
            <span className="text-xs text-gray-500">
              {selectedCellData.storageUnit || 'No unit'}
            </span>
          </div>

          {/* Cell value */}
          <div className="mb-1">
            <span className="text-sm font-semibold text-gray-700">Value: </span>
            <span className="text-sm text-gray-900">{formatCellValue(selectedCellData)}</span>
          </div>

          {/* Formula (if present) */}
          {selectedCellData.formula && (
            <div className="mt-2 pt-2 border-t border-gray-200">
              <span className="text-xs font-semibold text-gray-600">Formula: </span>
              <code className="text-xs text-gray-800 font-mono bg-gray-100 px-2 py-1 rounded">
                {selectedCellData.formula}
              </code>
            </div>
          )}

          {/* Warning message (if present) */}
          {selectedCellData.warning && (
            <div className="mt-2 pt-2 border-t border-orange-200 bg-orange-50 -mx-4 px-4 py-2 rounded">
              <p className="text-xs text-orange-700">{selectedCellData.warning}</p>
            </div>
          )}
        </div>
      ) : (
        <div className="px-4 py-3 text-center">
          <p className="text-sm text-gray-500">No cell selected</p>
          <p className="text-xs text-gray-400 mt-1">Tap a cell to view details</p>
        </div>
      )}
    </div>
  );
}
