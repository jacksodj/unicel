// Error handling tests for Unicel
//
// These tests verify that the application handles error conditions correctly:
// - Invalid inputs are rejected with appropriate errors
// - Edge cases are handled gracefully
// - Error messages are informative
// - System remains in valid state after errors

use unicel_lib::core::cell::Cell;
use unicel_lib::core::table::{CellAddr, Sheet};
use unicel_lib::core::units::{BaseDimension, Unit};
use unicel_lib::core::workbook::Workbook;
use unicel_lib::formats::json::WorkbookFile;
use std::fs;
use std::path::PathBuf;

/// Helper to create a temporary test file path
fn temp_test_path(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("unicel_error_test_{}.usheet", name));
    path
}

/// Clean up test file if it exists
fn cleanup_test_file(path: &PathBuf) {
    let _ = fs::remove_file(path);
}

// ============================================================================
// Cell Address Errors
// ============================================================================

#[test]
fn test_invalid_cell_address_parsing() {
    // Invalid column (empty)
    assert!(CellAddr::from_string("").is_err());

    // Invalid column (numbers only)
    assert!(CellAddr::from_string("123").is_err());

    // Invalid format (no row)
    assert!(CellAddr::from_string("ABC").is_err());

    // Note: A0 might be valid in this implementation (0-indexed rows)
}

#[test]
fn test_invalid_cell_operations() {
    let mut sheet = Sheet::new();

    // Setting a cell should work
    let addr = CellAddr::new("A", 1);
    let result = sheet.set(addr.clone(), Cell::new(10.0, Unit::dimensionless()));
    assert!(result.is_ok());

    // Getting a non-existent cell returns None (not an error)
    let missing = sheet.get(&CellAddr::new("Z", 99));
    assert!(missing.is_none());
}

// ============================================================================
// Sheet Errors
// ============================================================================

#[test]
fn test_sheet_name_validation() {
    let mut sheet = Sheet::new();

    // Empty names are allowed (system will provide default)
    sheet.set_name("");
    assert_eq!(sheet.name(), "");

    // Very long names are allowed
    let long_name = "A".repeat(1000);
    sheet.set_name(&long_name);
    assert_eq!(sheet.name(), long_name);
}

#[test]
fn test_circular_reference_detection() {
    let mut sheet = Sheet::new();

    // Create circular reference: A1 = A2, A2 = A1
    sheet.set(
        CellAddr::new("A", 1),
        Cell::with_formula("=A2")
    ).unwrap();

    // Setting A2 to reference A1 should fail (circular reference detected)
    let result = sheet.set(
        CellAddr::new("A", 2),
        Cell::with_formula("=A1")
    );

    // Should detect circular reference
    assert!(result.is_err(), "Circular reference should be detected during set()");
}

#[test]
fn test_self_reference() {
    let mut sheet = Sheet::new();

    // Cell references itself - should fail during set()
    let result = sheet.set(
        CellAddr::new("A", 1),
        Cell::with_formula("=A1 + 1")
    );

    // Should detect self-reference as circular dependency
    assert!(result.is_err(), "Self-reference should be detected during set()");
}

// ============================================================================
// Workbook Errors
// ============================================================================

#[test]
fn test_cannot_remove_last_sheet() {
    let mut workbook = Workbook::new("Test");

    // Workbook starts with one sheet
    assert_eq!(workbook.sheet_count(), 1);

    // Trying to remove the last sheet should fail
    let result = workbook.remove_sheet(0);
    assert!(result.is_err());

    // Workbook should still have the sheet
    assert_eq!(workbook.sheet_count(), 1);
}

#[test]
fn test_invalid_sheet_index() {
    let mut workbook = Workbook::new("Test");

    // Getting invalid index returns None
    assert!(workbook.get_sheet(999).is_none());
    assert!(workbook.get_sheet_mut(999).is_none());

    // Setting active sheet to invalid index should fail
    let result = workbook.set_active_sheet(999);
    assert!(result.is_err());
}

#[test]
fn test_duplicate_sheet_names() {
    let mut workbook = Workbook::new("Test");

    // Add sheet with duplicate name
    workbook.add_sheet_with_name("Sheet1");

    // System should handle this (either allow duplicates or auto-rename)
    assert_eq!(workbook.sheet_count(), 2);

    // Both sheets should exist
    assert!(workbook.get_sheet(0).is_some());
    assert!(workbook.get_sheet(1).is_some());
}

#[test]
fn test_rename_sheet_validation() {
    let mut workbook = Workbook::new("Test");

    // Rename to empty string (should be allowed)
    let result = workbook.rename_sheet(0, "");
    assert!(result.is_ok());

    // Rename to very long name
    let long_name = "A".repeat(1000);
    let result = workbook.rename_sheet(0, &long_name);
    assert!(result.is_ok());

    // Rename invalid index
    let result = workbook.rename_sheet(999, "Invalid");
    assert!(result.is_err());
}

// ============================================================================
// Formula Errors
// ============================================================================

