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
  | { type: 'Text'; text: string }
  | { type: 'Error'; message: string };

export interface WorkbookInfo {
  name: string;
  sheet_names: string[];
  active_sheet_index: number;
  is_dirty: boolean;
}

export interface UnitPreferences {
  metric_system: 'CGS' | 'MKS';
  metric_length: string;
  metric_mass: string;
  metric_time: string;
  imperial_length: string;
  imperial_mass: string;
  imperial_time: string;
  digital_storage_unit: string;
  time_rate_unit: string;
  currency: string;
  currency_rates: Record<string, number>;
  metric_temperature: string;
  imperial_temperature: string;
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

  async getUnitPreferences(): Promise<UnitPreferences> {
    return invoke('get_unit_preferences');
  },

  async updateUnitPreferences(preferences: UnitPreferences): Promise<void> {
    return invoke('update_unit_preferences', { preferences });
  },

  async setMetricSystem(system: 'CGS' | 'MKS'): Promise<void> {
    return invoke('set_metric_system', { system });
  },

  async setCurrencyRate(currency: string, rate: number): Promise<void> {
    return invoke('set_currency_rate', { currency, rate });
  },

  async getCurrencies(): Promise<string[]> {
    return invoke('get_currencies');
  },

  async getUnitsInUse(): Promise<string[]> {
    return invoke('get_units_in_use');
  },

  async exportDebugToClipboard(): Promise<void> {
    return invoke('export_debug_to_clipboard');
  },

  async exportToExcel(path: string): Promise<void> {
    return invoke('export_to_excel', { path });
  },

  async getExampleWorkbookPath(filename: string): Promise<string> {
    return invoke('get_example_workbook_path', { filename });
  },

  async listExampleWorkbooks(): Promise<Array<[string, string]>> {
    return invoke('list_example_workbooks');
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

  async saveExcelFileDialog(defaultPath?: string): Promise<string | null> {
    const path = await save({
      defaultPath,
      filters: [
        {
          name: 'Excel Workbook',
          extensions: ['xlsx'],
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
    case 'Text':
      value = { type: 'text' as const, text: data.value.text };
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
