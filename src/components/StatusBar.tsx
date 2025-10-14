interface StatusBarProps {
  displayMode: 'AsEntered' | 'Metric' | 'Imperial';
  autoRecalculate: boolean;
  cellCount: number;
  selectedCell?: { col: string; row: number } | null;
  cellUnit?: string;
}

export default function StatusBar({
  displayMode,
  autoRecalculate,
  cellCount,
  selectedCell,
  cellUnit,
}: StatusBarProps) {
  const displayModeLabels = {
    AsEntered: '✏️ As Entered',
    Metric: '🌍 Metric',
    Imperial: '🇺🇸 Imperial',
  };

  const selectedCellAddress = selectedCell
    ? `${selectedCell.col}${selectedCell.row}`
    : null;

  return (
    <div className="border-t border-gray-300 bg-gray-50 px-4 py-1.5 text-xs text-gray-700">
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
  );
}
