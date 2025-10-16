// Formula Functions Showcase
//
// This example demonstrates currently implemented formula functions:
// - Aggregation: SUM, AVERAGE, COUNT, MIN, MAX, MEDIAN, STDEV, VAR
// - Mathematical: ABS, ROUND, FLOOR, CEIL, TRUNC, MOD, SIGN, SQRT, POWER
// - Logic: IF, AND, OR, NOT
// - Comparison: GT, LT, GTE, LTE, EQ, NE
// - Unit Operations: CONVERT, PERCENT

use std::path::PathBuf;
use unicel_lib::core::cell::Cell;
use unicel_lib::core::table::CellAddr;
use unicel_lib::core::units::{BaseDimension, Unit};
use unicel_lib::core::workbook::Workbook;
use unicel_lib::formats::json::WorkbookFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut workbook = Workbook::new("Formula Functions Showcase");

    // Sheet 1: Mathematical Functions
    create_mathematical_sheet(&mut workbook)?;

    // Sheet 2: Aggregation Functions
    workbook.add_sheet_with_name("Aggregation Functions");
    create_aggregation_sheet(&mut workbook)?;

    // Sheet 3: Logic Functions
    workbook.add_sheet_with_name("Logic Functions");
    create_logic_sheet(&mut workbook)?;

    // Sheet 4: Real-World Examples
    workbook.add_sheet_with_name("Real-World Examples");
    create_examples_sheet(&mut workbook)?;

    // Save to file
    let file = WorkbookFile::from_workbook(&workbook);
    let path = PathBuf::from("examples/formula_functions_showcase.usheet");
    file.save_to_file(&path)?;

    println!("✓ Created formula_functions_showcase.usheet");
    println!("  - Mathematical, aggregation, and logic functions");
    println!("  - 4 sheets with practical examples");
    println!("  - Unit-aware calculations throughout");

    Ok(())
}

