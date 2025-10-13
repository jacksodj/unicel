import { useState } from 'react';
import Grid from './Grid';
import Ribbon from './Ribbon';
import StatusBar from './StatusBar';
import { ToastContainer } from './Toast';
import { LoadingOverlay } from './LoadingSpinner';
import { Cell, CellAddress, getCellAddress } from '../types/workbook';

interface ToastMessage {
  id: string;
  message: string;
  type?: 'info' | 'success' | 'warning' | 'error';
}

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
  const [displayMode, setDisplayMode] = useState<'AsEntered' | 'Metric' | 'Imperial'>('AsEntered');
  const [isDirty, setIsDirty] = useState(false);
  const [toasts, setToasts] = useState<ToastMessage[]>([]);
  const [isLoading, setIsLoading] = useState(false);

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
    setIsDirty(true);

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

  const addToast = (message: string, type: 'info' | 'success' | 'warning' | 'error' = 'info') => {
    const id = Date.now().toString();
    setToasts((prev) => [...prev, { id, message, type }]);
  };

  const removeToast = (id: string) => {
    setToasts((prev) => prev.filter((toast) => toast.id !== id));
  };

  const handleDisplayModeChange = (mode: 'AsEntered' | 'Metric' | 'Imperial') => {
    setDisplayMode(mode);
    addToast(`Display mode changed to ${mode}`, 'info');
    // In a real implementation, this would trigger unit conversion for display
  };

  const handleNew = () => {
    if (isDirty && !confirm('You have unsaved changes. Create new workbook?')) {
      return;
    }
    setCells(new Map());
    setSelectedCell(null);
    setIsDirty(false);
    addToast('New workbook created', 'success');
  };

  const handleOpen = () => {
    // TODO: Implement file open dialog via Tauri
    addToast('File open dialog - to be implemented with Tauri', 'info');
  };

  const handleSave = () => {
    setIsLoading(true);
    // Simulate async save operation
    setTimeout(() => {
      setIsLoading(false);
      setIsDirty(false);
      addToast('Workbook saved successfully', 'success');
    }, 1000);
  };

  const handleSaveAs = () => {
    // TODO: Implement save as dialog via Tauri
    addToast('Save As dialog - to be implemented with Tauri', 'info');
  };

  const getSelectedCellUnit = (): string | undefined => {
    if (!selectedCell) return undefined;
    const cellAddr = getCellAddress(selectedCell.col, selectedCell.row);
    const cell = cells.get(cellAddr);
    return cell?.storageUnit || cell?.displayUnit;
  };

  return (
    <>
      {isLoading && <LoadingOverlay message="Saving workbook..." />}
      <ToastContainer toasts={toasts} onRemove={removeToast} />
      <div className="h-screen w-screen flex flex-col bg-white">
      {/* Title bar */}
      <div className="bg-gray-800 text-white px-4 py-2">
        <h1 className="text-lg font-bold">Unicel - Unit-Aware Spreadsheet</h1>
      </div>

      {/* Ribbon */}
      <Ribbon
        displayMode={displayMode}
        onDisplayModeChange={handleDisplayModeChange}
        onNew={handleNew}
        onOpen={handleOpen}
        onSave={handleSave}
        onSaveAs={handleSaveAs}
        isDirty={isDirty}
      />

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
      <StatusBar
        displayMode={displayMode}
        autoRecalculate={true}
        cellCount={cells.size}
        selectedCell={selectedCell}
        cellUnit={getSelectedCellUnit()}
      />
      </div>
    </>
  );
}
