import { useState, useEffect } from 'react';
import Grid from './Grid';
import Ribbon from './Ribbon';
import StatusBar from './StatusBar';
import { ToastContainer } from './Toast';
import { LoadingOverlay } from './LoadingSpinner';
import { Cell, CellAddress, getCellAddress } from '../types/workbook';
import { tauriApi, convertCellData } from '../api/tauri';

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
  const [workbookName, setWorkbookName] = useState('Untitled');

  // Initialize workbook on mount
  useEffect(() => {
    const initWorkbook = async () => {
      try {
        await tauriApi.createWorkbook('Untitled');
        await loadCellsFromBackend();
        addToast('Workbook initialized', 'success');
      } catch (error) {
        addToast(`Failed to initialize: ${error}`, 'error');
      }
    };
    initWorkbook();
  }, []);

  const loadCellsFromBackend = async () => {
    try {
      const cellsData = await tauriApi.getSheetCells();
      const newCells = new Map<string, Cell>();

      for (const [address, cellData] of cellsData) {
        newCells.set(address, convertCellData(cellData));
      }

      setCells(newCells);
    } catch (error) {
      addToast(`Failed to load cells: ${error}`, 'error');
    }
  };

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

  // Removed parseInputValue - now handled by backend

  const handleCellEdit = async (address: CellAddress, value: string) => {
    if (value === '') {
      // Cancel edit
      setEditingCell(null);
      return;
    }

    const cellAddr = getCellAddress(address.col, address.row);

    try {
      // Send to backend
      const cellData = await tauriApi.setCell(cellAddr, value);
      const newCell = convertCellData(cellData);

      // Update local state
      const newCells = new Map(cells);
      newCells.set(cellAddr, newCell);
      setCells(newCells);
      setIsDirty(true);

      setEditingCell(null);
      setSelectedCell(address);
    } catch (error) {
      addToast(`Failed to set cell: ${error}`, 'error');
      setEditingCell(null);
    }
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

  const handleDisplayModeChange = async (mode: 'AsEntered' | 'Metric' | 'Imperial') => {
    try {
      setDisplayMode(mode);
      await tauriApi.setDisplayMode(mode);
      await loadCellsFromBackend(); // Reload cells with new display mode
      addToast(`Display mode changed to ${mode}`, 'success');
    } catch (error) {
      addToast(`Failed to change display mode: ${error}`, 'error');
    }
  };

  const handleNew = async () => {
    if (isDirty && !confirm('You have unsaved changes. Create new workbook?')) {
      return;
    }

    try {
      setIsLoading(true);
      await tauriApi.createWorkbook('Untitled');
      setCells(new Map());
      setSelectedCell(null);
      setIsDirty(false);
      setWorkbookName('Untitled');
      addToast('New workbook created', 'success');
    } catch (error) {
      addToast(`Failed to create workbook: ${error}`, 'error');
    } finally {
      setIsLoading(false);
    }
  };

  const handleOpen = async () => {
    try {
      const filePath = await tauriApi.openFileDialog();
      if (!filePath) return;

      setIsLoading(true);
      await tauriApi.loadWorkbook(filePath);
      await loadCellsFromBackend();

      const info = await tauriApi.getWorkbookInfo();
      setWorkbookName(info.name);
      setIsDirty(false);

      addToast('Workbook loaded successfully', 'success');
    } catch (error) {
      addToast(`Failed to open workbook: ${error}`, 'error');
    } finally {
      setIsLoading(false);
    }
  };

  const handleSave = async () => {
    try {
      setIsLoading(true);

      // Check if we have a current file
      const currentFile = await tauriApi.getCurrentFile();
      if (currentFile) {
        await tauriApi.saveWorkbook(currentFile);
        setIsDirty(false);
        addToast('Workbook saved successfully', 'success');
      } else {
        // No current file, do Save As
        await handleSaveAs();
      }
    } catch (error) {
      addToast(`Failed to save workbook: ${error}`, 'error');
    } finally {
      setIsLoading(false);
    }
  };

  const handleSaveAs = async () => {
    try {
      const filePath = await tauriApi.saveFileDialog();
      if (!filePath) return;

      setIsLoading(true);
      await tauriApi.saveWorkbook(filePath);
      setIsDirty(false);

      addToast('Workbook saved successfully', 'success');
    } catch (error) {
      addToast(`Failed to save workbook: ${error}`, 'error');
    } finally {
      setIsLoading(false);
    }
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
        <h1 className="text-lg font-bold">
          Unicel - {workbookName}
          {isDirty && ' *'}
        </h1>
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
