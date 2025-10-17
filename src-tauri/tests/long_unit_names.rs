// Test long-form unit name recognition and display conversion
//
// This test ensures that unit names like "miles", "feet", "hours" etc. are properly
// recognized and converted in display mode.

use unicel_lib::commands::workbook::{
    create_workbook_impl, get_sheet_cells_impl, parse_cell_input, set_cell_impl, AppState,
    CellValueData, DisplayMode,
};

#[test]
fn test_long_unit_name_recognition() {
    // Test that long-form unit names are properly recognized
    let test_cases = vec![
        // Length units
        ("60 miles", "miles"),
        ("100 feet", "feet"),
        ("50 meters", "meters"),
        ("10 kilometers", "kilometers"),
        ("5 yards", "yards"),
        ("12 inches", "inches"),
        // Mass units
        ("100 pounds", "pounds"),
        ("16 ounces", "ounces"),
        ("500 grams", "grams"),
        ("2 kilograms", "kilograms"),
        // Time units
        ("60 seconds", "seconds"),
        ("30 minutes", "minutes"),
        ("2 hours", "hours"),
        // Temperature units
        ("32 Fahrenheit", "Fahrenheit"),
        ("0 Celsius", "Celsius"),
        ("273 Kelvin", "Kelvin"),
    ];

    for (input, expected_unit) in test_cases {
        let cell = parse_cell_input(input).expect(&format!("Failed to parse: {}", input));
        let unit_str = cell.storage_unit().canonical();
        assert_eq!(
            unit_str, expected_unit,
            "Expected unit '{}' for input '{}', got '{}'",
            expected_unit, input, unit_str
        );
    }
}

#[test]
fn test_miles_to_km_conversion() {
    // Test the specific case mentioned in the bug report:
    // "60 miles" should convert to kilometers in Metric mode

    let state = AppState::default();

    // Create a workbook
    create_workbook_impl(&state, "Test".to_string()).unwrap();

    // Set a cell with "60 miles"
    set_cell_impl(&state, "A1".to_string(), "60 miles".to_string()).unwrap();

    // Switch to Metric mode
    *state.display_mode.lock().unwrap() = DisplayMode::Metric;

    // Set metric preferences to use km for length
    let mut prefs = state.unit_preferences.lock().unwrap().clone();
    prefs.metric_length = "km".to_string();
    *state.unit_preferences.lock().unwrap() = prefs.clone();

    // Get the cells
    let cells = get_sheet_cells_impl(&state).unwrap();
    let (addr, cell_data) = &cells[0];

    assert_eq!(addr, "A1");
    assert_eq!(cell_data.storage_unit, "miles");

    // Check conversion: 60 miles = 96.5604 km
    if let CellValueData::Number { value } = cell_data.value {
        // Allow 0.1% tolerance for floating point
        let expected = 96.5604;
        let error = (value - expected).abs() / expected;
        assert!(
            error < 0.001,
            "Expected ~{} km, got {} km (error: {:.4}%)",
            expected,
            value,
            error * 100.0
        );
    } else {
        panic!("Expected number value");
    }

    assert_eq!(
        cell_data.display_unit.as_ref().unwrap(),
        "km",
        "Display unit should be km"
    );
}

#[test]
fn test_feet_to_meters_conversion() {
    let state = AppState::default();

    create_workbook_impl(&state, "Test".to_string()).unwrap();
    set_cell_impl(&state, "A1".to_string(), "100 feet".to_string()).unwrap();

    *state.display_mode.lock().unwrap() = DisplayMode::Metric;

    let mut prefs = state.unit_preferences.lock().unwrap().clone();
    prefs.metric_length = "m".to_string();
    *state.unit_preferences.lock().unwrap() = prefs;

    let cells = get_sheet_cells_impl(&state).unwrap();
    let (_, cell_data) = &cells[0];

    assert_eq!(cell_data.storage_unit, "feet");

    // 100 feet = 30.48 meters
    if let CellValueData::Number { value } = cell_data.value {
        let expected = 30.48;
        let error = (value - expected).abs() / expected;
        assert!(error < 0.001, "Expected ~{} m, got {} m", expected, value);
    } else {
        panic!("Expected number value");
    }

    assert_eq!(cell_data.display_unit.as_ref().unwrap(), "m");
}