fn create_mathematical_sheet(workbook: &mut Workbook) -> Result<(), Box<dyn std::error::Error>> {
    let sheet = workbook.active_sheet_mut();
    sheet.set_name("Mathematical Functions");

    // Title
    sheet.set(
        CellAddr::new("A", 1),
        Cell::with_text("MATHEMATICAL FUNCTIONS"),
    )?;

    // Headers
    sheet.set(CellAddr::new("A", 3), Cell::with_text("Function"))?;
    sheet.set(CellAddr::new("B", 3), Cell::with_text("Input"))?;
    sheet.set(CellAddr::new("C", 3), Cell::with_text("Formula"))?;
    sheet.set(CellAddr::new("D", 3), Cell::with_text("Result"))?;
    sheet.set(CellAddr::new("E", 3), Cell::with_text("Use Case"))?;

    // SQRT
    sheet.set(CellAddr::new("A", 5), Cell::with_text("SQRT"))?;
    sheet.set(
        CellAddr::new("B", 5),
        Cell::new(
            144.0,
            Unit::compound("m^2", vec![(BaseDimension::Length, 2)], vec![]),
        ),
    )?;
    sheet.set(CellAddr::new("C", 5), Cell::with_text("=SQRT(B5)"))?;
    sheet.set(CellAddr::new("D", 5), Cell::with_formula("=SQRT(B5)"))?;
    sheet.set(
        CellAddr::new("E", 5),
        Cell::with_text("Side length from area"),
    )?;

    // ABS
    sheet.set(CellAddr::new("A", 6), Cell::with_text("ABS"))?;
    sheet.set(
        CellAddr::new("B", 6),
        Cell::new(-15.5, Unit::simple("degC", BaseDimension::Temperature)),
    )?;
    sheet.set(CellAddr::new("C", 6), Cell::with_text("=ABS(B6)"))?;
    sheet.set(CellAddr::new("D", 6), Cell::with_formula("=ABS(B6)"))?;
    sheet.set(
        CellAddr::new("E", 6),
        Cell::with_text("Temperature magnitude"),
    )?;

    // ROUND
    sheet.set(CellAddr::new("A", 7), Cell::with_text("ROUND"))?;
    sheet.set(
        CellAddr::new("B", 7),
        Cell::new(123.456, Unit::simple("USD", BaseDimension::Currency)),
    )?;
    sheet.set(CellAddr::new("C", 7), Cell::with_text("=ROUND(B7, 2)"))?;
    sheet.set(CellAddr::new("D", 7), Cell::with_formula("=ROUND(B7, 2)"))?;
    sheet.set(CellAddr::new("E", 7), Cell::with_text("Currency to cents"))?;

    // CEILING
    sheet.set(CellAddr::new("A", 8), Cell::with_text("CEILING"))?;
    sheet.set(
        CellAddr::new("B", 8),
        Cell::new(2.3, Unit::simple("hours", BaseDimension::Time)),
    )?;
    sheet.set(CellAddr::new("C", 8), Cell::with_text("=CEILING(B8)"))?;
    sheet.set(CellAddr::new("D", 8), Cell::with_formula("=CEILING(B8)"))?;
    sheet.set(
        CellAddr::new("E", 8),
        Cell::with_text("Round up billable hours"),
    )?;

    // FLOOR
    sheet.set(CellAddr::new("A", 9), Cell::with_text("FLOOR"))?;
    sheet.set(
        CellAddr::new("B", 9),
        Cell::new(
            8.9,
            Unit::simple("liters", BaseDimension::Custom("Volume".to_string())),
        ),
    )?;
    sheet.set(CellAddr::new("C", 9), Cell::with_text("=FLOOR(B9)"))?;
    sheet.set(CellAddr::new("D", 9), Cell::with_formula("=FLOOR(B9)"))?;
    sheet.set(
        CellAddr::new("E", 9),
        Cell::with_text("Full containers only"),
    )?;

    // MOD
    sheet.set(CellAddr::new("A", 10), Cell::with_text("MOD"))?;
    sheet.set(
        CellAddr::new("B", 10),
        Cell::new(100.0, Unit::simple("km", BaseDimension::Length)),
    )?;
    sheet.set(
        CellAddr::new("F", 10),
        Cell::new(30.0, Unit::simple("km", BaseDimension::Length)),
    )?;
    sheet.set(CellAddr::new("C", 10), Cell::with_text("=MOD(B10, F10)"))?;
    sheet.set(CellAddr::new("D", 10), Cell::with_formula("=MOD(B10, F10)"))?;
    sheet.set(
        CellAddr::new("E", 10),
        Cell::with_text("Remainder distance"),
    )?;

    // POWER
    sheet.set(CellAddr::new("A", 11), Cell::with_text("POWER"))?;
    sheet.set(
        CellAddr::new("B", 11),
        Cell::new(5.0, Unit::simple("m", BaseDimension::Length)),
    )?;
    sheet.set(CellAddr::new("C", 11), Cell::with_text("=POWER(B11, 3)"))?;
    sheet.set(CellAddr::new("D", 11), Cell::with_formula("=POWER(B11, 3)"))?;
    sheet.set(CellAddr::new("E", 11), Cell::with_text("Volume of cube"))?;

    // TRUNC
    sheet.set(CellAddr::new("A", 12), Cell::with_text("TRUNC"))?;
    sheet.set(
        CellAddr::new("B", 12),
        Cell::new(123.789, Unit::simple("USD", BaseDimension::Currency)),
    )?;
    sheet.set(CellAddr::new("C", 12), Cell::with_text("=TRUNC(B12)"))?;
    sheet.set(CellAddr::new("D", 12), Cell::with_formula("=TRUNC(B12)"))?;
    sheet.set(
        CellAddr::new("E", 12),
        Cell::with_text("Truncate to integer"),
    )?;

    // SIGN
    sheet.set(CellAddr::new("A", 13), Cell::with_text("SIGN"))?;
    sheet.set(
        CellAddr::new("B", 13),
        Cell::new(-42.0, Unit::simple("USD", BaseDimension::Currency)),
    )?;
    sheet.set(CellAddr::new("C", 13), Cell::with_text("=SIGN(B13)"))?;
    sheet.set(CellAddr::new("D", 13), Cell::with_formula("=SIGN(B13)"))?;
    sheet.set(
        CellAddr::new("E", 13),
        Cell::with_text("Profit/loss indicator"),
    )?;

    // Recalculate
    let changed_cells: Vec<CellAddr> = (5..=13).map(|row| CellAddr::new("B", row)).collect();
    sheet.recalculate(&changed_cells)?;

    Ok(())
}

