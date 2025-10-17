use std::env;
use std::fs;
use unicel_lib::core::cell::Cell;
use unicel_lib::core::table::CellAddr;
use unicel_lib::core::units::Unit;
use unicel_lib::core::workbook::Workbook;
use unicel_lib::formats::excel::export_to_excel;

#[test]
fn test_excel_export_string_concatenation() {
    // Create a workbook with string concatenation formulas
    let mut workbook = Workbook::new("String Concat Test");

    let sheet = workbook.active_sheet_mut();

    // Add some test cells with string concatenation
    sheet
        .set(
            CellAddr::new("A", 1),
            Cell::with_formula(r#"="Hello" + " world""#),
        )
        .unwrap();

    sheet
        .set(
            CellAddr::new("A", 2),
            Cell::with_formula(r#"="Count: " + B2"#),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("B", 2),
            Cell::new(42.0, Unit::dimensionless()),
        )
        .unwrap();

    sheet
        .set(
            CellAddr::new("A", 3),
            Cell::with_formula(r#"="Total: " + B3 + " items""#),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("B", 3),
            Cell::new(100.0, Unit::dimensionless()),
        )
        .unwrap();

    // Add a numeric addition formula (should NOT convert to CONCATENATE)
    sheet
        .set(CellAddr::new("A", 4), Cell::with_formula("=B4 + C4"))
        .unwrap();
    sheet
        .set(
            CellAddr::new("B", 4),
            Cell::new(10.0, Unit::dimensionless()),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("C", 4),
            Cell::new(20.0, Unit::dimensionless()),
        )
        .unwrap();

    // Export to Excel
    let temp_file = env::temp_dir().join("test_string_concatenation.xlsx");
    export_to_excel(&workbook, temp_file.to_str().unwrap()).expect("Export failed");

    // Verify the file was created
    assert!(fs::metadata(&temp_file).is_ok());

    // Clean up
    fs::remove_file(&temp_file).ok();
}

#[test]
fn test_excel_export_mixed_concatenation() {
    // Create a workbook with mixed formulas
    let mut workbook = Workbook::new("Mixed Test");

    let sheet = workbook.active_sheet_mut();

    // String + cell reference
    sheet
        .set(
            CellAddr::new("A", 1),
            Cell::with_formula(r#"="Result: " + B1"#),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("B", 1),
            Cell::new(3.14, Unit::dimensionless()),
        )
        .unwrap();

    // Cell reference + string
    sheet
        .set(
            CellAddr::new("A", 2),
            Cell::with_formula(r#"=B2 + " meters""#),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("B", 2),
            Cell::new(
                100.0,
                Unit::simple("m", unicel_lib::core::units::BaseDimension::Length),
            ),
        )
        .unwrap();

    // Multiple concatenations
    sheet
        .set(
            CellAddr::new("A", 3),
            Cell::with_formula(r#"="The" + " " + "answer""#),
        )
        .unwrap();

    // Export to Excel
    let temp_file = env::temp_dir().join("test_mixed_concatenation.xlsx");
    export_to_excel(&workbook, temp_file.to_str().unwrap()).expect("Export failed");

    // Verify the file was created
    assert!(fs::metadata(&temp_file).is_ok());

    // Clean up
    fs::remove_file(&temp_file).ok();
}
