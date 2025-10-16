// Integration tests for full workbook workflow
//
// These tests verify that multiple components work together correctly:
// - Workbook creation and management
// - Cell operations with formulas
// - Formula evaluation and dependencies
// - File serialization/deserialization
// - Unit conversions

use std::fs;
use std::path::PathBuf;
use unicel_lib::core::cell::Cell;
use unicel_lib::core::table::CellAddr;
use unicel_lib::core::units::{BaseDimension, Unit};
use unicel_lib::core::workbook::{DisplayPreference, Workbook};
use unicel_lib::formats::json::WorkbookFile;

/// Helper to create a temporary test file path
fn temp_test_path(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("unicel_test_{}.usheet", name));
    path
}

/// Clean up test file if it exists
fn cleanup_test_file(path: &PathBuf) {
    let _ = fs::remove_file(path);
}

#[test]
fn test_full_workbook_lifecycle() {
    // Test: Create -> Populate -> Save -> Load -> Verify

    // Create workbook with data
    let mut workbook = Workbook::new("Integration Test");

    // Add some cells with values
    let sheet = workbook.active_sheet_mut();
    sheet
        .set(
            CellAddr::new("A", 1),
            Cell::new(100.0, Unit::simple("m", BaseDimension::Length)),
        )
        .unwrap();

    sheet
        .set(
            CellAddr::new("A", 2),
            Cell::new(50.0, Unit::simple("m", BaseDimension::Length)),
        )
        .unwrap();

    // Add a formula that references other cells
    sheet
        .set(CellAddr::new("A", 3), Cell::with_formula("=A1 + A2"))
        .unwrap();

    // Add a text cell
    sheet
        .set(CellAddr::new("B", 1), Cell::with_text("Total Length"))
        .unwrap();

    // Evaluate formulas (recalculate from changed cells)
    workbook
        .active_sheet_mut()
        .recalculate(&[CellAddr::new("A", 1), CellAddr::new("A", 2)])
        .unwrap();

    // Verify formula result
    let result_cell = workbook.active_sheet().get(&CellAddr::new("A", 3)).unwrap();
    assert_eq!(result_cell.as_number(), Some(150.0));

    // Save to file
    let path = temp_test_path("lifecycle");
    let file = WorkbookFile::from_workbook(&workbook);
    file.save_to_file(&path).unwrap();

    // Load from file
    let loaded_file = WorkbookFile::load_from_file(&path).unwrap();
    let loaded_workbook = loaded_file.to_workbook().unwrap();

    // Verify loaded workbook
    assert_eq!(loaded_workbook.name(), "Integration Test");
    assert_eq!(loaded_workbook.sheet_count(), 1);

    // Verify cells
    let loaded_sheet = loaded_workbook.active_sheet();
    let a1 = loaded_sheet.get(&CellAddr::new("A", 1)).unwrap();
    assert_eq!(a1.as_number(), Some(100.0));
    assert_eq!(a1.storage_unit().canonical(), "m");

    let a3 = loaded_sheet.get(&CellAddr::new("A", 3)).unwrap();
    assert!(a3.is_formula());
    assert_eq!(a3.formula(), Some("=A1 + A2"));

    let b1 = loaded_sheet.get(&CellAddr::new("B", 1)).unwrap();
    assert!(b1.is_text());
    assert_eq!(b1.as_text(), Some("Total Length"));

    cleanup_test_file(&path);
}

