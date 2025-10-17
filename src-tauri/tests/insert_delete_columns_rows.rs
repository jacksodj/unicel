// Tests for column and row insertion and deletion

use unicel_lib::core::{
    cell::Cell,
    table::{CellAddr, Sheet},
    units::{BaseDimension, Unit},
};

#[test]
fn test_insert_column_before_shifts_cells() {
    let mut sheet = Sheet::new();

    // Set up initial state: A1=100, B1=200, C1=300
    sheet
        .set(
            CellAddr::new("A", 1),
            Cell::new(100.0, Unit::dimensionless()),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("B", 1),
            Cell::new(200.0, Unit::dimensionless()),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("C", 1),
            Cell::new(300.0, Unit::dimensionless()),
        )
        .unwrap();

    // Insert column before B
    sheet.insert_column_before("B").unwrap();

    // A1 should stay at 100
    assert_eq!(
        sheet
            .get(&CellAddr::new("A", 1))
            .and_then(|c| c.as_number()),
        Some(100.0)
    );

    // B1 should be empty (newly inserted)
    assert!(sheet.get(&CellAddr::new("B", 1)).is_none());

    // Old B1 (200) should now be at C1
    assert_eq!(
        sheet
            .get(&CellAddr::new("C", 1))
            .and_then(|c| c.as_number()),
        Some(200.0)
    );

    // Old C1 (300) should now be at D1
    assert_eq!(
        sheet
            .get(&CellAddr::new("D", 1))
            .and_then(|c| c.as_number()),
        Some(300.0)
    );
}

#[test]
fn test_insert_column_after() {
    let mut sheet = Sheet::new();

    // Set up: A1=100, B1=200
    sheet
        .set(
            CellAddr::new("A", 1),
            Cell::new(100.0, Unit::dimensionless()),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("B", 1),
            Cell::new(200.0, Unit::dimensionless()),
        )
        .unwrap();

    // Insert column after A (should insert at B)
    sheet.insert_column_after("A").unwrap();

    // A1 should stay at 100
    assert_eq!(
        sheet
            .get(&CellAddr::new("A", 1))
            .and_then(|c| c.as_number()),
        Some(100.0)
    );

    // B1 should be empty (newly inserted)
    assert!(sheet.get(&CellAddr::new("B", 1)).is_none());

    // Old B1 (200) should now be at C1
    assert_eq!(
        sheet
            .get(&CellAddr::new("C", 1))
            .and_then(|c| c.as_number()),
        Some(200.0)
    );
}

#[test]
fn test_insert_column_updates_formulas() {
    let mut sheet = Sheet::new();

    // Set up: A1=100, B1=200, C1=A1+B1
    sheet
        .set(
            CellAddr::new("A", 1),
            Cell::new(100.0, Unit::dimensionless()),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("B", 1),
            Cell::new(200.0, Unit::dimensionless()),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("C", 1),
            Cell::with_formula("=A1+B1".to_string()),
        )
        .unwrap();

    // Insert column before B
    sheet.insert_column_before("B").unwrap();

    // Formula in D1 (was C1) should now be =A1+C1 (B shifted to C)
    let formula_cell = sheet.get(&CellAddr::new("D", 1)).unwrap();
    let formula = formula_cell.formula().unwrap();
    assert!(
        formula.contains("A1") && formula.contains("C1"),
        "Formula should be updated to reference C1 instead of B1, got: {}",
        formula
    );
}

#[test]
fn test_insert_row_before_shifts_cells() {
    let mut sheet = Sheet::new();

    // Set up: A1=100, A2=200, A3=300
    sheet
        .set(
            CellAddr::new("A", 1),
            Cell::new(100.0, Unit::dimensionless()),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("A", 2),
            Cell::new(200.0, Unit::dimensionless()),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("A", 3),
            Cell::new(300.0, Unit::dimensionless()),
        )
        .unwrap();

    // Insert row before 2
    sheet.insert_row_before(2).unwrap();

    // A1 should stay at 100
    assert_eq!(
        sheet
            .get(&CellAddr::new("A", 1))
            .and_then(|c| c.as_number()),
        Some(100.0)
    );

    // A2 should be empty (newly inserted)
    assert!(sheet.get(&CellAddr::new("A", 2)).is_none());

    // Old A2 (200) should now be at A3
    assert_eq!(
        sheet
            .get(&CellAddr::new("A", 3))
            .and_then(|c| c.as_number()),
        Some(200.0)
    );

    // Old A3 (300) should now be at A4
    assert_eq!(
        sheet
            .get(&CellAddr::new("A", 4))
            .and_then(|c| c.as_number()),
        Some(300.0)
    );
}

