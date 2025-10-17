/// Manual verification test for Excel export with conversion-aware arithmetic
///
/// Run this test and manually open the generated Excel file to verify:
/// - Row 2 (costs): Should show 1,536,000 $, 1,500 $, 1.46484375 $
/// - Row 3 (costs/month): Should show 1,536,000 $/Month, 1,500 $/Month, 1.46484375 $/Month
///
/// Before the fix, all cells showed "1500" because formulas were exported without evaluation.
/// After the fix, they show the correctly computed values with unit conversions applied.
use unicel_lib::core::cell::Cell;
use unicel_lib::core::table::CellAddr;
use unicel_lib::core::units::{BaseDimension, Unit};
use unicel_lib::core::workbook::Workbook;
use unicel_lib::formats::excel::export_to_excel;

#[test]
#[ignore] // Run manually with: cargo test --test excel_conversion_manual_check -- --ignored --nocapture
fn manual_check_excel_conversion_arithmetic() {
    // Create a workbook with conversion-aware arithmetic
    let mut workbook = Workbook::new("Conversion Arithmetic Test");

    let sheet = workbook.active_sheet_mut();

    // Create storage units
    let tb_unit = Unit::simple("TB", BaseDimension::DigitalStorage);
    let gb_unit = Unit::simple("GB", BaseDimension::DigitalStorage);
    let mb_unit = Unit::simple("MB", BaseDimension::DigitalStorage);
    let dollar_per_gb = Unit::compound(
        "$/GB",
        vec![(BaseDimension::Currency, 1)],
        vec![(BaseDimension::DigitalStorage, 1)],
    );
    let dollar_per_gb_month = Unit::compound(
        "$/GB*Month",
        vec![(BaseDimension::Currency, 1)],
        vec![(BaseDimension::DigitalStorage, 1), (BaseDimension::Time, 1)],
    );

    // Row 1: Storage amounts (headers)
    sheet
        .set(CellAddr::new("A", 1), Cell::new(100.0, tb_unit.clone()))
        .unwrap();
    sheet
        .set(CellAddr::new("B", 1), Cell::new(100.0, gb_unit.clone()))
        .unwrap();
    sheet
        .set(CellAddr::new("C", 1), Cell::new(100.0, mb_unit.clone()))
        .unwrap();

    // Row 2: Cost per GB
    sheet
        .set(
            CellAddr::new("D", 2),
            Cell::new(15.0, dollar_per_gb.clone()),
        )
        .unwrap();

    // Row 2 formulas: storage × cost per GB = total cost
    sheet
        .set(CellAddr::new("A", 2), Cell::with_formula("=A1*D2"))
        .unwrap();
    sheet
        .set(CellAddr::new("B", 2), Cell::with_formula("=B1*D2"))
        .unwrap();
    sheet
        .set(CellAddr::new("C", 2), Cell::with_formula("=C1*D2"))
        .unwrap();

    // Row 3: Cost per GB per month
    sheet
        .set(CellAddr::new("D", 3), Cell::new(15.0, dollar_per_gb_month))
        .unwrap();

    // Row 3 formulas: storage × cost per GB per month = total cost per month
    sheet
        .set(CellAddr::new("A", 3), Cell::with_formula("=A1*D3"))
        .unwrap();
    sheet
        .set(CellAddr::new("B", 3), Cell::with_formula("=B1*D3"))
        .unwrap();
    sheet
        .set(CellAddr::new("C", 3), Cell::with_formula("=C1*D3"))
        .unwrap();

    // Recalculate all formulas
    let formula_cells = vec![
        CellAddr::new("A", 2),
        CellAddr::new("B", 2),
        CellAddr::new("C", 2),
        CellAddr::new("A", 3),
        CellAddr::new("B", 3),
        CellAddr::new("C", 3),
    ];
    sheet.recalculate(&formula_cells).unwrap();

    // Print computed values
    println!("\n=== Computed values in Unicel ===");
    println!("Row 1: 100 TB, 100 GB, 100 MB");
    println!("Row 2 (@ 15 $/GB):");
    println!(
        "  A2: {} $",
        sheet
            .get(&CellAddr::new("A", 2))
            .unwrap()
            .as_number()
            .unwrap()
    );
    println!(
        "  B2: {} $",
        sheet
            .get(&CellAddr::new("B", 2))
            .unwrap()
            .as_number()
            .unwrap()
    );
    println!(
        "  C2: {} $",
        sheet
            .get(&CellAddr::new("C", 2))
            .unwrap()
            .as_number()
            .unwrap()
    );
    println!("Row 3 (@ 15 $/GB*Month):");
    println!(
        "  A3: {} $/Month",
        sheet
            .get(&CellAddr::new("A", 3))
            .unwrap()
            .as_number()
            .unwrap()
    );
    println!(
        "  B3: {} $/Month",
        sheet
            .get(&CellAddr::new("B", 3))
            .unwrap()
            .as_number()
            .unwrap()
    );
    println!(
        "  C3: {} $/Month",
        sheet
            .get(&CellAddr::new("C", 3))
            .unwrap()
            .as_number()
            .unwrap()
    );

    // Export to Excel
    let output_file = "/tmp/excel_conversion_arithmetic_manual_check.xlsx";
    export_to_excel(&workbook, output_file).expect("Export failed");

    println!("\n=== Excel file created ===");
    println!("Location: {}", output_file);
    println!("\nPlease open this file in Excel and verify:");
    println!("  Row 2 cells show: 1,536,000 $, 1,500 $, 1.46484375 $");
    println!("  Row 3 cells show: 1,536,000 $/Month, 1,500 $/Month, 1.46484375 $/Month");
    println!("\nThe values should be static numbers (blue italic), not formulas.");
    println!("Check the 'Unit Metadata' sheet for formula information.");
}
