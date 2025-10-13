import { useState } from 'react';
import Grid from './Grid';
import { Cell, CellAddress, getCellAddress } from '../types/workbook';

// Mock data for testing
function createMockCells(): Map<string, Cell> {
  const cells = new Map<string, Cell>();

  // Add some test data
  cells.set('A1', {
    value: { type: 'number', value: 100 },
    storageUnit: 'm',
    displayUnit: 'm',
  });

  cells.set('A2', {
    value: { type: 'number', value: 200 },
    storageUnit: 'm',
    displayUnit: 'm',
  });

  cells.set('A3', {
    value: { type: 'number', value: 300 },
    storageUnit: 'm',
    formula: '=A1 + A2',
  });

  cells.set('B1', {
    value: { type: 'number', value: 3.28084 },
    storageUnit: 'm',
    displayUnit: 'ft',
  });

  cells.set('C1', {
    value: { type: 'number', value: 50 },
    storageUnit: 'USD',
  });

  cells.set('C2', {
    value: { type: 'number', value: 100 },
    storageUnit: 'USD',
    warning: 'Unit mismatch in formula',
  });

  return cells;
}

interface SpreadsheetProps {
  sheetName?: string;
}

export default function Spreadsheet({ sheetName = 'Sheet1' }: SpreadsheetProps) {
  const [cells] = useState<Map<string, Cell>>(createMockCells());
  const [selectedCell, setSelectedCell] = useState<CellAddress | null>(null);

  const handleCellSelect = (address: CellAddress) => {
    setSelectedCell(address);
  };

  const getSelectedCellInfo = (): string => {
    if (!selectedCell) return 'No cell selected';

    const address = getCellAddress(selectedCell.col, selectedCell.row);
    const cell = cells.get(address);

    if (!cell) return `${address}: Empty`;

    let info = `${address}: `;
    if (cell.formula) {
      info += cell.formula;
    } else if (cell.value.type === 'number') {
      info += `${cell.value.value} ${cell.displayUnit || cell.storageUnit}`;
    }

    return info;
  };

  return (
    <div className="h-screen w-screen flex flex-col bg-white">
      {/* Header */}
      <div className="bg-gray-800 text-white px-4 py-2 flex items-center justify-between">
        <h1 className="text-xl font-bold">Unicel - Unit-Aware Spreadsheet</h1>
        <div className="flex gap-4">
          <button className="px-3 py-1 bg-blue-600 hover:bg-blue-700 rounded text-sm">
            New
          </button>
          <button className="px-3 py-1 bg-blue-600 hover:bg-blue-700 rounded text-sm">
            Open
          </button>
          <button className="px-3 py-1 bg-blue-600 hover:bg-blue-700 rounded text-sm">
            Save
          </button>
        </div>
      </div>

      {/* Formula bar */}
      <div className="border-b border-gray-300 p-2 bg-gray-50">
        <div className="flex items-center gap-2">
          <span className="text-sm font-semibold text-gray-700 w-16">
            {selectedCell ? getCellAddress(selectedCell.col, selectedCell.row) : ''}
          </span>
          <input
            type="text"
            className="flex-1 px-2 py-1 border border-gray-300 rounded text-sm"
            value={getSelectedCellInfo()}
            readOnly
            placeholder="Select a cell..."
          />
        </div>
      </div>

      {/* Sheet tabs */}
      <div className="border-b border-gray-300 bg-gray-50 px-2 py-1">
        <button className="px-3 py-1 bg-white border border-gray-300 rounded-t text-sm font-semibold">
          {sheetName}
        </button>
      </div>

      {/* Grid */}
      <div className="flex-1 overflow-hidden">
        <Grid
          cells={cells}
          selectedCell={selectedCell}
          onCellSelect={handleCellSelect}
        />
      </div>

      {/* Status bar */}
      <div className="border-t border-gray-300 bg-gray-100 px-4 py-1 text-xs text-gray-600">
        <div className="flex justify-between">
          <span>Ready</span>
          <span>Display: As Entered | Auto-Calculate: On</span>
        </div>
      </div>
    </div>
  );
}
