// Manual test example for long unit name recognition
//
// This demonstrates that long-form unit names like "miles", "feet", "hours"
// are correctly recognized and converted in Metric/Imperial display modes.

use unicel_lib::commands::workbook::{
    create_workbook_impl, get_sheet_cells_impl, set_cell_impl, AppState, DisplayMode,
};

fn main() {
    println!("=== Long Unit Name Recognition Test ===\n");

    // Create a new workbook
    let state = AppState::default();
    create_workbook_impl(&state, "Test Workbook".to_string()).unwrap();

    // Add some cells with long-form unit names
    println!("Adding cells with long-form units:");
    println!("  A1: 60 miles");
    println!("  A2: 100 feet");
    println!("  A3: 2 hours");
    println!("  A4: 100 pounds\n");

    set_cell_impl(&state, "A1".to_string(), "60 miles".to_string()).unwrap();
    set_cell_impl(&state, "A2".to_string(), "100 feet".to_string()).unwrap();
    set_cell_impl(&state, "A3".to_string(), "2 hours".to_string()).unwrap();
    set_cell_impl(&state, "A4".to_string(), "100 pounds".to_string()).unwrap();

    // Display in AsEntered mode (no conversion)
    println!("Display Mode: AsEntered (no conversion)");
    *state.display_mode.lock().unwrap() = DisplayMode::AsEntered;
    print_cells(&state);

    // Display in Metric mode (should convert)
    println!("Display Mode: Metric (should convert to metric units)");
    *state.display_mode.lock().unwrap() = DisplayMode::Metric;

    let mut prefs = state.unit_preferences.lock().unwrap().clone();
    prefs.metric_length = "km".to_string();
    prefs.metric_mass = "kg".to_string();
    prefs.metric_time = "min".to_string();
    *state.unit_preferences.lock().unwrap() = prefs;

    print_cells(&state);

    // Display in Imperial mode (should keep imperial units or convert)
    println!("Display Mode: Imperial (keep or convert within imperial system)");
    *state.display_mode.lock().unwrap() = DisplayMode::Imperial;
    print_cells(&state);

    println!("\n=== Test Complete ===");
}

fn print_cells(state: &AppState) {
    let cells = get_sheet_cells_impl(state).unwrap();
    for (addr, cell_data) in cells {
        let value_str = match &cell_data.value {
            unicel_lib::commands::workbook::CellValueData::Number { value } => {
                format!("{:.2}", value)
            }
            unicel_lib::commands::workbook::CellValueData::Text { text } => text.clone(),
            unicel_lib::commands::workbook::CellValueData::Empty => "".to_string(),
            unicel_lib::commands::workbook::CellValueData::Error { message } => {
                format!("ERROR: {}", message)
            }
        };

        let display_unit = cell_data
            .display_unit
            .as_ref()
            .unwrap_or(&cell_data.storage_unit);

        println!(
            "  {}: {} {} (stored as: {})",
            addr, value_str, display_unit, cell_data.storage_unit
        );
    }
    println!();
}
