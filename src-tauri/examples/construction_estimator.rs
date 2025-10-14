// Construction Estimator Example
//
// This example demonstrates:
// - Dimensional calculations (sqft, board feet)
// - Cost calculations with automatic unit cancellation
// - Metric/Imperial display toggle
// - Real-world construction planning use case

use unicel_lib::core::cell::Cell;
use unicel_lib::core::table::CellAddr;
use unicel_lib::core::units::{BaseDimension, Unit};
use unicel_lib::core::workbook::Workbook;
use unicel_lib::formats::json::WorkbookFile;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut workbook = Workbook::new("Construction Estimator");
    let sheet = workbook.active_sheet_mut();
    sheet.set_name("Material Estimate");

    // Header row
    sheet.set(CellAddr::new("A", 1), Cell::with_text("Item"))?;
    sheet.set(CellAddr::new("B", 1), Cell::with_text("Length"))?;
    sheet.set(CellAddr::new("C", 1), Cell::with_text("Width"))?;
    sheet.set(CellAddr::new("D", 1), Cell::with_text("Area"))?;
    sheet.set(CellAddr::new("E", 1), Cell::with_text("Cost per sqft"))?;
    sheet.set(CellAddr::new("F", 1), Cell::with_text("Total Cost"))?;

    // Row 2: Flooring for main room
    sheet.set(CellAddr::new("A", 2), Cell::with_text("Main Room Flooring"))?;
    sheet.set(CellAddr::new("B", 2), Cell::new(25.0, Unit::simple("ft", BaseDimension::Length)))?;
    sheet.set(CellAddr::new("C", 2), Cell::new(20.0, Unit::simple("ft", BaseDimension::Length)))?;
    sheet.set(CellAddr::new("D", 2), Cell::with_formula("=B2 * C2"))?;
    sheet.set(CellAddr::new("E", 2), Cell::new(5.50, Unit::simple("USD", BaseDimension::Currency)))?;
    sheet.set(CellAddr::new("F", 2), Cell::with_formula("=D2 * E2"))?;

    // Row 3: Bedroom flooring
    sheet.set(CellAddr::new("A", 3), Cell::with_text("Bedroom Flooring"))?;
    sheet.set(CellAddr::new("B", 3), Cell::new(15.0, Unit::simple("ft", BaseDimension::Length)))?;
    sheet.set(CellAddr::new("C", 3), Cell::new(12.0, Unit::simple("ft", BaseDimension::Length)))?;
    sheet.set(CellAddr::new("D", 3), Cell::with_formula("=B3 * C3"))?;
    sheet.set(CellAddr::new("E", 3), Cell::new(5.50, Unit::simple("USD", BaseDimension::Currency)))?;
    sheet.set(CellAddr::new("F", 3), Cell::with_formula("=D3 * E3"))?;

    // Row 4: Kitchen tile
    sheet.set(CellAddr::new("A", 4), Cell::with_text("Kitchen Tile"))?;
    sheet.set(CellAddr::new("B", 4), Cell::new(12.0, Unit::simple("ft", BaseDimension::Length)))?;
    sheet.set(CellAddr::new("C", 4), Cell::new(10.0, Unit::simple("ft", BaseDimension::Length)))?;
    sheet.set(CellAddr::new("D", 4), Cell::with_formula("=B4 * C4"))?;
    sheet.set(CellAddr::new("E", 4), Cell::new(8.75, Unit::simple("USD", BaseDimension::Currency)))?;
    sheet.set(CellAddr::new("F", 4), Cell::with_formula("=D4 * E4"))?;

    // Row 6: Total area and cost
    sheet.set(CellAddr::new("A", 6), Cell::with_text("TOTAL"))?;
    sheet.set(CellAddr::new("D", 6), Cell::with_formula("=SUM(D2:D4)"))?;
    sheet.set(CellAddr::new("F", 6), Cell::with_formula("=SUM(F2:F4)"))?;

    // Row 8-12: Lumber calculations (board feet)
    sheet.set(CellAddr::new("A", 8), Cell::with_text("Lumber Type"))?;
    sheet.set(CellAddr::new("B", 8), Cell::with_text("Length"))?;
    sheet.set(CellAddr::new("C", 8), Cell::with_text("Quantity"))?;
    sheet.set(CellAddr::new("D", 8), Cell::with_text("Total Length"))?;
    sheet.set(CellAddr::new("E", 8), Cell::with_text("Cost per ft"))?;
    sheet.set(CellAddr::new("F", 8), Cell::with_text("Total Cost"))?;

    // 2x4 studs
    sheet.set(CellAddr::new("A", 9), Cell::with_text("2x4 Studs"))?;
    sheet.set(CellAddr::new("B", 9), Cell::new(8.0, Unit::simple("ft", BaseDimension::Length)))?;
    sheet.set(CellAddr::new("C", 9), Cell::new(24.0, Unit::dimensionless()))?;
    sheet.set(CellAddr::new("D", 9), Cell::with_formula("=B9 * C9"))?;
    sheet.set(CellAddr::new("E", 9), Cell::new(0.55, Unit::simple("USD", BaseDimension::Currency)))?;
    sheet.set(CellAddr::new("F", 9), Cell::with_formula("=D9 * E9"))?;

    // 2x6 joists
    sheet.set(CellAddr::new("A", 10), Cell::with_text("2x6 Joists"))?;
    sheet.set(CellAddr::new("B", 10), Cell::new(12.0, Unit::simple("ft", BaseDimension::Length)))?;
    sheet.set(CellAddr::new("C", 10), Cell::new(16.0, Unit::dimensionless()))?;
    sheet.set(CellAddr::new("D", 10), Cell::with_formula("=B10 * C10"))?;
    sheet.set(CellAddr::new("E", 10), Cell::new(1.25, Unit::simple("USD", BaseDimension::Currency)))?;
    sheet.set(CellAddr::new("F", 10), Cell::with_formula("=D10 * E10"))?;

    // Plywood sheets
    sheet.set(CellAddr::new("A", 11), Cell::with_text("Plywood 4x8"))?;
    sheet.set(CellAddr::new("B", 11), Cell::new(8.0, Unit::simple("ft", BaseDimension::Length)))?;
    sheet.set(CellAddr::new("C", 11), Cell::new(10.0, Unit::dimensionless()))?;
    sheet.set(CellAddr::new("D", 11), Cell::with_formula("=B11 * C11"))?;
    sheet.set(CellAddr::new("E", 11), Cell::new(1.85, Unit::simple("USD", BaseDimension::Currency)))?;
    sheet.set(CellAddr::new("F", 11), Cell::with_formula("=D11 * E11"))?;

    // Lumber total
    sheet.set(CellAddr::new("A", 13), Cell::with_text("LUMBER TOTAL"))?;
    sheet.set(CellAddr::new("D", 13), Cell::with_formula("=SUM(D9:D11)"))?;
    sheet.set(CellAddr::new("F", 13), Cell::with_formula("=SUM(F9:F11)"))?;

    // Grand total
    sheet.set(CellAddr::new("A", 15), Cell::with_text("GRAND TOTAL"))?;
    sheet.set(CellAddr::new("F", 15), Cell::with_formula("=F6 + F13"))?;

    // Notes section
    sheet.set(CellAddr::new("A", 17), Cell::with_text("NOTES:"))?;
    sheet.set(CellAddr::new("A", 18), Cell::with_text("- All flooring prices include installation"))?;
    sheet.set(CellAddr::new("A", 19), Cell::with_text("- Lumber prices are per linear foot"))?;
    sheet.set(CellAddr::new("A", 20), Cell::with_text("- Toggle Metric/Imperial to see conversions"))?;
    sheet.set(CellAddr::new("A", 21), Cell::with_text("- Area formulas automatically cancel units"))?;

    // Recalculate all formulas
    let changed_cells: Vec<CellAddr> = vec![
        CellAddr::new("B", 2), CellAddr::new("C", 2), CellAddr::new("E", 2),
        CellAddr::new("B", 3), CellAddr::new("C", 3), CellAddr::new("E", 3),
        CellAddr::new("B", 4), CellAddr::new("C", 4), CellAddr::new("E", 4),
        CellAddr::new("B", 9), CellAddr::new("C", 9), CellAddr::new("E", 9),
        CellAddr::new("B", 10), CellAddr::new("C", 10), CellAddr::new("E", 10),
        CellAddr::new("B", 11), CellAddr::new("C", 11), CellAddr::new("E", 11),
    ];
    sheet.recalculate(&changed_cells)?;

    // Save to file
    let file = WorkbookFile::from_workbook(&workbook);
    let path = PathBuf::from("examples/construction_estimator.usheet");
    file.save_to_file(&path)?;

    println!("✓ Created construction_estimator.usheet");
    println!("  - Material list with dimensional calculations");
    println!("  - Automatic unit cancellation (ft × ft = sqft)");
    println!("  - Cost calculations (sqft × USD/sqft = USD)");
    println!("  - Lumber calculations with quantities");
    println!("  - Grand total: calculated from all sections");

    Ok(())
}
