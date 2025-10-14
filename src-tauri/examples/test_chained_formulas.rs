// Test chained formulas: formulas that depend on other formulas
use unicel_lib::core::cell::Cell;
use unicel_lib::core::table::CellAddr;
use unicel_lib::core::units::{BaseDimension, Unit};
use unicel_lib::core::workbook::Workbook;
use unicel_lib::formats::json::WorkbookFile;

fn main() {
    println!("Testing chained formulas...\n");

    let mut workbook = Workbook::new("Test");
    let sheet1 = workbook.active_sheet_mut();

    // Set up input cells
    sheet1.set(CellAddr::new("A", 1), Cell::new(10.0, Unit::simple("USD", BaseDimension::Currency))).unwrap();
    sheet1.set(CellAddr::new("B", 1), Cell::new(2.0, Unit::dimensionless())).unwrap();

    sheet1.set(CellAddr::new("A", 2), Cell::new(20.0, Unit::simple("USD", BaseDimension::Currency))).unwrap();
    sheet1.set(CellAddr::new("B", 2), Cell::new(3.0, Unit::dimensionless())).unwrap();

    // Set multiplication formulas (like E5, E6 in AWS estimator)
    sheet1.set(CellAddr::new("C", 1), Cell::with_formula("=A1 * B1")).unwrap(); // Should be 20
    sheet1.set(CellAddr::new("C", 2), Cell::with_formula("=A2 * B2")).unwrap(); // Should be 60

    // Set SUM formula that depends on formula cells (like E8 in AWS estimator)
    sheet1.set(CellAddr::new("C", 3), Cell::with_formula("=SUM(C1:C2)")).unwrap(); // Should be 80

    println!("Before recalculation:");
    if let Some(cell) = sheet1.get(&CellAddr::new("C", 3)) {
        println!("  C3 (SUM) value: {:?}", cell.value());
    }

    // Recalculate with ONLY input cells (same pattern as AWS estimator)
    sheet1.recalculate(&[
        CellAddr::new("A", 1), CellAddr::new("B", 1),
        CellAddr::new("A", 2), CellAddr::new("B", 2),
    ]).unwrap();

    println!("\nAfter recalculation:");
    if let Some(cell) = sheet1.get(&CellAddr::new("C", 1)) {
        println!("  C1 (A1*B1) value: {:?}", cell.value());
    }
    if let Some(cell) = sheet1.get(&CellAddr::new("C", 2)) {
        println!("  C2 (A2*B2) value: {:?}", cell.value());
    }
    if let Some(cell) = sheet1.get(&CellAddr::new("C", 3)) {
        println!("  C3 (SUM) value: {:?}", cell.value());
        println!("  C3 (SUM) unit: {}", cell.storage_unit().canonical());
    }

    // Serialize and check
    let file = WorkbookFile::from_workbook(&workbook);
    let json = file.to_json().unwrap();
    let file2 = WorkbookFile::from_json(&json).unwrap();
    let workbook2 = file2.to_workbook().unwrap();

    println!("\nAfter serialize/deserialize:");
    if let Some(cell) = workbook2.active_sheet().get(&CellAddr::new("C", 3)) {
        println!("  C3 (SUM) value: {:?}", cell.value());
        println!("  C3 (SUM) unit: {}", cell.storage_unit().canonical());

        if cell.value().is_empty() {
            println!("\n❌ FAILED! Chained formula (SUM of formulas) was not evaluated");
        } else if cell.as_number() == Some(80.0) {
            println!("\n✅ SUCCESS! Chained formulas work correctly");
        } else {
            println!("\n❌ FAILED! Expected 80.0 USD, got {:?}", cell.as_number());
        }
    }
}
