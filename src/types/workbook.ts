// Type definitions for Unicel workbook data structures

export interface CellValue {
  type: 'empty' | 'number' | 'text' | 'error';
  value?: number;
  text?: string;
  error?: string;
}

export interface Cell {
  value: CellValue;
  storageUnit: string;
  displayUnit?: string;
  formula?: string;
  warning?: string;
}

export interface CellAddress {
  col: string; // Column letter (A, B, C, ...)
  row: number; // Row number (1, 2, 3, ...)
}

export interface Sheet {
  name: string;
  cells: Map<string, Cell>; // Key is "A1", "B2", etc.
}

export interface Workbook {
  name: string;
  sheets: Sheet[];
  activeSheetIndex: number;
  settings: WorkbookSettings;
  dirty: boolean;
}

export interface WorkbookSettings {
  displayPreference: 'AsEntered' | 'Metric' | 'Imperial';
  autoRecalculate: boolean;
  showWarnings: boolean;
}

// Helper function to convert column number to letter
export function colNumberToLetter(num: number): string {
  let letter = '';
  while (num > 0) {
    const remainder = (num - 1) % 26;
    letter = String.fromCharCode(65 + remainder) + letter;
    num = Math.floor((num - 1) / 26);
  }
  return letter;
}

// Helper function to convert column letter to number
export function colLetterToNumber(letter: string): number {
  let num = 0;
  for (let i = 0; i < letter.length; i++) {
    num = num * 26 + (letter.charCodeAt(i) - 64);
  }
  return num;
}

// Helper function to get cell address string
export function getCellAddress(col: string, row: number): string {
  return `${col}${row}`;
}

// Helper function to parse cell address
export function parseCellAddress(address: string): CellAddress | null {
  const match = address.match(/^([A-Z]+)(\d+)$/);
  if (!match || !match[1] || !match[2]) return null;
  return {
    col: match[1],
    row: parseInt(match[2], 10),
  };
}
