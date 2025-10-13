// Cell structure for unit-aware spreadsheet
//
// Design:
// - Storage vs Display: Values are stored with their original units (storage_unit).
//   Display conversion is applied based on display_unit preference.
// - Formula Support: Cells can contain either a direct value or a formula.
// - Warning System: Incompatible operations are flagged but not blocked.

use super::units::Unit;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A spreadsheet cell with unit-aware value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cell {
    /// The value type (either computed or formula)
    value: CellValue,

    /// The unit as originally entered (storage unit)
    storage_unit: Unit,

    /// The unit to display (for Metric/Imperial toggle)
    /// If None, uses storage_unit
    display_unit: Option<Unit>,

    /// Original formula text if this is a formula cell
    formula: Option<String>,

    /// Warning message if the cell has a unit compatibility issue
    warning: Option<String>,
}

/// The value of a cell
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CellValue {
    /// Empty cell
    Empty,

    /// A numeric value (could be from direct entry or formula result)
    Number(f64),

    /// An error (e.g., division by zero, circular reference)
    Error(String),
}

impl Cell {
    /// Create an empty cell
    pub fn empty() -> Self {
        Self {
            value: CellValue::Empty,
            storage_unit: Unit::dimensionless(),
            display_unit: None,
            formula: None,
            warning: None,
        }
    }

    /// Create a cell with a direct value and unit
    pub fn new(value: f64, unit: Unit) -> Self {
        Self {
            value: CellValue::Number(value),
            storage_unit: unit,
            display_unit: None,
            formula: None,
            warning: None,
        }
    }

    /// Create a cell with a formula
    pub fn with_formula(formula: impl Into<String>) -> Self {
        Self {
            value: CellValue::Empty, // Will be computed during evaluation
            storage_unit: Unit::dimensionless(),
            display_unit: None,
            formula: Some(formula.into()),
            warning: None,
        }
    }

    /// Get the cell value
    pub fn value(&self) -> &CellValue {
        &self.value
    }

    /// Get the numeric value if this cell contains a number
    pub fn as_number(&self) -> Option<f64> {
        match self.value {
            CellValue::Number(n) => Some(n),
            _ => None,
        }
    }

    /// Get the storage unit
    pub fn storage_unit(&self) -> &Unit {
        &self.storage_unit
    }

    /// Get the display unit (falls back to storage unit if not set)
    pub fn display_unit(&self) -> &Unit {
        self.display_unit.as_ref().unwrap_or(&self.storage_unit)
    }

    /// Get the formula text if this is a formula cell
    pub fn formula(&self) -> Option<&str> {
        self.formula.as_deref()
    }

    /// Check if this cell has a formula
    pub fn is_formula(&self) -> bool {
        self.formula.is_some()
    }

    /// Check if this cell is empty
    pub fn is_empty(&self) -> bool {
        matches!(self.value, CellValue::Empty)
    }

    /// Check if this cell has an error
    pub fn is_error(&self) -> bool {
        matches!(self.value, CellValue::Error(_))
    }

    /// Get the warning message if any
    pub fn warning(&self) -> Option<&str> {
        self.warning.as_deref()
    }

    /// Check if this cell has a warning
    pub fn has_warning(&self) -> bool {
        self.warning.is_some()
    }

    /// Set the cell value (for formula evaluation results)
    pub fn set_value(&mut self, value: CellValue) {
        self.value = value;
    }

    /// Set the storage unit (when value is computed)
    pub fn set_storage_unit(&mut self, unit: Unit) {
        self.storage_unit = unit;
    }

    /// Set the display unit for Metric/Imperial toggle
    pub fn set_display_unit(&mut self, unit: Option<Unit>) {
        self.display_unit = unit;
    }

    /// Set a warning message
    pub fn set_warning(&mut self, warning: Option<String>) {
        self.warning = warning;
    }

    /// Clear the display unit (revert to storage unit)
    pub fn clear_display_unit(&mut self) {
        self.display_unit = None;
    }

    /// Get the formatted display string
    /// This shows the value in the display unit
    pub fn formatted(&self) -> String {
        match &self.value {
            CellValue::Empty => String::new(),
            CellValue::Number(n) => {
                let unit = self.display_unit();
                if unit.is_dimensionless() {
                    format!("{}", n)
                } else {
                    format!("{} {}", n, unit)
                }
            }
            CellValue::Error(e) => format!("ERROR: {}", e),
        }
    }
}

