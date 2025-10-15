// Workbook management

use crate::core::table::{CellAddr, Sheet};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkbookError {
    #[error("Sheet not found: {0}")]
    SheetNotFound(String),

    #[error("Sheet name already exists: {0}")]
    SheetNameExists(String),

    #[error("Cannot remove the last sheet")]
    CannotRemoveLastSheet,

    #[error("Invalid sheet index: {0}")]
    InvalidSheetIndex(usize),

    #[error("Named range already exists: {0}")]
    NamedRangeExists(String),

    #[error("Named range not found: {0}")]
    NamedRangeNotFound(String),

    #[error("Invalid name: {0}")]
    InvalidName(String),
}

/// Display preference for units
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisplayPreference {
    /// Display units as entered (no conversion)
    AsEntered,
    /// Prefer metric units (m, kg, etc.)
    Metric,
    /// Prefer imperial units (ft, lb, etc.)
    Imperial,
}

impl Default for DisplayPreference {
    fn default() -> Self {
        Self::AsEntered
    }
}

/// Workbook settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbookSettings {
    /// Display preference for units
    pub display_preference: DisplayPreference,

    /// Auto-recalculate on cell change
    pub auto_recalculate: bool,

    /// Show warnings for unit mismatches
    pub show_warnings: bool,
}

impl Default for WorkbookSettings {
    fn default() -> Self {
        Self {
            display_preference: DisplayPreference::AsEntered,
            auto_recalculate: true,
            show_warnings: true,
        }
    }
}

/// A workbook containing multiple sheets
#[derive(Debug)]
pub struct Workbook {
    /// Workbook name/title
    name: String,

    /// Sheets in the workbook
    sheets: Vec<Sheet>,

    /// Currently active sheet index
    active_sheet: usize,

    /// Workbook settings
    settings: WorkbookSettings,

    /// Named cell references (name -> (sheet_index, cell_address))
    named_ranges: HashMap<String, (usize, CellAddr)>,

    /// Dirty flag (has unsaved changes)
    dirty: bool,
}

impl Workbook {
    /// Create a new workbook with a single sheet
    pub fn new(name: impl Into<String>) -> Self {
        let mut workbook = Self {
            name: name.into(),
            sheets: vec![Sheet::with_name("Sheet1")],
            active_sheet: 0,
            settings: WorkbookSettings::default(),
            named_ranges: HashMap::new(),
            dirty: false,
        };
        workbook.mark_clean(); // New workbook starts clean
        workbook
    }

