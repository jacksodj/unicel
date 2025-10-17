// Test percentage unit handling in formulas

use unicel_lib::core::cell::Cell;
use unicel_lib::core::table::{CellAddr, Sheet};
use unicel_lib::core::units::{BaseDimension, Unit};

#[test]
fn test_percentage_multiplication() {
    let mut sheet = Sheet::new();

    // Set up cells:
    // B2 = 250000 $/yr
    let b2 = CellAddr::new("B", 2);
    sheet
        .set(
            b2.clone(),
            Cell::new(
                250000.0,
                Unit::compound(
                    "$/yr",
                    vec![(BaseDimension::Currency, 1)],
                    vec![(BaseDimension::Custom("yr".to_string()), 1)],
                ),
            ),
        )
        .unwrap();

    // C7 = 10% (stored as 0.1 with % unit)
    let c7 = CellAddr::new("C", 7);
    sheet
        .set(
            c7.clone(),
            Cell::new(
                0.1,
                Unit::simple("%", BaseDimension::Custom("%".to_string())),
            ),
        )
        .unwrap();

    // D8 = =B2*C7
    let d8 = CellAddr::new("D", 8);
    sheet
        .set(d8.clone(), Cell::with_formula("=B2*C7".to_string()))
        .unwrap();

    // Recalculate
    sheet.recalculate(&[d8.clone()]).unwrap();

    // Check result
    let result = sheet.get(&d8).unwrap();

    // Expected: 25000 $/yr (percentage unit should be removed)
    assert_eq!(result.as_number(), Some(25000.0), "Value should be 25000");
    assert_eq!(
        result.storage_unit().canonical(),
        "$/yr",
        "Unit should be $/yr (percentage removed)"
    );
}

#[test]
fn test_percentage_division() {
    let mut sheet = Sheet::new();

    // A1 = 100 USD
    let a1 = CellAddr::new("A", 1);
    sheet
        .set(
            a1.clone(),
            Cell::new(100.0, Unit::simple("USD", BaseDimension::Currency)),
        )
        .unwrap();

    // B1 = 20% (stored as 0.2 with % unit)
    let b1 = CellAddr::new("B", 1);
    sheet
        .set(
            b1.clone(),
            Cell::new(
                0.2,
                Unit::simple("%", BaseDimension::Custom("%".to_string())),
            ),
        )
        .unwrap();

    // C1 = =A1/B1 (100 USD / 20% = 500 USD)
    let c1 = CellAddr::new("C", 1);
    sheet
        .set(c1.clone(), Cell::with_formula("=A1/B1".to_string()))
        .unwrap();

    // Recalculate
    sheet.recalculate(&[c1.clone()]).unwrap();

    // Check result
    let result = sheet.get(&c1).unwrap();

    // Expected: 500 USD (percentage unit should be removed)
    assert_eq!(result.as_number(), Some(500.0), "Value should be 500");
    assert_eq!(
        result.storage_unit().canonical(),
        "USD",
        "Unit should be USD (percentage removed)"
    );
}

#[test]
fn test_is_percentage_unit() {
    let percent_unit = Unit::simple("%", BaseDimension::Custom("%".to_string()));
    let dollar_unit = Unit::simple("$", BaseDimension::Currency);

    assert_eq!(percent_unit.canonical(), "%");
    assert_ne!(dollar_unit.canonical(), "%");
}
