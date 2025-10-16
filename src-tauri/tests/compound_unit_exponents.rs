// Test compound unit conversions with exponents
// This tests the fix for the bug where exponents in denominators weren't properly handled

use unicel_lib::commands::workbook::{self, AppState, CellValueData};
use unicel_lib::core::cell::Cell;
use unicel_lib::core::table::CellAddr;
use unicel_lib::core::units::{BaseDimension, Unit};

/// Helper function to parse a unit string into a Unit struct
fn parse_unit(unit_str: &str) -> Unit {
    if let Some(pos) = unit_str.find('/') {
        let numerator_str = &unit_str[..pos];
        let denominator_str = &unit_str[pos + 1..];

        let (num_dim, num_power) = parse_dimension_with_power(numerator_str);
        let (den_dim, den_power) = parse_dimension_with_power(denominator_str);

        return Unit::compound(
            unit_str.to_string(),
            vec![(num_dim, num_power)],
            vec![(den_dim, den_power)],
        );
    }

    if let Some(pos) = unit_str.find('^') {
        let base_str = &unit_str[..pos];
        let power_str = &unit_str[pos + 1..];

        if let Ok(power) = power_str.parse::<i32>() {
            let dimension = get_base_dimension(base_str);
            return Unit::compound(unit_str.to_string(), vec![(dimension, power)], vec![]);
        }
    }

    let dimension = get_base_dimension(unit_str);
    Unit::simple(unit_str, dimension)
}

fn parse_dimension_with_power(unit_str: &str) -> (BaseDimension, i32) {
    if let Some(pos) = unit_str.find('^') {
        let base_str = &unit_str[..pos];
        let power_str = &unit_str[pos + 1..];

        if let Ok(power) = power_str.parse::<i32>() {
            return (get_base_dimension(base_str), power);
        }
    }

    (get_base_dimension(unit_str), 1)
}

fn get_base_dimension(unit_str: &str) -> BaseDimension {
    match unit_str {
        "m" | "cm" | "mm" | "km" | "in" | "ft" | "yd" | "mi" => BaseDimension::Length,
        "g" | "kg" | "mg" | "oz" | "lb" => BaseDimension::Mass,
        "s" | "min" | "hr" | "h" | "hour" => BaseDimension::Time,
        "C" | "F" | "K" => BaseDimension::Temperature,
        "USD" | "EUR" | "GBP" | "$" => BaseDimension::Currency,
        _ => BaseDimension::Custom(unit_str.to_string()),
    }
}

#[test]
fn test_area_conversion_ft2_to_m2() {
    // Test case: 100 ft^2 -> m^2
    // Expected: 100 * (0.3048)^2 = 100 * 0.09290304 = 9.290304 m^2
    let state = AppState::default();

    // Create workbook
    workbook::create_workbook_impl(&state, "Test".to_string()).unwrap();

    // Set a cell with area value
    let cell = Cell::new(100.0, parse_unit("ft^2"));

    {
        let mut wb = state.workbook.lock().unwrap();
        let workbook = wb.as_mut().unwrap();
        let addr = CellAddr::from_string("A1").unwrap();
        workbook.active_sheet_mut().set(addr, cell).unwrap();
    }

    // Switch to metric mode
    workbook::set_display_mode_impl(&state, "Metric".to_string()).unwrap();

    // Get the cell data
    let cells = workbook::get_sheet_cells_impl(&state).unwrap();
    let cell_data = cells.iter().find(|(addr, _)| addr == "A1").unwrap();

    // Extract the value
    if let CellValueData::Number { value } = cell_data.1.value {
        // The expected conversion factor is (0.3048)^2 = 0.09290304
        let expected = 100.0 * 0.09290304;
        let tolerance = 0.001; // Allow small floating point error

        assert!(
            (value - expected).abs() < tolerance,
            "Expected {} m^2, got {} m^2 (error: {})",
            expected,
            value,
            (value - expected).abs()
        );
    } else {
        panic!("Expected numeric value");
    }
}

