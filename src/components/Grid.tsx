import { useState, useRef, useEffect } from 'react';
import { Cell, CellAddress, getCellAddress, colNumberToLetter } from '../types/workbook';

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
  const inputRef = useRef<HTMLInputElement>(null);

  // Initialize edit value and focus input when editing starts
  useEffect(() => {
    if (editingCell) {
      const address = getCellAddress(editingCell.col, editingCell.row);
      const cell = cells.get(address);
      setEditValue(getEditValue(cell));

      // Focus after a brief delay to ensure input is rendered
      setTimeout(() => {
        if (inputRef.current) {
          inputRef.current.focus();
          inputRef.current.select();
        }
      }, 0);
    }
  }, [editingCell, cells]);
  // Generate column headers (A, B, C, ...)
  const columns = Array.from({ length: colCount }, (_, i) => colNumberToLetter(i + 1));

  // Generate row numbers (1, 2, 3, ...)
  const rows = Array.from({ length: rowCount }, (_, i) => i + 1);

  const handleCellClick = (col: string, row: number) => {
    // If clicking on already selected cell, start editing
    if (selectedCell?.col === col && selectedCell?.row === row && onCellDoubleClick) {
      onCellDoubleClick({ col, row });
    } else if (onCellSelect) {
      // Otherwise, just select the cell
      onCellSelect({ col, row });
    }
  };

  const formatCellValue = (cell: Cell): string => {
    if (cell.value.type === 'empty') {
      return '';
    }
    if (cell.value.type === 'error') {
      return `#ERROR: ${cell.value.error}`;
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
    if (cell.value.type === 'number' && cell.value.value !== undefined) {
      const unit = cell.storageUnit;
      return unit ? `${cell.value.value} ${unit}` : `${cell.value.value}`;
    }
    return '';
  };

  const handleKeyDown = (e: React.KeyboardEvent, col: string, row: number) => {
    if (e.key === 'Enter' && onCellEdit) {
      onCellEdit({ col, row }, editValue);
      e.preventDefault();
    } else if (e.key === 'Escape' && onCellEdit) {
      onCellEdit({ col, row }, ''); // Cancel edit
      e.preventDefault();
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
                const hasWarning = cell?.warning !== undefined;

                return (
                  <td
                    key={address}
                    className={`
                      border border-gray-300 min-w-[100px] h-8 px-0 text-sm
                      ${!isEditing && 'cursor-pointer hover:bg-blue-50'}
                      ${isSelected ? 'bg-blue-100 ring-2 ring-blue-500' : ''}
                      ${hasWarning ? 'bg-orange-50' : ''}
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
                        onChange={(e) => setEditValue(e.target.value)}
                        onKeyDown={(e) => handleKeyDown(e, col, row)}
                      />
                    ) : (
                      <div className="px-2">{cell ? formatCellValue(cell) : ''}</div>
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