    /// Get the workbook name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the workbook name
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
        self.mark_dirty();
    }

    /// Get the number of sheets
    pub fn sheet_count(&self) -> usize {
        self.sheets.len()
    }

    /// Get a sheet by index
    pub fn get_sheet(&self, index: usize) -> Option<&Sheet> {
        self.sheets.get(index)
    }

    /// Get a mutable reference to a sheet by index
    pub fn get_sheet_mut(&mut self, index: usize) -> Option<&mut Sheet> {
        self.sheets.get_mut(index)
    }

    /// Get a sheet by name
    pub fn get_sheet_by_name(&self, name: &str) -> Option<&Sheet> {
        self.sheets.iter().find(|s| s.name() == name)
    }

    /// Get a mutable reference to a sheet by name
    pub fn get_sheet_by_name_mut(&mut self, name: &str) -> Option<&mut Sheet> {
        self.sheets.iter_mut().find(|s| s.name() == name)
    }

    /// Get the active sheet
    pub fn active_sheet(&self) -> &Sheet {
        &self.sheets[self.active_sheet]
    }

    /// Get a mutable reference to the active sheet
    pub fn active_sheet_mut(&mut self) -> &mut Sheet {
        &mut self.sheets[self.active_sheet]
    }

    /// Get the active sheet index
    pub fn active_sheet_index(&self) -> usize {
        self.active_sheet
    }

    /// Set the active sheet by index
    pub fn set_active_sheet(&mut self, index: usize) -> Result<(), WorkbookError> {
        if index >= self.sheets.len() {
            return Err(WorkbookError::InvalidSheetIndex(index));
        }
        self.active_sheet = index;
        Ok(())
    }

    /// Add a new sheet with a default name
    pub fn add_sheet(&mut self) -> usize {
        let sheet_num = self.sheets.len() + 1;
        let name = format!("Sheet{}", sheet_num);
        self.add_sheet_with_name(name)
    }

    /// Add a new sheet with a specific name
    pub fn add_sheet_with_name(&mut self, name: impl Into<String>) -> usize {
        let name = name.into();
        let sheet = Sheet::with_name(name);
        self.sheets.push(sheet);
        self.mark_dirty();
        self.sheets.len() - 1
    }

    /// Remove a sheet by index
    pub fn remove_sheet(&mut self, index: usize) -> Result<Sheet, WorkbookError> {
        if self.sheets.len() <= 1 {
            return Err(WorkbookError::CannotRemoveLastSheet);
        }

        if index >= self.sheets.len() {
            return Err(WorkbookError::InvalidSheetIndex(index));
        }

        // Adjust active sheet if needed
        if self.active_sheet == index {
            self.active_sheet = if index > 0 { index - 1 } else { 0 };
        } else if self.active_sheet > index {
            self.active_sheet -= 1;
        }

        self.mark_dirty();
        Ok(self.sheets.remove(index))
    }

    /// Rename a sheet
    pub fn rename_sheet(&mut self, index: usize, new_name: impl Into<String>) -> Result<(), WorkbookError> {
        let new_name = new_name.into();

        // Check if name already exists (excluding the sheet being renamed)
        if self.sheets.iter().enumerate().any(|(i, s)| i != index && s.name() == new_name) {
            return Err(WorkbookError::SheetNameExists(new_name));
        }

        let sheet = self.sheets.get_mut(index)
            .ok_or_else(|| WorkbookError::InvalidSheetIndex(index))?;

        sheet.set_name(new_name);
        self.mark_dirty();
        Ok(())
    }

    /// Get all sheet names
    pub fn sheet_names(&self) -> Vec<String> {
        self.sheets.iter().map(|s| s.name().to_string()).collect()
    }

    /// Get the workbook settings
    pub fn settings(&self) -> &WorkbookSettings {
        &self.settings
    }

    /// Get mutable workbook settings
    pub fn settings_mut(&mut self) -> &mut WorkbookSettings {
        self.mark_dirty();
        &mut self.settings
    }

    /// Set display preference
    pub fn set_display_preference(&mut self, pref: DisplayPreference) {
        self.settings.display_preference = pref;
        self.mark_dirty();
    }

    /// Check if the workbook has unsaved changes
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Mark the workbook as having unsaved changes
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Mark the workbook as saved (no unsaved changes)
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    /// Add or update a named range
    pub fn set_named_range(
        &mut self,
        name: impl Into<String>,
        sheet_index: usize,
        addr: CellAddr,
    ) -> Result<(), WorkbookError> {
        let name = name.into();

        // Validate name
        if !Self::is_valid_name(&name) {
            return Err(WorkbookError::InvalidName(name));
        }

        // Validate sheet index
        if sheet_index >= self.sheets.len() {
            return Err(WorkbookError::InvalidSheetIndex(sheet_index));
        }

        self.named_ranges.insert(name, (sheet_index, addr));
        self.mark_dirty();
        Ok(())
    }

    /// Get a named range
    pub fn get_named_range(&self, name: &str) -> Option<(usize, &CellAddr)> {
        self.named_ranges.get(name).map(|(idx, addr)| (*idx, addr))
    }

    /// Remove a named range
    pub fn remove_named_range(&mut self, name: &str) -> Result<(), WorkbookError> {
        if self.named_ranges.remove(name).is_some() {
            self.mark_dirty();
            Ok(())
        } else {
            Err(WorkbookError::NamedRangeNotFound(name.to_string()))
        }
    }

    /// List all named ranges
    pub fn list_named_ranges(&self) -> Vec<(String, usize, CellAddr)> {
        self.named_ranges
            .iter()
            .map(|(name, (sheet_idx, addr))| (name.clone(), *sheet_idx, addr.clone()))
            .collect()
    }

    /// Resolve all named ranges to their current values
    /// Returns a HashMap mapping name to (value, unit)
    pub fn resolve_named_ranges(&self) -> HashMap<String, (f64, crate::core::units::Unit)> {
        let mut resolved = HashMap::new();

        for (name, (sheet_idx, addr)) in &self.named_ranges {
            if let Some(sheet) = self.get_sheet(*sheet_idx) {
                if let Some(cell) = sheet.get(addr) {
                    if let Some(value) = cell.as_number() {
                        resolved.insert(name.clone(), (value, cell.storage_unit().clone()));
                    }
                }
            }
        }

        resolved
    }

    /// Check if a name is valid for a named range
    /// Must start with lowercase letter or underscore, contain only alphanumerics and underscores
    fn is_valid_name(name: &str) -> bool {
        if name.is_empty() {
            return false;
        }

        let mut chars = name.chars();
        let first = chars.next().unwrap();

        // Must start with lowercase letter or underscore
        if !first.is_ascii_lowercase() && first != '_' {
            return false;
        }

        // Rest must be alphanumeric or underscore
        chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
    }
}

