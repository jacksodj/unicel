// Workbook types and helper functions for Tauri commands

use crate::core::{
    cell::{Cell, CellValue},
    settings::UnitPreferences,
    table::CellAddr,
    units::{BaseDimension, Unit},
    workbook::Workbook,
};
use crate::formats::json::WorkbookFile;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

/// Display mode for unit display
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DisplayMode {
    AsEntered,
    Metric,
    Imperial,
}

/// Application state
pub struct AppState {
    pub workbook: Mutex<Option<Workbook>>,
    pub current_file: Mutex<Option<String>>,
    pub display_mode: Mutex<DisplayMode>,
    pub unit_preferences: Mutex<UnitPreferences>,
    pub recent_files: Mutex<Vec<String>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            workbook: Mutex::new(None),
            current_file: Mutex::new(None),
            display_mode: Mutex::new(DisplayMode::AsEntered),
            unit_preferences: Mutex::new(UnitPreferences::default()),
            recent_files: Mutex::new(Vec::new()),
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
    Text { text: String },
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
        CellValue::Text(t) => CellValueData::Text { text: t.clone() },
        CellValue::Error(e) => CellValueData::Error { message: e.clone() },
    };

    let storage_unit = cell.storage_unit().canonical().to_string();
    let display_unit_canonical = cell.display_unit().canonical().to_string();

    CellData {
        value,
        storage_unit: format_unit_display(&storage_unit),
        display_unit: if display_unit_canonical != storage_unit {
            Some(format_unit_display(&display_unit_canonical))
        } else {
            None
        },
        formula: cell.formula().map(|s| s.to_string()),
        warning: cell.warning().map(|s| s.to_string()),
    }
}

pub fn cell_to_data_with_mode(
    cell: &Cell,
    mode: &DisplayMode,
    preferences: &UnitPreferences,
) -> CellData {
    use crate::core::units::UnitLibrary;

    let storage_unit = cell.storage_unit().canonical().to_string();

    tracing::debug!(
        "cell_to_data_with_mode: mode={:?}, storage_unit={:?}, contains_star={}, contains_slash={}",
        mode,
        storage_unit,
        storage_unit.contains('*'),
        storage_unit.contains('/')
    );

    // Determine display unit based on mode and preferences
    let display_unit_str = get_display_unit_for_mode(&storage_unit, mode, preferences);

    tracing::debug!("  -> target display unit: {:?}", display_unit_str);

    // Convert value if we have a different display unit
    let (display_value, display_unit_final) = if let Some(target_unit) = display_unit_str {
        if let Some(original_value) = cell.as_number() {
            // Try to convert compound units
            if let Some(converted) =
                convert_compound_unit(original_value, &storage_unit, &target_unit)
            {
                tracing::debug!(
                    "  compound conversion succeeded: {} {} -> {} {}",
                    original_value,
                    storage_unit,
                    converted,
                    target_unit
                );
                (
                    CellValueData::Number { value: converted },
                    Some(format_unit_display(&target_unit)),
                )
            } else {
                // Try simple conversion with normalized units (for currency symbols)
                let library = UnitLibrary::new();
                let storage_norm = normalize_unit(&storage_unit);
                let target_norm = normalize_unit(&target_unit);

                if let Some(converted) =
                    library.convert(original_value, &storage_norm, &target_norm)
                {
                    tracing::debug!(
                        "  simple conversion succeeded: {} {} -> {} {}",
                        original_value,
                        storage_unit,
                        converted,
                        target_unit
                    );
                    (
                        CellValueData::Number { value: converted },
                        Some(format_unit_display(&target_unit)),
                    )
                } else {
                    // Conversion failed, use original
                    tracing::warn!(
                        "  conversion FAILED: {} {} -> {}",
                        original_value,
                        storage_unit,
                        target_unit
                    );
                    (
                        CellValueData::Number {
                            value: original_value,
                        },
                        None,
                    )
                }
            }
        } else {
            // Not a number, use original
            (
                match cell.value() {
                    CellValue::Empty => CellValueData::Empty,
                    CellValue::Number(n) => CellValueData::Number { value: *n },
                    CellValue::Text(t) => CellValueData::Text { text: t.clone() },
                    CellValue::Error(e) => CellValueData::Error { message: e.clone() },
                },
                None,
            )
        }
    } else {
        // No conversion needed
        (
            match cell.value() {
                CellValue::Empty => CellValueData::Empty,
                CellValue::Number(n) => CellValueData::Number { value: *n },
                CellValue::Text(t) => CellValueData::Text { text: t.clone() },
                CellValue::Error(e) => CellValueData::Error { message: e.clone() },
            },
            None,
        )
    };

    CellData {
        value: display_value,
        storage_unit: format_unit_display(&storage_unit),
        display_unit: display_unit_final,
        formula: cell.formula().map(|s| s.to_string()),
        warning: cell.warning().map(|s| s.to_string()),
    }
}

/// Normalize currency symbols ("$" -> "USD", etc.)
fn normalize_unit(unit: &str) -> String {
    match unit {
        "$" => "USD".to_string(),
        "€" => "EUR".to_string(),
        "£" => "GBP".to_string(),
        _ => unit.to_string(),
    }
}