#[test]
fn test_multi_sheet_workflow() {
    // Test: Create multiple sheets, add data, verify isolation

    let mut workbook = Workbook::new("Multi-Sheet Test");

    // Add second sheet
    let sheet2_idx = workbook.add_sheet_with_name("Sheet2");

    // Add data to first sheet
    workbook
        .get_sheet_mut(0)
        .unwrap()
        .set(
            CellAddr::new("A", 1),
            Cell::new(100.0, Unit::dimensionless()),
        )
        .unwrap();

    // Add data to second sheet
    workbook
        .get_sheet_mut(sheet2_idx)
        .unwrap()
        .set(
            CellAddr::new("A", 1),
            Cell::new(200.0, Unit::dimensionless()),
        )
        .unwrap();

    // Verify isolation
    let sheet1_val = workbook
        .get_sheet(0)
        .unwrap()
        .get(&CellAddr::new("A", 1))
        .unwrap()
        .as_number()
        .unwrap();
    let sheet2_val = workbook
        .get_sheet(sheet2_idx)
        .unwrap()
        .get(&CellAddr::new("A", 1))
        .unwrap()
        .as_number()
        .unwrap();

    assert_eq!(sheet1_val, 100.0);
    assert_eq!(sheet2_val, 200.0);

    // Save and load to verify persistence
    let path = temp_test_path("multisheet");
    let file = WorkbookFile::from_workbook(&workbook);
    file.save_to_file(&path).unwrap();

    let loaded_file = WorkbookFile::load_from_file(&path).unwrap();
    let loaded_workbook = loaded_file.to_workbook().unwrap();

    assert_eq!(loaded_workbook.sheet_count(), 2);
    assert_eq!(loaded_workbook.get_sheet(0).unwrap().name(), "Sheet1");
    assert_eq!(loaded_workbook.get_sheet(1).unwrap().name(), "Sheet2");

    cleanup_test_file(&path);
}

#[test]
fn test_formula_dependency_chain() {
    // Test: A1 = 10, A2 = 20, A3 = A1 + A2 (formula), verify evaluation

    let mut workbook = Workbook::new("Dependency Test");
    let sheet = workbook.active_sheet_mut();

    // Base values
    sheet
        .set(
            CellAddr::new("A", 1),
            Cell::new(10.0, Unit::dimensionless()),
        )
        .unwrap();

    sheet
        .set(
            CellAddr::new("A", 2),
            Cell::new(20.0, Unit::dimensionless()),
        )
        .unwrap();

    // Formula depends on A1 and A2
    sheet
        .set(CellAddr::new("A", 3), Cell::with_formula("=A1 + A2"))
        .unwrap();

    // Evaluate
    workbook
        .active_sheet_mut()
        .recalculate(&[CellAddr::new("A", 1), CellAddr::new("A", 2)])
        .unwrap();

    // Verify results
    let a3 = workbook.active_sheet().get(&CellAddr::new("A", 3)).unwrap();
    assert_eq!(a3.as_number(), Some(30.0), "A3 should be 30");

    // Update A1 and verify formula updates
    workbook
        .active_sheet_mut()
        .set(
            CellAddr::new("A", 1),
            Cell::new(15.0, Unit::dimensionless()),
        )
        .unwrap();

    workbook
        .active_sheet_mut()
        .recalculate(&[CellAddr::new("A", 1)])
        .unwrap();

    let a3 = workbook.active_sheet().get(&CellAddr::new("A", 3)).unwrap();
    assert_eq!(
        a3.as_number(),
        Some(35.0),
        "A3 should be 35 after A1 update"
    );
}

#[test]
fn test_unit_conversion_workflow() {
    // Test: Enter values in different units, verify storage and display

    let mut workbook = Workbook::new("Unit Test");
    let sheet = workbook.active_sheet_mut();

    // Add values with different units
    sheet
        .set(
            CellAddr::new("A", 1),
            Cell::new(100.0, Unit::simple("m", BaseDimension::Length)),
        )
        .unwrap();

    sheet
        .set(
            CellAddr::new("A", 2),
            Cell::new(50.0, Unit::simple("cm", BaseDimension::Length)),
        )
        .unwrap();

    // Add formula combining different units (should warn but compute)
    sheet
        .set(CellAddr::new("A", 3), Cell::with_formula("=A1 + A2"))
        .unwrap();

    workbook
        .active_sheet_mut()
        .recalculate(&[CellAddr::new("A", 1), CellAddr::new("A", 2)])
        .unwrap();

    // Verify formula computed despite unit difference
    let result = workbook.active_sheet().get(&CellAddr::new("A", 3)).unwrap();
    assert!(result.as_number().is_some());

    // Test display preference toggle
    workbook.set_display_preference(DisplayPreference::Metric);
    assert_eq!(
        workbook.settings().display_preference,
        DisplayPreference::Metric
    );

    workbook.set_display_preference(DisplayPreference::Imperial);
    assert_eq!(
        workbook.settings().display_preference,
        DisplayPreference::Imperial
    );

    // Verify persistence of display preference
    let path = temp_test_path("units");
    let file = WorkbookFile::from_workbook(&workbook);
    file.save_to_file(&path).unwrap();

    let loaded_file = WorkbookFile::load_from_file(&path).unwrap();
    let loaded_workbook = loaded_file.to_workbook().unwrap();

    assert_eq!(
        loaded_workbook.settings().display_preference,
        DisplayPreference::Imperial
    );

    cleanup_test_file(&path);
}