fn create_aggregation_sheet(workbook: &mut Workbook) -> Result<(), Box<dyn std::error::Error>> {
    let sheet = workbook.get_sheet_mut(1).ok_or("Sheet 1 not found")?;

    // Title
    sheet.set(
        CellAddr::new("A", 1),
        Cell::with_text("AGGREGATION FUNCTIONS"),
    )?;

    // Headers
    sheet.set(CellAddr::new("A", 3), Cell::with_text("Function"))?;
    sheet.set(CellAddr::new("B", 3), Cell::with_text("Data Range"))?;
    sheet.set(CellAddr::new("C", 3), Cell::with_text("Formula"))?;
    sheet.set(CellAddr::new("D", 3), Cell::with_text("Result"))?;
    sheet.set(CellAddr::new("E", 3), Cell::with_text("Description"))?;

    // Sample data row
    sheet.set(CellAddr::new("A", 5), Cell::with_text("Sample Data:"))?;
    for i in 0..5 {
        let row = 6 + i;
        sheet.set(
            CellAddr::new("B", row),
            Cell::new(
                (i as f64 + 1.0) * 10.0,
                Unit::simple("USD", BaseDimension::Currency),
            ),
        )?;
    }

    // SUM
    sheet.set(CellAddr::new("A", 12), Cell::with_text("SUM"))?;
    sheet.set(CellAddr::new("B", 12), Cell::with_text("B6:B10"))?;
    sheet.set(CellAddr::new("C", 12), Cell::with_text("=SUM(B6:B10)"))?;
    sheet.set(CellAddr::new("D", 12), Cell::with_formula("=SUM(B6:B10)"))?;
    sheet.set(CellAddr::new("E", 12), Cell::with_text("Sum all values"))?;

    // AVERAGE
    sheet.set(CellAddr::new("A", 13), Cell::with_text("AVERAGE"))?;
    sheet.set(CellAddr::new("B", 13), Cell::with_text("B6:B10"))?;
    sheet.set(CellAddr::new("C", 13), Cell::with_text("=AVERAGE(B6:B10)"))?;
    sheet.set(
        CellAddr::new("D", 13),
        Cell::with_formula("=AVERAGE(B6:B10)"),
    )?;
    sheet.set(CellAddr::new("E", 13), Cell::with_text("Mean value"))?;

    // COUNT
    sheet.set(CellAddr::new("A", 14), Cell::with_text("COUNT"))?;
    sheet.set(CellAddr::new("B", 14), Cell::with_text("B6:B10"))?;
    sheet.set(CellAddr::new("C", 14), Cell::with_text("=COUNT(B6:B10)"))?;
    sheet.set(CellAddr::new("D", 14), Cell::with_formula("=COUNT(B6:B10)"))?;
    sheet.set(
        CellAddr::new("E", 14),
        Cell::with_text("Count non-empty cells"),
    )?;

    // MIN
    sheet.set(CellAddr::new("A", 15), Cell::with_text("MIN"))?;
    sheet.set(CellAddr::new("B", 15), Cell::with_text("B6:B10"))?;
    sheet.set(CellAddr::new("C", 15), Cell::with_text("=MIN(B6:B10)"))?;
    sheet.set(CellAddr::new("D", 15), Cell::with_formula("=MIN(B6:B10)"))?;
    sheet.set(CellAddr::new("E", 15), Cell::with_text("Minimum value"))?;

    // MAX
    sheet.set(CellAddr::new("A", 16), Cell::with_text("MAX"))?;
    sheet.set(CellAddr::new("B", 16), Cell::with_text("B6:B10"))?;
    sheet.set(CellAddr::new("C", 16), Cell::with_text("=MAX(B6:B10)"))?;
    sheet.set(CellAddr::new("D", 16), Cell::with_formula("=MAX(B6:B10)"))?;
    sheet.set(CellAddr::new("E", 16), Cell::with_text("Maximum value"))?;

    // MEDIAN
    sheet.set(CellAddr::new("A", 17), Cell::with_text("MEDIAN"))?;
    sheet.set(CellAddr::new("B", 17), Cell::with_text("B6:B10"))?;
    sheet.set(CellAddr::new("C", 17), Cell::with_text("=MEDIAN(B6:B10)"))?;
    sheet.set(
        CellAddr::new("D", 17),
        Cell::with_formula("=MEDIAN(B6:B10)"),
    )?;
    sheet.set(CellAddr::new("E", 17), Cell::with_text("Middle value"))?;

    // Recalculate
    let changed_cells: Vec<CellAddr> = (6..=10).map(|row| CellAddr::new("B", row)).collect();
    sheet.recalculate(&changed_cells)?;

    Ok(())
}

