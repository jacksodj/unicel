import { useState, useEffect } from 'react';
import { tauriApi } from '../api/tauri';

interface StatusBarProps {
  displayMode: 'AsEntered' | 'Metric' | 'Imperial';
  autoRecalculate: boolean;
  cellCount: number;
  selectedCell?: { col: string; row: number } | null;
  cellUnit?: string;
  onSelectCell?: (cell: { col: string; row: number }) => void;
}

// Helper function to parse cell address like "A1" into { col: "A", row: 1 }
const parseCellAddress = (address: string): { col: string; row: number } => {
  const match = address.match(/^([A-Z]+)(\d+)$/);
  if (!match || !match[1] || !match[2]) {
    throw new Error(`Invalid cell address: ${address}`);
  }
  return { col: match[1], row: parseInt(match[2], 10) };
};

export default function StatusBar({
  displayMode,
  autoRecalculate,
  cellCount,
  selectedCell,
  cellUnit,
  onSelectCell,
}: StatusBarProps) {
  const [baseUnits, setBaseUnits] = useState<string[]>([]);

  const displayModeLabels = {
    AsEntered: '✏️ As Entered',
    Metric: '🌍 Metric',
    Imperial: '🇺🇸 Imperial',
  };

  const selectedCellAddress = selectedCell
    ? `${selectedCell.col}${selectedCell.row}`
    : null;

  // Load base units when cell count changes
  useEffect(() => {
    const loadBaseUnits = async () => {
      try {
        const units = await tauriApi.getBaseUnitsInUse();
        setBaseUnits(units);
      } catch (error) {
        console.warn('Failed to load base units:', error);
      }
    };

    if (cellCount > 0) {
      loadBaseUnits();
    } else {
      setBaseUnits([]);
    }
  }, [cellCount]);

  // Handle unit click - find cells with that unit and navigate to first one
  const handleUnitClick = async (unit: string) => {
    try {
      const cells = await tauriApi.getCellsWithBaseUnit(unit);
      const firstCell = cells[0];
      if (firstCell && onSelectCell) {
        const cellAddress = parseCellAddress(firstCell);
        onSelectCell(cellAddress);
      }
    } catch (error) {
      console.error('Failed to find cells with unit:', unit, error);
    }
  };

  return (
    <div className="border-t border-gray-300 bg-gray-50 text-xs text-gray-700">
      {/* Main status bar */}
      <div className="px-4 py-1.5">
        <div className="flex justify-between items-center">
          <div className="flex gap-4 items-center">
            <span className="flex items-center gap-1.5 font-semibold text-green-600">
              <span className="inline-block w-2 h-2 bg-green-500 rounded-full"></span>
              Ready
            </span>
            {selectedCellAddress && (
              <span className="font-mono bg-gray-200 px-2 py-0.5 rounded border border-gray-300">
                {selectedCellAddress}
              </span>
            )}
            {cellCount > 0 && (
              <span className="text-gray-600">
                {cellCount} cell{cellCount !== 1 ? 's' : ''} with data
              </span>
            )}
          </div>

          <div className="flex gap-4 items-center">
            {selectedCell && cellUnit && (
              <div className="flex items-center gap-1.5 px-2.5 py-1 bg-blue-50 border border-blue-200 rounded-md">
                <span className="text-blue-600 text-sm">📏</span>
                <span className="font-semibold text-blue-700">{cellUnit}</span>
              </div>
            )}
            <span className="font-medium">{displayModeLabels[displayMode]}</span>
            <span className="flex items-center gap-1.5 text-gray-600">
              <span className={autoRecalculate ? 'text-green-600' : 'text-gray-400'}>
                {autoRecalculate ? '✓' : '✗'}
              </span>
              Auto-Calculate
            </span>
          </div>
        </div>
      </div>

      {/* Base units footer */}
      {baseUnits.length > 0 && (
        <div className="flex items-center gap-2 px-4 py-1.5 border-t border-gray-200 bg-gray-100">
          <span className="text-xs font-medium text-gray-600">Units:</span>
          <div className="flex gap-1.5 flex-wrap">
            {baseUnits.map((unit) => (
              <button
                key={unit}
                onClick={() => handleUnitClick(unit)}
                className="px-2 py-0.5 bg-blue-50 border border-blue-200 rounded hover:bg-blue-100
                         transition-colors cursor-pointer text-sm font-medium text-blue-700"
                title={`Show cells with ${unit}`}
              >
                {unit}
              </button>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
