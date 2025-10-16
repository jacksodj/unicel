// Test complex unit cancellation with user-entered compound units
use unicel_lib::core::cell::Cell;
use unicel_lib::core::table::{CellAddr, Sheet};
use unicel_lib::core::units::{BaseDimension, Unit};

fn main() {
    println!("Testing complex unit cancellation (USD/hr × hr/month)...\n");

    let mut sheet = Sheet::new();

    // Simulate user entering "0.0416 USD/hr"
    sheet
        .set(
            CellAddr::new("C", 5),
            Cell::new(
                0.0416,
                Unit::compound(
                    "USD/hr",
                    vec![(BaseDimension::Currency, 1)],
                    vec![(BaseDimension::Time, 1)],
                ),
            ),
        )
        .unwrap();

    // Simulate user entering "730 hr/month"
    // This should be parsed as a compound unit: hr in numerator, month in denominator
    sheet
        .set(
            CellAddr::new("D", 5),
            Cell::new(
                730.0,
                Unit::compound(
                    "hr/month",
                    vec![(BaseDimension::Time, 1)],
                    vec![(BaseDimension::Custom("month".to_string()), 1)],
                ),
            ),
        )
        .unwrap();

    println!("Cell C5: 0.0416 USD/hr");
    println!("Cell D5: 730 hr/month");
    println!("\nExpected result: USD/hr × hr/month");
    println!("  hr cancels out, leaving: USD/month\n");

    // Test multiplication
    match sheet.evaluate_formula("=C5 * D5") {
        Ok((value, unit)) => {
            println!("  Evaluated Value: {}", value);
            println!("  Evaluated Unit: {}", unit.canonical());
            println!();

            if unit.canonical().contains("hr") {
                println!("  ❌ FAILED! Unit still contains 'hr' - cancellation didn't work");
                println!("  Expected: USD/month or USD");
                println!("  Got: {}", unit.canonical());
            } else if unit.canonical() == "USD/month" || unit.canonical() == "USD" {
                println!("  ✅ SUCCESS! Units cancelled correctly");
            } else {
                println!("  ⚠️  UNEXPECTED: Got unit '{}'", unit.canonical());
            }
        }
        Err(e) => {
            println!("  ❌ FAILED! Evaluation error: {}", e);
        }
    }
}
