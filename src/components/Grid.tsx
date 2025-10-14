import { useState, useRef, useEffect } from 'react';
import { Cell, CellAddress, getCellAddress, colNumberToLetter, colLetterToNumber } from '../types/workbook';

interface GridProps {
  cells: Map<string, Cell>;
  rowCount?: number;
  colCount?: number;
  onCellSelect?: (address: CellAddress) => void;
  onCellEdit?: (address: CellAddress, value: string) => void;
  onCellDoubleClick?: (address: CellAddress) => void;
  selectedCell?: CellAddress | null;
  editingCell?: CellAddress | null;
}

export default function Grid({
  cells,
  rowCount = 50,
  colCount = 26,
  onCellSelect,
  onCellEdit,
  onCellDoubleClick,
  selectedCell,
  editingCell,
}: GridProps) {
  const [editValue, setEditValue] = useState('');
  const [isFormulaMode, setIsFormulaMode] = useState(false);
  const [pickerCell, setPickerCell] = useState<CellAddress | null>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  // Initialize edit value and focus input when editing starts
  useEffect(() => {
    if (editingCell) {
      const address = getCellAddress(editingCell.col, editingCell.row);
      const cell = cells.get(address);
      const value = getEditValue(cell);
      setEditValue(value);
      setIsFormulaMode(value.startsWith('='));
      setPickerCell(null);

      // Focus after a brief delay to ensure input is rendered
      setTimeout(() => {
        if (inputRef.current) {
          inputRef.current.focus();
          inputRef.current.select();
        }
      }, 0);
    }
  }, [editingCell, cells]);

  // Handle arrow key navigation in normal mode (not editing)
  useEffect(() => {
    const handleGlobalKeyDown = (e: KeyboardEvent) => {
      // Only handle navigation when not editing and a cell is selected
      if (editingCell || !selectedCell || !onCellSelect) return;

      const currentColNum = colLetterToNumber(selectedCell.col);
      let newCol = selectedCell.col;
      let newRow = selectedCell.row;

      switch (e.key) {
        case 'ArrowUp':
          e.preventDefault();
          newRow = Math.max(1, selectedCell.row - 1);
          break;
        case 'ArrowDown':
          e.preventDefault();
          newRow = Math.min(rowCount, selectedCell.row + 1);
          break;
        case 'ArrowLeft':
          e.preventDefault();
          newCol = colNumberToLetter(Math.max(1, currentColNum - 1));
          break;
        case 'ArrowRight':
          e.preventDefault();
          newCol = colNumberToLetter(Math.min(colCount, currentColNum + 1));
          break;
        case 'Enter':
          // Start editing on Enter
          if (onCellDoubleClick) {
            e.preventDefault();
            onCellDoubleClick(selectedCell);
          }
          return;
        default:
          return;
      }

      onCellSelect({ col: newCol, row: newRow });
    };

    // Attach to document so we catch all keyboard events
    document.addEventListener('keydown', handleGlobalKeyDown);
    return () => document.removeEventListener('keydown', handleGlobalKeyDown);
  }, [selectedCell, editingCell, onCellSelect, onCellDoubleClick, rowCount, colCount]);
  // Generate column headers (A, B, C, ...)
  const columns = Array.from({ length: colCount }, (_, i) => colNumberToLetter(i + 1));

  // Generate row numbers (1, 2, 3, ...)
  const rows = Array.from({ length: rowCount }, (_, i) => i + 1);

  const handleCellClick = (col: string, row: number) => {
    // In formula mode with cell picker active, clicking should pick the cell
    if (isFormulaMode && editingCell && pickerCell) {
      insertCellReference(col, row);
      return;
    }

    // Single click now starts editing immediately
    if (onCellDoubleClick) {
      onCellDoubleClick({ col, row });
    }
  };

  const formatCellValue = (cell: Cell): string => {
    if (cell.value.type === 'empty') {
      return '';
    }
    if (cell.value.type === 'error') {
      return `#ERROR: ${cell.value.error}`;
    }
    if (cell.value.type === 'text' && cell.value.text !== undefined) {
      return cell.value.text;
    }
    if (cell.value.type === 'number' && cell.value.value !== undefined) {
      const unit = cell.displayUnit || cell.storageUnit;
      if (unit) {
        return `${cell.value.value} ${unit}`;
      }
      return `${cell.value.value}`;
    }
    return '';
  };

  const isCellSelected = (col: string, row: number): boolean => {
    return selectedCell?.col === col && selectedCell?.row === row;
  };

  const isCellEditing = (col: string, row: number): boolean => {
    return editingCell?.col === col && editingCell?.row === row;
  };

  const getEditValue = (cell: Cell | undefined): string => {
    if (!cell) return '';
    if (cell.formula) return cell.formula;
    if (cell.value.type === 'text' && cell.value.text !== undefined) {
      return cell.value.text;
    }
    if (cell.value.type === 'number' && cell.value.value !== undefined) {
      const unit = cell.storageUnit;
      return unit ? `${cell.value.value} ${unit}` : `${cell.value.value}`;
    }
    return '';
  };

  const insertCellReference = (col: string, row: number) => {
    const cellRef = getCellAddress(col, row);
    setEditValue((prev) => prev + cellRef);
    setPickerCell(null);
    // Return focus to input
    setTimeout(() => inputRef.current?.focus(), 0);
  };

  const movePicker = (deltaCol: number, deltaRow: number) => {
    if (!editingCell) return;

    const currentCol = pickerCell?.col || editingCell.col;
    const currentRow = pickerCell?.row || editingCell.row;

    const currentColNum = colLetterToNumber(currentCol);
    const newColNum = Math.max(1, Math.min(colCount, currentColNum + deltaCol));
    const newRow = Math.max(1, Math.min(rowCount, currentRow + deltaRow));

    setPickerCell({
      col: colNumberToLetter(newColNum),
      row: newRow,
    });
  };

  const handleKeyDown = (e: React.KeyboardEvent, col: string, row: number) => {
    // Handle formula mode with cell picker
    if (isFormulaMode && (e.key === 'ArrowUp' || e.key === 'ArrowDown' || e.key === 'ArrowLeft' || e.key === 'ArrowRight')) {
      e.preventDefault();

      // Initialize picker on first arrow key press (start from current cell)
      if (!pickerCell) {
        // Start picker at current editing cell
        setPickerCell({ col, row });
        return; // Don't move on first press, just initialize
      }

      // Move picker
      switch (e.key) {
        case 'ArrowUp':
          movePicker(0, -1);
          break;
        case 'ArrowDown':
          movePicker(0, 1);
          break;
        case 'ArrowLeft':
          movePicker(-1, 0);
          break;
        case 'ArrowRight':
          movePicker(1, 0);
          break;
      }
      return;
    }

    // In formula mode with picker active, math operators insert cell reference + operator
    // Only if picker is active (user has pressed arrow keys)
    if (isFormulaMode && pickerCell && (e.key === '+' || e.key === '-' || e.key === '*' || e.key === '/' || e.key === '(' || e.key === ')')) {
      e.preventDefault();
      const cellRef = getCellAddress(pickerCell.col, pickerCell.row);
      setEditValue((prev) => prev + cellRef + e.key);
      setPickerCell(null);
      setTimeout(() => inputRef.current?.focus(), 0);
      return;
    }

    if (e.key === 'Enter') {
      // In formula mode with picker active, insert cell reference
      if (isFormulaMode && pickerCell) {
        insertCellReference(pickerCell.col, pickerCell.row);
        e.preventDefault();
        return;
      }

      // Otherwise, save and navigate to next cell
      if (onCellEdit) {
        onCellEdit({ col, row }, editValue);
        // Navigate to next row in same column
        const nextRow = row + 1;
        if (nextRow <= rowCount && onCellDoubleClick) {
          setTimeout(() => onCellDoubleClick({ col, row: nextRow }), 50);
        }
        e.preventDefault();
      }
    } else if (e.key === 'Escape') {
      if (pickerCell) {
        // Cancel picker mode
        setPickerCell(null);
        e.preventDefault();
      } else if (onCellEdit) {
        // Cancel edit
        onCellEdit({ col, row }, '');
        e.preventDefault();
      }
    }
  };

  const handleInputChange = (value: string) => {
    setEditValue(value);
    // Detect formula mode when user types =
    if (value.startsWith('=') && !isFormulaMode) {
      setIsFormulaMode(true);
    } else if (!value.startsWith('=') && isFormulaMode) {
      setIsFormulaMode(false);
      setPickerCell(null);
    }
  };

  return (
    <div className="overflow-auto h-full w-full border border-gray-300">
      <table className="border-collapse">
        <thead className="sticky top-0 bg-gray-100 z-10">
          <tr>
            {/* Corner cell */}
            <th className="border border-gray-300 bg-gray-200 w-12 h-8 text-xs font-semibold text-gray-600"></th>
            {/* Column headers */}
            {columns.map((col) => (
              <th
                key={col}
                className="border border-gray-300 bg-gray-100 min-w-[100px] h-8 text-xs font-semibold text-gray-700"
              >
                {col}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {rows.map((row) => (
            <tr key={row}>
              {/* Row header */}
              <td className="border border-gray-300 bg-gray-100 w-12 h-8 text-xs font-semibold text-gray-700 text-center sticky left-0">
                {row}
              </td>
              {/* Cells */}
              {columns.map((col) => {
                const address = getCellAddress(col, row);
                const cell = cells.get(address);
                const isSelected = isCellSelected(col, row);
                const isEditing = isCellEditing(col, row);
                const isPicker = pickerCell?.col === col && pickerCell?.row === row;
                const hasWarning = Boolean(cell?.warning && cell.warning.length > 0);

                const hasFormula = Boolean(cell?.formula && cell.formula.length > 0);
                const isNumeric = cell?.value.type === 'number';

                return (
                  <td
                    key={address}
                    className={`
                      border border-gray-300 min-w-[100px] h-8 px-0 text-sm relative
                      ${!isEditing && 'cursor-pointer hover:bg-blue-50 transition-colors'}
                      ${isSelected ? 'bg-blue-100 ring-2 ring-blue-500 ring-inset' : ''}
                      ${isPicker ? 'bg-green-200 ring-2 ring-green-500 ring-inset' : ''}
                      ${hasWarning ? 'bg-orange-50 border-orange-300' : ''}
                      ${hasFormula && !hasWarning && !isSelected ? 'bg-blue-50/60' : ''}
                    `}
                    onClick={() => !isEditing && handleCellClick(col, row)}
                    onDoubleClick={() => !isEditing && onCellDoubleClick?.({ col, row })}
                    title={cell?.warning || cell?.formula || undefined}
                  >
                    {isEditing ? (
                      <input
                        ref={inputRef}
                        type="text"
                        className="w-full h-full px-2 border-none focus:outline-none bg-white"
                        value={editValue}
                        onChange={(e) => handleInputChange(e.target.value)}
                        onKeyDown={(e) => handleKeyDown(e, col, row)}
                      />
                    ) : (
                      <div className={`px-2 flex items-center h-full ${isNumeric ? 'justify-end' : ''}`}>
                        {hasFormula && !hasWarning && (
                          <span className="text-blue-500 text-[11px] mr-1 opacity-55" title={cell?.formula || ''}>
                            ƒ
                          </span>
                        )}
                        <span className={`flex-1 ${isNumeric ? 'text-right' : ''}`}>
                          {cell ? formatCellValue(cell) : ''}
                        </span>
                        {hasWarning && (
                          <span className="text-orange-500 text-xs ml-1" title={cell?.warning || ''}>
                            ⚠️
                          </span>
                        )}
                      </div>
                    )}
                  </td>
                );
              })}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
