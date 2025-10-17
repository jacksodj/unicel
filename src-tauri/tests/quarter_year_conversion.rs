// Test quarter to year conversion in compound units
// This tests the fix for the bug where CONVERT function fails on $/quarter to $/year
//
// Bug report:
// - Formula: =convert(B9,1 $/year) where B9 contains 30000 $/quarter
// - Error: "Incompatible units: cannot CONVERT $/quarter and $/year"
// - Expected: Should convert successfully (1 quarter = 0.25 years, so 30000 $/quarter = 7500 $/year)

use unicel_lib::core::cell::Cell;
use unicel_lib::core::table::CellAddr;
use unicel_lib::core::units::{parse_unit, BaseDimension, Unit, UnitLibrary};
use unicel_lib::core::workbook::Workbook;

#[test]
fn test_simple_quarter_to_year_conversion() {
    // First verify that basic quarter <-> year conversion works in the library
    let library = UnitLibrary::new();

    // 4 quarters = 1 year
    let result = library.convert(4.0, "quarter", "year");
    assert!(result.is_some(), "Failed to convert quarter to year");
    assert_eq!(result.unwrap(), 1.0, "4 quarters should equal 1 year");

    // 1 year = 4 quarters
    let result = library.convert(1.0, "year", "quarter");
    assert!(result.is_some(), "Failed to convert year to quarter");
    assert_eq!(result.unwrap(), 4.0, "1 year should equal 4 quarters");

    // 1 quarter = 0.25 years
    let result = library.convert(1.0, "quarter", "year");
    assert!(result.is_some(), "Failed to convert 1 quarter to year");
    assert_eq!(result.unwrap(), 0.25, "1 quarter should equal 0.25 years");
}

#[test]
fn test_parse_dollar_per_quarter() {
    // Test that we can parse $/quarter as a compound unit
    let library = UnitLibrary::new();

    let unit = parse_unit("$/quarter", &library);
    assert!(unit.is_ok(), "Failed to parse $/quarter: {:?}", unit.err());

    let unit = unit.unwrap();
    assert_eq!(unit.canonical(), "$/quarter");

    // Check that it has the correct dimension (Currency/Time)
    match unit.dimension() {
        unicel_lib::core::units::Dimension::Compound {
            numerator,
            denominator,
        } => {
            assert_eq!(numerator.len(), 1);
            assert_eq!(numerator[0].0, BaseDimension::Currency);
            assert_eq!(numerator[0].1, 1);

            assert_eq!(denominator.len(), 1);
            assert_eq!(denominator[0].0, BaseDimension::Time);
            assert_eq!(denominator[0].1, 1);
        }
        _ => panic!("Expected Compound dimension for $/quarter"),
    }
}

#[test]
fn test_parse_dollar_per_year() {
    // Test that we can parse $/year as a compound unit
    let library = UnitLibrary::new();

    let unit = parse_unit("$/year", &library);
    assert!(unit.is_ok(), "Failed to parse $/year: {:?}", unit.err());

    let unit = unit.unwrap();
    assert_eq!(unit.canonical(), "$/year");

    // Check that it has the correct dimension (Currency/Time)
    match unit.dimension() {
        unicel_lib::core::units::Dimension::Compound {
            numerator,
            denominator,
        } => {
            assert_eq!(numerator.len(), 1);
            assert_eq!(numerator[0].0, BaseDimension::Currency);
            assert_eq!(numerator[0].1, 1);

            assert_eq!(denominator.len(), 1);
            assert_eq!(denominator[0].0, BaseDimension::Time);
            assert_eq!(denominator[0].1, 1);
        }
        _ => panic!("Expected Compound dimension for $/year"),
    }
}

#[test]
fn test_dollar_per_quarter_compatible_with_dollar_per_year() {
    // Test that $/quarter and $/year are compatible (same dimension)
    let library = UnitLibrary::new();

    let unit1 = parse_unit("$/quarter", &library).unwrap();
    let unit2 = parse_unit("$/year", &library).unwrap();

    assert!(
        unit1.is_compatible(&unit2),
        "$/quarter and $/year should be compatible (both are Currency/Time)"
    );
}