impl CellValue {
    /// Check if this is an empty value
    pub fn is_empty(&self) -> bool {
        matches!(self, CellValue::Empty)
    }

    /// Check if this is a number
    pub fn is_number(&self) -> bool {
        matches!(self, CellValue::Number(_))
    }

    /// Check if this is an error
    pub fn is_error(&self) -> bool {
        matches!(self, CellValue::Error(_))
    }

    /// Get as a number if possible
    pub fn as_number(&self) -> Option<f64> {
        match self {
            CellValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Get the error message if this is an error
    pub fn as_error(&self) -> Option<&str> {
        match self {
            CellValue::Error(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.formatted())
    }
}

impl fmt::Display for CellValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CellValue::Empty => write!(f, ""),
            CellValue::Number(n) => write!(f, "{}", n),
            CellValue::Error(e) => write!(f, "ERROR: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::units::BaseDimension;

    #[test]
    fn test_empty_cell() {
        let cell = Cell::empty();
        assert!(cell.is_empty());
        assert!(!cell.is_formula());
        assert!(!cell.is_error());
        assert!(!cell.has_warning());
        assert_eq!(cell.formatted(), "");
    }

    #[test]
    fn test_cell_with_value() {
        let unit = Unit::simple("m", BaseDimension::Length);
        let cell = Cell::new(100.0, unit);

        assert!(!cell.is_empty());
        assert_eq!(cell.as_number(), Some(100.0));
        assert_eq!(cell.storage_unit().canonical(), "m");
        assert_eq!(cell.formatted(), "100 m");
    }

    #[test]
    fn test_dimensionless_cell() {
        let cell = Cell::new(42.0, Unit::dimensionless());
        assert_eq!(cell.formatted(), "42");
    }

    #[test]
    fn test_formula_cell() {
        let cell = Cell::with_formula("=A1 + B1");

        assert!(cell.is_formula());
        assert_eq!(cell.formula(), Some("=A1 + B1"));
        assert!(cell.is_empty()); // Not evaluated yet
    }

    #[test]
    fn test_cell_with_warning() {
        let mut cell = Cell::new(100.0, Unit::simple("m", BaseDimension::Length));
        cell.set_warning(Some("Unit mismatch detected".to_string()));

        assert!(cell.has_warning());
        assert_eq!(cell.warning(), Some("Unit mismatch detected"));
    }

    #[test]
    fn test_display_unit_conversion() {
        let storage_unit = Unit::simple("m", BaseDimension::Length);
        let display_unit = Unit::simple("ft", BaseDimension::Length);

        let mut cell = Cell::new(100.0, storage_unit);

        // Initially uses storage unit
        assert_eq!(cell.display_unit().canonical(), "m");

        // Set display unit
        cell.set_display_unit(Some(display_unit));
        assert_eq!(cell.display_unit().canonical(), "ft");

        // Clear display unit (revert to storage)
        cell.clear_display_unit();
        assert_eq!(cell.display_unit().canonical(), "m");
    }

    #[test]
    fn test_cell_value_types() {
        let empty = CellValue::Empty;
        let number = CellValue::Number(42.0);
        let error = CellValue::Error("Division by zero".to_string());

        assert!(empty.is_empty());
        assert!(number.is_number());
        assert!(error.is_error());

        assert_eq!(number.as_number(), Some(42.0));
        assert_eq!(error.as_error(), Some("Division by zero"));
    }

    #[test]
    fn test_cell_modification() {
        let mut cell = Cell::empty();

        // Set value
        cell.set_value(CellValue::Number(50.0));
        assert_eq!(cell.as_number(), Some(50.0));

        // Set unit
        cell.set_storage_unit(Unit::simple("kg", BaseDimension::Mass));
        assert_eq!(cell.storage_unit().canonical(), "kg");

        // Set error
        cell.set_value(CellValue::Error("Invalid operation".to_string()));
        assert!(cell.is_error());
    }

    #[test]
    fn test_cell_display() {
        let cell = Cell::new(100.0, Unit::simple("m", BaseDimension::Length));
        assert_eq!(format!("{}", cell), "100 m");

        let error_cell = Cell {
            value: CellValue::Error("Test error".to_string()),
            storage_unit: Unit::dimensionless(),
            display_unit: None,
            formula: None,
            warning: None,
        };
        assert_eq!(format!("{}", error_cell), "ERROR: Test error");
    }
}
