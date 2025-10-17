// JSON serialization for workbook files (.usheet format)

use crate::core::cell::{Cell, CellValue};
use crate::core::table::{CellAddr, Sheet};
use crate::core::units::{BaseDimension, Unit};
use crate::core::workbook::{DisplayPreference, Workbook, WorkbookSettings};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SerializationError {
    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid version: expected {expected}, got {actual}")]
    InvalidVersion { expected: String, actual: String },

    #[error("Workbook construction error: {0}")]
    WorkbookError(String),
}

/// File format version
const FORMAT_VERSION: &str = "1.0";

/// Serializable workbook format
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkbookFile {
    /// Format version for compatibility checking
    version: String,

    /// Metadata about the file
    metadata: FileMetadata,

    /// Workbook data
    workbook: WorkbookData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetadata {
    /// When the file was created
    created_at: String,

    /// When the file was last modified
    modified_at: String,

    /// Application version that created the file
    app_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkbookData {
    /// Workbook name
    name: String,

    /// Settings
    settings: WorkbookSettingsData,

    /// Sheets
    sheets: Vec<SheetData>,

    /// Active sheet index
    active_sheet: usize,

    /// Named ranges (name -> (sheet_index, cell_address))
    #[serde(default)]
    named_ranges: HashMap<String, NamedRangeData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamedRangeData {
    sheet_index: usize,
    cell_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkbookSettingsData {
    display_preference: String, // "AsEntered", "Metric", "Imperial"
    auto_recalculate: bool,
    show_warnings: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SheetData {
    /// Sheet name
    name: String,

    /// Cells (sparse storage)
    cells: HashMap<String, CellData>,

    /// Column widths (column -> width in pixels)
    #[serde(default)]
    column_widths: HashMap<String, f64>,

    /// Row heights (row -> height in pixels)
    #[serde(default)]
    row_heights: HashMap<usize, f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CellData {
    /// Cell value
    value: CellValueData,

    /// Storage unit (canonical form)
    storage_unit: String,

    /// Display unit (if different from storage)
    display_unit: Option<String>,

    /// Formula (if present)
    formula: Option<String>,

    /// Warning message (if any)
    warning: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CellValueData {
    Empty,
    Number { value: f64 },
    Text { text: String },
    Error { message: String },
}

impl WorkbookFile {
    /// Create a new workbook file from a workbook
    pub fn from_workbook(workbook: &Workbook) -> Self {
        let now = chrono::Utc::now().to_rfc3339();

        Self {
            version: FORMAT_VERSION.to_string(),
            metadata: FileMetadata {
                created_at: now.clone(),
                modified_at: now,
                app_version: env!("CARGO_PKG_VERSION").to_string(),
            },
            workbook: WorkbookData::from_workbook(workbook),
        }
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> Result<String, SerializationError> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Parse from JSON string
    pub fn from_json(json: &str) -> Result<Self, SerializationError> {
        Ok(serde_json::from_str(json)?)
    }

    /// Save to file
    pub fn save_to_file(&self, path: &std::path::Path) -> Result<(), SerializationError> {
        let json = self.to_json()?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load from file
    pub fn load_from_file(path: &std::path::Path) -> Result<Self, SerializationError> {
        let json = std::fs::read_to_string(path)?;
        let file = Self::from_json(&json)?;

        // Check version compatibility
        if file.version != FORMAT_VERSION {
            return Err(SerializationError::InvalidVersion {
                expected: FORMAT_VERSION.to_string(),
                actual: file.version,
            });
        }

        Ok(file)
    }

    /// Convert back to a Workbook
    pub fn to_workbook(&self) -> Result<Workbook, SerializationError> {
        self.workbook.to_workbook()
    }
}

impl WorkbookData {
    fn from_workbook(workbook: &Workbook) -> Self {
        let sheets: Vec<SheetData> = (0..workbook.sheet_count())
            .filter_map(|i| workbook.get_sheet(i))
            .map(SheetData::from_sheet)
            .collect();

        // Serialize named ranges
        let named_ranges: HashMap<String, NamedRangeData> = workbook
            .list_named_ranges()
            .into_iter()
            .map(|(name, sheet_index, addr)| {
                (
                    name,
                    NamedRangeData {
                        sheet_index,
                        cell_address: addr.to_string(),
                    },
                )
            })
            .collect();

        Self {
            name: workbook.name().to_string(),
            settings: WorkbookSettingsData::from_settings(workbook.settings()),
            sheets,
            active_sheet: workbook.active_sheet_index(),
            named_ranges,
        }
    }

    fn to_workbook(&self) -> Result<Workbook, SerializationError> {
        let mut workbook = Workbook::new(&self.name);

        // Process sheets - reuse default sheet for first one, add rest
        for (i, sheet_data) in self.sheets.iter().enumerate() {
            let idx = if i == 0 {
                // Rename the default sheet to match first sheet
                workbook.rename_sheet(0, &sheet_data.name).ok();
                0
            } else {
                // Add additional sheets
                workbook.add_sheet_with_name(&sheet_data.name)
            };

            // Populate sheet with cells, column widths, and row heights
            if let Some(wb_sheet) = workbook.get_sheet_mut(idx) {
                for (addr_str, cell_data) in &sheet_data.cells {
                    if let Ok(addr) = CellAddr::from_string(addr_str) {
                        if let Ok(cell) = cell_data.to_cell() {
                            wb_sheet.set(addr, cell).ok();
                        }
                    }
                }

                // Restore column widths
                for (col, width) in &sheet_data.column_widths {
                    wb_sheet.set_column_width(col.to_string(), *width);
                }

                // Restore row heights
                for (row, height) in &sheet_data.row_heights {
                    wb_sheet.set_row_height(*row, *height);
                }
            }
        }

        // Set active sheet
        if self.active_sheet < workbook.sheet_count() {
            workbook.set_active_sheet(self.active_sheet).ok();
        }

        // Apply settings
        workbook.set_display_preference(self.settings.to_display_preference());
        workbook.settings_mut().auto_recalculate = self.settings.auto_recalculate;
        workbook.settings_mut().show_warnings = self.settings.show_warnings;

        // Restore named ranges
        for (name, range_data) in &self.named_ranges {
            if let Ok(addr) = CellAddr::from_string(&range_data.cell_address) {
                workbook
                    .set_named_range(name, range_data.sheet_index, addr)
                    .ok();
            }
        }

        workbook.mark_clean();

        Ok(workbook)
    }
}

impl WorkbookSettingsData {
    fn from_settings(settings: &WorkbookSettings) -> Self {
        Self {
            display_preference: match settings.display_preference {
                DisplayPreference::AsEntered => "AsEntered",
                DisplayPreference::Metric => "Metric",
                DisplayPreference::Imperial => "Imperial",
            }
            .to_string(),
            auto_recalculate: settings.auto_recalculate,
            show_warnings: settings.show_warnings,
        }
    }

    fn to_display_preference(&self) -> DisplayPreference {
        match self.display_preference.as_str() {
            "Metric" => DisplayPreference::Metric,
            "Imperial" => DisplayPreference::Imperial,
            _ => DisplayPreference::AsEntered,
        }
    }
}

impl SheetData {
    fn from_sheet(sheet: &Sheet) -> Self {
        let cells: HashMap<String, CellData> = sheet
            .cell_addresses()
            .into_iter()
            .filter_map(|addr| {
                sheet
                    .get(&addr)
                    .map(|cell| (addr.to_string(), CellData::from_cell(cell)))
            })
            .collect();

        Self {
            name: sheet.name().to_string(),
            cells,
            column_widths: sheet.get_all_column_widths().clone(),
            row_heights: sheet.get_all_row_heights().clone(),
        }
    }
}

impl CellData {
    fn from_cell(cell: &Cell) -> Self {
        Self {
            value: CellValueData::from_cell_value(cell.value()),
            storage_unit: cell.storage_unit().canonical().to_string(),
            display_unit: cell
                .display_unit()
                .canonical()
                .ne(cell.storage_unit().canonical())
                .then(|| cell.display_unit().canonical().to_string()),
            formula: cell.formula().map(|s| s.to_string()),
            warning: cell.warning().map(|s| s.to_string()),
        }
    }

    fn to_cell(&self) -> Result<Cell, SerializationError> {
        // Parse unit from canonical form
        let unit = parse_unit_from_canonical(&self.storage_unit);

        let mut cell = match &self.value {
            CellValueData::Empty => Cell::empty(),
            CellValueData::Number { value } => Cell::new(*value, unit.clone()),
            CellValueData::Text { text } => Cell::with_text(text.clone()),
            CellValueData::Error { message } => {
                let mut c = Cell::empty();
                c.set_value(CellValue::Error(message.clone()));
                c
            }
        };

        // Set formula if present
        if let Some(formula) = &self.formula {
            cell = Cell::with_formula(formula.clone());
            // Set the evaluated value
            if let CellValueData::Number { value } = &self.value {
                cell.set_value(CellValue::Number(*value));
                cell.set_storage_unit(unit.clone());
            }
        }

        // Set display unit if different
        if let Some(display_unit_str) = &self.display_unit {
            let display_unit = parse_unit_from_canonical(display_unit_str);
            cell.set_display_unit(Some(display_unit));
        }

        // Set warning
        if let Some(warning) = &self.warning {
            cell.set_warning(Some(warning.clone()));
        }

        Ok(cell)
    }
}

impl CellValueData {
    fn from_cell_value(value: &CellValue) -> Self {
        match value {
            CellValue::Empty => Self::Empty,
            CellValue::Number(n) => Self::Number { value: *n },
            CellValue::Text(t) => Self::Text { text: t.clone() },
            CellValue::Error(e) => Self::Error { message: e.clone() },
        }
    }
}

/// Parse a unit from its canonical form
/// This handles simple units, compound units with powers, and units with division
fn parse_unit_from_canonical(canonical: &str) -> Unit {
    if canonical.is_empty() {
        return Unit::dimensionless();
    }

    // Check if it's a compound unit with division (e.g., "USD/ft", "mi/hr", "$/ft^2")
    if let Some(pos) = canonical.find('/') {
        let numerator_str = &canonical[..pos];
        let denominator_str = &canonical[pos + 1..];

        let (num_dim, num_power) = parse_dimension_with_power(numerator_str);
        let (den_dim, den_power) = parse_dimension_with_power(denominator_str);

        return Unit::compound(
            canonical.to_string(),
            vec![(num_dim, num_power)],
            vec![(den_dim, den_power)],
        );
    }

    // Check if it's a compound unit with multiplication (e.g., "ft*ft", "kg*m")
    if let Some(pos) = canonical.find('*') {
        let left_str = &canonical[..pos];
        let right_str = &canonical[pos + 1..];

        let (left_dim, left_power) = parse_dimension_with_power(left_str);
        let (right_dim, right_power) = parse_dimension_with_power(right_str);

        return Unit::compound(
            canonical.to_string(),
            vec![(left_dim.clone(), left_power), (right_dim, right_power)],
            vec![],
        );
    }

    // Check if it's a unit with power (e.g., "ft^2", "m^3")
    if let Some(pos) = canonical.find('^') {
        let base_str = &canonical[..pos];
        let power_str = &canonical[pos + 1..];

        if let Ok(power) = power_str.parse::<i32>() {
            let dimension = get_base_dimension_for_json(base_str);
            return Unit::compound(canonical.to_string(), vec![(dimension, power)], vec![]);
        }
    }

    // Simple unit
    let dimension = get_base_dimension_for_json(canonical);
    Unit::simple(canonical, dimension)
}

/// Parse a unit string and extract dimension with power (e.g., "ft^2" -> (Length, 2))
fn parse_dimension_with_power(unit_str: &str) -> (BaseDimension, i32) {
    if let Some(pos) = unit_str.find('^') {
        let base_str = &unit_str[..pos];
        let power_str = &unit_str[pos + 1..];

        if let Ok(power) = power_str.parse::<i32>() {
            return (get_base_dimension_for_json(base_str), power);
        }
    }

    (get_base_dimension_for_json(unit_str), 1)
}

/// Get base dimension for a unit string (helper for JSON parsing)
fn get_base_dimension_for_json(unit_str: &str) -> BaseDimension {
    match unit_str {
        "m" | "cm" | "mm" | "km" | "in" | "ft" | "yd" | "mi" => BaseDimension::Length,
        "g" | "kg" | "mg" | "oz" | "lb" => BaseDimension::Mass,
        "s" | "min" | "hr" | "h" | "hour" | "day" | "month" | "quarter" | "year" | "yr" => {
            BaseDimension::Time
        }
        "C" | "F" | "K" => BaseDimension::Temperature,
        "USD" | "EUR" | "GBP" | "$" => BaseDimension::Currency,
        "B" | "KB" | "MB" | "GB" | "TB" | "PB" | "Kb" | "Mb" | "Gb" | "Tb" | "Pb" | "Tok"
        | "MTok" => BaseDimension::DigitalStorage,
        _ => BaseDimension::Custom(unit_str.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::units::BaseDimension;

    #[test]
    fn test_serialize_empty_workbook() {
        let workbook = Workbook::new("Test Workbook");
        let file = WorkbookFile::from_workbook(&workbook);

        assert_eq!(file.version, FORMAT_VERSION);
        assert_eq!(file.workbook.name, "Test Workbook");
        assert_eq!(file.workbook.sheets.len(), 1);
    }

    #[test]
    fn test_serialize_deserialize_workbook() {
        let mut workbook = Workbook::new("Test");

        // Add a cell
        let addr = CellAddr::new("A", 1);
        let cell = Cell::new(100.0, Unit::simple("m", BaseDimension::Length));
        workbook.active_sheet_mut().set(addr, cell).unwrap();

        // Serialize
        let file = WorkbookFile::from_workbook(&workbook);
        let json = file.to_json().unwrap();

        // Deserialize
        let file2 = WorkbookFile::from_json(&json).unwrap();
        let workbook2 = file2.to_workbook().unwrap();

        // Verify
        assert_eq!(workbook2.name(), "Test");
        assert_eq!(workbook2.sheet_count(), 1);

        let addr = CellAddr::new("A", 1);
        let cell = workbook2.active_sheet().get(&addr).unwrap();
        assert_eq!(cell.as_number(), Some(100.0));
        assert_eq!(cell.storage_unit().canonical(), "m");
    }

    #[test]
    fn test_round_trip_with_formula() {
        let mut workbook = Workbook::new("Test");

        // Add cells
        workbook
            .active_sheet_mut()
            .set(
                CellAddr::new("A", 1),
                Cell::new(10.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        workbook
            .active_sheet_mut()
            .set(CellAddr::new("A", 2), Cell::with_formula("=A1 * 2"))
            .unwrap();

        // Round trip
        let file = WorkbookFile::from_workbook(&workbook);
        let json = file.to_json().unwrap();
        let file2 = WorkbookFile::from_json(&json).unwrap();
        let workbook2 = file2.to_workbook().unwrap();

        // Verify formula
        let cell = workbook2
            .active_sheet()
            .get(&CellAddr::new("A", 2))
            .unwrap();
        assert!(cell.is_formula());
        assert_eq!(cell.formula(), Some("=A1 * 2"));
    }

    #[test]
    fn test_multiple_sheets() {
        let mut workbook = Workbook::new("Multi-Sheet");
        workbook.add_sheet_with_name("Sheet2");
        workbook.add_sheet_with_name("Sheet3");

        // Round trip
        let file = WorkbookFile::from_workbook(&workbook);
        let json = file.to_json().unwrap();
        let file2 = WorkbookFile::from_json(&json).unwrap();
        let workbook2 = file2.to_workbook().unwrap();

        assert_eq!(workbook2.sheet_count(), 3);
        assert_eq!(workbook2.get_sheet(0).unwrap().name(), "Sheet1");
        assert_eq!(workbook2.get_sheet(1).unwrap().name(), "Sheet2");
        assert_eq!(workbook2.get_sheet(2).unwrap().name(), "Sheet3");
    }

    #[test]
    fn test_named_ranges_serialization() {
        let mut workbook = Workbook::new("Test");

        // Add some cells
        workbook
            .active_sheet_mut()
            .set(
                CellAddr::new("A", 1),
                Cell::new(100.0, Unit::simple("USD", BaseDimension::Currency)),
            )
            .unwrap();

        workbook
            .active_sheet_mut()
            .set(
                CellAddr::new("B", 1),
                Cell::new(0.15, Unit::dimensionless()),
            )
            .unwrap();

        // Add named ranges
        workbook
            .set_named_range("revenue", 0, CellAddr::new("A", 1))
            .unwrap();
        workbook
            .set_named_range("tax_rate", 0, CellAddr::new("B", 1))
            .unwrap();

        // Serialize
        let file = WorkbookFile::from_workbook(&workbook);
        let json = file.to_json().unwrap();

        // Verify JSON contains named ranges
        assert!(json.contains("named_ranges"));
        assert!(json.contains("revenue"));
        assert!(json.contains("tax_rate"));

        // Deserialize
        let file2 = WorkbookFile::from_json(&json).unwrap();
        let workbook2 = file2.to_workbook().unwrap();

        // Verify named ranges are restored
        let ranges = workbook2.list_named_ranges();
        assert_eq!(ranges.len(), 2);

        let revenue_range = workbook2.get_named_range("revenue");
        assert!(revenue_range.is_some());
        let (sheet_idx, addr) = revenue_range.unwrap();
        assert_eq!(sheet_idx, 0);
        assert_eq!(addr.to_string(), "A1");

        let tax_rate_range = workbook2.get_named_range("tax_rate");
        assert!(tax_rate_range.is_some());
        let (sheet_idx, addr) = tax_rate_range.unwrap();
        assert_eq!(sheet_idx, 0);
        assert_eq!(addr.to_string(), "B1");
    }

    #[test]
    fn test_column_widths_and_row_heights_serialization() {
        let mut workbook = Workbook::new("Test");

        // Set custom column widths and row heights
        let sheet = workbook.active_sheet_mut();
        sheet.set_column_width("A".to_string(), 150.0);
        sheet.set_column_width("B".to_string(), 200.0);
        sheet.set_column_width("C".to_string(), 100.0);
        sheet.set_row_height(1, 30.0);
        sheet.set_row_height(2, 45.0);
        sheet.set_row_height(10, 60.0);

        // Add some cells to verify everything works together
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(100.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // Serialize
        let file = WorkbookFile::from_workbook(&workbook);
        let json = file.to_json().unwrap();

        // Verify JSON contains dimensions
        assert!(json.contains("column_widths"));
        assert!(json.contains("row_heights"));

        // Deserialize
        let file2 = WorkbookFile::from_json(&json).unwrap();
        let workbook2 = file2.to_workbook().unwrap();

        // Verify column widths are restored
        let sheet2 = workbook2.active_sheet();
        assert_eq!(sheet2.get_column_width("A"), Some(150.0));
        assert_eq!(sheet2.get_column_width("B"), Some(200.0));
        assert_eq!(sheet2.get_column_width("C"), Some(100.0));
        assert_eq!(sheet2.get_column_width("D"), None); // Not set

        // Verify row heights are restored
        assert_eq!(sheet2.get_row_height(1), Some(30.0));
        assert_eq!(sheet2.get_row_height(2), Some(45.0));
        assert_eq!(sheet2.get_row_height(10), Some(60.0));
        assert_eq!(sheet2.get_row_height(5), None); // Not set

        // Verify cell data still works
        let cell = sheet2.get(&CellAddr::new("A", 1)).unwrap();
        assert_eq!(cell.as_number(), Some(100.0));
        assert_eq!(cell.storage_unit().canonical(), "m");
    }
}
