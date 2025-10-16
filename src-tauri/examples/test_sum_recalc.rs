// Test SUM function recalculation
use unicel_lib::core::cell::Cell;
use unicel_lib::core::table::{CellAddr, Sheet};
use unicel_lib::core::units::{BaseDimension, Unit};

fn main() {
    println!("Testing SUM recalculation...\n");

    let mut sheet = Sheet::new();

    // Set up cells
    sheet
        .set(
            CellAddr::new("A", 1),
            Cell::new(10.0, Unit::simple("USD", BaseDimension::Currency)),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("A", 2),
            Cell::new(20.0, Unit::simple("USD", BaseDimension::Currency)),
        )
        .unwrap();
    sheet
        .set(
            CellAddr::new("A", 3),
            Cell::new(30.0, Unit::simple("USD", BaseDimension::Currency)),
        )
        .unwrap();

    // Set SUM formula
    sheet
        .set(CellAddr::new("A", 4), Cell::with_formula("=SUM(A1:A3)"))
        .unwrap();

    println!("Before recalculation:");
    if let Some(cell) = sheet.get(&CellAddr::new("A", 4)) {
        println!("  A4 value: {:?}", cell.value());
        println!("  A4 unit: {}", cell.storage_unit().canonical());
    }

    // Recalculate
    sheet
        .recalculate(&[
            CellAddr::new("A", 1),
            CellAddr::new("A", 2),
            CellAddr::new("A", 3),
        ])
        .unwrap();

    println!("\nAfter recalculation:");
    if let Some(cell) = sheet.get(&CellAddr::new("A", 4)) {
        println!("  A4 value: {:?}", cell.value());
        println!("  A4 unit: {}", cell.storage_unit().canonical());
        println!("  A4 as_number: {:?}", cell.as_number());

        if cell.as_number() == Some(60.0) && cell.storage_unit().canonical() == "USD" {
            println!("\n✅ SUCCESS! SUM recalculation works correctly");
        } else {
            println!(
                "\n❌ FAILED! Expected 60.0 USD, got {:?} {}",
                cell.as_number(),
                cell.storage_unit().canonical()
            );
        }
    }
}
