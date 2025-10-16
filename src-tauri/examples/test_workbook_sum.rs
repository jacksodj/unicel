// Test SUM recalculation through Workbook API
use unicel_lib::core::cell::Cell;
use unicel_lib::core::table::CellAddr;
use unicel_lib::core::units::{BaseDimension, Unit};
use unicel_lib::core::workbook::Workbook;
use unicel_lib::formats::json::WorkbookFile;

fn main() {
    println!("Testing SUM recalculation via Workbook API...\n");

    let mut workbook = Workbook::new("Test");

    // Use the same pattern as AWS cost estimator
    let sheet1 = workbook.active_sheet_mut();

    // Set up cells
    sheet1
        .set(
            CellAddr::new("A", 1),
            Cell::new(10.0, Unit::simple("USD", BaseDimension::Currency)),
        )
        .unwrap();
    sheet1
        .set(
            CellAddr::new("A", 2),
            Cell::new(20.0, Unit::simple("USD", BaseDimension::Currency)),
        )
        .unwrap();
    sheet1
        .set(
            CellAddr::new("A", 3),
            Cell::new(30.0, Unit::simple("USD", BaseDimension::Currency)),
        )
        .unwrap();

    // Set SUM formula
    sheet1
        .set(CellAddr::new("A", 4), Cell::with_formula("=SUM(A1:A3)"))
        .unwrap();

    println!("Before recalculation:");
    if let Some(cell) = sheet1.get(&CellAddr::new("A", 4)) {
        println!("  A4 value: {:?}", cell.value());
        println!("  A4 unit: {}", cell.storage_unit().canonical());
    }

    // Recalculate (same pattern as AWS estimator)
    sheet1
        .recalculate(&[
            CellAddr::new("A", 1),
            CellAddr::new("A", 2),
            CellAddr::new("A", 3),
        ])
        .unwrap();

    println!("\nAfter recalculation:");
    if let Some(cell) = sheet1.get(&CellAddr::new("A", 4)) {
        println!("  A4 value: {:?}", cell.value());
        println!("  A4 unit: {}", cell.storage_unit().canonical());
    }

    // Now serialize and check the JSON
    let file = WorkbookFile::from_workbook(&workbook);
    let json = file.to_json().unwrap();

    // Parse and check
    let file2 = WorkbookFile::from_json(&json).unwrap();
    let workbook2 = file2.to_workbook().unwrap();

    println!("\nAfter serialize/deserialize:");
    if let Some(cell) = workbook2.active_sheet().get(&CellAddr::new("A", 4)) {
        println!("  A4 value: {:?}", cell.value());
        println!("  A4 unit: {}", cell.storage_unit().canonical());

        if cell.value().is_empty() {
            println!("\n❌ FAILED! Cell value was not persisted through serialization");
        } else if cell.as_number() == Some(60.0) && cell.storage_unit().canonical() == "USD" {
            println!("\n✅ SUCCESS! SUM recalculation and serialization work correctly");
        } else {
            println!(
                "\n❌ FAILED! Expected 60.0 USD, got {:?} {}",
                cell.as_number(),
                cell.storage_unit().canonical()
            );
        }
    }
}