#[test]
fn test_reciprocal_area_conversion_1_per_ft2() {
    // Test case: 1 1/ft^2 -> 1/m^2
    // This is the MAIN bug case mentioned in the task
    // Expected: 1 / (0.3048)^2 = 1 / 0.09290304 = 10.7639104167 1/m^2
    let state = AppState::default();

    // Create workbook
    workbook::create_workbook_impl(&state, "Test".to_string()).unwrap();

    // Set a cell with reciprocal area value (1 per square foot)
    let cell = Cell::new(
        1.0,
        Unit::compound(
            "1/ft^2".to_string(),
            vec![],
            vec![(BaseDimension::Length, 2)],
        ),
    );

    {
        let mut wb = state.workbook.lock().unwrap();
        let workbook = wb.as_mut().unwrap();
        let addr = CellAddr::from_string("A1").unwrap();
        workbook.active_sheet_mut().set(addr, cell).unwrap();
    }

    // Switch to metric mode
    workbook::set_display_mode_impl(&state, "Metric".to_string()).unwrap();

    // Get the cell data
    let cells = workbook::get_sheet_cells_impl(&state).unwrap();
    let cell_data = cells.iter().find(|(addr, _)| addr == "A1").unwrap();

    // Extract the value
    if let CellValueData::Number { value } = cell_data.1.value {
        // The expected conversion factor is 1 / (0.3048)^2 = 10.7639104167
        let expected = 1.0 / (0.3048 * 0.3048);
        let tolerance = 0.001;

        assert!(
            (value - expected).abs() < tolerance,
            "Expected {} 1/m^2, got {} 1/m^2 (error: {})",
            expected,
            value,
            (value - expected).abs()
        );
    } else {
        panic!("Expected numeric value");
    }
}

#[test]
fn test_volume_conversion_ft3_to_m3() {
    // Test case: 10 ft^3 -> m^3
    // Expected: 10 * (0.3048)^3 = 10 * 0.028316846592 = 0.28316846592 m^3
    let state = AppState::default();

    workbook::create_workbook_impl(&state, "Test".to_string()).unwrap();

    let cell = Cell::new(10.0, parse_unit("ft^3"));

    {
        let mut wb = state.workbook.lock().unwrap();
        let workbook = wb.as_mut().unwrap();
        let addr = CellAddr::from_string("A1").unwrap();
        workbook.active_sheet_mut().set(addr, cell).unwrap();
    }

    workbook::set_display_mode_impl(&state, "Metric".to_string()).unwrap();

    let cells = workbook::get_sheet_cells_impl(&state).unwrap();
    let cell_data = cells.iter().find(|(addr, _)| addr == "A1").unwrap();

    if let CellValueData::Number { value } = cell_data.1.value {
        let expected = 10.0 * 0.3048_f64.powi(3);
        let tolerance = 0.001;

        assert!(
            (value - expected).abs() < tolerance,
            "Expected {} m^3, got {} m^3",
            expected,
            value
        );
    } else {
        panic!("Expected numeric value");
    }
}

#[test]
fn test_price_per_area_conversion() {
    // Test case: $15/ft^2 -> USD/m^2
    // Expected: 15 / (0.3048)^2 = 15 / 0.09290304 = 161.45865625 USD/m^2
    let state = AppState::default();

    workbook::create_workbook_impl(&state, "Test".to_string()).unwrap();

    let cell = Cell::new(
        15.0,
        Unit::compound(
            "$/ft^2".to_string(),
            vec![(BaseDimension::Currency, 1)],
            vec![(BaseDimension::Length, 2)],
        ),
    );

    {
        let mut wb = state.workbook.lock().unwrap();
        let workbook = wb.as_mut().unwrap();
        let addr = CellAddr::from_string("A1").unwrap();
        workbook.active_sheet_mut().set(addr, cell).unwrap();
    }

    workbook::set_display_mode_impl(&state, "Metric".to_string()).unwrap();

    let cells = workbook::get_sheet_cells_impl(&state).unwrap();
    let cell_data = cells.iter().find(|(addr, _)| addr == "A1").unwrap();

    if let CellValueData::Number { value } = cell_data.1.value {
        let expected = 15.0 / (0.3048 * 0.3048);
        let tolerance = 0.01;

        assert!(
            (value - expected).abs() < tolerance,
            "Expected {} USD/m^2, got {} USD/m^2",
            expected,
            value
        );
    } else {
        panic!("Expected numeric value");
    }
}