#[test]
fn test_invalid_formula_syntax() {
    let mut sheet = Sheet::new();

    // Invalid formulas may be rejected during set() or evaluation
    let formulas = vec![
        "=",           // Empty expression
        "=+",          // Invalid operator
        "=A1 +",       // Incomplete expression
        "=A1 + + B1",  // Double operator
        "=(A1",        // Unclosed parenthesis
        "=A1)",        // Unmatched parenthesis
    ];

    for (i, formula) in formulas.iter().enumerate() {
        let addr = CellAddr::new("A", i + 1);
        let result = sheet.set(addr.clone(), Cell::with_formula(*formula));

        // System may reject invalid formulas during set()
        if result.is_err() {
            // Validation happened at set time - this is good!
            continue;
        }

        // If set succeeded, cell should show error after evaluation
        sheet.recalculate(&[addr.clone()]).ok();

        let cell = sheet.get(&addr).unwrap();
        assert!(cell.is_error() || cell.is_empty(),
                "Cell should have error for formula: {}", formula);
    }
}

#[test]
fn test_division_by_zero() {
    let mut sheet = Sheet::new();

    sheet.set(CellAddr::new("A", 1), Cell::new(10.0, Unit::dimensionless())).unwrap();
    sheet.set(CellAddr::new("A", 2), Cell::new(0.0, Unit::dimensionless())).unwrap();
    sheet.set(CellAddr::new("A", 3), Cell::with_formula("=A1 / A2")).unwrap();

    sheet.recalculate(&[CellAddr::new("A", 1), CellAddr::new("A", 2)]).ok();

    let result = sheet.get(&CellAddr::new("A", 3)).unwrap();
    assert!(result.is_error(), "Division by zero should produce error");
}

#[test]
fn test_reference_to_nonexistent_cell() {
    let mut sheet = Sheet::new();

    // Formula references cell that doesn't exist
    sheet.set(
        CellAddr::new("A", 1),
        Cell::with_formula("=Z999")
    ).unwrap();

    sheet.recalculate(&[]).ok();

    // System handles this gracefully - cell exists but may be empty or have specific value
    let cell = sheet.get(&CellAddr::new("A", 1)).unwrap();

    // The formula should evaluate (not crash), result depends on implementation:
    // - Might be empty (unevaluated)
    // - Might be error
    // - Might be 0 (treating missing as 0)
    // Just verify the system doesn't panic
    assert!(cell.is_formula());
}

#[test]
fn test_incompatible_unit_operations() {
    let mut sheet = Sheet::new();

    // Add length and mass (incompatible for addition)
    sheet.set(
        CellAddr::new("A", 1),
        Cell::new(10.0, Unit::simple("m", BaseDimension::Length))
    ).unwrap();

    sheet.set(
        CellAddr::new("A", 2),
        Cell::new(5.0, Unit::simple("kg", BaseDimension::Mass))
    ).unwrap();

    sheet.set(
        CellAddr::new("A", 3),
        Cell::with_formula("=A1 + A2")
    ).unwrap();

    sheet.recalculate(&[CellAddr::new("A", 1), CellAddr::new("A", 2)]).ok();

    let result = sheet.get(&CellAddr::new("A", 3)).unwrap();

    // Should either error or warn
    assert!(result.is_error() || result.has_warning(),
            "Incompatible units should produce error or warning");
}

// ============================================================================
// File Format Errors
// ============================================================================

#[test]
fn test_invalid_json_format() {
    let path = temp_test_path("invalid_json");

    // Write invalid JSON
    fs::write(&path, "{ invalid json }").unwrap();

    // Loading should fail
    let result = WorkbookFile::load_from_file(&path);
    assert!(result.is_err());

    cleanup_test_file(&path);
}

#[test]
fn test_incompatible_version() {
    let path = temp_test_path("wrong_version");

    // Create workbook with wrong version
    let json = r#"{
        "version": "99.99",
        "metadata": {
            "created_at": "2025-01-01T00:00:00Z",
            "modified_at": "2025-01-01T00:00:00Z",
            "app_version": "0.1.0"
        },
        "workbook": {
            "name": "Test",
            "settings": {
                "display_preference": "AsEntered",
                "auto_recalculate": true,
                "show_warnings": true
            },
            "sheets": [],
            "active_sheet": 0
        }
    }"#;

    fs::write(&path, json).unwrap();

    // Loading should fail due to version mismatch
    let result = WorkbookFile::load_from_file(&path);
    assert!(result.is_err());

    cleanup_test_file(&path);
}

