// Workbook types and helper functions for Tauri commands

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use crate::core::{
    cell::{Cell, CellValue},
    table::CellAddr,
    units::{BaseDimension, Unit},
    workbook::Workbook,
};
use crate::formats::json::WorkbookFile;

/// Application state
pub struct AppState {
    pub workbook: Mutex<Option<Workbook>>,
    pub current_file: Mutex<Option<String>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            workbook: Mutex::new(None),
            current_file: Mutex::new(None),
        }
    }
}

/// Cell data for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellData {
    pub value: CellValueData,
    pub storage_unit: String,
    pub display_unit: Option<String>,
    pub formula: Option<String>,
    pub warning: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CellValueData {
    Empty,
    Number { value: f64 },
    Error { message: String },
}

#[derive(Debug, Serialize)]
pub struct WorkbookInfo {
    pub name: String,
    pub sheet_names: Vec<String>,
    pub active_sheet_index: usize,
    pub is_dirty: bool,
}

// Helper functions

pub fn cell_to_data(cell: &Cell) -> CellData {
    let value = match cell.value() {
        CellValue::Empty => CellValueData::Empty,
        CellValue::Number(n) => CellValueData::Number { value: *n },
        CellValue::Error(e) => CellValueData::Error {
            message: e.clone(),
        },
    };

    CellData {
        value,
        storage_unit: cell.storage_unit().canonical().to_string(),
        display_unit: if cell.display_unit().canonical() != cell.storage_unit().canonical() {
            Some(cell.display_unit().canonical().to_string())
        } else {
            None
        },
        formula: cell.formula().map(|s| s.to_string()),
        warning: cell.warning().map(|s| s.to_string()),
    }
}

pub fn parse_cell_input(input: &str) -> Result<Cell, String> {
    // Check if it's a formula
    if input.starts_with('=') {
        return Ok(Cell::with_formula(input.to_string()));
    }

    // Parse number with optional unit
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.is_empty() {
        return Ok(Cell::empty());
    }

    let value: f64 = parts[0].parse().map_err(|_| "Invalid number")?;
    let unit_str = if parts.len() > 1 {
        parts[1..].join(" ")
    } else {
        String::new()
    };

    let unit = if unit_str.is_empty() {
        Unit::dimensionless()
    } else {
        // Simple unit parsing - in a real implementation, use the unit library
        parse_unit(&unit_str)
    };

    Ok(Cell::new(value, unit))
}

pub fn parse_unit(unit_str: &str) -> Unit {
    // Map common units to their dimensions
    let dimension = match unit_str {
        "m" | "cm" | "mm" | "km" | "in" | "ft" | "yd" | "mi" => BaseDimension::Length,
        "g" | "kg" | "mg" | "oz" | "lb" => BaseDimension::Mass,
        "s" | "min" | "hr" | "h" | "day" => BaseDimension::Time,
        "C" | "F" | "K" => BaseDimension::Temperature,
        "USD" | "EUR" | "GBP" => BaseDimension::Currency,
        _ => BaseDimension::Custom(unit_str.to_string()),
    };

    Unit::simple(unit_str, dimension)
}

// Workbook operations (library functions, not Tauri commands)

pub fn create_workbook_impl(state: &AppState, name: String) -> Result<(), String> {
    let workbook = Workbook::new(name);
    *state.workbook.lock().unwrap() = Some(workbook);
    *state.current_file.lock().unwrap() = None;
    Ok(())
}

pub fn get_workbook_info_impl(state: &AppState) -> Result<WorkbookInfo, String> {
    let workbook_guard = state.workbook.lock().unwrap();
    let workbook = workbook_guard.as_ref().ok_or("No workbook loaded")?;

    Ok(WorkbookInfo {
        name: workbook.name().to_string(),
        sheet_names: workbook.sheet_names(),
        active_sheet_index: workbook.active_sheet_index(),
        is_dirty: workbook.is_dirty(),
    })
}

pub fn get_sheet_cells_impl(state: &AppState) -> Result<Vec<(String, CellData)>, String> {
    let workbook_guard = state.workbook.lock().unwrap();
    let workbook = workbook_guard.as_ref().ok_or("No workbook loaded")?;

    let sheet = workbook.active_sheet();
    let cells: Vec<(String, CellData)> = sheet
        .cell_addresses()
        .into_iter()
        .filter_map(|addr| {
            sheet.get(&addr).map(|cell| {
                (addr.to_string(), cell_to_data(cell))
            })
        })
        .collect();

    Ok(cells)
}

pub fn set_cell_impl(
    state: &AppState,
    address: String,
    value: String,
) -> Result<CellData, String> {
    let mut workbook_guard = state.workbook.lock().unwrap();
    let workbook = workbook_guard.as_mut().ok_or("No workbook loaded")?;

    let addr = CellAddr::from_string(&address).map_err(|e| e.to_string())?;
    let cell = parse_cell_input(&value)?;

    workbook
        .active_sheet_mut()
        .set(addr, cell.clone())
        .map_err(|e| e.to_string())?;

    Ok(cell_to_data(&cell))
}

pub fn save_workbook_impl(state: &AppState, path: String) -> Result<(), String> {
    let workbook_guard = state.workbook.lock().unwrap();
    let workbook = workbook_guard.as_ref().ok_or("No workbook loaded")?;

    let file = WorkbookFile::from_workbook(workbook);
    file.save_to_file(std::path::Path::new(&path))
        .map_err(|e| e.to_string())?;

    // Update current file path
    drop(workbook_guard);
    *state.current_file.lock().unwrap() = Some(path);

    Ok(())
}

pub fn load_workbook_impl(state: &AppState, path: String) -> Result<(), String> {
    let file = WorkbookFile::load_from_file(std::path::Path::new(&path))
        .map_err(|e| e.to_string())?;

    let workbook = file.to_workbook().map_err(|e| e.to_string())?;

    *state.workbook.lock().unwrap() = Some(workbook);
    *state.current_file.lock().unwrap() = Some(path);

    Ok(())
}

pub fn get_current_file_impl(state: &AppState) -> Option<String> {
    state.current_file.lock().unwrap().clone()
}
