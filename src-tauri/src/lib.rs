// Core library for Unicel unit-aware spreadsheet

#[macro_use]
pub mod app_builder;
pub mod commands;
pub mod core;
pub mod formats;
pub mod mcp;

// Re-export main types
pub use core::{
    cell::{Cell, CellValue},
    units::Unit,
    workbook::Workbook,
};

// Mobile entry point - must be in lib.rs for iOS/Android builds
#[cfg(mobile)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn mobile_main() {
    // Define all commands using the shared macro
    define_commands!();

    // Initialize logging
    app_builder::init_logging();

    // Build and run the app
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(commands::AppState::default())
        .invoke_handler(tauri::generate_handler![
            create_workbook,
            get_workbook_info,
            get_sheet_cells,
            set_cell,
            save_workbook,
            load_workbook,
            get_current_file,
            get_recent_files,
            set_display_mode,
            get_unit_preferences,
            update_unit_preferences,
            set_metric_system,
            set_currency_rate,
            get_currencies,
            get_units_in_use,
            get_base_units_in_use,
            get_cells_with_base_unit,
            export_debug_to_clipboard,
            export_to_excel,
            get_example_workbook_path,
            list_example_workbooks,
            set_active_sheet,
            add_sheet,
            rename_sheet,
            delete_sheet,
            sheet_has_data,
            list_named_ranges,
            create_named_range,
            delete_named_range,
            get_named_range,
            get_named_range_for_cell,
            set_column_width,
            get_column_width,
            set_row_height,
            get_row_height,
            get_all_column_widths,
            get_all_row_heights,
            insert_column_before,
            insert_column_after,
            insert_row_before,
            insert_row_after,
            delete_column,
            delete_row,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