#[test]
fn test_corrupted_workbook_file() {
    let path = temp_test_path("corrupted");

    // Write partial/corrupted JSON
    fs::write(&path, r#"{"version": "1.0", "metadata": {"#).unwrap();

    // Loading should fail gracefully
    let result = WorkbookFile::load_from_file(&path);
    assert!(result.is_err());

    cleanup_test_file(&path);
}

#[test]
fn test_missing_file() {
    let path = temp_test_path("nonexistent");

    // Ensure file doesn't exist
    cleanup_test_file(&path);

    // Loading should fail with IO error
    let result = WorkbookFile::load_from_file(&path);
    assert!(result.is_err());
}

#[test]
fn test_save_to_invalid_path() {
    let workbook = Workbook::new("Test");
    let file = WorkbookFile::from_workbook(&workbook);

    // Try to save to invalid path (directory that doesn't exist)
    let path = PathBuf::from("/nonexistent/directory/that/doesnt/exist/file.usheet");
    let result = file.save_to_file(&path);
    assert!(result.is_err());
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_empty_sheet_operations() {
    let sheet = Sheet::new();

    // Operations on empty sheet should work
    assert_eq!(sheet.cell_addresses().len(), 0);
    assert!(sheet.get(&CellAddr::new("A", 1)).is_none());
}

#[test]
fn test_very_large_numbers() {
    let mut sheet = Sheet::new();

    // Very large numbers
    sheet.set(
        CellAddr::new("A", 1),
        Cell::new(1e308, Unit::dimensionless())
    ).unwrap();

    sheet.set(
        CellAddr::new("A", 2),
        Cell::with_formula("=A1 * 2")
    ).unwrap();

    sheet.recalculate(&[CellAddr::new("A", 1)]).ok();

    let result = sheet.get(&CellAddr::new("A", 2)).unwrap();

    // Should handle infinity or very large number
    if let Some(val) = result.as_number() {
        assert!(val.is_infinite() || val > 1e308);
    }
}

#[test]
fn test_very_small_numbers() {
    let mut sheet = Sheet::new();

    // Very small positive number
    sheet.set(
        CellAddr::new("A", 1),
        Cell::new(1e-308, Unit::dimensionless())
    ).unwrap();

    sheet.set(
        CellAddr::new("A", 2),
        Cell::with_formula("=A1 / 2")
    ).unwrap();

    sheet.recalculate(&[CellAddr::new("A", 1)]).ok();

    let result = sheet.get(&CellAddr::new("A", 2)).unwrap();

    // Should handle underflow gracefully (might become 0)
    assert!(result.as_number().is_some());
}

#[test]
fn test_nan_handling() {
    let mut sheet = Sheet::new();

    // Create NaN through 0/0
    sheet.set(CellAddr::new("A", 1), Cell::new(0.0, Unit::dimensionless())).unwrap();
    sheet.set(CellAddr::new("A", 2), Cell::new(0.0, Unit::dimensionless())).unwrap();
    sheet.set(CellAddr::new("A", 3), Cell::with_formula("=A1 / A2")).unwrap();

    sheet.recalculate(&[CellAddr::new("A", 1), CellAddr::new("A", 2)]).ok();

    let result = sheet.get(&CellAddr::new("A", 3)).unwrap();

    // Should handle NaN as error
    assert!(result.is_error());
}

#[test]
fn test_negative_numbers_in_formulas() {
    let mut sheet = Sheet::new();

    sheet.set(CellAddr::new("A", 1), Cell::new(-10.0, Unit::dimensionless())).unwrap();
    sheet.set(CellAddr::new("A", 2), Cell::new(-5.0, Unit::dimensionless())).unwrap();
    sheet.set(CellAddr::new("A", 3), Cell::with_formula("=A1 + A2")).unwrap();

    sheet.recalculate(&[CellAddr::new("A", 1), CellAddr::new("A", 2)]).unwrap();

    let result = sheet.get(&CellAddr::new("A", 3)).unwrap();
    assert_eq!(result.as_number(), Some(-15.0));
}

#[test]
fn test_empty_workbook_name() {
    let workbook = Workbook::new("");
    assert_eq!(workbook.name(), "");

    // Should be able to save/load
    let file = WorkbookFile::from_workbook(&workbook);
    let path = temp_test_path("empty_name");

    file.save_to_file(&path).unwrap();
    let loaded = WorkbookFile::load_from_file(&path).unwrap();
    let loaded_wb = loaded.to_workbook().unwrap();

    assert_eq!(loaded_wb.name(), "");

    cleanup_test_file(&path);
}

#[test]
fn test_unicode_in_names() {
    let mut workbook = Workbook::new("Test 测试 🎉");
    assert!(workbook.name().contains("测试"));
    assert!(workbook.name().contains("🎉"));

    // Unicode in sheet names
    workbook.add_sheet_with_name("Лист тест");
    let sheet = workbook.get_sheet(1).unwrap();
    assert_eq!(sheet.name(), "Лист тест");

    // Should serialize/deserialize correctly
    let file = WorkbookFile::from_workbook(&workbook);
    let path = temp_test_path("unicode");

    file.save_to_file(&path).unwrap();
    let loaded = WorkbookFile::load_from_file(&path).unwrap();
    let loaded_wb = loaded.to_workbook().unwrap();

    assert_eq!(loaded_wb.name(), "Test 测试 🎉");
    assert_eq!(loaded_wb.get_sheet(1).unwrap().name(), "Лист тест");

    cleanup_test_file(&path);
}