impl Default for Workbook {
    fn default() -> Self {
        Self::new("Untitled")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::cell::Cell;
    use crate::core::table::CellAddr;
    use crate::core::units::{BaseDimension, Unit};

    #[test]
    fn test_workbook_creation() {
        let wb = Workbook::new("Test Workbook");
        assert_eq!(wb.name(), "Test Workbook");
        assert_eq!(wb.sheet_count(), 1);
        assert_eq!(wb.active_sheet().name(), "Sheet1");
        assert!(!wb.is_dirty());
    }

    #[test]
    fn test_add_sheets() {
        let mut wb = Workbook::new("Test");

        let idx1 = wb.add_sheet();
        assert_eq!(idx1, 1);
        assert_eq!(wb.sheet_count(), 2);

        let idx2 = wb.add_sheet_with_name("Custom");
        assert_eq!(idx2, 2);
        assert_eq!(wb.get_sheet(2).unwrap().name(), "Custom");
        assert!(wb.is_dirty());
    }

    #[test]
    fn test_remove_sheet() {
        let mut wb = Workbook::new("Test");
        wb.add_sheet();
        wb.add_sheet();

        assert_eq!(wb.sheet_count(), 3);

        let removed = wb.remove_sheet(1).unwrap();
        assert_eq!(removed.name(), "Sheet2");
        assert_eq!(wb.sheet_count(), 2);
    }

    #[test]
    fn test_cannot_remove_last_sheet() {
        let mut wb = Workbook::new("Test");

        let result = wb.remove_sheet(0);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), WorkbookError::CannotRemoveLastSheet));
    }

    #[test]
    fn test_rename_sheet() {
        let mut wb = Workbook::new("Test");
        wb.add_sheet();

        wb.rename_sheet(0, "First Sheet").unwrap();
        assert_eq!(wb.get_sheet(0).unwrap().name(), "First Sheet");

        wb.rename_sheet(1, "Second Sheet").unwrap();
        assert_eq!(wb.get_sheet(1).unwrap().name(), "Second Sheet");
    }

    #[test]
    fn test_duplicate_sheet_name() {
        let mut wb = Workbook::new("Test");
        wb.add_sheet();

        wb.rename_sheet(0, "MySheet").unwrap();

        let result = wb.rename_sheet(1, "MySheet");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), WorkbookError::SheetNameExists(_)));
    }

    #[test]
    fn test_active_sheet() {
        let mut wb = Workbook::new("Test");
        wb.add_sheet();
        wb.add_sheet();

        assert_eq!(wb.active_sheet_index(), 0);

        wb.set_active_sheet(2).unwrap();
        assert_eq!(wb.active_sheet_index(), 2);
        assert_eq!(wb.active_sheet().name(), "Sheet3");
    }

    #[test]
    fn test_get_sheet_by_name() {
        let mut wb = Workbook::new("Test");
        wb.add_sheet_with_name("MySheet");

        let sheet = wb.get_sheet_by_name("MySheet");
        assert!(sheet.is_some());
        assert_eq!(sheet.unwrap().name(), "MySheet");

        let not_found = wb.get_sheet_by_name("NonExistent");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_workbook_settings() {
        let mut wb = Workbook::new("Test");

        assert_eq!(wb.settings().display_preference, DisplayPreference::AsEntered);

        wb.set_display_preference(DisplayPreference::Metric);
        assert_eq!(wb.settings().display_preference, DisplayPreference::Metric);
        assert!(wb.is_dirty());
    }

    #[test]
    fn test_dirty_flag() {
        let mut wb = Workbook::new("Test");
        wb.mark_clean();
        assert!(!wb.is_dirty());

        wb.add_sheet();
        assert!(wb.is_dirty());

        wb.mark_clean();
        assert!(!wb.is_dirty());

        wb.set_name("New Name");
        assert!(wb.is_dirty());
    }

    #[test]
    fn test_sheet_operations_in_workbook() {
        let mut wb = Workbook::new("Test");

        // Add a cell to the active sheet
        let addr = CellAddr::new("A", 1);
        let cell = Cell::new(42.0, Unit::simple("m", BaseDimension::Length));

        wb.active_sheet_mut().set(addr.clone(), cell).unwrap();

        // Retrieve the cell
        let retrieved = wb.active_sheet().get(&addr);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().as_number(), Some(42.0));
    }

    #[test]
    fn test_set_named_range() {
        let mut wb = Workbook::new("Test");
        let addr = CellAddr::new("A", 1);

        // Set a named range
        wb.set_named_range("revenue", 0, addr.clone()).unwrap();

        // Retrieve it
        let result = wb.get_named_range("revenue");
        assert!(result.is_some());
        let (sheet_idx, cell_addr) = result.unwrap();
        assert_eq!(sheet_idx, 0);
        assert_eq!(cell_addr, &addr);
    }

    #[test]
    fn test_named_range_validation() {
        let mut wb = Workbook::new("Test");
        let addr = CellAddr::new("A", 1);

        // Valid names
        assert!(wb.set_named_range("revenue", 0, addr.clone()).is_ok());
        assert!(wb.set_named_range("tax_rate", 0, addr.clone()).is_ok());
        assert!(wb.set_named_range("_private", 0, addr.clone()).is_ok());
        assert!(wb.set_named_range("value123", 0, addr.clone()).is_ok());

        // Invalid names (uppercase, starts with number, etc.)
        assert!(wb.set_named_range("Revenue", 0, addr.clone()).is_err()); // Starts with uppercase
        assert!(wb.set_named_range("123value", 0, addr.clone()).is_err()); // Starts with number
        assert!(wb.set_named_range("my-value", 0, addr.clone()).is_err()); // Contains hyphen
        assert!(wb.set_named_range("", 0, addr.clone()).is_err()); // Empty
    }

    #[test]
    fn test_remove_named_range() {
        let mut wb = Workbook::new("Test");
        let addr = CellAddr::new("A", 1);

        wb.set_named_range("revenue", 0, addr).unwrap();
        assert!(wb.get_named_range("revenue").is_some());

        // Remove it
        wb.remove_named_range("revenue").unwrap();
        assert!(wb.get_named_range("revenue").is_none());

        // Removing non-existent range should error
        let result = wb.remove_named_range("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_list_named_ranges() {
        let mut wb = Workbook::new("Test");

        wb.set_named_range("revenue", 0, CellAddr::new("A", 1)).unwrap();
        wb.set_named_range("cost", 0, CellAddr::new("A", 2)).unwrap();
        wb.set_named_range("profit", 0, CellAddr::new("A", 3)).unwrap();

        let ranges = wb.list_named_ranges();
        assert_eq!(ranges.len(), 3);

        // Check that all names are present
        let names: Vec<String> = ranges.iter().map(|(name, _, _)| name.clone()).collect();
        assert!(names.contains(&"revenue".to_string()));
        assert!(names.contains(&"cost".to_string()));
        assert!(names.contains(&"profit".to_string()));
    }

    #[test]
    fn test_named_range_invalid_sheet() {
        let mut wb = Workbook::new("Test");
        let addr = CellAddr::new("A", 1);

        // Try to set named range for non-existent sheet
        let result = wb.set_named_range("test", 999, addr);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), WorkbookError::InvalidSheetIndex(999)));
    }

    #[test]
    fn test_named_range_dirty_flag() {
        let mut wb = Workbook::new("Test");
        wb.mark_clean();
        assert!(!wb.is_dirty());

        let addr = CellAddr::new("A", 1);
        wb.set_named_range("revenue", 0, addr).unwrap();
        assert!(wb.is_dirty());

        wb.mark_clean();
        wb.remove_named_range("revenue").unwrap();
        assert!(wb.is_dirty());
    }

    #[test]
    fn test_named_range_overwrite() {
        let mut wb = Workbook::new("Test");

        wb.set_named_range("value", 0, CellAddr::new("A", 1)).unwrap();

        let result = wb.get_named_range("value");
        assert_eq!(result.unwrap().1, &CellAddr::new("A", 1));

        // Overwrite with new address
        wb.set_named_range("value", 0, CellAddr::new("B", 2)).unwrap();

        let result = wb.get_named_range("value");
        assert_eq!(result.unwrap().1, &CellAddr::new("B", 2));
    }
}
