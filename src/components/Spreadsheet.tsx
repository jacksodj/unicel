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
  const [cells, setCells] = useState<Map<string, Cell>>(createMockCells());
  const [selectedCell, setSelectedCell] = useState<CellAddress | null>(null);
  const [editingCell, setEditingCell] = useState<CellAddress | null>(null);
  const [formulaBarValue, setFormulaBarValue] = useState('');

  const handleCellSelect = (address: CellAddress) => {
    setSelectedCell(address);
    setEditingCell(null); // Stop editing when selecting a different cell

    // Update formula bar
    const cellAddr = getCellAddress(address.col, address.row);
    const cell = cells.get(cellAddr);
    if (cell?.formula) {
      setFormulaBarValue(cell.formula);
    } else if (cell?.value.type === 'number') {
      const unit = cell.storageUnit;
      setFormulaBarValue(unit ? `${cell.value.value} ${unit}` : `${cell.value.value}`);
    } else {
      setFormulaBarValue('');
    }
  };

  const parseInputValue = (input: string): Cell => {
    // Check if it's a formula
    if (input.startsWith('=')) {
      return {
        value: { type: 'empty' },
        storageUnit: '',
        formula: input,
      };
    }

    // Parse number with optional unit
    const match = input.trim().match(/^([-+]?\d+\.?\d*)\s*(.*)$/);
    if (match && match[1] !== undefined) {
      const value = parseFloat(match[1]);
      const unit = (match[2] || '').trim();
      return {
        value: { type: 'number', value },
        storageUnit: unit,
      };
    }

    // Empty or invalid
    return {
      value: { type: 'empty' },
      storageUnit: '',
    };
  };

  const handleCellEdit = (address: CellAddress, value: string) => {
    if (value === '') {
      // Cancel edit
      setEditingCell(null);
      return;
    }

    const cellAddr = getCellAddress(address.col, address.row);
    const newCell = parseInputValue(value);

    const newCells = new Map(cells);
    newCells.set(cellAddr, newCell);
    setCells(newCells);

    setEditingCell(null);
    setSelectedCell(address);
  };

  const handleCellDoubleClick = (address: CellAddress) => {
    setEditingCell(address);
  };

  const handleFormulaBarEdit = () => {
    if (selectedCell) {
      handleCellEdit(selectedCell, formulaBarValue);
    }
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
            className="flex-1 px-2 py-1 border border-gray-300 rounded text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
            value={formulaBarValue}
            onChange={(e) => setFormulaBarValue(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === 'Enter') {
                handleFormulaBarEdit();
              }
            }}
            placeholder="Select a cell to edit..."
            disabled={!selectedCell}
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
          editingCell={editingCell}
          onCellSelect={handleCellSelect}
          onCellEdit={handleCellEdit}
          onCellDoubleClick={handleCellDoubleClick}
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
