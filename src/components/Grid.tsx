import { Cell, CellAddress, getCellAddress, colNumberToLetter } from '../types/workbook';

interface GridProps {
  cells: Map<string, Cell>;
  rowCount?: number;
  colCount?: number;
  onCellSelect?: (address: CellAddress) => void;
  selectedCell?: CellAddress | null;
}

export default function Grid({
  cells,
  rowCount = 50,
  colCount = 26,
  onCellSelect,
  selectedCell,
}: GridProps) {
  // Generate column headers (A, B, C, ...)
  const columns = Array.from({ length: colCount }, (_, i) => colNumberToLetter(i + 1));

  // Generate row numbers (1, 2, 3, ...)
  const rows = Array.from({ length: rowCount }, (_, i) => i + 1);

  const handleCellClick = (col: string, row: number) => {
    if (onCellSelect) {
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
                const hasWarning = cell?.warning !== undefined;

                return (
                  <td
                    key={address}
                    className={`
                      border border-gray-300 min-w-[100px] h-8 px-2 text-sm
                      cursor-pointer hover:bg-blue-50
                      ${isSelected ? 'bg-blue-100 ring-2 ring-blue-500' : ''}
                      ${hasWarning ? 'bg-orange-50' : ''}
                    `}
                    onClick={() => handleCellClick(col, row)}
                    title={cell?.warning || cell?.formula || undefined}
                  >
                    {cell ? formatCellValue(cell) : ''}
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