#[test]
fn test_error_propagation() {
    // Test: Division by zero and error propagation through formulas

    let mut workbook = Workbook::new("Error Test");

    // Create division by zero
    {
        let sheet = workbook.active_sheet_mut();
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(10.0, Unit::dimensionless()),
            )
            .unwrap();
        sheet
            .set(CellAddr::new("A", 2), Cell::new(0.0, Unit::dimensionless()))
            .unwrap();
        sheet
            .set(CellAddr::new("A", 3), Cell::with_formula("=A1 / A2"))
            .unwrap();
    }

    workbook
        .active_sheet_mut()
        .recalculate(&[CellAddr::new("A", 1), CellAddr::new("A", 2)])
        .unwrap();

    // Verify error in result
    let result = workbook.active_sheet().get(&CellAddr::new("A", 3)).unwrap();
    assert!(result.is_error());

    // Add formula that depends on error cell
    {
        let sheet = workbook.active_sheet_mut();
        sheet
            .set(CellAddr::new("A", 4), Cell::with_formula("=A3 + 5"))
            .unwrap();
    }

    workbook
        .active_sheet_mut()
        .recalculate(&[CellAddr::new("A", 3)])
        .unwrap();

    // Verify error propagates
    let result2 = workbook.active_sheet().get(&CellAddr::new("A", 4)).unwrap();
    assert!(result2.is_error());
}

#[test]
fn test_mixed_cell_types() {
    // Test: Workbook with numbers, text, formulas, and empty cells

    let mut workbook = Workbook::new("Mixed Types");
    let sheet = workbook.active_sheet_mut();

    // Add various cell types
    sheet
        .set(
            CellAddr::new("A", 1),
            Cell::new(42.0, Unit::dimensionless()),
        )
        .unwrap();
    sheet
        .set(CellAddr::new("A", 2), Cell::with_text("Hello"))
        .unwrap();
    sheet
        .set(CellAddr::new("A", 3), Cell::with_formula("=A1 * 2"))
        .unwrap();
    sheet.set(CellAddr::new("A", 4), Cell::empty()).unwrap();

    workbook
        .active_sheet_mut()
        .recalculate(&[CellAddr::new("A", 1)])
        .unwrap();

    // Verify types
    assert!(workbook
        .active_sheet()
        .get(&CellAddr::new("A", 1))
        .unwrap()
        .as_number()
        .is_some());
    assert!(workbook
        .active_sheet()
        .get(&CellAddr::new("A", 2))
        .unwrap()
        .is_text());
    assert!(workbook
        .active_sheet()
        .get(&CellAddr::new("A", 3))
        .unwrap()
        .is_formula());
    assert!(workbook
        .active_sheet()
        .get(&CellAddr::new("A", 4))
        .unwrap()
        .is_empty());

    // Save and load
    let path = temp_test_path("mixed_types");
    let file = WorkbookFile::from_workbook(&workbook);
    file.save_to_file(&path).unwrap();

    let loaded_file = WorkbookFile::load_from_file(&path).unwrap();
    let loaded_workbook = loaded_file.to_workbook().unwrap();

    // Verify types persisted
    let loaded_sheet = loaded_workbook.active_sheet();
    assert!(loaded_sheet
        .get(&CellAddr::new("A", 1))
        .unwrap()
        .as_number()
        .is_some());
    assert!(loaded_sheet.get(&CellAddr::new("A", 2)).unwrap().is_text());
    assert!(loaded_sheet
        .get(&CellAddr::new("A", 3))
        .unwrap()
        .is_formula());
    assert!(loaded_sheet.get(&CellAddr::new("A", 4)).unwrap().is_empty());

    cleanup_test_file(&path);
}

