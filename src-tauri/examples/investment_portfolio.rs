// Investment Portfolio Tracker Example
//
// This example demonstrates:
// - Stock positions with shares (dimensionless) and prices
// - Multi-currency holdings (USD, EUR, GBP)
// - Return calculations with proper unit handling
// - Asset allocation summary
// - Percentage calculations

use unicel_lib::core::cell::Cell;
use unicel_lib::core::table::CellAddr;
use unicel_lib::core::units::{BaseDimension, Unit};
use unicel_lib::core::workbook::Workbook;
use unicel_lib::formats::json::WorkbookFile;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut workbook = Workbook::new("Investment Portfolio Tracker");

    // Sheet 1: US Stocks
    let sheet1 = workbook.active_sheet_mut();
    sheet1.set_name("US Stocks");

    // Header
    sheet1.set(CellAddr::new("A", 1), Cell::with_text("US Stock Holdings"))?;

    sheet1.set(CellAddr::new("A", 3), Cell::with_text("Symbol"))?;
    sheet1.set(CellAddr::new("B", 3), Cell::with_text("Shares"))?;
    sheet1.set(CellAddr::new("C", 3), Cell::with_text("Cost Basis"))?;
    sheet1.set(CellAddr::new("D", 3), Cell::with_text("Current Price"))?;
    sheet1.set(CellAddr::new("E", 3), Cell::with_text("Total Cost"))?;
    sheet1.set(CellAddr::new("F", 3), Cell::with_text("Current Value"))?;
    sheet1.set(CellAddr::new("G", 3), Cell::with_text("Gain/Loss"))?;

    // AAPL
    sheet1.set(CellAddr::new("A", 4), Cell::with_text("AAPL"))?;
    sheet1.set(CellAddr::new("B", 4), Cell::new(100.0, Unit::dimensionless()))?;
    sheet1.set(CellAddr::new("C", 4), Cell::new(145.50, Unit::simple("USD", BaseDimension::Currency)))?;
    sheet1.set(CellAddr::new("D", 4), Cell::new(178.25, Unit::simple("USD", BaseDimension::Currency)))?;
    sheet1.set(CellAddr::new("E", 4), Cell::with_formula("=B4 * C4"))?;
    sheet1.set(CellAddr::new("F", 4), Cell::with_formula("=B4 * D4"))?;
    sheet1.set(CellAddr::new("G", 4), Cell::with_formula("=F4 - E4"))?;

    // MSFT
    sheet1.set(CellAddr::new("A", 5), Cell::with_text("MSFT"))?;
    sheet1.set(CellAddr::new("B", 5), Cell::new(75.0, Unit::dimensionless()))?;
    sheet1.set(CellAddr::new("C", 5), Cell::new(310.00, Unit::simple("USD", BaseDimension::Currency)))?;
    sheet1.set(CellAddr::new("D", 5), Cell::new(385.50, Unit::simple("USD", BaseDimension::Currency)))?;
    sheet1.set(CellAddr::new("E", 5), Cell::with_formula("=B5 * C5"))?;
    sheet1.set(CellAddr::new("F", 5), Cell::with_formula("=B5 * D5"))?;
    sheet1.set(CellAddr::new("G", 5), Cell::with_formula("=F5 - E5"))?;

    // GOOGL
    sheet1.set(CellAddr::new("A", 6), Cell::with_text("GOOGL"))?;
    sheet1.set(CellAddr::new("B", 6), Cell::new(50.0, Unit::dimensionless()))?;
    sheet1.set(CellAddr::new("C", 6), Cell::new(125.75, Unit::simple("USD", BaseDimension::Currency)))?;
    sheet1.set(CellAddr::new("D", 6), Cell::new(142.80, Unit::simple("USD", BaseDimension::Currency)))?;
    sheet1.set(CellAddr::new("E", 6), Cell::with_formula("=B6 * C6"))?;
    sheet1.set(CellAddr::new("F", 6), Cell::with_formula("=B6 * D6"))?;
    sheet1.set(CellAddr::new("G", 6), Cell::with_formula("=F6 - E6"))?;

    // TSLA
    sheet1.set(CellAddr::new("A", 7), Cell::with_text("TSLA"))?;
    sheet1.set(CellAddr::new("B", 7), Cell::new(30.0, Unit::dimensionless()))?;
    sheet1.set(CellAddr::new("C", 7), Cell::new(215.00, Unit::simple("USD", BaseDimension::Currency)))?;
    sheet1.set(CellAddr::new("D", 7), Cell::new(248.50, Unit::simple("USD", BaseDimension::Currency)))?;
    sheet1.set(CellAddr::new("E", 7), Cell::with_formula("=B7 * C7"))?;
    sheet1.set(CellAddr::new("F", 7), Cell::with_formula("=B7 * D7"))?;
    sheet1.set(CellAddr::new("G", 7), Cell::with_formula("=F7 - E7"))?;

    // Totals
    sheet1.set(CellAddr::new("A", 9), Cell::with_text("TOTAL US STOCKS"))?;
    sheet1.set(CellAddr::new("E", 9), Cell::with_formula("=SUM(E4:E7)"))?;
    sheet1.set(CellAddr::new("F", 9), Cell::with_formula("=SUM(F4:F7)"))?;
    sheet1.set(CellAddr::new("G", 9), Cell::with_formula("=SUM(G4:G7)"))?;

    // Return percentage
    sheet1.set(CellAddr::new("A", 11), Cell::with_text("Return %"))?;
    sheet1.set(CellAddr::new("B", 11), Cell::with_formula("=G9 / E9 * 100"))?;

    // Recalculate Sheet 1
    let changed1: Vec<CellAddr> = vec![
        CellAddr::new("B", 4), CellAddr::new("C", 4), CellAddr::new("D", 4),
        CellAddr::new("B", 5), CellAddr::new("C", 5), CellAddr::new("D", 5),
        CellAddr::new("B", 6), CellAddr::new("C", 6), CellAddr::new("D", 6),
        CellAddr::new("B", 7), CellAddr::new("C", 7), CellAddr::new("D", 7),
    ];
    sheet1.recalculate(&changed1)?;

    // Sheet 2: European Holdings (EUR)
    let sheet2_idx = workbook.add_sheet_with_name("EU Holdings");
    let sheet2 = workbook.get_sheet_mut(sheet2_idx).unwrap();

    sheet2.set(CellAddr::new("A", 1), Cell::with_text("European Stock Holdings (EUR)"))?;

    sheet2.set(CellAddr::new("A", 3), Cell::with_text("Symbol"))?;
    sheet2.set(CellAddr::new("B", 3), Cell::with_text("Shares"))?;
    sheet2.set(CellAddr::new("C", 3), Cell::with_text("Cost Basis"))?;
    sheet2.set(CellAddr::new("D", 3), Cell::with_text("Current Price"))?;
    sheet2.set(CellAddr::new("E", 3), Cell::with_text("Total Cost"))?;
    sheet2.set(CellAddr::new("F", 3), Cell::with_text("Current Value"))?;
    sheet2.set(CellAddr::new("G", 3), Cell::with_text("Gain/Loss"))?;

    // SAP
    sheet2.set(CellAddr::new("A", 4), Cell::with_text("SAP"))?;
    sheet2.set(CellAddr::new("B", 4), Cell::new(80.0, Unit::dimensionless()))?;
    sheet2.set(CellAddr::new("C", 4), Cell::new(115.50, Unit::simple("EUR", BaseDimension::Currency)))?;
    sheet2.set(CellAddr::new("D", 4), Cell::new(128.75, Unit::simple("EUR", BaseDimension::Currency)))?;
    sheet2.set(CellAddr::new("E", 4), Cell::with_formula("=B4 * C4"))?;
    sheet2.set(CellAddr::new("F", 4), Cell::with_formula("=B4 * D4"))?;
    sheet2.set(CellAddr::new("G", 4), Cell::with_formula("=F4 - E4"))?;

    // Siemens
    sheet2.set(CellAddr::new("A", 5), Cell::with_text("SIE"))?;
    sheet2.set(CellAddr::new("B", 5), Cell::new(60.0, Unit::dimensionless()))?;
    sheet2.set(CellAddr::new("C", 5), Cell::new(142.00, Unit::simple("EUR", BaseDimension::Currency)))?;
    sheet2.set(CellAddr::new("D", 5), Cell::new(168.50, Unit::simple("EUR", BaseDimension::Currency)))?;
    sheet2.set(CellAddr::new("E", 5), Cell::with_formula("=B5 * C5"))?;
    sheet2.set(CellAddr::new("F", 5), Cell::with_formula("=B5 * D5"))?;
    sheet2.set(CellAddr::new("G", 5), Cell::with_formula("=F5 - E5"))?;

    // Totals
    sheet2.set(CellAddr::new("A", 7), Cell::with_text("TOTAL EU HOLDINGS"))?;
    sheet2.set(CellAddr::new("E", 7), Cell::with_formula("=SUM(E4:E5)"))?;
    sheet2.set(CellAddr::new("F", 7), Cell::with_formula("=SUM(F4:F5)"))?;
    sheet2.set(CellAddr::new("G", 7), Cell::with_formula("=SUM(G4:G5)"))?;

    // Return percentage
    sheet2.set(CellAddr::new("A", 9), Cell::with_text("Return %"))?;
    sheet2.set(CellAddr::new("B", 9), Cell::with_formula("=G7 / E7 * 100"))?;

    // Recalculate Sheet 2
    let changed2: Vec<CellAddr> = vec![
        CellAddr::new("B", 4), CellAddr::new("C", 4), CellAddr::new("D", 4),
        CellAddr::new("B", 5), CellAddr::new("C", 5), CellAddr::new("D", 5),
    ];
    sheet2.recalculate(&changed2)?;

    // Sheet 3: UK Holdings (GBP)
    let sheet3_idx = workbook.add_sheet_with_name("UK Holdings");
    let sheet3 = workbook.get_sheet_mut(sheet3_idx).unwrap();

    sheet3.set(CellAddr::new("A", 1), Cell::with_text("UK Stock Holdings (GBP)"))?;

    sheet3.set(CellAddr::new("A", 3), Cell::with_text("Symbol"))?;
    sheet3.set(CellAddr::new("B", 3), Cell::with_text("Shares"))?;
    sheet3.set(CellAddr::new("C", 3), Cell::with_text("Cost Basis"))?;
    sheet3.set(CellAddr::new("D", 3), Cell::with_text("Current Price"))?;
    sheet3.set(CellAddr::new("E", 3), Cell::with_text("Total Cost"))?;
    sheet3.set(CellAddr::new("F", 3), Cell::with_text("Current Value"))?;
    sheet3.set(CellAddr::new("G", 3), Cell::with_text("Gain/Loss"))?;

    // HSBC
    sheet3.set(CellAddr::new("A", 4), Cell::with_text("HSBA"))?;
    sheet3.set(CellAddr::new("B", 4), Cell::new(500.0, Unit::dimensionless()))?;
    sheet3.set(CellAddr::new("C", 4), Cell::new(5.85, Unit::simple("GBP", BaseDimension::Currency)))?;
    sheet3.set(CellAddr::new("D", 4), Cell::new(6.45, Unit::simple("GBP", BaseDimension::Currency)))?;
    sheet3.set(CellAddr::new("E", 4), Cell::with_formula("=B4 * C4"))?;
    sheet3.set(CellAddr::new("F", 4), Cell::with_formula("=B4 * D4"))?;
    sheet3.set(CellAddr::new("G", 4), Cell::with_formula("=F4 - E4"))?;

    // BP
    sheet3.set(CellAddr::new("A", 5), Cell::with_text("BP"))?;
    sheet3.set(CellAddr::new("B", 5), Cell::new(400.0, Unit::dimensionless()))?;
    sheet3.set(CellAddr::new("C", 5), Cell::new(4.20, Unit::simple("GBP", BaseDimension::Currency)))?;
    sheet3.set(CellAddr::new("D", 5), Cell::new(4.75, Unit::simple("GBP", BaseDimension::Currency)))?;
    sheet3.set(CellAddr::new("E", 5), Cell::with_formula("=B5 * C5"))?;
    sheet3.set(CellAddr::new("F", 5), Cell::with_formula("=B5 * D5"))?;
    sheet3.set(CellAddr::new("G", 5), Cell::with_formula("=F5 - E5"))?;

    // Totals
    sheet3.set(CellAddr::new("A", 7), Cell::with_text("TOTAL UK HOLDINGS"))?;
    sheet3.set(CellAddr::new("E", 7), Cell::with_formula("=SUM(E4:E5)"))?;
    sheet3.set(CellAddr::new("F", 7), Cell::with_formula("=SUM(F4:F5)"))?;
    sheet3.set(CellAddr::new("G", 7), Cell::with_formula("=SUM(G4:G5)"))?;

    // Return percentage
    sheet3.set(CellAddr::new("A", 9), Cell::with_text("Return %"))?;
    sheet3.set(CellAddr::new("B", 9), Cell::with_formula("=G7 / E7 * 100"))?;

    // Recalculate Sheet 3
    let changed3: Vec<CellAddr> = vec![
        CellAddr::new("B", 4), CellAddr::new("C", 4), CellAddr::new("D", 4),
        CellAddr::new("B", 5), CellAddr::new("C", 5), CellAddr::new("D", 5),
    ];
    sheet3.recalculate(&changed3)?;

    // Sheet 4: Portfolio Summary
    let sheet4_idx = workbook.add_sheet_with_name("Summary");
    let sheet4 = workbook.get_sheet_mut(sheet4_idx).unwrap();

    sheet4.set(CellAddr::new("A", 1), Cell::with_text("Portfolio Summary"))?;

    sheet4.set(CellAddr::new("A", 3), Cell::with_text("Asset Allocation by Region"))?;
    sheet4.set(CellAddr::new("A", 5), Cell::with_text("Region"))?;
    sheet4.set(CellAddr::new("B", 5), Cell::with_text("Current Value"))?;
    sheet4.set(CellAddr::new("C", 5), Cell::with_text("Currency"))?;

    // Note: In a real implementation, we'd convert all to a single currency
    // For this demo, we'll just list them separately
    sheet4.set(CellAddr::new("A", 6), Cell::with_text("United States"))?;
    sheet4.set(CellAddr::new("B", 6), Cell::new(54825.0, Unit::simple("USD", BaseDimension::Currency)))?;
    sheet4.set(CellAddr::new("C", 6), Cell::with_text("USD"))?;

    sheet4.set(CellAddr::new("A", 7), Cell::with_text("Europe"))?;
    sheet4.set(CellAddr::new("B", 7), Cell::new(19400.0, Unit::simple("EUR", BaseDimension::Currency)))?;
    sheet4.set(CellAddr::new("C", 7), Cell::with_text("EUR"))?;

    sheet4.set(CellAddr::new("A", 8), Cell::with_text("United Kingdom"))?;
    sheet4.set(CellAddr::new("B", 8), Cell::new(5125.0, Unit::simple("GBP", BaseDimension::Currency)))?;
    sheet4.set(CellAddr::new("C", 8), Cell::with_text("GBP"))?;

    sheet4.set(CellAddr::new("A", 10), Cell::with_text("NOTES:"))?;
    sheet4.set(CellAddr::new("A", 11), Cell::with_text("- Each region tracks holdings in local currency"))?;
    sheet4.set(CellAddr::new("A", 12), Cell::with_text("- Shares (dimensionless) × Price (USD) = Value (USD)"))?;
    sheet4.set(CellAddr::new("A", 13), Cell::with_text("- Gain/Loss calculated with proper unit handling"))?;
    sheet4.set(CellAddr::new("A", 14), Cell::with_text("- Return % is dimensionless (USD/USD × 100)"))?;
    sheet4.set(CellAddr::new("A", 15), Cell::with_text("- Multi-currency demonstration (USD, EUR, GBP)"))?;

    // Save to file
    let file = WorkbookFile::from_workbook(&workbook);
    let path = PathBuf::from("examples/investment_portfolio.usheet");
    file.save_to_file(&path)?;

    println!("✓ Created investment_portfolio.usheet");
    println!("  - Stock positions with shares and cost basis");
    println!("  - Multi-currency holdings (USD, EUR, GBP)");
    println!("  - Return calculations (dimensionless shares × currency = currency)");
    println!("  - Gain/Loss tracking with proper unit handling");
    println!("  - 4 sheets: US Stocks, EU Holdings, UK Holdings, Summary");

    Ok(())
}