/// Convert compound unit values (e.g., 10 ft*ft → m*m, 15 $/ft -> USD/m)
fn convert_compound_unit(value: f64, from_unit: &str, to_unit: &str) -> Option<f64> {
    use crate::core::units::UnitLibrary;

    let library = UnitLibrary::new();

    // Handle power notation (e.g., ft^2 -> m^2)
    // This must be checked BEFORE division to handle cases like "1/ft^2"
    if let (Some(from_pos), Some(to_pos)) = (from_unit.find('^'), to_unit.find('^')) {
        // Check if the ^ is NOT part of a division (denominator)
        let from_has_div = from_unit
            .find('/')
            .is_some_and(|div_pos| div_pos < from_pos);
        let to_has_div = to_unit.find('/').is_some_and(|div_pos| div_pos < to_pos);

        // Only handle pure power notation here (not in denominators)
        if !from_has_div && !to_has_div {
            let from_base = &from_unit[..from_pos];
            let from_power_str = &from_unit[from_pos + 1..];
            let to_base = &to_unit[..to_pos];
            let to_power_str = &to_unit[to_pos + 1..];

            // Parse the power
            if let (Ok(from_power), Ok(to_power)) =
                (from_power_str.parse::<i32>(), to_power_str.parse::<i32>())
            {
                if from_power == to_power {
                    // Get conversion factor for base unit
                    if let Some(base_factor) = library.convert(1.0, from_base, to_base) {
                        // Raise to the power
                        let combined_factor = base_factor.powi(from_power);
                        return Some(value * combined_factor);
                    }
                }
            }
        }
    }

    // Handle multiplication (e.g., ft*ft)
    if let (Some(from_pos), Some(to_pos)) = (from_unit.find('*'), to_unit.find('*')) {
        let from_left = &from_unit[..from_pos];
        let from_right = &from_unit[from_pos + 1..];
        let to_left = &to_unit[..to_pos];
        let to_right = &to_unit[to_pos + 1..];

        // Get conversion factors for each component
        let factor_left = library.convert(1.0, from_left, to_left)?;
        let factor_right = library.convert(1.0, from_right, to_right)?;

        // For multiplication, multiply the factors
        let combined_factor = factor_left * factor_right;
        return Some(value * combined_factor);
    }

    // Handle division (e.g., mi/hr, $/ft, 1/ft^2)
    if let (Some(from_pos), Some(to_pos)) = (from_unit.find('/'), to_unit.find('/')) {
        let from_left = &from_unit[..from_pos];
        let from_right = &from_unit[from_pos + 1..];
        let to_left = &to_unit[..to_pos];
        let to_right = &to_unit[to_pos + 1..];

        // Handle numerator (left side) - may have exponents like "ft^2"
        let factor_left = if let (Some(from_exp_pos), Some(to_exp_pos)) =
            (from_left.find('^'), to_left.find('^'))
        {
            // Numerator has exponents (e.g., "ft^2/s" -> "m^2/s")
            let from_base = &from_left[..from_exp_pos];
            let from_power_str = &from_left[from_exp_pos + 1..];
            let to_base = &to_left[..to_exp_pos];
            let to_power_str = &to_left[to_exp_pos + 1..];

            // Parse the powers
            if let (Ok(from_power), Ok(to_power)) =
                (from_power_str.parse::<i32>(), to_power_str.parse::<i32>())
            {
                if from_power == to_power {
                    // Normalize currency symbols for base units
                    let from_base_norm = normalize_unit(from_base);
                    let to_base_norm = normalize_unit(to_base);

                    // Get base conversion factor and raise to power
                    let base_factor = library.convert(1.0, &from_base_norm, &to_base_norm)?;
                    Some(base_factor.powi(from_power))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            // No exponents in numerator, handle normally
            let from_left_norm = normalize_unit(from_left);
            let to_left_norm = normalize_unit(to_left);

            library
                .convert(1.0, &from_left_norm, &to_left_norm)
                .or_else(|| {
                    // Try converting just the left side if it's the same dimension
                    if from_left_norm == to_left_norm {
                        Some(1.0)
                    } else {
                        None
                    }
                })
        }?;

        // Handle denominator (right side) - may have exponents like "ft^2"
        let factor_right = if let (Some(from_exp_pos), Some(to_exp_pos)) =
            (from_right.find('^'), to_right.find('^'))
        {
            // Denominator has exponents (e.g., "ft^2" -> "m^2")
            let from_base = &from_right[..from_exp_pos];
            let from_power_str = &from_right[from_exp_pos + 1..];
            let to_base = &to_right[..to_exp_pos];
            let to_power_str = &to_right[to_exp_pos + 1..];

            // Parse the powers
            if let (Ok(from_power), Ok(to_power)) =
                (from_power_str.parse::<i32>(), to_power_str.parse::<i32>())
            {
                if from_power == to_power {
                    // Get base conversion factor and raise to power
                    let base_factor = convert_time_unit(1.0, from_base, to_base)
                        .or_else(|| library.convert(1.0, from_base, to_base))?;
                    Some(base_factor.powi(from_power))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            // No exponents in denominator, handle normally
            convert_time_unit(1.0, from_right, to_right)
                .or_else(|| library.convert(1.0, from_right, to_right))
        }?;

        // For division, divide the factors
        let combined_factor = factor_left / factor_right;
        return Some(value * combined_factor);
    }

    None
}

/// Convert time units with custom factors for rates
fn convert_time_unit(value: f64, from: &str, to: &str) -> Option<f64> {
    // Conversion factors to hours
    let to_hours = |unit: &str| -> Option<f64> {
        match unit {
            "s" => Some(1.0 / 3600.0),
            "min" => Some(1.0 / 60.0),
            "hr" | "h" => Some(1.0),
            "day" => Some(24.0),
            "month" => Some(730.0),    // Average 30.42 days
            "quarter" => Some(2190.0), // 3 months = 91.26 days * 24
            "year" => Some(8760.0),    // 365 days
            _ => None,
        }
    };

    let from_hours = to_hours(from)?;
    let to_hours_val = to_hours(to)?;

    Some(value * from_hours / to_hours_val)
}

pub fn parse_cell_input(input: &str) -> Result<Cell, String> {
    // Check if it's a formula
    if input.starts_with('=') {
        return Ok(Cell::with_formula(input.to_string()));
    }

    let input = input.trim();

    // Check if it's empty
    if input.is_empty() {
        return Ok(Cell::empty());
    }

    // Check for percentage (e.g., "15%", "15 %")
    if let Some(number_str) = input.strip_suffix('%') {
        let number_str = number_str.trim();
        if let Ok(value) = number_str.parse::<f64>() {
            // Store as fraction (15% -> 0.15)
            return Ok(Cell::new(
                value / 100.0,
                Unit::simple("%", BaseDimension::Custom("%".to_string())),
            ));
        }
    }

    // Check if it starts with a currency symbol (e.g., "$15", "USD 15")
    if input.starts_with('$')
        || input.starts_with("USD")
        || input.starts_with("EUR")
        || input.starts_with("GBP")
    {
        // Try to parse as currency-first format
        if let Some((currency, number_part)) = parse_currency_first(input) {
            return Ok(Cell::new(number_part, parse_unit(currency)));
        }
    }

    // Try to parse as number with optional unit (standard format: "15 USD", "100 m", "15 $/ft")
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(Cell::empty());
    }

    // Try to parse the first part as a number
    match parts[0].parse::<f64>() {
        Ok(value) => {
            // Successfully parsed as number, check for unit
            let unit_str = if parts.len() > 1 {
                parts[1..].join(" ")
            } else {
                String::new()
            };

            let unit = if unit_str.is_empty() {
                Unit::dimensionless()
            } else {
                parse_unit(&unit_str)
            };

            Ok(Cell::new(value, unit))
        }
        Err(_) => {
            // Not a number, treat as plain text
            Ok(Cell::with_text(input))
        }
    }
}

/// Parse currency-first format like "$15", "USD 100", "$15/ft"
fn parse_currency_first(input: &str) -> Option<(&str, f64)> {
    // Handle "$15" or "$15.5"
    if let Some(number_str) = input.strip_prefix('$') {
        if let Ok(value) = number_str.parse::<f64>() {
            return Some(("$", value));
        }
    }

    // Handle "USD 100", "EUR 50.5"
    if let Some(number_str) = input.strip_prefix("USD ") {
        if let Ok(value) = number_str.parse::<f64>() {
            return Some(("USD", value));
        }
    }
    if let Some(number_str) = input.strip_prefix("EUR ") {
        if let Ok(value) = number_str.parse::<f64>() {
            return Some(("EUR", value));
        }
    }
    if let Some(number_str) = input.strip_prefix("GBP ") {
        if let Ok(value) = number_str.parse::<f64>() {
            return Some(("GBP", value));
        }
    }
    None
}

pub fn parse_unit(unit_str: &str) -> Unit {
    // Check if it's a compound unit with division (e.g., "USD/ft", "mi/hr", "$/ft^2")
    if let Some(pos) = unit_str.find('/') {
        let numerator_str = &unit_str[..pos];
        let denominator_str = &unit_str[pos + 1..];

        let (num_dim, num_power) = parse_dimension_with_power(numerator_str);
        let (den_dim, den_power) = parse_dimension_with_power(denominator_str);

        return Unit::compound(
            unit_str.to_string(),
            vec![(num_dim, num_power)],
            vec![(den_dim, den_power)],
        );
    }

    // Check if it's a compound unit with multiplication (e.g., "ft*ft", "kg*m")
    if let Some(pos) = unit_str.find('*') {
        let left_str = &unit_str[..pos];
        let right_str = &unit_str[pos + 1..];

        let (left_dim, left_power) = parse_dimension_with_power(left_str);
        let (right_dim, right_power) = parse_dimension_with_power(right_str);

        return Unit::compound(
            unit_str.to_string(),
            vec![(left_dim.clone(), left_power), (right_dim, right_power)],
            vec![],
        );
    }

    // Check if it's a unit with power (e.g., "ft^2", "m^3")
    if let Some(pos) = unit_str.find('^') {
        let base_str = &unit_str[..pos];
        let power_str = &unit_str[pos + 1..];

        if let Ok(power) = power_str.parse::<i32>() {
            let dimension = get_base_dimension(base_str);
            return Unit::compound(unit_str.to_string(), vec![(dimension, power)], vec![]);
        }
    }

    // Simple unit
    let dimension = get_base_dimension(unit_str);
    Unit::simple(unit_str, dimension)
}

/// Parse a unit string and extract dimension with power (e.g., "ft^2" -> (Length, 2))
fn parse_dimension_with_power(unit_str: &str) -> (BaseDimension, i32) {
    if let Some(pos) = unit_str.find('^') {
        let base_str = &unit_str[..pos];
        let power_str = &unit_str[pos + 1..];

        if let Ok(power) = power_str.parse::<i32>() {
            return (get_base_dimension(base_str), power);
        }
    }

    (get_base_dimension(unit_str), 1)
}

fn get_base_dimension(unit_str: &str) -> BaseDimension {
    match unit_str {
        // Length - short forms
        "m" | "cm" | "mm" | "km" | "in" | "ft" | "yd" | "mi" => BaseDimension::Length,
        // Length - long forms (singular and plural)
        "meter" | "meters" | "centimeter" | "centimeters" | "millimeter" | "millimeters"
        | "kilometer" | "kilometers" | "inch" | "inches" | "foot" | "feet" | "yard" | "yards"
        | "mile" | "miles" => BaseDimension::Length,

        // Mass - short forms
        "g" | "kg" | "mg" | "oz" | "lb" => BaseDimension::Mass,
        // Mass - long forms (singular and plural)
        "gram" | "grams" | "kilogram" | "kilograms" | "milligram" | "milligrams" | "ounce"
        | "ounces" | "pound" | "pounds" => BaseDimension::Mass,

        // Time - short forms (basic time units are Time dimension)
        "s" | "min" | "hr" | "h" => BaseDimension::Time,
        // Time - long forms (singular and plural)
        "second" | "seconds" | "minute" | "minutes" | "hour" | "hours" => BaseDimension::Time,

        // Period units (day, month, quarter, year) are also Time dimension
        // This allows proper conversion between rates like $/quarter and $/year
        "day" | "days" | "month" | "months" | "quarter" | "quarters" | "year" | "years" | "yr" => {
            BaseDimension::Time
        }

        // Temperature - short forms
        "C" | "F" | "K" => BaseDimension::Temperature,
        // Temperature - long forms
        "Celsius" | "celsius" | "Fahrenheit" | "fahrenheit" | "Kelvin" | "kelvin" => {
            BaseDimension::Temperature
        }

        // Currency
        "USD" | "EUR" | "GBP" | "$" => BaseDimension::Currency,

        // Digital Storage
        "B" | "KB" | "MB" | "GB" | "TB" | "PB" | "Kb" | "Mb" | "Gb" | "Tb" | "Pb" | "Tok"
        | "MTok" => BaseDimension::DigitalStorage,
        // Digital Storage - long forms
        "byte" | "bytes" | "kilobyte" | "kilobytes" | "megabyte" | "megabytes" | "gigabyte"
        | "gigabytes" | "terabyte" | "terabytes" | "petabyte" | "petabytes" => {
            BaseDimension::DigitalStorage
        }

        _ => BaseDimension::Custom(unit_str.to_string()),
    }
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
    let display_mode = state.display_mode.lock().unwrap().clone();
    let preferences = state.unit_preferences.lock().unwrap().clone();

    tracing::debug!("get_sheet_cells_impl: display_mode={:?}, metric_time={}, imperial_time={}, time_rate_unit={}",
        display_mode, preferences.metric_time, preferences.imperial_time, preferences.time_rate_unit);

    let sheet = workbook.active_sheet();
    let cells: Vec<(String, CellData)> = sheet
        .cell_addresses()
        .into_iter()
        .filter_map(|addr| {
            sheet.get(&addr).map(|cell| {
                (
                    addr.to_string(),
                    cell_to_data_with_mode(cell, &display_mode, &preferences),
                )
            })
        })
        .collect();

    Ok(cells)
}

pub fn set_cell_impl(state: &AppState, address: String, value: String) -> Result<CellData, String> {
    use crate::core::cell_input::{parse_cell_input as parse_label_input, CellInput};

    let mut workbook_guard = state.workbook.lock().unwrap();
    let workbook = workbook_guard.as_mut().ok_or("No workbook loaded")?;

    let addr = CellAddr::from_string(&address).map_err(|e| e.to_string())?;
    let active_sheet_idx = workbook.active_sheet_index();

    // Parse input to check for inline label definition
    let parsed_input = parse_label_input(&value).map_err(|e| e.to_string())?;

    let cell = match parsed_input {
        CellInput::Plain(content) => {
            // No label, parse normally
            parse_cell_input(&content)?
        }
        CellInput::Labeled { label, content, .. } => {
            // Has label, register it as named range
            workbook
                .set_named_range(&label, active_sheet_idx, addr.clone())
                .map_err(|e| e.to_string())?;

            // Parse the content
            parse_cell_input(&content)?
        }
    };

    // Get named range mapping for dependency tracking
    let named_range_mapping = workbook.get_named_range_mapping_for_sheet(active_sheet_idx);

    // Set the cell with named range context for dependency tracking
    workbook
        .active_sheet_mut()
        .set_with_named_ranges(addr.clone(), cell.clone(), Some(&named_range_mapping))
        .map_err(|e| e.to_string())?;

    // Resolve all named ranges to their current values for recalculation
    let named_refs = workbook.resolve_named_ranges();

    // Always recalculate dependent cells when ANY cell changes
    // This ensures formulas that reference this cell get updated
    workbook
        .active_sheet_mut()
        .recalculate_with_named_refs(std::slice::from_ref(&addr), Some(&named_refs))
        .map_err(|e| e.to_string())?;

    // Return the updated cell (which may have been evaluated)
    let updated_cell = workbook
        .active_sheet()
        .get(&addr)
        .ok_or("Cell not found after setting")?;

    Ok(cell_to_data(updated_cell))
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
    let file =
        WorkbookFile::load_from_file(std::path::Path::new(&path)).map_err(|e| e.to_string())?;

    let mut workbook = file.to_workbook().map_err(|e| e.to_string())?;

    // Extract filename from path and set it as the workbook name
    if let Some(filename) = std::path::Path::new(&path)
        .file_stem()
        .and_then(|s| s.to_str())
    {
        workbook.set_name(filename);
    }

    // Save the original active sheet index
    let original_active = workbook.active_sheet_index();

    // Recalculate all formulas in ALL sheets after loading
    // This ensures all formula cells have up-to-date values
    for sheet_idx in 0..workbook.sheet_count() {
        workbook.set_active_sheet(sheet_idx).ok();

        let changed_cells: Vec<CellAddr> = workbook
            .active_sheet()
            .cell_addresses()
            .into_iter()
            .filter(|addr| {
                workbook
                    .active_sheet()
                    .get(addr)
                    .map(|cell| cell.formula().is_some())
                    .unwrap_or(false)
            })
            .collect();

        if !changed_cells.is_empty() {
            // Resolve named ranges before recalculation
            let named_refs = workbook.resolve_named_ranges();
            workbook
                .active_sheet_mut()
                .recalculate_with_named_refs(&changed_cells, Some(&named_refs))
                .map_err(|e| e.to_string())?;
        }
    }

    // Restore the original active sheet
    workbook.set_active_sheet(original_active).ok();

    *state.workbook.lock().unwrap() = Some(workbook);
    *state.current_file.lock().unwrap() = Some(path.clone());

    // Add to recent files
    add_to_recent_files_impl(state, path);

    Ok(())
}

pub fn get_current_file_impl(state: &AppState) -> Option<String> {
    state.current_file.lock().unwrap().clone()
}

pub fn set_display_mode_impl(state: &AppState, mode: String) -> Result<(), String> {
    let display_mode = match mode.as_str() {
        "AsEntered" => DisplayMode::AsEntered,
        "Metric" => DisplayMode::Metric,
        "Imperial" => DisplayMode::Imperial,
        _ => return Err(format!("Invalid display mode: {}", mode)),
    };

    tracing::info!("Setting display mode to: {:?}", display_mode);
    *state.display_mode.lock().unwrap() = display_mode;
    Ok(())
}

/// Format a unit string for better display (e.g., "ft^2" → "ft²", "ft*ft" → "ft²")
fn format_unit_display(unit: &str) -> String {
    // Check for ^2 notation (preferred internal format)
    if let Some(pos) = unit.find("^2") {
        let base = &unit[..pos];
        let rest = &unit[pos + 2..];
        // Convert ^2 to superscript ²
        if rest.is_empty() {
            return format!("{}²", base);
        } else {
            return format!("{}²{}", base, format_unit_display(rest));
        }
    }

    // Check for ^3 notation
    if let Some(pos) = unit.find("^3") {
        let base = &unit[..pos];
        let rest = &unit[pos + 2..];
        // Convert ^3 to superscript ³
        if rest.is_empty() {
            return format!("{}³", base);
        } else {
            return format!("{}³{}", base, format_unit_display(rest));
        }
    }

    // Check for squared units (same unit multiplied) - legacy support
    if let Some(pos) = unit.find('*') {
        let left = &unit[..pos];
        let right = &unit[pos + 1..];

        if left == right {
            // Same unit squared: ft*ft → ft²
            return format!("{}²", left);
        } else {
            // Different units: keep with · symbol
            return format!("{}·{}", left, right);
        }
    }

    // Check for division
    if let Some(pos) = unit.find('/') {
        let left = &unit[..pos];
        let right = &unit[pos + 1..];
        return format!("{}/{}", left, right);
    }

    // No compound unit, return as-is
    unit.to_string()
}

/// Get the preferred display unit for a given storage unit based on display mode and preferences
fn get_display_unit_for_mode(
    storage_unit: &str,
    mode: &DisplayMode,
    preferences: &UnitPreferences,
) -> Option<String> {
    // Handle compound units (e.g., "ft*ft", "m/s", "ft^2")
    if storage_unit.contains('*') || storage_unit.contains('/') || storage_unit.contains('^') {
        return get_compound_display_unit(storage_unit, mode, preferences);
    }

    match mode {
        DisplayMode::AsEntered => None, // Use storage unit as-is
        DisplayMode::Metric => {
            // Use preferences to determine target unit
            let base_dim = get_base_dimension(storage_unit);
            match base_dim {
                BaseDimension::Length => {
                    if storage_unit != preferences.metric_length {
                        Some(preferences.metric_length.clone())
                    } else {
                        None
                    }
                }
                BaseDimension::Mass => {
                    if storage_unit != preferences.metric_mass {
                        Some(preferences.metric_mass.clone())
                    } else {
                        None
                    }
                }
                BaseDimension::Time => {
                    tracing::debug!(
                        "  Time dimension (Metric): storage_unit={}, metric_time={}",
                        storage_unit,
                        preferences.metric_time
                    );
                    if storage_unit != preferences.metric_time {
                        Some(preferences.metric_time.clone())
                    } else {
                        None
                    }
                }
                BaseDimension::Temperature => {
                    if storage_unit != preferences.metric_temperature {
                        Some(preferences.metric_temperature.clone())
                    } else {
                        None
                    }
                }
                BaseDimension::Currency => {
                    if storage_unit != preferences.currency {
                        Some(preferences.currency.clone())
                    } else {
                        None
                    }
                }
                BaseDimension::DigitalStorage => {
                    if storage_unit != preferences.digital_storage_unit {
                        Some(preferences.digital_storage_unit.clone())
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
        DisplayMode::Imperial => {
            let base_dim = get_base_dimension(storage_unit);
            match base_dim {
                BaseDimension::Length => {
                    if storage_unit != preferences.imperial_length {
                        Some(preferences.imperial_length.clone())
                    } else {
                        None
                    }
                }
                BaseDimension::Mass => {
                    if storage_unit != preferences.imperial_mass {
                        Some(preferences.imperial_mass.clone())
                    } else {
                        None
                    }
                }
                BaseDimension::Time => {
                    tracing::debug!(
                        "  Time dimension (Imperial): storage_unit={}, imperial_time={}",
                        storage_unit,
                        preferences.imperial_time
                    );
                    if storage_unit != preferences.imperial_time {
                        Some(preferences.imperial_time.clone())
                    } else {
                        None
                    }
                }
                BaseDimension::Temperature => {
                    if storage_unit != preferences.imperial_temperature {
                        Some(preferences.imperial_temperature.clone())
                    } else {
                        None
                    }
                }
                BaseDimension::Currency => {
                    if storage_unit != preferences.currency {
                        Some(preferences.currency.clone())
                    } else {
                        None
                    }
                }
                BaseDimension::DigitalStorage => {
                    if storage_unit != preferences.digital_storage_unit {
                        Some(preferences.digital_storage_unit.clone())
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
    }
}

/// Get display unit for compound units based on display mode
fn get_compound_display_unit(
    storage_unit: &str,
    mode: &DisplayMode,
    preferences: &UnitPreferences,
) -> Option<String> {
    // Handle power notation (e.g., "ft^2", "m^3")
    // But NOT if the ^ is part of a division (e.g., "mi/hr^2")
    if let Some(pos) = storage_unit.find('^') {
        // Check if the ^ is NOT part of a division (denominator)
        let has_div = storage_unit.find('/').is_some_and(|div_pos| div_pos < pos);

        // Only handle pure power notation here (not in denominators)
        if !has_div {
            let base = &storage_unit[..pos];
            let power = &storage_unit[pos + 1..];

            // Convert the base unit
            let base_converted = get_display_unit_for_mode(base, mode, preferences)
                .unwrap_or_else(|| base.to_string());

            // Return with same power
            return Some(format!("{}^{}", base_converted, power));
        }
    }

    if let Some(pos) = storage_unit.find('*') {
        let left = &storage_unit[..pos];
        let right = &storage_unit[pos + 1..];

        // Convert each component
        let left_converted =
            get_display_unit_for_mode(left, mode, preferences).unwrap_or_else(|| left.to_string());
        let right_converted = get_display_unit_for_mode(right, mode, preferences)
            .unwrap_or_else(|| right.to_string());

        // Return compound unit
        Some(format!("{}*{}", left_converted, right_converted))
    } else if let Some(pos) = storage_unit.find('/') {
        let left = &storage_unit[..pos];
        let right = &storage_unit[pos + 1..];

        // Convert numerator (left side) - may have exponents
        let left_converted = if let Some(exp_pos) = left.find('^') {
            // Has exponent: convert base and keep exponent
            let base = &left[..exp_pos];
            let power = &left[exp_pos + 1..];
            let base_converted = get_display_unit_for_mode(base, mode, preferences)
                .unwrap_or_else(|| base.to_string());
            format!("{}^{}", base_converted, power)
        } else {
            // No exponent: convert normally
            get_display_unit_for_mode(left, mode, preferences).unwrap_or_else(|| left.to_string())
        };

        // Convert denominator (right side) - may have exponents
        let right_converted = if let Some(exp_pos) = right.find('^') {
            // Has exponent: extract base and convert it with preferences
            let base = &right[..exp_pos];
            let power = &right[exp_pos + 1..];

            // Convert base according to preferences
            let base_converted = get_display_unit_for_mode(base, mode, preferences)
                .unwrap_or_else(|| base.to_string());
            format!("{}^{}", base_converted, power)
        } else {
            // No exponent: check if it's a time unit for special rate handling
            let right_dim = get_base_dimension(right);
            if right_dim == BaseDimension::Time && mode != &DisplayMode::AsEntered {
                // Use the time rate unit preference for rates (e.g., $/hr -> $/month)
                if right != preferences.time_rate_unit.as_str() {
                    preferences.time_rate_unit.clone()
                } else {
                    right.to_string()
                }
            } else {
                get_display_unit_for_mode(right, mode, preferences)
                    .unwrap_or_else(|| right.to_string())
            }
        };

        // Return compound unit
        Some(format!("{}/{}", left_converted, right_converted))
    } else {
        None
    }
}

// Unit preferences commands

pub fn get_unit_preferences_impl(state: &AppState) -> Result<UnitPreferences, String> {
    let prefs = state.unit_preferences.lock().unwrap();
    Ok(prefs.clone())
}

pub fn update_unit_preferences_impl(
    state: &AppState,
    preferences: UnitPreferences,
) -> Result<(), String> {
    tracing::info!("update_unit_preferences_impl called with: metric_time={}, imperial_time={}, time_rate_unit={}",
        preferences.metric_time, preferences.imperial_time, preferences.time_rate_unit);
    *state.unit_preferences.lock().unwrap() = preferences;
    tracing::info!("  preferences updated successfully");
    Ok(())
}

pub fn set_metric_system_impl(state: &AppState, system: String) -> Result<(), String> {
    use crate::core::settings::MetricSystem;

    let metric_system = match system.as_str() {
        "CGS" => MetricSystem::CGS,
        "MKS" => MetricSystem::MKS,
        _ => return Err(format!("Invalid metric system: {}", system)),
    };

    let mut prefs = state.unit_preferences.lock().unwrap();
    prefs.metric_system = metric_system;

    // Update default units based on system choice
    match prefs.metric_system {
        MetricSystem::CGS => {
            prefs.metric_length = "cm".to_string();
            prefs.metric_mass = "g".to_string();
        }
        MetricSystem::MKS => {
            prefs.metric_length = "m".to_string();
            prefs.metric_mass = "kg".to_string();
        }
    }

    Ok(())
}

pub fn set_currency_rate_impl(state: &AppState, currency: String, rate: f64) -> Result<(), String> {
    let mut prefs = state.unit_preferences.lock().unwrap();
    prefs.set_currency_rate(currency, rate);
    Ok(())
}

pub fn get_currencies_impl(state: &AppState) -> Result<Vec<String>, String> {
    let prefs = state.unit_preferences.lock().unwrap();
    Ok(prefs.get_currencies())
}

/// Get all units currently in use in the active sheet
pub fn get_units_in_use_impl(state: &AppState) -> Result<Vec<String>, String> {
    use std::collections::HashSet;

    let workbook_guard = state.workbook.lock().unwrap();
    let workbook = workbook_guard.as_ref().ok_or("No workbook loaded")?;

    let sheet = workbook.active_sheet();
    let mut units = HashSet::new();

    for addr in sheet.cell_addresses() {
        if let Some(cell) = sheet.get(&addr) {
            let storage_unit = cell.storage_unit().canonical().to_string();

            // Skip dimensionless units
            if storage_unit.is_empty() || storage_unit == "1" {
                continue;
            }

            // Add the unit
            units.insert(storage_unit.clone());

            // Also add component units for compound units
            if let Some(pos) = storage_unit.find('*') {
                units.insert(storage_unit[..pos].to_string());
                units.insert(storage_unit[pos + 1..].to_string());
            } else if let Some(pos) = storage_unit.find('/') {
                units.insert(storage_unit[..pos].to_string());
                units.insert(storage_unit[pos + 1..].to_string());
            }
        }
    }

    let mut result: Vec<String> = units.into_iter().collect();
    result.sort();
    Ok(result)
}

/// Get all base units currently in use in the active sheet
pub fn get_base_units_in_use_impl(state: &AppState) -> Result<Vec<String>, String> {
    use std::collections::HashSet;

    let workbook_guard = state.workbook.lock().unwrap();
    let workbook = workbook_guard.as_ref().ok_or("No workbook loaded")?;
    let sheet = workbook.active_sheet();
    let mut base_units = HashSet::new();

    for addr in sheet.cell_addresses() {
        if let Some(cell) = sheet.get(&addr) {
            let unit = cell.storage_unit();
            for base in unit.base_units() {
                base_units.insert(base);
            }
        }
    }

    let mut result: Vec<String> = base_units.into_iter().collect();
    result.sort();
    Ok(result)
}

#[tauri::command]
pub fn get_base_units_in_use(state: tauri::State<AppState>) -> Result<Vec<String>, String> {
    get_base_units_in_use_impl(&state)
}

/// Get all cell addresses that contain a specific base unit
pub fn get_cells_with_base_unit_impl(
    state: &AppState,
    base_unit: &str,
) -> Result<Vec<String>, String> {
    let workbook_guard = state.workbook.lock().unwrap();
    let workbook = workbook_guard.as_ref().ok_or("No workbook loaded")?;
    let sheet = workbook.active_sheet();
    let mut matching_cells = Vec::new();

    for addr in sheet.cell_addresses() {
        if let Some(cell) = sheet.get(&addr) {
            let unit = cell.storage_unit();
            let bases = unit.base_units();

            if bases.contains(&base_unit.to_string()) {
                matching_cells.push(addr.to_string());
            }
        }
    }

    // Sort for consistent ordering
    matching_cells.sort();
    Ok(matching_cells)
}

#[tauri::command]
pub fn get_cells_with_base_unit(
    state: tauri::State<AppState>,
    base_unit: String,
) -> Result<Vec<String>, String> {
    get_cells_with_base_unit_impl(&state, &base_unit)
}

/// Generate debug export text and copy to clipboard
pub fn export_debug_to_clipboard_impl(
    state: &AppState,
    frontend_version: Option<String>,
    frontend_commit: Option<String>,
) -> Result<(), String> {
    let debug_text = get_debug_export_impl(state, frontend_version, frontend_commit)?;

    // Copy to clipboard using arboard
    use arboard::Clipboard;
    let mut clipboard =
        Clipboard::new().map_err(|e| format!("Failed to access clipboard: {}", e))?;
    clipboard
        .set_text(debug_text)
        .map_err(|e| format!("Failed to write to clipboard: {}", e))?;

    Ok(())
}

/// Generate debug export text for clipboard
pub fn get_debug_export_impl(
    state: &AppState,
    frontend_version: Option<String>,
    frontend_commit: Option<String>,
) -> Result<String, String> {
    let workbook_guard = state.workbook.lock().unwrap();
    let workbook = workbook_guard.as_ref().ok_or("No workbook loaded")?;
    let display_mode = state.display_mode.lock().unwrap();
    let preferences = state.unit_preferences.lock().unwrap();

    let mut output = String::new();

    // Version information
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const GIT_COMMIT: &str = env!("GIT_COMMIT");

    // Backend version
    output.push_str(&format!(
        "version: backend={} commit={}\n",
        VERSION, GIT_COMMIT
    ));

    // Frontend version
    if let (Some(fe_version), Some(fe_commit)) = (frontend_version, frontend_commit) {
        output.push_str(&format!(
            "         frontend={} commit={}\n\n",
            fe_version, fe_commit
        ));
    } else {
        output.push_str("         frontend=unknown commit=unknown\n\n");
    }

    // Display mode
    output.push_str(&format!("display: {:?}\n", display_mode));

    // Unit settings
    output.push_str("\nunit settings:\n");
    output.push_str(&format!(
        "  metric system: {:?}\n",
        preferences.metric_system
    ));
    output.push_str(&format!("  metric length: {}\n", preferences.metric_length));
    output.push_str(&format!("  metric mass: {}\n", preferences.metric_mass));
    output.push_str(&format!("  metric time: {}\n", preferences.metric_time));
    output.push_str(&format!(
        "  imperial length: {}\n",
        preferences.imperial_length
    ));
    output.push_str(&format!("  imperial mass: {}\n", preferences.imperial_mass));
    output.push_str(&format!("  imperial time: {}\n", preferences.imperial_time));
    output.push_str(&format!(
        "  time rate unit: {}\n",
        preferences.time_rate_unit
    ));
    output.push_str(&format!("  currency: {}\n", preferences.currency));
    output.push_str(&format!(
        "  digital storage: {}\n",
        preferences.digital_storage_unit
    ));
    output.push_str(&format!(
        "  metric temperature: {}\n",
        preferences.metric_temperature
    ));
    output.push_str(&format!(
        "  imperial temperature: {}\n",
        preferences.imperial_temperature
    ));

    // Get all cells
    let sheet = workbook.active_sheet();
    let mut cell_addresses: Vec<_> = sheet.cell_addresses();
    cell_addresses.sort_by(|a, b| {
        // Sort by row first, then column
        if a.row != b.row {
            a.row.cmp(&b.row)
        } else {
            a.col.cmp(&b.col)
        }
    });

    // Cells section
    output.push_str("\ncells:\n");
    for addr in cell_addresses {
        if let Some(cell) = sheet.get(&addr) {
            // Get storage value (unconverted)
            let storage_data = cell_to_data(cell);
            let storage_str = format_cell_value(&storage_data.value, &storage_data.storage_unit);

            // Get display value (converted based on mode)
            let display_data = cell_to_data_with_mode(cell, &display_mode, &preferences);
            let display_str = if let Some(display_unit) = &display_data.display_unit {
                format_cell_value(&display_data.value, display_unit)
            } else {
                storage_str.clone()
            };

            // Show formula if present
            if let Some(formula) = &storage_data.formula {
                output.push_str(&format!("{}: {} : {}\n", addr, formula, display_str));
            } else {
                output.push_str(&format!("{}: {} : {}\n", addr, storage_str, display_str));
            }
        }
    }

    Ok(output)
}

/// Format cell value with unit for debug export
fn format_cell_value(value: &CellValueData, unit: &str) -> String {
    match value {
        CellValueData::Empty => String::new(),
        CellValueData::Number { value } => {
            // Special handling for percentages: convert 0.15 -> "15%"
            if unit == "%" {
                format!("{}%", value * 100.0)
            } else if unit.is_empty() || unit == "1" {
                format!("{}", value)
            } else {
                format!("{} {}", value, unit)
            }
        }
        CellValueData::Text { text } => text.clone(),
        CellValueData::Error { message } => format!("#ERROR: {}", message),
    }
}

/// Export workbook to Excel format
pub fn export_to_excel_impl(state: &AppState, path: String) -> Result<(), String> {
    let workbook_guard = state.workbook.lock().unwrap();
    let workbook = workbook_guard.as_ref().ok_or("No workbook loaded")?;

    // Use the export_to_excel function from formats module
    use crate::formats::export_to_excel;

    export_to_excel(workbook, &path).map_err(|e| format!("Failed to export to Excel: {}", e))?;

    Ok(())
}

/// Set the active sheet by index
pub fn set_active_sheet_impl(state: &AppState, index: usize) -> Result<(), String> {
    let mut workbook_guard = state.workbook.lock().unwrap();
    let workbook = workbook_guard.as_mut().ok_or("No workbook loaded")?;

    workbook
        .set_active_sheet(index)
        .map_err(|e| e.to_string())?;

    // Recalculate all formulas in the newly active sheet
    // This ensures all formula cells have up-to-date values
    let changed_cells: Vec<CellAddr> = workbook
        .active_sheet()
        .cell_addresses()
        .into_iter()
        .filter(|addr| {
            workbook
                .active_sheet()
                .get(addr)
                .map(|cell| cell.formula().is_some())
                .unwrap_or(false)
        })
        .collect();

    if !changed_cells.is_empty() {
        // Resolve named ranges before recalculation
        let named_refs = workbook.resolve_named_ranges();
        workbook
            .active_sheet_mut()
            .recalculate_with_named_refs(&changed_cells, Some(&named_refs))
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// Add a new sheet to the workbook
pub fn add_sheet_impl(state: &AppState) -> Result<usize, String> {
    let mut workbook_guard = state.workbook.lock().unwrap();
    let workbook = workbook_guard.as_mut().ok_or("No workbook loaded")?;

    let new_index = workbook.add_sheet();
    Ok(new_index)
}

/// Rename a sheet
pub fn rename_sheet_impl(state: &AppState, index: usize, new_name: String) -> Result<(), String> {
    let mut workbook_guard = state.workbook.lock().unwrap();
    let workbook = workbook_guard.as_mut().ok_or("No workbook loaded")?;

    workbook
        .rename_sheet(index, new_name)
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Delete a sheet (cannot delete last sheet)
pub fn delete_sheet_impl(state: &AppState, index: usize) -> Result<(), String> {
    let mut workbook_guard = state.workbook.lock().unwrap();
    let workbook = workbook_guard.as_mut().ok_or("No workbook loaded")?;

    workbook.remove_sheet(index).map_err(|e| e.to_string())?;
    Ok(())
}

/// Check if a sheet has any data (non-empty cells)
pub fn sheet_has_data_impl(state: &AppState, index: usize) -> Result<bool, String> {
    let workbook_guard = state.workbook.lock().unwrap();
    let workbook = workbook_guard.as_ref().ok_or("No workbook loaded")?;

    let sheet = workbook.get_sheet(index).ok_or("Sheet not found")?;
    let has_data = !sheet.cell_addresses().is_empty();

    Ok(has_data)
}

// Recent files management

/// Add a file to the recent files list
pub fn add_to_recent_files_impl(state: &AppState, path: String) {
    let mut recent_files = state.recent_files.lock().unwrap();

    // Remove the path if it already exists (we'll re-add it at the front)
    recent_files.retain(|p| p != &path);

    // Add to the front of the list
    recent_files.insert(0, path);

    // Keep only the 3 most recent files
    recent_files.truncate(3);
}

/// Get the list of recent files
pub fn get_recent_files_impl(state: &AppState) -> Vec<String> {
    state.recent_files.lock().unwrap().clone()
}

// Named range commands

/// Data structure for named range information
#[derive(Debug, Serialize, Deserialize)]
pub struct NamedRangeInfo {
    pub name: String,
    pub sheet_index: usize,
    pub cell_address: String,
}

/// List all named ranges in the workbook
pub fn list_named_ranges_impl(state: &AppState) -> Result<Vec<NamedRangeInfo>, String> {
    let workbook_guard = state.workbook.lock().unwrap();
    let workbook = workbook_guard.as_ref().ok_or("No workbook loaded")?;

    let ranges = workbook
        .list_named_ranges()
        .into_iter()
        .map(|(name, sheet_index, addr)| NamedRangeInfo {
            name,
            sheet_index,
            cell_address: addr.to_string(),
        })
        .collect();

    Ok(ranges)
}

/// Create a named range
pub fn create_named_range_impl(
    state: &AppState,
    name: String,
    sheet_index: usize,
    cell_address: String,
) -> Result<(), String> {
    let mut workbook_guard = state.workbook.lock().unwrap();
    let workbook = workbook_guard.as_mut().ok_or("No workbook loaded")?;

    let addr = CellAddr::from_string(&cell_address).map_err(|e| e.to_string())?;

    workbook
        .set_named_range(&name, sheet_index, addr)
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Delete a named range
pub fn delete_named_range_impl(state: &AppState, name: String) -> Result<(), String> {
    let mut workbook_guard = state.workbook.lock().unwrap();
    let workbook = workbook_guard.as_mut().ok_or("No workbook loaded")?;

    workbook
        .remove_named_range(&name)
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Get information about a specific named range
pub fn get_named_range_impl(state: &AppState, name: String) -> Result<NamedRangeInfo, String> {
    let workbook_guard = state.workbook.lock().unwrap();
    let workbook = workbook_guard.as_ref().ok_or("No workbook loaded")?;

    let (sheet_index, addr) = workbook
        .get_named_range(&name)
        .ok_or_else(|| format!("Named range not found: {}", name))?;

    Ok(NamedRangeInfo {
        name,
        sheet_index,
        cell_address: addr.to_string(),
    })
}

/// Get the named range for a specific cell address
/// Returns None if the cell doesn't have a named range
pub fn get_named_range_for_cell_impl(
    state: &AppState,
    sheet_index: usize,
    cell_address: String,
) -> Result<Option<String>, String> {
    let workbook_guard = state.workbook.lock().unwrap();
    let workbook = workbook_guard.as_ref().ok_or("No workbook loaded")?;

    let addr = CellAddr::from_string(&cell_address).map_err(|e| e.to_string())?;

    Ok(workbook.get_named_range_for_cell(sheet_index, &addr))
}
