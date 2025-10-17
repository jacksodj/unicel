import { useState, useEffect } from 'react';
import Grid from './Grid';
import Ribbon from './Ribbon';
import StatusBar from './StatusBar';
import { ToastContainer } from './Toast';
import { LoadingOverlay } from './LoadingSpinner';
import UnitPreferencesDialog from './UnitPreferencesDialog';
import ExamplePickerDialog from './ExamplePickerDialog';
import NamedRangesDialog from './NamedRangesDialog';
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

export default function Spreadsheet() {
  const [cells, setCells] = useState<Map<string, Cell>>(createMockCells());
  const [selectedCell, setSelectedCell] = useState<CellAddress | null>(null);
  const [editingCell, setEditingCell] = useState<CellAddress | null>(null);
  const [formulaBarValue, setFormulaBarValue] = useState('');
  const [displayMode, setDisplayMode] = useState<'AsEntered' | 'Metric' | 'Imperial'>('AsEntered');
  const [isDirty, setIsDirty] = useState(false);
  const [toasts, setToasts] = useState<ToastMessage[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [loadingMessage, setLoadingMessage] = useState('Loading...');
  const [workbookName, setWorkbookName] = useState('Untitled');
  const [showPreferences, setShowPreferences] = useState(false);
  const [showExamplePicker, setShowExamplePicker] = useState(false);
  const [showNamedRanges, setShowNamedRanges] = useState(false);
  const [sheetNames, setSheetNames] = useState<string[]>(['Sheet1']);
  const [activeSheetIndex, setActiveSheetIndex] = useState(0);
  const [renamingSheetIndex, setRenamingSheetIndex] = useState<number | null>(null);
  const [renameValue, setRenameValue] = useState('');
  const [previousUnit, setPreviousUnit] = useState<string | null>(null);
  const [cellNamedRange, setCellNamedRange] = useState<string | null>(null);
  const [editingNamedRange, setEditingNamedRange] = useState(false);
  const [namedRangeEditValue, setNamedRangeEditValue] = useState('');

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
      const [cellsData, workbookInfo] = await Promise.all([
        tauriApi.getSheetCells(),
        tauriApi.getWorkbookInfo(),
      ]);

      const newCells = new Map<string, Cell>();
      for (const [address, cellData] of cellsData) {
        newCells.set(address, convertCellData(cellData));
      }

      setCells(newCells);
      setSheetNames(workbookInfo.sheet_names);
      setActiveSheetIndex(workbookInfo.active_sheet_index);
    } catch (error) {
      addToast(`Failed to load cells: ${error}`, 'error');
    }
  };

  const handleCellSelect = async (address: CellAddress) => {
    setSelectedCell(address);
    setEditingCell(null); // Stop editing when selecting a different cell
    setPreviousUnit(null); // Clear previous unit when selecting a different cell
    setEditingNamedRange(false); // Stop editing named range

    // Update formula bar
    const cellAddr = getCellAddress(address.col, address.row);
    const cell = cells.get(cellAddr);
    if (cell?.formula) {
      setFormulaBarValue(cell.formula);
    } else if (cell?.value.type === 'text') {
      setFormulaBarValue(cell.value.text || '');
    } else if (cell?.value.type === 'number' && cell.value.value !== undefined) {
      const unit = cell.storageUnit;
      // Special handling for percentages: convert 0.15 -> "15%"
      if (unit === '%') {
        setFormulaBarValue(`${cell.value.value * 100}%`);
      } else {
        setFormulaBarValue(unit ? `${cell.value.value} ${unit}` : `${cell.value.value}`);
      }
    } else {
      setFormulaBarValue('');
    }

    // Fetch named range for this cell
    try {
      const namedRange = await tauriApi.getNamedRangeForCell(activeSheetIndex, cellAddr);
      setCellNamedRange(namedRange);
    } catch (error) {
      console.error('Error fetching named range:', error);
      setCellNamedRange(null);
    }
  };

  // Removed parseInputValue - now handled by backend

  const handleCellEdit = async (address: CellAddress, value: string) => {
    const cellAddr = getCellAddress(address.col, address.row);

    try {
      // Process the value to append previous unit if applicable
      let processedValue = value;

      // Only append unit if:
      // 1. Value is not empty
      // 2. Value is not a formula (doesn't start with =)
      // 3. Value appears to be a bare number (no unit already present)
      // 4. We have a previous unit to append
      if (value && !value.startsWith('=') && previousUnit) {
        const trimmedValue = value.trim();
        // Check if it's a bare number (optionally with decimal point and/or negative sign)
        const isBareNumber = /^-?\d+\.?\d*$/.test(trimmedValue);

        if (isBareNumber) {
          processedValue = `${trimmedValue} ${previousUnit}`;
        }
      }

      // Set the cell (backend will recalculate all dependent cells)
      await tauriApi.setCell(cellAddr, processedValue);

      // Reload all cells from backend to get recalculated values
      await loadCellsFromBackend();

      setIsDirty(true);
      setEditingCell(null);
      setSelectedCell(address);
      setPreviousUnit(null); // Clear previous unit after edit
    } catch (error) {
      addToast(`Failed to set cell: ${error}`, 'error');
      setEditingCell(null);
      setPreviousUnit(null); // Clear previous unit on error too
    }
  };

  const handleCellDoubleClick = (address: CellAddress) => {
    // Initialize formula bar with cell's current value when editing starts
    const cellAddr = getCellAddress(address.col, address.row);
    const cell = cells.get(cellAddr);

    let initialValue = '';
    // Store the previous unit for later use
    setPreviousUnit(cell?.storageUnit || null);

    if (cell?.formula) {
      initialValue = cell.formula;
    } else if (cell?.value.type === 'text') {
      initialValue = cell.value.text || '';
    } else if (cell?.value.type === 'number' && cell.value.value !== undefined) {
      const unit = cell.storageUnit;
      if (unit === '%') {
        initialValue = `${cell.value.value * 100}%`;
      } else {
        initialValue = unit ? `${cell.value.value} ${unit}` : `${cell.value.value}`;
      }
    }

    setFormulaBarValue(initialValue);
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
      setLoadingMessage('Creating new workbook...');
      setIsLoading(true);
      await tauriApi.createWorkbook('Untitled');
      setCells(new Map());
      setSelectedCell(null);
      setIsDirty(false);
      setWorkbookName('Untitled');
      setSheetNames(['Sheet1']);
      setActiveSheetIndex(0);
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

      setLoadingMessage('Opening workbook...');
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
      setLoadingMessage('Saving workbook...');
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

      setLoadingMessage('Saving workbook...');
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

  const handleOpenPreferences = () => {
    setShowPreferences(true);
  };

  const handleClosePreferences = () => {
    setShowPreferences(false);
  };

  const handleSavePreferences = async () => {
    // Reload cells to apply new preferences
    await loadCellsFromBackend();
    addToast('Unit preferences saved', 'success');
  };

  const handleDebugExport = async () => {
    try {
      await tauriApi.exportDebugToClipboard();
      addToast('Debug info copied to clipboard', 'success');
    } catch (error) {
      addToast(`Failed to export debug info: ${error}`, 'error');
    }
  };

  const handleExportExcel = async () => {
    try {
      // Generate default filename from current workbook
      const currentFile = await tauriApi.getCurrentFile();
      let defaultPath: string | undefined;

      if (currentFile) {
        // Replace .usheet extension with .xlsx
        defaultPath = currentFile.replace(/\.usheet(\.json)?$/, '.xlsx');
      } else {
        // No file loaded, use workbook name
        defaultPath = `${workbookName}.xlsx`;
      }

      const filePath = await tauriApi.saveExcelFileDialog(defaultPath);
      if (!filePath) return;

      setLoadingMessage('Exporting to Excel...');
      setIsLoading(true);
      await tauriApi.exportToExcel(filePath);
      addToast('Workbook exported to Excel successfully', 'success');
    } catch (error) {
      addToast(`Failed to export to Excel: ${error}`, 'error');
    } finally {
      setIsLoading(false);
    }
  };

  const handleOpenExampleDialog = () => {
    setShowExamplePicker(true);
  };

  const handleCloseExamplePicker = () => {
    setShowExamplePicker(false);
  };

  const handleOpenNamedRanges = () => {
    setShowNamedRanges(true);
  };

  const handleCloseNamedRanges = () => {
    setShowNamedRanges(false);
  };

  const handleSelectExample = async (filename: string) => {
    try {
      const examplePath = await tauriApi.getExampleWorkbookPath(filename);

      setLoadingMessage('Opening example...');
      setIsLoading(true);
      await tauriApi.loadWorkbook(examplePath);
      await loadCellsFromBackend();

      const info = await tauriApi.getWorkbookInfo();
      setWorkbookName(info.name);
      setIsDirty(false);

      addToast('Example loaded successfully', 'success');
    } catch (error) {
      addToast(`Failed to open example: ${error}`, 'error');
    } finally {
      setIsLoading(false);
    }
  };

  const handleSheetChange = async (index: number) => {
    try {
      await tauriApi.setActiveSheet(index);
      await loadCellsFromBackend();
      setSelectedCell(null); // Clear selection when switching sheets
    } catch (error) {
      addToast(`Failed to switch sheet: ${error}`, 'error');
    }
  };

  const handleAddSheet = async () => {
    try {
      const newIndex = await tauriApi.addSheet();
      await loadCellsFromBackend(); // Reload to get updated sheet names
      await tauriApi.setActiveSheet(newIndex); // Switch to new sheet
      await loadCellsFromBackend(); // Reload cells for new sheet
      setIsDirty(true);
      addToast('New sheet created', 'success');
    } catch (error) {
      addToast(`Failed to add sheet: ${error}`, 'error');
    }
  };

  const handleSheetDoubleClick = (index: number, currentName: string) => {
    setRenamingSheetIndex(index);
    setRenameValue(currentName);
  };

  const handleRenameSheet = async () => {
    if (renamingSheetIndex === null || !renameValue.trim()) {
      setRenamingSheetIndex(null);
      return;
    }

    try {
      await tauriApi.renameSheet(renamingSheetIndex, renameValue.trim());
      await loadCellsFromBackend(); // Reload to get updated sheet names
      setRenamingSheetIndex(null);
      setIsDirty(true);
      addToast('Sheet renamed successfully', 'success');
    } catch (error) {
      addToast(`Failed to rename sheet: ${error}`, 'error');
      setRenamingSheetIndex(null);
    }
  };

  const handleRenameCancelOrBlur = () => {
    setRenamingSheetIndex(null);
  };

  const handleStartEditingNamedRange = () => {
    setEditingNamedRange(true);
    setNamedRangeEditValue(cellNamedRange || '');
  };

  const handleSaveNamedRange = async () => {
    if (!selectedCell) {
      setEditingNamedRange(false);
      return;
    }

    const cellAddr = getCellAddress(selectedCell.col, selectedCell.row);
    const trimmedValue = namedRangeEditValue.trim();

    // If value hasn't changed, just cancel editing
    if (trimmedValue === (cellNamedRange || '')) {
      setEditingNamedRange(false);
      return;
    }

    try {
      if (trimmedValue === '') {
        // Delete the named range if empty (user cleared it)
        if (cellNamedRange) {
          await tauriApi.deleteNamedRange(cellNamedRange);
          setCellNamedRange(null);
          addToast('Named range deleted', 'success');
          setIsDirty(true);
        }
      } else {
        // Create or update named range
        // First delete old one if it exists
        if (cellNamedRange) {
          await tauriApi.deleteNamedRange(cellNamedRange);
        }
        // Then create new one
        await tauriApi.createNamedRange(trimmedValue, activeSheetIndex, cellAddr);
        setCellNamedRange(trimmedValue);
        addToast('Named range saved', 'success');
        setIsDirty(true);
      }
      setEditingNamedRange(false);
    } catch (error) {
      addToast(`Failed to save named range: ${error}`, 'error');
      setEditingNamedRange(false);
    }
  };

  const handleCancelNamedRangeEdit = () => {
    setEditingNamedRange(false);
    setNamedRangeEditValue('');
  };

  const handleDeleteSheet = async (index: number, sheetName: string) => {
    if (sheetNames.length <= 1) {
      addToast('Cannot delete the last sheet', 'error');
      return;
    }

    try {
      // Check if sheet has any data
      const hasData = await tauriApi.sheetHasData(index);

      // Only show confirmation if sheet has data
      if (
        hasData &&
        !confirm(
          `Are you sure you want to delete sheet "${sheetName}"? This action cannot be undone.`
        )
      ) {
        return;
      }

      await tauriApi.deleteSheet(index);
      await loadCellsFromBackend(); // Reload to get updated sheet names and active sheet
      setIsDirty(true);
      addToast('Sheet deleted successfully', 'success');
    } catch (error) {
      addToast(`Failed to delete sheet: ${error}`, 'error');
    }
  };

  // Keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Check for Ctrl/Cmd key combinations
      const isMod = e.metaKey || e.ctrlKey;

      if (isMod && e.key === 'Tab') {
        // Ctrl+Tab (or Cmd+Tab on Mac) cycles through sheet tabs
        e.preventDefault();
        if (sheetNames.length > 1) {
          const direction = e.shiftKey ? -1 : 1;
          const newIndex = (activeSheetIndex + direction + sheetNames.length) % sheetNames.length;
          handleSheetChange(newIndex);
        }
      } else if (isMod && e.key === 's') {
        e.preventDefault();
        handleSave();
      } else if (isMod && e.shiftKey && e.key === 'S') {
        e.preventDefault();
        handleSaveAs();
      } else if (isMod && e.key === 'o') {
        e.preventDefault();
        handleOpen();
      } else if (isMod && e.key === 'n') {
        e.preventDefault();
        handleNew();
      } else if ((e.key === 'Delete' || e.key === 'Backspace') && selectedCell && !editingCell) {
        // Clear selected cell when Delete or Backspace is pressed (not in edit mode)
        e.preventDefault();
        handleCellEdit(selectedCell, '');
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [isDirty, selectedCell, editingCell, sheetNames.length, activeSheetIndex]);

  return (
    <>
      {isLoading && <LoadingOverlay message={loadingMessage} />}
      <ToastContainer toasts={toasts} onRemove={removeToast} />
      <UnitPreferencesDialog
        isOpen={showPreferences}
        onClose={handleClosePreferences}
        onSave={handleSavePreferences}
      />
      <ExamplePickerDialog
        isOpen={showExamplePicker}
        onClose={handleCloseExamplePicker}
        onSelectExample={handleSelectExample}
      />
      <NamedRangesDialog
        isOpen={showNamedRanges}
        onClose={handleCloseNamedRanges}
        currentSheetIndex={activeSheetIndex}
      />
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
          onOpenPreferences={handleOpenPreferences}
          onOpenNamedRanges={handleOpenNamedRanges}
          onDebugExport={handleDebugExport}
          onExportExcel={handleExportExcel}
          onOpenExampleDialog={handleOpenExampleDialog}
          isDirty={isDirty}
        />

        {/* Formula bar */}
        <div className="border-b border-gray-300 p-2 bg-gray-50">
          <div className="flex items-center gap-2">
            {selectedCell && (
              editingNamedRange ? (
                <input
                  type="text"
                  className="px-2 py-1 border border-blue-500 rounded text-sm focus:outline-none min-w-[80px]"
                  value={namedRangeEditValue}
                  onChange={(e) => setNamedRangeEditValue(e.target.value)}
                  onKeyDown={(e) => {
                    if (e.key === 'Enter') {
                      handleSaveNamedRange();
                    } else if (e.key === 'Escape') {
                      handleCancelNamedRangeEdit();
                    }
                  }}
                  onBlur={handleSaveNamedRange}
                  placeholder={getCellAddress(selectedCell.col, selectedCell.row)}
                  autoFocus
                  autoComplete="off"
                  autoCorrect="off"
                  autoCapitalize="off"
                  spellCheck={false}
                />
              ) : (
                <span
                  className="text-sm font-semibold text-gray-700 cursor-pointer hover:bg-gray-200 px-2 py-1 rounded transition-colors min-w-[60px]"
                  onClick={handleStartEditingNamedRange}
                  title="Click to edit named range"
                >
                  {cellNamedRange || getCellAddress(selectedCell.col, selectedCell.row)}
                </span>
              )
            )}
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
              autoComplete="off"
              autoCorrect="off"
              autoCapitalize="off"
              spellCheck={false}
            />
          </div>
        </div>

        {/* Sheet tabs */}
        <div className="border-b border-gray-300 bg-gray-50 px-2 py-1 flex gap-1 items-center">
          {sheetNames.map((name, index) => (
            <div key={index} className="relative">
              {renamingSheetIndex === index ? (
                <input
                  type="text"
                  className="px-3 py-1 border border-blue-500 rounded-t text-sm font-semibold focus:outline-none"
                  value={renameValue}
                  onChange={(e) => setRenameValue(e.target.value)}
                  onKeyDown={(e) => {
                    if (e.key === 'Enter') {
                      handleRenameSheet();
                    } else if (e.key === 'Escape') {
                      handleRenameCancelOrBlur();
                    }
                  }}
                  onBlur={handleRenameCancelOrBlur}
                  autoFocus
                  autoComplete="off"
                  autoCorrect="off"
                  autoCapitalize="off"
                  spellCheck={false}
                />
              ) : (
                <button
                  className={`px-3 py-1 border border-gray-300 rounded-t text-sm font-semibold transition-colors flex items-center gap-2 ${
                    index === activeSheetIndex
                      ? 'bg-white border-b-transparent'
                      : 'bg-gray-200 hover:bg-gray-100'
                  }`}
                  onClick={() => handleSheetChange(index)}
                  onDoubleClick={() => handleSheetDoubleClick(index, name)}
                >
                  <span>{name}</span>
                  {sheetNames.length > 1 && (
                    <span
                      className="text-gray-500 hover:text-red-600 transition-colors"
                      onClick={(e) => {
                        e.stopPropagation();
                        handleDeleteSheet(index, name);
                      }}
                      title="Delete sheet"
                    >
                      ×
                    </span>
                  )}
                </button>
              )}
            </div>
          ))}
          <button
            className="px-3 py-1 text-gray-600 hover:text-gray-800 hover:bg-gray-100 rounded text-sm font-bold transition-colors"
            onClick={handleAddSheet}
            title="Add new sheet"
          >
            +
          </button>
        </div>

        {/* Grid */}
        <div className="flex-1 overflow-hidden">
          <Grid
            cells={cells}
            selectedCell={selectedCell}
            editingCell={editingCell}
            editValue={formulaBarValue}
            onEditValueChange={setFormulaBarValue}
            onCellSelect={handleCellSelect}
            onCellEdit={handleCellEdit}
            onCellDoubleClick={handleCellDoubleClick}
            activeSheetIndex={activeSheetIndex}
          />
        </div>

        {/* Status bar */}
        <StatusBar
          displayMode={displayMode}
          autoRecalculate={true}
          cellCount={cells.size}
          selectedCell={selectedCell}
          cellUnit={getSelectedCellUnit()}
          onSelectCell={setSelectedCell}
        />
      </div>
    </>
  );
}