#[test]
fn test_insert_row_after() {
    let mut sheet = Sheet::new();

    // Set up: A1=100, A2=200
    sheet
        .set(
            CellAddr::new("A", 1),
            Cell::new(100.0, Unit::dimensionless()),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("A", 2),
            Cell::new(200.0, Unit::dimensionless()),
        )
        .unwrap();

    // Insert row after 1 (should insert at 2)
    sheet.insert_row_after(1).unwrap();

    // A1 should stay at 100
    assert_eq!(
        sheet
            .get(&CellAddr::new("A", 1))
            .and_then(|c| c.as_number()),
        Some(100.0)
    );

    // A2 should be empty (newly inserted)
    assert!(sheet.get(&CellAddr::new("A", 2)).is_none());

    // Old A2 (200) should now be at A3
    assert_eq!(
        sheet
            .get(&CellAddr::new("A", 3))
            .and_then(|c| c.as_number()),
        Some(200.0)
    );
}

#[test]
fn test_insert_row_updates_formulas() {
    let mut sheet = Sheet::new();

    // Set up: A1=100, A2=200, A3=A1+A2
    sheet
        .set(
            CellAddr::new("A", 1),
            Cell::new(100.0, Unit::dimensionless()),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("A", 2),
            Cell::new(200.0, Unit::dimensionless()),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("A", 3),
            Cell::with_formula("=A1+A2".to_string()),
        )
        .unwrap();

    // Insert row before 2
    sheet.insert_row_before(2).unwrap();

    // Formula in A4 (was A3) should now be =A1+A3 (A2 shifted to A3)
    let formula_cell = sheet.get(&CellAddr::new("A", 4)).unwrap();
    let formula = formula_cell.formula().unwrap();
    assert!(
        formula.contains("A1") && formula.contains("A3"),
        "Formula should be updated to reference A3 instead of A2, got: {}",
        formula
    );
}

#[test]
fn test_delete_column_shifts_cells_left() {
    let mut sheet = Sheet::new();

    // Set up: A1=100, B1=200, C1=300
    sheet
        .set(
            CellAddr::new("A", 1),
            Cell::new(100.0, Unit::dimensionless()),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("B", 1),
            Cell::new(200.0, Unit::dimensionless()),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("C", 1),
            Cell::new(300.0, Unit::dimensionless()),
        )
        .unwrap();

    // Delete column B
    sheet.delete_column("B").unwrap();

    // A1 should stay at 100
    assert_eq!(
        sheet
            .get(&CellAddr::new("A", 1))
            .and_then(|c| c.as_number()),
        Some(100.0)
    );

    // Old C1 (300) should now be at B1
    assert_eq!(
        sheet
            .get(&CellAddr::new("B", 1))
            .and_then(|c| c.as_number()),
        Some(300.0)
    );

    // C1 should be empty
    assert!(sheet.get(&CellAddr::new("C", 1)).is_none());
}

#[test]
fn test_delete_column_creates_ref_errors() {
    let mut sheet = Sheet::new();

    // Set up: A1=100, B1=200, C1=A1+B1
    sheet
        .set(
            CellAddr::new("A", 1),
            Cell::new(100.0, Unit::dimensionless()),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("B", 1),
            Cell::new(200.0, Unit::dimensionless()),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("C", 1),
            Cell::with_formula("=A1+B1".to_string()),
        )
        .unwrap();

    // Delete column B
    sheet.delete_column("B").unwrap();

    // Formula in B1 (was C1) should contain #REF! for the deleted B1 reference
    let formula_cell = sheet.get(&CellAddr::new("B", 1)).unwrap();
    let formula = formula_cell.formula().unwrap();
    assert!(
        formula.contains("#REF!"),
        "Formula should contain #REF! for deleted reference, got: {}",
        formula
    );
}