fn create_logic_sheet(workbook: &mut Workbook) -> Result<(), Box<dyn std::error::Error>> {
    let sheet = workbook.get_sheet_mut(2).ok_or("Sheet 2 not found")?;

    // Title
    sheet.set(
        CellAddr::new("A", 1),
        Cell::with_text("LOGIC & COMPARISON FUNCTIONS"),
    )?;

    // Headers
    sheet.set(CellAddr::new("A", 3), Cell::with_text("Function"))?;
    sheet.set(CellAddr::new("B", 3), Cell::with_text("Input"))?;
    sheet.set(CellAddr::new("C", 3), Cell::with_text("Formula"))?;
    sheet.set(CellAddr::new("D", 3), Cell::with_text("Result"))?;
    sheet.set(CellAddr::new("E", 3), Cell::with_text("Description"))?;

    // Test values
    sheet.set(CellAddr::new("A", 5), Cell::with_text("Test Values:"))?;
    sheet.set(
        CellAddr::new("B", 5),
        Cell::new(100.0, Unit::simple("USD", BaseDimension::Currency)),
    )?;
    sheet.set(
        CellAddr::new("C", 5),
        Cell::new(75.0, Unit::simple("USD", BaseDimension::Currency)),
    )?;
    sheet.set(
        CellAddr::new("D", 5),
        Cell::new(150.0, Unit::simple("USD", BaseDimension::Currency)),
    )?;

    // GT (Greater Than)
    sheet.set(CellAddr::new("A", 7), Cell::with_text("GT"))?;
    sheet.set(CellAddr::new("B", 7), Cell::with_text("B5 vs C5"))?;
    sheet.set(CellAddr::new("C", 7), Cell::with_text("=GT(B5, C5)"))?;
    sheet.set(CellAddr::new("D", 7), Cell::with_formula("=GT(B5, C5)"))?;
    sheet.set(
        CellAddr::new("E", 7),
        Cell::with_text("Greater than: 100 > 75"),
    )?;

    // LT (Less Than)
    sheet.set(CellAddr::new("A", 8), Cell::with_text("LT"))?;
    sheet.set(CellAddr::new("B", 8), Cell::with_text("B5 vs D5"))?;
    sheet.set(CellAddr::new("C", 8), Cell::with_text("=LT(B5, D5)"))?;
    sheet.set(CellAddr::new("D", 8), Cell::with_formula("=LT(B5, D5)"))?;
    sheet.set(
        CellAddr::new("E", 8),
        Cell::with_text("Less than: 100 < 150"),
    )?;

    // EQ (Equal)
    sheet.set(CellAddr::new("A", 9), Cell::with_text("EQ"))?;
    sheet.set(CellAddr::new("B", 9), Cell::with_text("B5 vs B5"))?;
    sheet.set(CellAddr::new("C", 9), Cell::with_text("=EQ(B5, B5)"))?;
    sheet.set(CellAddr::new("D", 9), Cell::with_formula("=EQ(B5, B5)"))?;
    sheet.set(CellAddr::new("E", 9), Cell::with_text("Equal: 100 = 100"))?;

    // IF function
    sheet.set(CellAddr::new("A", 11), Cell::with_text("IF"))?;
    sheet.set(CellAddr::new("B", 11), Cell::with_text("Conditional"))?;
    sheet.set(
        CellAddr::new("C", 11),
        Cell::with_text("=IF(GT(B5, C5), 1, 0)"),
    )?;
    sheet.set(
        CellAddr::new("D", 11),
        Cell::with_formula("=IF(GT(B5, C5), 1, 0)"),
    )?;
    sheet.set(
        CellAddr::new("E", 11),
        Cell::with_text("If 100>75 then 1 else 0"),
    )?;

    // AND function
    sheet.set(CellAddr::new("A", 12), Cell::with_text("AND"))?;
    sheet.set(CellAddr::new("B", 12), Cell::with_text("Both true"))?;
    sheet.set(
        CellAddr::new("C", 12),
        Cell::with_text("=AND(GT(B5, C5), LT(B5, D5))"),
    )?;
    sheet.set(
        CellAddr::new("D", 12),
        Cell::with_formula("=AND(GT(B5, C5), LT(B5, D5))"),
    )?;
    sheet.set(
        CellAddr::new("E", 12),
        Cell::with_text("100>75 AND 100<150"),
    )?;

    // OR function
    sheet.set(CellAddr::new("A", 13), Cell::with_text("OR"))?;
    sheet.set(CellAddr::new("B", 13), Cell::with_text("Either true"))?;
    sheet.set(
        CellAddr::new("C", 13),
        Cell::with_text("=OR(GT(B5, D5), LT(B5, D5))"),
    )?;
    sheet.set(
        CellAddr::new("D", 13),
        Cell::with_formula("=OR(GT(B5, D5), LT(B5, D5))"),
    )?;
    sheet.set(
        CellAddr::new("E", 13),
        Cell::with_text("100>150 OR 100<150"),
    )?;

    // NOT function
    sheet.set(CellAddr::new("A", 14), Cell::with_text("NOT"))?;
    sheet.set(CellAddr::new("B", 14), Cell::with_text("Negation"))?;
    sheet.set(CellAddr::new("C", 14), Cell::with_text("=NOT(GT(B5, D5))"))?;
    sheet.set(
        CellAddr::new("D", 14),
        Cell::with_formula("=NOT(GT(B5, D5))"),
    )?;
    sheet.set(
        CellAddr::new("E", 14),
        Cell::with_text("NOT(100>150) = true"),
    )?;

    // Recalculate
    let changed_cells: Vec<CellAddr> = vec![
        CellAddr::new("B", 5),
        CellAddr::new("C", 5),
        CellAddr::new("D", 5),
    ];
    sheet.recalculate(&changed_cells)?;

    Ok(())
}

