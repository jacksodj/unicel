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
    AsEntered: 'As Entered',
    Metric: 'Metric',
    Imperial: 'Imperial',
  };

  return (
    <div className="border-t border-gray-300 bg-gray-100 px-4 py-1 text-xs text-gray-600">
      <div className="flex justify-between items-center">
        <div className="flex gap-4">
          <span className="font-semibold">Ready</span>
          {cellCount > 0 && (
            <span>{cellCount} cells with data</span>
          )}
        </div>

        <div className="flex gap-4 items-center">
          {selectedCell && cellUnit && (
            <div className="flex items-center gap-1 px-2 py-0.5 bg-blue-100 border border-blue-300 rounded">
              <span className="text-blue-700">📏</span>
              <span className="font-semibold text-blue-800">{cellUnit}</span>
            </div>
          )}
          <span>Display: {displayModeLabels[displayMode]}</span>
          <span className="flex items-center gap-1">
            {autoRecalculate ? '✓' : '✗'} Auto-Calculate
          </span>
        </div>
      </div>
    </div>
  );
}