#[test]
fn test_delete_row_shifts_cells_up() {
    let mut sheet = Sheet::new();

    // Set up: A1=100, A2=200, A3=300
    sheet
        .set(
            CellAddr::new("A", 1),
            Cell::new(100.0, Unit::dimensionless()),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("A", 2),
            Cell::new(200.0, Unit::dimensionless()),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("A", 3),
            Cell::new(300.0, Unit::dimensionless()),
        )
        .unwrap();

    // Delete row 2
    sheet.delete_row(2).unwrap();

    // A1 should stay at 100
    assert_eq!(
        sheet
            .get(&CellAddr::new("A", 1))
            .and_then(|c| c.as_number()),
        Some(100.0)
    );

    // Old A3 (300) should now be at A2
    assert_eq!(
        sheet
            .get(&CellAddr::new("A", 2))
            .and_then(|c| c.as_number()),
        Some(300.0)
    );

    // A3 should be empty
    assert!(sheet.get(&CellAddr::new("A", 3)).is_none());
}

#[test]
fn test_delete_row_creates_ref_errors() {
    let mut sheet = Sheet::new();

    // Set up: A1=100, A2=200, A3=A1+A2
    sheet
        .set(
            CellAddr::new("A", 1),
            Cell::new(100.0, Unit::dimensionless()),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("A", 2),
            Cell::new(200.0, Unit::dimensionless()),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("A", 3),
            Cell::with_formula("=A1+A2".to_string()),
        )
        .unwrap();

    // Delete row 2
    sheet.delete_row(2).unwrap();

    // Formula in A2 (was A3) should contain #REF! for the deleted A2 reference
    let formula_cell = sheet.get(&CellAddr::new("A", 2)).unwrap();
    let formula = formula_cell.formula().unwrap();
    assert!(
        formula.contains("#REF!"),
        "Formula should contain #REF! for deleted reference, got: {}",
        formula
    );
}

#[test]
fn test_column_width_shifts_with_insert() {
    let mut sheet = Sheet::new();

    // Set column widths: A=100, B=200, C=300
    sheet.set_column_width("A".to_string(), 100.0);
    sheet.set_column_width("B".to_string(), 200.0);
    sheet.set_column_width("C".to_string(), 300.0);

    // Insert column before B
    sheet.insert_column_before("B").unwrap();

    // A width should stay 100
    assert_eq!(sheet.get_column_width("A"), Some(100.0));

    // B width should be None (new column)
    assert_eq!(sheet.get_column_width("B"), None);

    // Old B width (200) should now be at C
    assert_eq!(sheet.get_column_width("C"), Some(200.0));

    // Old C width (300) should now be at D
    assert_eq!(sheet.get_column_width("D"), Some(300.0));
}

#[test]
fn test_column_width_shifts_with_delete() {
    let mut sheet = Sheet::new();

    // Set column widths: A=100, B=200, C=300
    sheet.set_column_width("A".to_string(), 100.0);
    sheet.set_column_width("B".to_string(), 200.0);
    sheet.set_column_width("C".to_string(), 300.0);

    // Delete column B
    sheet.delete_column("B").unwrap();

    // A width should stay 100
    assert_eq!(sheet.get_column_width("A"), Some(100.0));

    // Old C width (300) should now be at B
    assert_eq!(sheet.get_column_width("B"), Some(300.0));

    // C should be None
    assert_eq!(sheet.get_column_width("C"), None);
}

#[test]
fn test_row_height_shifts_with_insert() {
    let mut sheet = Sheet::new();

    // Set row heights: 1=50, 2=100, 3=150
    sheet.set_row_height(1, 50.0);
    sheet.set_row_height(2, 100.0);
    sheet.set_row_height(3, 150.0);

    // Insert row before 2
    sheet.insert_row_before(2).unwrap();

    // Row 1 height should stay 50
    assert_eq!(sheet.get_row_height(1), Some(50.0));

    // Row 2 height should be None (new row)
    assert_eq!(sheet.get_row_height(2), None);

    // Old row 2 height (100) should now be at row 3
    assert_eq!(sheet.get_row_height(3), Some(100.0));

    // Old row 3 height (150) should now be at row 4
    assert_eq!(sheet.get_row_height(4), Some(150.0));
}

