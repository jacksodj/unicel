// AWS Cost Estimator Example
//
// This example demonstrates:
// - EC2/RDS instance pricing with compound units (USD/hr)
// - Data transfer calculations (GB/mo × USD/GB)
// - Multi-region cost comparison
// - Scaling scenario projections
// - Complex unit cancellation

use unicel_lib::core::cell::Cell;
use unicel_lib::core::table::CellAddr;
use unicel_lib::core::units::{BaseDimension, Unit};
use unicel_lib::core::workbook::Workbook;
use unicel_lib::formats::json::WorkbookFile;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut workbook = Workbook::new("AWS Cost Estimator");

    // Sheet 1: US East (Virginia)
    let sheet1 = workbook.active_sheet_mut();
    sheet1.set_name("US East");

    // Header
    sheet1.set(CellAddr::new("A", 1), Cell::with_text("AWS Cost Estimate - US East (Virginia)"))?;

    // EC2 Section
    sheet1.set(CellAddr::new("A", 3), Cell::with_text("EC2 Instances"))?;
    sheet1.set(CellAddr::new("A", 4), Cell::with_text("Instance Type"))?;
    sheet1.set(CellAddr::new("B", 4), Cell::with_text("Quantity"))?;
    sheet1.set(CellAddr::new("C", 4), Cell::with_text("Cost per Hour"))?;
    sheet1.set(CellAddr::new("D", 4), Cell::with_text("Hours/Month"))?;
    sheet1.set(CellAddr::new("E", 4), Cell::with_text("Monthly Cost"))?;

    // t3.medium instances
    sheet1.set(CellAddr::new("A", 5), Cell::with_text("t3.medium"))?;
    sheet1.set(CellAddr::new("B", 5), Cell::new(3.0, Unit::dimensionless()))?;
    sheet1.set(CellAddr::new("C", 5), Cell::new(
        0.0416,
        Unit::compound("USD/hr", vec![(BaseDimension::Currency, 1)], vec![(BaseDimension::Time, 1)])
    ))?;
    sheet1.set(CellAddr::new("D", 5), Cell::new(730.0, Unit::simple("hr", BaseDimension::Time)))?;
    sheet1.set(CellAddr::new("E", 5), Cell::with_formula("=B5 * C5 * D5"))?;

    // m5.large instances
    sheet1.set(CellAddr::new("A", 6), Cell::with_text("m5.large"))?;
    sheet1.set(CellAddr::new("B", 6), Cell::new(2.0, Unit::dimensionless()))?;
    sheet1.set(CellAddr::new("C", 6), Cell::new(
        0.096,
        Unit::compound("USD/hr", vec![(BaseDimension::Currency, 1)], vec![(BaseDimension::Time, 1)])
    ))?;
    sheet1.set(CellAddr::new("D", 6), Cell::new(730.0, Unit::simple("hr", BaseDimension::Time)))?;
    sheet1.set(CellAddr::new("E", 6), Cell::with_formula("=B6 * C6 * D6"))?;

    // c5.xlarge instances
    sheet1.set(CellAddr::new("A", 7), Cell::with_text("c5.xlarge"))?;
    sheet1.set(CellAddr::new("B", 7), Cell::new(1.0, Unit::dimensionless()))?;
    sheet1.set(CellAddr::new("C", 7), Cell::new(
        0.17,
        Unit::compound("USD/hr", vec![(BaseDimension::Currency, 1)], vec![(BaseDimension::Time, 1)])
    ))?;
    sheet1.set(CellAddr::new("D", 7), Cell::new(730.0, Unit::simple("hr", BaseDimension::Time)))?;
    sheet1.set(CellAddr::new("E", 7), Cell::with_formula("=B7 * C7 * D7"))?;

    // EC2 Subtotal
    sheet1.set(CellAddr::new("A", 8), Cell::with_text("EC2 Subtotal"))?;
    sheet1.set(CellAddr::new("E", 8), Cell::with_formula("=SUM(E5:E7)"))?;

    // RDS Section
    sheet1.set(CellAddr::new("A", 10), Cell::with_text("RDS Databases"))?;
    sheet1.set(CellAddr::new("A", 11), Cell::with_text("Instance Type"))?;
    sheet1.set(CellAddr::new("B", 11), Cell::with_text("Quantity"))?;
    sheet1.set(CellAddr::new("C", 11), Cell::with_text("Cost per Hour"))?;
    sheet1.set(CellAddr::new("D", 11), Cell::with_text("Hours/Month"))?;
    sheet1.set(CellAddr::new("E", 11), Cell::with_text("Monthly Cost"))?;

    // db.t3.medium
    sheet1.set(CellAddr::new("A", 12), Cell::with_text("db.t3.medium"))?;
    sheet1.set(CellAddr::new("B", 12), Cell::new(1.0, Unit::dimensionless()))?;
    sheet1.set(CellAddr::new("C", 12), Cell::new(
        0.068,
        Unit::compound("USD/hr", vec![(BaseDimension::Currency, 1)], vec![(BaseDimension::Time, 1)])
    ))?;
    sheet1.set(CellAddr::new("D", 12), Cell::new(730.0, Unit::simple("hr", BaseDimension::Time)))?;
    sheet1.set(CellAddr::new("E", 12), Cell::with_formula("=B12 * C12 * D12"))?;

    // db.m5.large
    sheet1.set(CellAddr::new("A", 13), Cell::with_text("db.m5.large"))?;
    sheet1.set(CellAddr::new("B", 13), Cell::new(1.0, Unit::dimensionless()))?;
    sheet1.set(CellAddr::new("C", 13), Cell::new(
        0.192,
        Unit::compound("USD/hr", vec![(BaseDimension::Currency, 1)], vec![(BaseDimension::Time, 1)])
    ))?;
    sheet1.set(CellAddr::new("D", 13), Cell::new(730.0, Unit::simple("hr", BaseDimension::Time)))?;
    sheet1.set(CellAddr::new("E", 13), Cell::with_formula("=B13 * C13 * D13"))?;

    // RDS Subtotal
    sheet1.set(CellAddr::new("A", 14), Cell::with_text("RDS Subtotal"))?;
    sheet1.set(CellAddr::new("E", 14), Cell::with_formula("=SUM(E12:E13)"))?;

    // Data Transfer Section
    sheet1.set(CellAddr::new("A", 16), Cell::with_text("Data Transfer"))?;
    sheet1.set(CellAddr::new("A", 17), Cell::with_text("Type"))?;
    sheet1.set(CellAddr::new("B", 17), Cell::with_text("GB per Month"))?;
    sheet1.set(CellAddr::new("C", 17), Cell::with_text("Cost per GB"))?;
    sheet1.set(CellAddr::new("D", 17), Cell::with_text("Monthly Cost"))?;

    // Outbound to Internet
    sheet1.set(CellAddr::new("A", 18), Cell::with_text("Outbound to Internet"))?;
    sheet1.set(CellAddr::new("B", 18), Cell::new(5000.0, Unit::simple("GB", BaseDimension::DigitalStorage)))?;
    sheet1.set(CellAddr::new("C", 18), Cell::new(
        0.09,
        Unit::compound("USD/GB", vec![(BaseDimension::Currency, 1)], vec![(BaseDimension::DigitalStorage, 1)])
    ))?;
    sheet1.set(CellAddr::new("D", 18), Cell::with_formula("=B18 * C18"))?;

    // Inter-region transfer
    sheet1.set(CellAddr::new("A", 19), Cell::with_text("Inter-region Transfer"))?;
    sheet1.set(CellAddr::new("B", 19), Cell::new(2000.0, Unit::simple("GB", BaseDimension::DigitalStorage)))?;
    sheet1.set(CellAddr::new("C", 19), Cell::new(
        0.02,
        Unit::compound("USD/GB", vec![(BaseDimension::Currency, 1)], vec![(BaseDimension::DigitalStorage, 1)])
    ))?;
    sheet1.set(CellAddr::new("D", 19), Cell::with_formula("=B19 * C19"))?;

    // Data Transfer Subtotal
    sheet1.set(CellAddr::new("A", 20), Cell::with_text("Data Transfer Subtotal"))?;
    sheet1.set(CellAddr::new("D", 20), Cell::with_formula("=SUM(D18:D19)"))?;

    // S3 Storage
    sheet1.set(CellAddr::new("A", 22), Cell::with_text("S3 Storage"))?;
    sheet1.set(CellAddr::new("A", 23), Cell::with_text("Storage GB"))?;
    sheet1.set(CellAddr::new("B", 23), Cell::with_text("Cost per GB/month"))?;
    sheet1.set(CellAddr::new("C", 23), Cell::with_text("Monthly Cost"))?;

    sheet1.set(CellAddr::new("A", 24), Cell::with_text("Standard Storage"))?;
    sheet1.set(CellAddr::new("A", 24), Cell::new(10000.0, Unit::simple("GB", BaseDimension::DigitalStorage)))?;
    sheet1.set(CellAddr::new("B", 24), Cell::new(
        0.023,
        Unit::compound("USD/GB", vec![(BaseDimension::Currency, 1)], vec![(BaseDimension::DigitalStorage, 1)])
    ))?;
    sheet1.set(CellAddr::new("C", 24), Cell::with_formula("=A24 * B24"))?;

    // Monthly Total
    sheet1.set(CellAddr::new("A", 26), Cell::with_text("MONTHLY TOTAL (US East)"))?;
    sheet1.set(CellAddr::new("E", 26), Cell::with_formula("=E8 + E14 + D20 + C24"))?;

    // Recalculate Sheet 1 - list input cells only, dependency graph will handle formulas
    let changed1: Vec<CellAddr> = vec![
        // EC2 instances (input cells only)
        CellAddr::new("B", 5), CellAddr::new("C", 5), CellAddr::new("D", 5),
        CellAddr::new("B", 6), CellAddr::new("C", 6), CellAddr::new("D", 6),
        CellAddr::new("B", 7), CellAddr::new("C", 7), CellAddr::new("D", 7),
        // RDS instances
        CellAddr::new("B", 12), CellAddr::new("C", 12), CellAddr::new("D", 12),
        CellAddr::new("B", 13), CellAddr::new("C", 13), CellAddr::new("D", 13),
        // Data transfer
        CellAddr::new("B", 18), CellAddr::new("C", 18),
        CellAddr::new("B", 19), CellAddr::new("C", 19),
        // S3 storage
        CellAddr::new("A", 24), CellAddr::new("B", 24),
    ];
    sheet1.recalculate(&changed1)?;

    // Sheet 2: EU West (Ireland) - Similar structure with EUR pricing
    let sheet2_idx = workbook.add_sheet_with_name("EU West");
    let sheet2 = workbook.get_sheet_mut(sheet2_idx).unwrap();

    sheet2.set(CellAddr::new("A", 1), Cell::with_text("AWS Cost Estimate - EU West (Ireland)"))?;

    // EC2 Section
    sheet2.set(CellAddr::new("A", 3), Cell::with_text("EC2 Instances"))?;
    sheet2.set(CellAddr::new("A", 4), Cell::with_text("Instance Type"))?;
    sheet2.set(CellAddr::new("B", 4), Cell::with_text("Quantity"))?;
    sheet2.set(CellAddr::new("C", 4), Cell::with_text("Cost per Hour"))?;
    sheet2.set(CellAddr::new("D", 4), Cell::with_text("Hours/Month"))?;
    sheet2.set(CellAddr::new("E", 4), Cell::with_text("Monthly Cost"))?;

    // t3.medium (EUR pricing slightly higher)
    sheet2.set(CellAddr::new("A", 5), Cell::with_text("t3.medium"))?;
    sheet2.set(CellAddr::new("B", 5), Cell::new(2.0, Unit::dimensionless()))?;
    sheet2.set(CellAddr::new("C", 5), Cell::new(
        0.046,
        Unit::compound("EUR/hr", vec![(BaseDimension::Currency, 1)], vec![(BaseDimension::Time, 1)])
    ))?;
    sheet2.set(CellAddr::new("D", 5), Cell::new(730.0, Unit::simple("hr", BaseDimension::Time)))?;
    sheet2.set(CellAddr::new("E", 5), Cell::with_formula("=B5 * C5 * D5"))?;

    // m5.large
    sheet2.set(CellAddr::new("A", 6), Cell::with_text("m5.large"))?;
    sheet2.set(CellAddr::new("B", 6), Cell::new(2.0, Unit::dimensionless()))?;
    sheet2.set(CellAddr::new("C", 6), Cell::new(
        0.107,
        Unit::compound("EUR/hr", vec![(BaseDimension::Currency, 1)], vec![(BaseDimension::Time, 1)])
    ))?;
    sheet2.set(CellAddr::new("D", 6), Cell::new(730.0, Unit::simple("hr", BaseDimension::Time)))?;
    sheet2.set(CellAddr::new("E", 6), Cell::with_formula("=B6 * C6 * D6"))?;

    // EC2 Subtotal
    sheet2.set(CellAddr::new("A", 7), Cell::with_text("EC2 Subtotal"))?;
    sheet2.set(CellAddr::new("E", 7), Cell::with_formula("=SUM(E5:E6)"))?;

    // Data Transfer
    sheet2.set(CellAddr::new("A", 9), Cell::with_text("Data Transfer"))?;
    sheet2.set(CellAddr::new("A", 10), Cell::with_text("Outbound to Internet"))?;
    sheet2.set(CellAddr::new("B", 10), Cell::new(3000.0, Unit::simple("GB", BaseDimension::DigitalStorage)))?;
    sheet2.set(CellAddr::new("C", 10), Cell::new(
        0.085,
        Unit::compound("EUR/GB", vec![(BaseDimension::Currency, 1)], vec![(BaseDimension::DigitalStorage, 1)])
    ))?;
    sheet2.set(CellAddr::new("D", 10), Cell::with_formula("=B10 * C10"))?;

    // Monthly Total EU
    sheet2.set(CellAddr::new("A", 12), Cell::with_text("MONTHLY TOTAL (EU West)"))?;
    sheet2.set(CellAddr::new("E", 12), Cell::with_formula("=E7 + D10"))?;

    // Recalculate Sheet 2 - list input cells only
    let changed2: Vec<CellAddr> = vec![
        // EC2 instances (input cells only)
        CellAddr::new("B", 5), CellAddr::new("C", 5), CellAddr::new("D", 5),
        CellAddr::new("B", 6), CellAddr::new("C", 6), CellAddr::new("D", 6),
        // Data transfer
        CellAddr::new("B", 10), CellAddr::new("C", 10),
    ];
    sheet2.recalculate(&changed2)?;

    // Sheet 3: Scaling Scenario
    let sheet3_idx = workbook.add_sheet_with_name("Scaling Scenario");
    let sheet3 = workbook.get_sheet_mut(sheet3_idx).unwrap();

    sheet3.set(CellAddr::new("A", 1), Cell::with_text("Scaling Scenario - 2x Traffic Growth"))?;

    sheet3.set(CellAddr::new("A", 3), Cell::with_text("Current Monthly Cost"))?;
    sheet3.set(CellAddr::new("B", 3), Cell::new(2000.0, Unit::simple("USD", BaseDimension::Currency)))?;

    sheet3.set(CellAddr::new("A", 4), Cell::with_text("Growth Multiplier"))?;
    sheet3.set(CellAddr::new("B", 4), Cell::new(2.0, Unit::dimensionless()))?;

    sheet3.set(CellAddr::new("A", 5), Cell::with_text("Projected Monthly Cost"))?;
    sheet3.set(CellAddr::new("B", 5), Cell::with_formula("=B3 * B4"))?;

    sheet3.set(CellAddr::new("A", 7), Cell::with_text("Annual Projection"))?;
    sheet3.set(CellAddr::new("B", 7), Cell::with_formula("=B5 * 12"))?;

    sheet3.set(CellAddr::new("A", 9), Cell::with_text("NOTE: This demonstrates:"))?;
    sheet3.set(CellAddr::new("A", 10), Cell::with_text("- Compound units (USD/hr × hr = USD)"))?;
    sheet3.set(CellAddr::new("A", 11), Cell::with_text("- Data transfer costs (GB × USD/GB = USD)"))?;
    sheet3.set(CellAddr::new("A", 12), Cell::with_text("- Multi-currency scenarios (USD vs EUR)"))?;
    sheet3.set(CellAddr::new("A", 13), Cell::with_text("- Scaling projections with dimensionless multipliers"))?;

    // Recalculate Sheet 3 - input cells only
    sheet3.recalculate(&[
        CellAddr::new("B", 3),
        CellAddr::new("B", 4),
    ])?;

    // Save to file
    let file = WorkbookFile::from_workbook(&workbook);
    let path = PathBuf::from("examples/aws_cost_estimator.usheet");
    file.save_to_file(&path)?;

    println!("✓ Created aws_cost_estimator.usheet");
    println!("  - EC2/RDS pricing with compound units (USD/hr × hr = USD)");
    println!("  - Data transfer calculations (GB × USD/GB = USD)");
    println!("  - Multi-region comparison (USD vs EUR)");
    println!("  - Scaling scenario projections");
    println!("  - 3 sheets: US East, EU West, Scaling Scenario");

    Ok(())
}