#[test]
fn test_mixed_compound_unit_mi_per_hr2() {
    // Test case: mi/hr^2 (acceleration)
    // This tests exponent in denominator with time units
    // Expected: convert mi -> km, hr^2 -> s^2
    // 1 mi = 1.60934 km
    // 1 hr = 3600 s, so 1 hr^2 = 12960000 s^2
    // Result: 1 mi/hr^2 = 1.60934 / 12960000 km/s^2 = 0.0000001241898... km/s^2
    let state = AppState::default();

    workbook::create_workbook_impl(&state, "Test".to_string()).unwrap();

    let cell = Cell::new(
        1.0,
        Unit::compound(
            "mi/hr^2".to_string(),
            vec![(BaseDimension::Length, 1)],
            vec![(BaseDimension::Time, 2)],
        ),
    );

    {
        let mut wb = state.workbook.lock().unwrap();
        let workbook = wb.as_mut().unwrap();
        let addr = CellAddr::from_string("A1").unwrap();
        workbook.active_sheet_mut().set(addr, cell).unwrap();
    }

    // Set preferences to use km and s
    {
        let mut prefs = state.unit_preferences.lock().unwrap();
        prefs.metric_length = "km".to_string();
        prefs.metric_time = "s".to_string();
    }

    workbook::set_display_mode_impl(&state, "Metric".to_string()).unwrap();

    let cells = workbook::get_sheet_cells_impl(&state).unwrap();
    let cell_data = cells.iter().find(|(addr, _)| addr == "A1").unwrap();

    if let CellValueData::Number { value } = cell_data.1.value {
        // mi -> km: 1.60934
        // hr -> s: 3600, so hr^2 -> s^2: 3600^2 = 12960000
        // Combined: 1.60934 / 12960000
        let expected = 1.60934 / 12960000.0;
        let tolerance = 0.0000001;

        assert!(
            (value - expected).abs() < tolerance,
            "Expected {} km/s^2, got {} km/s^2",
            expected,
            value
        );
    } else {
        panic!("Expected numeric value");
    }
}

#[test]
fn test_area_with_numerator_exponent_ft2_per_s() {
    // Test case: ft^2/s -> m^2/s
    // This tests exponent in NUMERATOR of division
    // Expected: (0.3048)^2 / 1 = 0.09290304
    let state = AppState::default();

    workbook::create_workbook_impl(&state, "Test".to_string()).unwrap();

    let cell = Cell::new(
        50.0,
        Unit::compound(
            "ft^2/s".to_string(),
            vec![(BaseDimension::Length, 2)],
            vec![(BaseDimension::Time, 1)],
        ),
    );

    {
        let mut wb = state.workbook.lock().unwrap();
        let workbook = wb.as_mut().unwrap();
        let addr = CellAddr::from_string("A1").unwrap();
        workbook.active_sheet_mut().set(addr, cell).unwrap();
    }

    workbook::set_display_mode_impl(&state, "Metric".to_string()).unwrap();

    let cells = workbook::get_sheet_cells_impl(&state).unwrap();
    let cell_data = cells.iter().find(|(addr, _)| addr == "A1").unwrap();

    if let CellValueData::Number { value } = cell_data.1.value {
        let expected = 50.0 * 0.3048_f64.powi(2);
        let tolerance = 0.001;

        assert!(
            (value - expected).abs() < tolerance,
            "Expected {} m^2/s, got {} m^2/s",
            expected,
            value
        );
    } else {
        panic!("Expected numeric value");
    }
}