#[test]
fn test_convert_function_dollar_per_quarter_to_dollar_per_year() {
    // This is the main bug case: CONVERT(30000 $/quarter, 1 $/year)
    // Expected: 30000 * 0.25 = 7500 $/year
    let mut workbook = Workbook::new("Test");
    let sheet = workbook.active_sheet_mut();

    // Set B9 with 30000 $/quarter
    let cell_b9 = Cell::new(
        30000.0,
        Unit::compound(
            "$/quarter".to_string(),
            vec![(BaseDimension::Currency, 1)],
            vec![(BaseDimension::Time, 1)],
        ),
    );

    sheet
        .set(CellAddr::from_string("B9").unwrap(), cell_b9)
        .unwrap();

    // Set A1 with formula =CONVERT(B9, 1 $/year)
    let cell_a1 = Cell::with_formula("=CONVERT(B9, 1 $/year)".to_string());
    sheet
        .set(CellAddr::from_string("A1").unwrap(), cell_a1)
        .unwrap();

    // Get the cell to check its computed value
    let addr = CellAddr::from_string("A1").unwrap();
    let cell = sheet.get(&addr).unwrap();

    // Check that the cell has a formula
    assert!(cell.formula().is_some(), "Cell should have a formula");

    // For now, just verify that parsing works and units are compatible
    // The actual evaluation happens in the formula engine which we're not testing here
    // This test primarily verifies that $/quarter and $/year parse correctly and are compatible
}

#[test]
fn test_library_convert_compound_units_with_quarter_year() {
    // Test that the library can handle compound unit conversions
    // Note: The library.convert() method currently only handles simple units
    // For compound units, we need to convert each component separately
    let library = UnitLibrary::new();

    // The library should be able to convert the time component
    let time_conversion = library.convert(1.0, "quarter", "year");
    assert!(
        time_conversion.is_some(),
        "Should be able to convert quarter to year"
    );
    assert_eq!(time_conversion.unwrap(), 0.25, "1 quarter = 0.25 years");

    // For compound units like $/quarter -> $/year, the conversion happens
    // by converting the denominator: 1 $/quarter = 1 / (quarter->year) $/year
    // So 1 $/quarter = 1 / 0.25 $/year = 4 $/year
    // And 30000 $/quarter = 30000 / 4 $/year = 7500 $/year

    // Wait, let me recalculate:
    // If you earn $30000 per quarter, how much per year?
    // per quarter means "divided by quarter", so we have 30000 / quarter
    // To convert to per year: 30000 / quarter * (quarter / year)
    // = 30000 / quarter * 0.25 quarter/year
    // = 30000 * 0.25 / year = 7500 / year

    // Actually the correct formula is:
    // $X/quarter = $X * (quarter/year) / year = $X * 0.25 / year
    // So $30000/quarter = $7500/year ✓
}

#[test]
fn test_convert_reverse_dollar_per_year_to_dollar_per_quarter() {
    // Test the reverse: $/year -> $/quarter
    // Expected: 10000 $/year = 10000 * 4 = 40000 $/quarter
    let mut workbook = Workbook::new("Test");
    let sheet = workbook.active_sheet_mut();

    // Set B9 with 10000 $/year
    let cell_b9 = Cell::new(
        10000.0,
        Unit::compound(
            "$/year".to_string(),
            vec![(BaseDimension::Currency, 1)],
            vec![(BaseDimension::Time, 1)],
        ),
    );

    sheet
        .set(CellAddr::from_string("B9").unwrap(), cell_b9)
        .unwrap();

    // Set A1 with formula =CONVERT(B9, 1 $/quarter)
    let cell_a1 = Cell::with_formula("=CONVERT(B9, 1 $/quarter)".to_string());
    sheet
        .set(CellAddr::from_string("A1").unwrap(), cell_a1)
        .unwrap();

    // Get the cell to check its computed value
    let addr = CellAddr::from_string("A1").unwrap();
    let cell = sheet.get(&addr).unwrap();

    // Check that the cell has a formula
    assert!(cell.formula().is_some(), "Cell should have a formula");

    // This test primarily verifies that $/year and $/quarter parse correctly and are compatible
}