#[test]
fn test_compound_unit_operations() {
    // Test: Multiplication and division creating compound units

    let mut workbook = Workbook::new("Compound Units");
    let sheet = workbook.active_sheet_mut();

    // Distance and time
    sheet
        .set(
            CellAddr::new("A", 1),
            Cell::new(100.0, Unit::simple("m", BaseDimension::Length)),
        )
        .unwrap();

    sheet
        .set(
            CellAddr::new("A", 2),
            Cell::new(10.0, Unit::simple("s", BaseDimension::Time)),
        )
        .unwrap();

    // Velocity = distance / time
    sheet
        .set(CellAddr::new("A", 3), Cell::with_formula("=A1 / A2"))
        .unwrap();

    workbook
        .active_sheet_mut()
        .recalculate(&[CellAddr::new("A", 1), CellAddr::new("A", 2)])
        .unwrap();

    let velocity = workbook.active_sheet().get(&CellAddr::new("A", 3)).unwrap();
    assert_eq!(velocity.as_number(), Some(10.0));

    // Verify compound unit (m/s)
    let unit_str = velocity.storage_unit().to_string();
    assert!(unit_str.contains("m") && unit_str.contains("s"));
}

#[test]
fn test_dirty_flag_tracking() {
    // Test: Dirty flag management through operations

    let mut workbook = Workbook::new("Dirty Test");
    workbook.mark_clean();
    assert!(!workbook.is_dirty());

    // Modify workbook
    workbook
        .active_sheet_mut()
        .set(
            CellAddr::new("A", 1),
            Cell::new(42.0, Unit::dimensionless()),
        )
        .unwrap();
    workbook.mark_dirty(); // Manually mark as dirty (application layer responsibility)

    assert!(workbook.is_dirty());

    // Save and verify clean
    let path = temp_test_path("dirty");
    let file = WorkbookFile::from_workbook(&workbook);
    file.save_to_file(&path).unwrap();

    // Load marks as clean
    let loaded_file = WorkbookFile::load_from_file(&path).unwrap();
    let loaded_workbook = loaded_file.to_workbook().unwrap();
    assert!(!loaded_workbook.is_dirty());

    cleanup_test_file(&path);
}

#[test]
fn test_large_workbook() {
    // Test: Performance with larger dataset

    let mut workbook = Workbook::new("Large Test");
    let sheet = workbook.active_sheet_mut();

    // Add 100 cells with values
    for i in 1..=100 {
        sheet
            .set(
                CellAddr::new("A", i),
                Cell::new(i as f64, Unit::dimensionless()),
            )
            .unwrap();
    }

    // Add formula at A101 that sums A1:A100
    sheet
        .set(CellAddr::new("A", 101), Cell::with_formula("=SUM(A1:A100)"))
        .unwrap();

    // Recalculate all cells (pass all addresses)
    let changed: Vec<CellAddr> = (1..=100).map(|i| CellAddr::new("A", i)).collect();
    workbook.active_sheet_mut().recalculate(&changed).unwrap();

    let sum = workbook
        .active_sheet()
        .get(&CellAddr::new("A", 101))
        .unwrap();
    assert_eq!(sum.as_number(), Some(5050.0)); // Sum of 1..100

    // Save and load to verify
    let path = temp_test_path("large");
    let file = WorkbookFile::from_workbook(&workbook);
    file.save_to_file(&path).unwrap();

    let loaded_file = WorkbookFile::load_from_file(&path).unwrap();
    let loaded_workbook = loaded_file.to_workbook().unwrap();

    assert_eq!(loaded_workbook.active_sheet().cell_addresses().len(), 101);

    cleanup_test_file(&path);
}
