// Test unit cancellation with user-style input parsing
use unicel_lib::core::cell::Cell;
use unicel_lib::core::table::{CellAddr, Sheet};

// Import the parse functions from workbook module
// We'll simulate what happens when a user enters "0.0416 USD/hr" and "730 hr/month"

fn main() {
    println!("Testing user input: USD/hr × hr/month...\n");

    let mut sheet = Sheet::new();

    // Parse "0.0416 USD/hr" like the UI would
    use unicel_lib::core::units::{BaseDimension, Unit};

    // Simulate parse_unit("USD/hr")
    let usd_per_hr = Unit::compound(
        "USD/hr",
        vec![(BaseDimension::Currency, 1)],
        vec![(BaseDimension::Time, 1)],
    );

    sheet
        .set(CellAddr::new("C", 5), Cell::new(0.0416, usd_per_hr))
        .unwrap();

    // Simulate parse_unit("hr/month") with the NEW fix
    // month should be Custom, not Time
    let hr_per_month = Unit::compound(
        "hr/month",
        vec![(BaseDimension::Time, 1)],
        vec![(BaseDimension::Custom("month".to_string()), 1)],
    );

    sheet
        .set(CellAddr::new("D", 5), Cell::new(730.0, hr_per_month))
        .unwrap();

    sheet
        .set(CellAddr::new("B", 5), Cell::new(3.0, Unit::dimensionless()))
        .unwrap();

    println!("Cells:");
    println!("  B5: 3 (dimensionless)");
    println!("  C5: 0.0416 USD/hr");
    println!("  D5: 730 hr/month");
    println!("\nFormula: =B5 * C5 * D5");
    println!("Expected: 3 × 0.0416 USD/hr × 730 hr/month");
    println!("  = 91.104 USD/month (or just USD)\n");

    // Test the multiplication
    match sheet.evaluate_formula("=B5 * C5 * D5") {
        Ok((value, unit)) => {
            println!("Result:");
            println!("  Value: {}", value);
            println!("  Unit: {}\n", unit.canonical());

            if unit.canonical().contains("hr") && !unit.canonical().contains("month") {
                println!("  ❌ FAILED! Unit still contains 'hr' without 'month' - cancellation didn't work properly");
                println!("  Expected: USD/month or USD");
            } else if unit.canonical() == "USD/month" || unit.canonical() == "USD" {
                println!("  ✅ SUCCESS! Units cancelled correctly");
                println!("  hr in USD/hr cancelled with hr in hr/month");
            } else {
                println!("  Result unit: {}", unit.canonical());
            }
        }
        Err(e) => {
            println!("  ❌ FAILED! Evaluation error: {}", e);
        }
    }
}