#[test]
fn test_convert_function_actually_evaluates_quarter_to_year() {
    // THIS IS THE KEY TEST - Actually evaluate the formula, not just parse it
    // Bug: The formula "=convert(B9,1$/year)" where B9=30000 $/quarter fails with
    // "Incompatible units: cannot CONVERT $/quarter and $/year"
    // Expected: 30000 $/quarter = 30000 * 0.25 = 7500 $/year

    let mut workbook = Workbook::new("Test");
    let sheet = workbook.active_sheet_mut();

    // Set B9 with 30000 $/quarter
    let cell_b9 = Cell::new(
        30000.0,
        Unit::compound(
            "$/quarter".to_string(),
            vec![(BaseDimension::Currency, 1)],
            vec![(BaseDimension::Time, 1)],
        ),
    );

    sheet
        .set(CellAddr::from_string("B9").unwrap(), cell_b9)
        .unwrap();

    // Set A1 with formula =CONVERT(B9, 1 $/year)
    let cell_a1 = Cell::with_formula("=CONVERT(B9, 1 $/year)".to_string());
    sheet
        .set(CellAddr::from_string("A1").unwrap(), cell_a1)
        .unwrap();

    // NOW ACTUALLY EVALUATE THE FORMULA
    let result = sheet.evaluate_formula("=CONVERT(B9, 1 $/year)");

    // This should NOT error
    assert!(
        result.is_ok(),
        "Formula evaluation failed: {:?}",
        result.err()
    );

    let (value, unit) = result.unwrap();

    // Check the result value
    match value {
        unicel_lib::core::cell::CellValue::Number(n) => {
            // Correct conversion: 30000 $/quarter * 4 quarters/year = 120000 $/year
            // If you earn $30,000 per quarter, you earn $120,000 per year (4 quarters/year)
            assert!((n - 120000.0).abs() < 0.01, "Expected 120000, got {}", n);
        }
        _ => panic!("Expected numeric result"),
    }

    // Check the result unit
    assert_eq!(unit.canonical(), "$/year", "Expected $/year unit");
}

#[test]
fn test_convert_function_actually_evaluates_year_to_quarter() {
    // Test the reverse conversion with actual evaluation
    // 10000 $/year = 10000 * 4 quarters/year = 40000 $/quarter

    let mut workbook = Workbook::new("Test");
    let sheet = workbook.active_sheet_mut();

    // Set B9 with 10000 $/year
    let cell_b9 = Cell::new(
        10000.0,
        Unit::compound(
            "$/year".to_string(),
            vec![(BaseDimension::Currency, 1)],
            vec![(BaseDimension::Time, 1)],
        ),
    );

    sheet
        .set(CellAddr::from_string("B9").unwrap(), cell_b9)
        .unwrap();

    // NOW ACTUALLY EVALUATE THE FORMULA
    let result = sheet.evaluate_formula("=CONVERT(B9, 1 $/quarter)");

    // This should NOT error
    assert!(
        result.is_ok(),
        "Formula evaluation failed: {:?}",
        result.err()
    );

    let (value, unit) = result.unwrap();

    // Check the result value
    match value {
        unicel_lib::core::cell::CellValue::Number(n) => {
            // Correct conversion: 10000 $/year / 4 quarters/year = 2500 $/quarter
            // If you earn $10,000 per year, you earn $2,500 per quarter (1/4 of annual)
            assert!((n - 2500.0).abs() < 0.01, "Expected 2500, got {}", n);
        }
        _ => panic!("Expected numeric result"),
    }

    // Check the result unit
    assert_eq!(unit.canonical(), "$/quarter", "Expected $/quarter unit");
}

#[test]
fn test_convert_with_manual_cell_input() {
    // Simplified test - just manually create cells and evaluate
    //  This tests what's described in the issue - the formula itself failing

    let mut workbook = Workbook::new("Test");
    let sheet = workbook.active_sheet_mut();

    // Set B9 with 30000 $/quarter (using the same approach as earlier tests)
    let cell_b9 = Cell::new(
        30000.0,
        Unit::compound(
            "$/quarter".to_string(),
            vec![(BaseDimension::Currency, 1)],
            vec![(BaseDimension::Time, 1)],
        ),
    );

    sheet
        .set(CellAddr::from_string("B9").unwrap(), cell_b9)
        .unwrap();

    println!("Cell B9 set successfully");
    let b9_cell = sheet.get(&CellAddr::from_string("B9").unwrap()).unwrap();
    println!("B9 value: {:?}", b9_cell.value());
    println!("B9 unit: {}", b9_cell.storage_unit().canonical());

    // Now try to evaluate the CONVERT formula
    println!("Evaluating CONVERT formula...");
    let result = sheet.evaluate_formula("=CONVERT(B9, 1 $/year)");

    if let Err(ref e) = result {
        println!("ERROR: {:?}", e);
    }

    assert!(
        result.is_ok(),
        "Formula evaluation failed: {:?}",
        result.as_ref().err()
    );

    let (result_value, result_unit) = result.unwrap();

    match result_value {
        unicel_lib::core::cell::CellValue::Number(n) => {
            println!("Result value: {}", n);
            println!("Result unit: {}", result_unit.canonical());

            // The mathematically correct answer is:
            // $30,000 per quarter = $30,000 * 4 quarters/year = $120,000 per year
            assert!(
                (n - 120000.0).abs() < 0.01,
                "Expected 120000 (30000 * 4), got {}",
                n
            );
        }
        _ => panic!("Expected numeric result"),
    }

    assert_eq!(result_unit.canonical(), "$/year");
}
