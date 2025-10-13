// Tauri API bindings

import { invoke } from '@tauri-apps/api/core';
import { open, save } from '@tauri-apps/plugin-dialog';

// Types matching Rust structures
export interface CellData {
  value: CellValueData;
  storage_unit: string;
  display_unit?: string;
  formula?: string;
  warning?: string;
}

export type CellValueData =
  | { type: 'Empty' }
  | { type: 'Number'; value: number }
  | { type: 'Error'; message: string };

export interface WorkbookInfo {
  name: string;
  sheet_names: string[];
  active_sheet_index: number;
  is_dirty: boolean;
}

// Tauri command wrappers
export const tauriApi = {
  // Workbook operations
  async createWorkbook(name: string): Promise<void> {
    return invoke('create_workbook', { name });
  },

  async getWorkbookInfo(): Promise<WorkbookInfo> {
    return invoke('get_workbook_info');
  },

  async getSheetCells(): Promise<Array<[string, CellData]>> {
    return invoke('get_sheet_cells');
  },

  async setCell(address: string, value: string): Promise<CellData> {
    return invoke('set_cell', { address, value });
  },

  async saveWorkbook(path: string): Promise<void> {
    return invoke('save_workbook', { path });
  },

  async loadWorkbook(path: string): Promise<void> {
    return invoke('load_workbook', { path });
  },

  async getCurrentFile(): Promise<string | null> {
    return invoke('get_current_file');
  },

  async setDisplayMode(mode: 'AsEntered' | 'Metric' | 'Imperial'): Promise<void> {
    return invoke('set_display_mode', { mode });
  },

  // File dialogs
  async openFileDialog(): Promise<string | null> {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: 'Unicel Workbook',
          extensions: ['usheet'],
        },
        {
          name: 'All Files',
          extensions: ['*'],
        },
      ],
    });

    if (selected && typeof selected === 'string') {
      return selected;
    }
    return null;
  },

  async saveFileDialog(defaultPath?: string): Promise<string | null> {
    const path = await save({
      defaultPath,
      filters: [
        {
          name: 'Unicel Workbook',
          extensions: ['usheet'],
        },
      ],
    });

    return path;
  },
};

// Convert Rust CellData to frontend Cell format
export function convertCellData(data: CellData) {
  let value;
  switch (data.value.type) {
    case 'Empty':
      value = { type: 'empty' as const };
      break;
    case 'Number':
      value = { type: 'number' as const, value: data.value.value };
      break;
    case 'Error':
      value = { type: 'error' as const, error: data.value.message };
      break;
  }

  return {
    value,
    storageUnit: data.storage_unit,
    displayUnit: data.display_unit,
    formula: data.formula,
    warning: data.warning,
  };
}