#[test]
fn test_pounds_to_kilograms_conversion() {
    let state = AppState::default();

    create_workbook_impl(&state, "Test".to_string()).unwrap();
    set_cell_impl(&state, "A1".to_string(), "100 pounds".to_string()).unwrap();

    *state.display_mode.lock().unwrap() = DisplayMode::Metric;

    let mut prefs = state.unit_preferences.lock().unwrap().clone();
    prefs.metric_mass = "kg".to_string();
    *state.unit_preferences.lock().unwrap() = prefs;

    let cells = get_sheet_cells_impl(&state).unwrap();
    let (_, cell_data) = &cells[0];

    assert_eq!(cell_data.storage_unit, "pounds");

    // 100 pounds = 45.3592 kg
    if let CellValueData::Number { value } = cell_data.value {
        let expected = 45.3592;
        let error = (value - expected).abs() / expected;
        assert!(error < 0.001, "Expected ~{} kg, got {} kg", expected, value);
    } else {
        panic!("Expected number value");
    }

    assert_eq!(cell_data.display_unit.as_ref().unwrap(), "kg");
}

#[test]
fn test_hours_conversion() {
    let state = AppState::default();

    create_workbook_impl(&state, "Test".to_string()).unwrap();
    set_cell_impl(&state, "A1".to_string(), "2 hours".to_string()).unwrap();

    *state.display_mode.lock().unwrap() = DisplayMode::Metric;

    let mut prefs = state.unit_preferences.lock().unwrap().clone();
    prefs.metric_time = "min".to_string(); // Convert hours to minutes
    *state.unit_preferences.lock().unwrap() = prefs;

    let cells = get_sheet_cells_impl(&state).unwrap();
    let (_, cell_data) = &cells[0];

    assert_eq!(cell_data.storage_unit, "hours");

    // 2 hours = 120 minutes
    if let CellValueData::Number { value } = cell_data.value {
        assert_eq!(value, 120.0, "Expected 120 minutes, got {}", value);
    } else {
        panic!("Expected number value");
    }

    assert_eq!(cell_data.display_unit.as_ref().unwrap(), "min");
}

#[test]
fn test_celsius_to_fahrenheit_conversion() {
    let state = AppState::default();

    create_workbook_impl(&state, "Test".to_string()).unwrap();
    set_cell_impl(&state, "A1".to_string(), "0 Celsius".to_string()).unwrap();

    *state.display_mode.lock().unwrap() = DisplayMode::Imperial;

    let mut prefs = state.unit_preferences.lock().unwrap().clone();
    prefs.imperial_temperature = "F".to_string();
    *state.unit_preferences.lock().unwrap() = prefs;

    let cells = get_sheet_cells_impl(&state).unwrap();
    let (_, cell_data) = &cells[0];

    assert_eq!(cell_data.storage_unit, "Celsius");

    // 0 Celsius = 32 Fahrenheit
    if let CellValueData::Number { value } = cell_data.value {
        assert_eq!(value, 32.0, "Expected 32°F, got {}°F", value);
    } else {
        panic!("Expected number value");
    }

    assert_eq!(cell_data.display_unit.as_ref().unwrap(), "F");
}

#[test]
fn test_singular_and_plural_forms() {
    // Test that both singular and plural forms work
    let test_cases = vec![
        ("1 mile", "mile"),
        ("2 miles", "miles"),
        ("1 foot", "foot"),
        ("10 feet", "feet"),
        ("1 hour", "hour"),
        ("3 hours", "hours"),
        ("1 pound", "pound"),
        ("5 pounds", "pounds"),
    ];

    for (input, expected_unit) in test_cases {
        let cell = parse_cell_input(input).expect(&format!("Failed to parse: {}", input));
        let unit_str = cell.storage_unit().canonical();
        assert_eq!(
            unit_str, expected_unit,
            "Expected unit '{}' for input '{}', got '{}'",
            expected_unit, input, unit_str
        );
    }
}

#[test]
fn test_mixed_case_temperature() {
    // Test case-insensitive temperature names
    let test_cases = vec![
        ("32 fahrenheit", "fahrenheit"),
        ("32 Fahrenheit", "Fahrenheit"),
        ("0 celsius", "celsius"),
        ("0 Celsius", "Celsius"),
        ("273 kelvin", "kelvin"),
        ("273 Kelvin", "Kelvin"),
    ];

    for (input, expected_unit) in test_cases {
        let cell = parse_cell_input(input).expect(&format!("Failed to parse: {}", input));
        let unit_str = cell.storage_unit().canonical();
        assert_eq!(
            unit_str, expected_unit,
            "Expected unit '{}' for input '{}', got '{}'",
            expected_unit, input, unit_str
        );
    }
}