#[test]
fn test_row_height_shifts_with_delete() {
    let mut sheet = Sheet::new();

    // Set row heights: 1=50, 2=100, 3=150
    sheet.set_row_height(1, 50.0);
    sheet.set_row_height(2, 100.0);
    sheet.set_row_height(3, 150.0);

    // Delete row 2
    sheet.delete_row(2).unwrap();

    // Row 1 height should stay 50
    assert_eq!(sheet.get_row_height(1), Some(50.0));

    // Old row 3 height (150) should now be at row 2
    assert_eq!(sheet.get_row_height(2), Some(150.0));

    // Row 3 should be None
    assert_eq!(sheet.get_row_height(3), None);
}

#[test]
fn test_insert_before_first_column() {
    let mut sheet = Sheet::new();

    // Set up: A1=100
    sheet
        .set(
            CellAddr::new("A", 1),
            Cell::new(100.0, Unit::dimensionless()),
        )
        .unwrap();

    // Insert column before A
    sheet.insert_column_before("A").unwrap();

    // A1 should be empty
    assert!(sheet.get(&CellAddr::new("A", 1)).is_none());

    // Old A1 (100) should now be at B1
    assert_eq!(
        sheet
            .get(&CellAddr::new("B", 1))
            .and_then(|c| c.as_number()),
        Some(100.0)
    );
}

#[test]
fn test_insert_before_first_row() {
    let mut sheet = Sheet::new();

    // Set up: A1=100
    sheet
        .set(
            CellAddr::new("A", 1),
            Cell::new(100.0, Unit::dimensionless()),
        )
        .unwrap();

    // Insert row before 1
    sheet.insert_row_before(1).unwrap();

    // A1 should be empty
    assert!(sheet.get(&CellAddr::new("A", 1)).is_none());

    // Old A1 (100) should now be at A2
    assert_eq!(
        sheet
            .get(&CellAddr::new("A", 2))
            .and_then(|c| c.as_number()),
        Some(100.0)
    );
}

#[test]
fn test_complex_formula_updates() {
    let mut sheet = Sheet::new();

    // Set up complex formula with multiple references
    sheet
        .set(
            CellAddr::new("A", 1),
            Cell::new(100.0, Unit::dimensionless()),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("B", 1),
            Cell::new(200.0, Unit::dimensionless()),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("C", 1),
            Cell::new(300.0, Unit::dimensionless()),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("D", 1),
            Cell::with_formula("=A1+B1*C1".to_string()),
        )
        .unwrap();

    // Insert column before B
    sheet.insert_column_before("B").unwrap();

    // Formula in E1 (was D1) should be =A1+C1*D1
    let formula_cell = sheet.get(&CellAddr::new("E", 1)).unwrap();
    let formula = formula_cell.formula().unwrap();
    assert!(
        formula.contains("A1") && formula.contains("C1") && formula.contains("D1"),
        "Formula should be updated correctly, got: {}",
        formula
    );
}

#[test]
fn test_units_preserved_during_operations() {
    let mut sheet = Sheet::new();

    // Set up with units
    sheet
        .set(
            CellAddr::new("A", 1),
            Cell::new(100.0, Unit::simple("m", BaseDimension::Length)),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("B", 1),
            Cell::new(200.0, Unit::simple("kg", BaseDimension::Mass)),
        )
        .unwrap();

    // Insert column before B
    sheet.insert_column_before("B").unwrap();

    // A1 should still have meters
    let a1 = sheet.get(&CellAddr::new("A", 1)).unwrap();
    assert_eq!(a1.storage_unit().canonical().to_string(), "m");

    // Old B1 (kg) should now be at C1
    let c1 = sheet.get(&CellAddr::new("C", 1)).unwrap();
    assert_eq!(c1.storage_unit().canonical().to_string(), "kg");
}
