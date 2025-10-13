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

        Self {
            name: workbook.name().to_string(),
            settings: WorkbookSettingsData::from_settings(workbook.settings()),
            sheets,
            active_sheet: workbook.active_sheet_index(),
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

            // Populate sheet with cells
            if let Some(wb_sheet) = workbook.get_sheet_mut(idx) {
                for (addr_str, cell_data) in &sheet_data.cells {
                    if let Ok(addr) = CellAddr::from_string(addr_str) {
                        if let Ok(cell) = cell_data.to_cell() {
                            wb_sheet.set(addr, cell).ok();
                        }
                    }
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
                sheet.get(&addr).map(|cell| {
                    (addr.to_string(), CellData::from_cell(cell))
                })
            })
            .collect();

        Self {
            name: sheet.name().to_string(),
            cells,
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
            CellValue::Error(e) => Self::Error {
                message: e.clone(),
            },
        }
    }
}

/// Parse a unit from its canonical form
/// This is a simplified parser for the MLP - in full version would use UnitLibrary
fn parse_unit_from_canonical(canonical: &str) -> Unit {
    if canonical.is_empty() {
        return Unit::dimensionless();
    }

    // For simple units, try to infer the dimension
    let dimension = match canonical {
        "m" | "cm" | "mm" | "km" | "in" | "ft" | "yd" | "mi" => BaseDimension::Length,
        "g" | "kg" | "mg" | "oz" | "lb" => BaseDimension::Mass,
        "s" | "min" | "hr" | "h" | "day" => BaseDimension::Time,
        "C" | "F" | "K" => BaseDimension::Temperature,
        "USD" | "EUR" | "GBP" => BaseDimension::Currency,
        _ => BaseDimension::Custom(canonical.to_string()),
    };

    Unit::simple(canonical, dimension)
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
        let cell = workbook2.active_sheet().get(&CellAddr::new("A", 2)).unwrap();
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
}