fn create_examples_sheet(workbook: &mut Workbook) -> Result<(), Box<dyn std::error::Error>> {
    let sheet = workbook.get_sheet_mut(3).ok_or("Sheet 3 not found")?;

    // Title
    sheet.set(
        CellAddr::new("A", 1),
        Cell::with_text("REAL-WORLD EXAMPLES"),
    )?;

    // Example 1: Sales Data Analysis
    sheet.set(
        CellAddr::new("A", 3),
        Cell::with_text("Example 1: Sales Data Analysis"),
    )?;
    sheet.set(CellAddr::new("A", 4), Cell::with_text("January:"))?;
    sheet.set(
        CellAddr::new("B", 4),
        Cell::new(1500.0, Unit::simple("USD", BaseDimension::Currency)),
    )?;
    sheet.set(CellAddr::new("A", 5), Cell::with_text("February:"))?;
    sheet.set(
        CellAddr::new("B", 5),
        Cell::new(2200.0, Unit::simple("USD", BaseDimension::Currency)),
    )?;
    sheet.set(CellAddr::new("A", 6), Cell::with_text("March:"))?;
    sheet.set(
        CellAddr::new("B", 6),
        Cell::new(1800.0, Unit::simple("USD", BaseDimension::Currency)),
    )?;
    sheet.set(CellAddr::new("A", 7), Cell::with_text("Total Sales:"))?;
    sheet.set(CellAddr::new("B", 7), Cell::with_formula("=SUM(B4:B6)"))?;
    sheet.set(CellAddr::new("C", 7), Cell::with_text("=SUM(B4:B6)"))?;
    sheet.set(CellAddr::new("A", 8), Cell::with_text("Average:"))?;
    sheet.set(CellAddr::new("B", 8), Cell::with_formula("=AVERAGE(B4:B6)"))?;
    sheet.set(CellAddr::new("C", 8), Cell::with_text("=AVERAGE(B4:B6)"))?;

    // Example 2: Price Calculation with Discount
    sheet.set(
        CellAddr::new("A", 10),
        Cell::with_text("Example 2: Price with Discount"),
    )?;
    sheet.set(CellAddr::new("A", 11), Cell::with_text("Original Price:"))?;
    sheet.set(
        CellAddr::new("B", 11),
        Cell::new(150.0, Unit::simple("USD", BaseDimension::Currency)),
    )?;
    sheet.set(CellAddr::new("A", 12), Cell::with_text("Discount (20%):"))?;
    sheet.set(
        CellAddr::new("B", 12),
        Cell::new(0.20, Unit::dimensionless()),
    )?;
    sheet.set(CellAddr::new("A", 13), Cell::with_text("Final Price:"))?;
    sheet.set(
        CellAddr::new("B", 13),
        Cell::with_formula("=B11 - (B11 * B12)"),
    )?;
    sheet.set(
        CellAddr::new("C", 13),
        Cell::with_text("=B11 - (B11 * B12)"),
    )?;

    // Example 3: Temperature Rounding
    sheet.set(
        CellAddr::new("A", 15),
        Cell::with_text("Example 3: Temperature Rounding"),
    )?;
    sheet.set(CellAddr::new("A", 16), Cell::with_text("Raw Temp:"))?;
    sheet.set(
        CellAddr::new("B", 16),
        Cell::new(23.456789, Unit::simple("degC", BaseDimension::Temperature)),
    )?;
    sheet.set(
        CellAddr::new("A", 17),
        Cell::with_text("Rounded (2 decimals):"),
    )?;
    sheet.set(CellAddr::new("B", 17), Cell::with_formula("=ROUND(B16, 2)"))?;
    sheet.set(CellAddr::new("C", 17), Cell::with_text("=ROUND(B16, 2)"))?;
    sheet.set(CellAddr::new("A", 18), Cell::with_text("Truncated:"))?;
    sheet.set(CellAddr::new("B", 18), Cell::with_formula("=TRUNC(B16)"))?;
    sheet.set(CellAddr::new("C", 18), Cell::with_text("=TRUNC(B16)"))?;

    // Example 4: Pythagorean Theorem
    sheet.set(
        CellAddr::new("A", 20),
        Cell::with_text("Example 4: Pythagorean Theorem"),
    )?;
    sheet.set(CellAddr::new("A", 21), Cell::with_text("Side A:"))?;
    sheet.set(
        CellAddr::new("B", 21),
        Cell::new(3.0, Unit::simple("m", BaseDimension::Length)),
    )?;
    sheet.set(CellAddr::new("A", 22), Cell::with_text("Side B:"))?;
    sheet.set(
        CellAddr::new("B", 22),
        Cell::new(4.0, Unit::simple("m", BaseDimension::Length)),
    )?;
    sheet.set(CellAddr::new("A", 23), Cell::with_text("Hypotenuse:"))?;
    sheet.set(
        CellAddr::new("B", 23),
        Cell::with_formula("=SQRT(POWER(B21, 2) + POWER(B22, 2))"),
    )?;
    sheet.set(
        CellAddr::new("C", 23),
        Cell::with_text("=SQRT(POWER(B21, 2) + POWER(B22, 2))"),
    )?;

    // Example 5: Conditional Pricing
    sheet.set(
        CellAddr::new("A", 25),
        Cell::with_text("Example 5: Bulk Discount Logic"),
    )?;
    sheet.set(CellAddr::new("A", 26), Cell::with_text("Quantity:"))?;
    sheet.set(
        CellAddr::new("B", 26),
        Cell::new(150.0, Unit::dimensionless()),
    )?;
    sheet.set(CellAddr::new("A", 27), Cell::with_text("Unit Price:"))?;
    sheet.set(
        CellAddr::new("B", 27),
        Cell::new(10.0, Unit::simple("USD", BaseDimension::Currency)),
    )?;
    sheet.set(
        CellAddr::new("A", 28),
        Cell::with_text("Gets Discount (>100):"),
    )?;
    sheet.set(
        CellAddr::new("B", 28),
        Cell::with_formula("=IF(GT(B26, 100), 1, 0)"),
    )?;
    sheet.set(
        CellAddr::new("C", 28),
        Cell::with_text("=IF(GT(B26, 100), 1, 0)"),
    )?;

    // Recalculate
    let changed_cells: Vec<CellAddr> = vec![
        CellAddr::new("B", 4),
        CellAddr::new("B", 5),
        CellAddr::new("B", 6),
        CellAddr::new("B", 11),
        CellAddr::new("B", 12),
        CellAddr::new("B", 16),
        CellAddr::new("B", 21),
        CellAddr::new("B", 22),
        CellAddr::new("B", 26),
        CellAddr::new("B", 27),
    ];
    sheet.recalculate(&changed_cells)?;

    Ok(())
}
