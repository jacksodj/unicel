// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, State};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use unicel_lib::commands::{AppState, CellData, NamedRangeInfo, WorkbookInfo};
use unicel_lib::core::settings::UnitPreferences;

// Tauri command definitions (must be in binary crate for macro to work)

#[tauri::command]
fn create_workbook(state: State<AppState>, name: String) -> Result<(), String> {
    unicel_lib::commands::create_workbook_impl(&state, name)
}

#[tauri::command]
fn get_workbook_info(state: State<AppState>) -> Result<WorkbookInfo, String> {
    unicel_lib::commands::get_workbook_info_impl(&state)
}

#[tauri::command]
fn get_sheet_cells(state: State<AppState>) -> Result<Vec<(String, CellData)>, String> {
    unicel_lib::commands::get_sheet_cells_impl(&state)
}

#[tauri::command]
fn set_cell(state: State<AppState>, address: String, value: String) -> Result<CellData, String> {
    unicel_lib::commands::set_cell_impl(&state, address, value)
}

#[tauri::command]
fn save_workbook(state: State<AppState>, path: String) -> Result<(), String> {
    unicel_lib::commands::save_workbook_impl(&state, path)
}

#[tauri::command]
fn load_workbook(state: State<AppState>, path: String) -> Result<(), String> {
    unicel_lib::commands::load_workbook_impl(&state, path)
}

#[tauri::command]
fn get_current_file(state: State<AppState>) -> Option<String> {
    unicel_lib::commands::get_current_file_impl(&state)
}

#[tauri::command]
fn get_recent_files(state: State<AppState>) -> Vec<String> {
    unicel_lib::commands::get_recent_files_impl(&state)
}

#[tauri::command]
fn set_display_mode(state: State<AppState>, mode: String) -> Result<(), String> {
    unicel_lib::commands::set_display_mode_impl(&state, mode)
}

#[tauri::command]
fn get_unit_preferences(state: State<AppState>) -> Result<UnitPreferences, String> {
    unicel_lib::commands::get_unit_preferences_impl(&state)
}

#[tauri::command]
fn update_unit_preferences(
    state: State<AppState>,
    preferences: UnitPreferences,
) -> Result<(), String> {
    unicel_lib::commands::update_unit_preferences_impl(&state, preferences)
}

#[tauri::command]
fn set_metric_system(state: State<AppState>, system: String) -> Result<(), String> {
    unicel_lib::commands::set_metric_system_impl(&state, system)
}

#[tauri::command]
fn set_currency_rate(state: State<AppState>, currency: String, rate: f64) -> Result<(), String> {
    unicel_lib::commands::set_currency_rate_impl(&state, currency, rate)
}

#[tauri::command]
fn get_currencies(state: State<AppState>) -> Result<Vec<String>, String> {
    unicel_lib::commands::get_currencies_impl(&state)
}

#[tauri::command]
fn get_units_in_use(state: State<AppState>) -> Result<Vec<String>, String> {
    unicel_lib::commands::get_units_in_use_impl(&state)
}

#[tauri::command]
fn get_base_units_in_use(state: State<AppState>) -> Result<Vec<String>, String> {
    unicel_lib::commands::get_base_units_in_use_impl(&state)
}

#[tauri::command]
fn get_cells_with_base_unit(
    state: State<AppState>,
    base_unit: String,
) -> Result<Vec<String>, String> {
    unicel_lib::commands::get_cells_with_base_unit_impl(&state, &base_unit)
}

#[tauri::command]
fn export_debug_to_clipboard(
    state: State<AppState>,
    frontend_version: Option<String>,
    frontend_commit: Option<String>,
) -> Result<(), String> {
    unicel_lib::commands::export_debug_to_clipboard_impl(&state, frontend_version, frontend_commit)
}

#[tauri::command]
fn export_to_excel(state: State<AppState>, path: String) -> Result<(), String> {
    unicel_lib::commands::export_to_excel_impl(&state, path)
}

#[tauri::command]
fn get_example_workbook_path(app: tauri::AppHandle, filename: String) -> Result<String, String> {
    use std::path::PathBuf;

    // Try to resolve as a bundled resource first (works in production)
    let resource_path = format!("examples/{}", filename);
    if let Ok(path) = app
        .path()
        .resolve(&resource_path, tauri::path::BaseDirectory::Resource)
    {
        if path.exists() {
            return Ok(path.to_string_lossy().to_string());
        }
    }

    // Fallback for development mode: look in src-tauri/examples/
    let dev_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join(&filename);

    if dev_path.exists() {
        return Ok(dev_path.to_string_lossy().to_string());
    }

    Err(format!("Example file not found: {}", filename))
}

#[tauri::command]
fn list_example_workbooks() -> Vec<(String, String)> {
    vec![
        (
            "unit_conversion_tutorial.usheet".to_string(),
            "Unit Conversion Tutorial".to_string(),
        ),
        (
            "aws_cost_estimator.usheet".to_string(),
            "AWS Cost Estimator".to_string(),
        ),
        (
            "construction_estimator.usheet".to_string(),
            "Construction Estimator".to_string(),
        ),
        (
            "investment_portfolio.usheet".to_string(),
            "Investment Portfolio Tracker".to_string(),
        ),
        (
            "formula_functions_showcase.usheet".to_string(),
            "Formula Functions Showcase".to_string(),
        ),
    ]
}

#[tauri::command]
fn set_active_sheet(state: State<AppState>, index: usize) -> Result<(), String> {
    unicel_lib::commands::set_active_sheet_impl(&state, index)
}

#[tauri::command]
fn add_sheet(state: State<AppState>) -> Result<usize, String> {
    unicel_lib::commands::add_sheet_impl(&state)
}

#[tauri::command]
fn rename_sheet(state: State<AppState>, index: usize, new_name: String) -> Result<(), String> {
    unicel_lib::commands::rename_sheet_impl(&state, index, new_name)
}

#[tauri::command]
fn delete_sheet(state: State<AppState>, index: usize) -> Result<(), String> {
    unicel_lib::commands::delete_sheet_impl(&state, index)
}

#[tauri::command]
fn sheet_has_data(state: State<AppState>, index: usize) -> Result<bool, String> {
    unicel_lib::commands::sheet_has_data_impl(&state, index)
}

#[tauri::command]
fn list_named_ranges(state: State<AppState>) -> Result<Vec<NamedRangeInfo>, String> {
    unicel_lib::commands::list_named_ranges_impl(&state)
}

#[tauri::command]
fn create_named_range(
    state: State<AppState>,
    name: String,
    sheet_index: usize,
    cell_address: String,
) -> Result<(), String> {
    unicel_lib::commands::create_named_range_impl(&state, name, sheet_index, cell_address)
}

#[tauri::command]
fn delete_named_range(state: State<AppState>, name: String) -> Result<(), String> {
    unicel_lib::commands::delete_named_range_impl(&state, name)
}

#[tauri::command]
fn get_named_range(state: State<AppState>, name: String) -> Result<NamedRangeInfo, String> {
    unicel_lib::commands::get_named_range_impl(&state, name)
}

#[tauri::command]
fn get_named_range_for_cell(
    state: State<AppState>,
    sheet_index: usize,
    cell_address: String,
) -> Result<Option<String>, String> {
    unicel_lib::commands::get_named_range_for_cell_impl(&state, sheet_index, cell_address)
}

#[tauri::command]
fn set_column_width(state: State<AppState>, col: String, width: f64) -> Result<(), String> {
    unicel_lib::commands::set_column_width_impl(&state, col, width)
}

#[tauri::command]
fn get_column_width(state: State<AppState>, col: String) -> Result<Option<f64>, String> {
    unicel_lib::commands::get_column_width_impl(&state, col)
}

#[tauri::command]
fn set_row_height(state: State<AppState>, row: usize, height: f64) -> Result<(), String> {
    unicel_lib::commands::set_row_height_impl(&state, row, height)
}

#[tauri::command]
fn get_row_height(state: State<AppState>, row: usize) -> Result<Option<f64>, String> {
    unicel_lib::commands::get_row_height_impl(&state, row)
}

#[tauri::command]
fn get_all_column_widths(
    state: State<AppState>,
) -> Result<std::collections::HashMap<String, f64>, String> {
    unicel_lib::commands::get_all_column_widths_impl(&state)
}

#[tauri::command]
fn get_all_row_heights(
    state: State<AppState>,
) -> Result<std::collections::HashMap<usize, f64>, String> {
    unicel_lib::commands::get_all_row_heights_impl(&state)
}

#[tauri::command]
fn insert_column_before(state: State<AppState>, col: String) -> Result<(), String> {
    unicel_lib::commands::insert_column_before_impl(&state, col)
}

#[tauri::command]
fn insert_column_after(state: State<AppState>, col: String) -> Result<(), String> {
    unicel_lib::commands::insert_column_after_impl(&state, col)
}

#[tauri::command]
fn insert_row_before(state: State<AppState>, row: usize) -> Result<(), String> {
    unicel_lib::commands::insert_row_before_impl(&state, row)
}

#[tauri::command]
fn insert_row_after(state: State<AppState>, row: usize) -> Result<(), String> {
    unicel_lib::commands::insert_row_after_impl(&state, row)
}

#[tauri::command]
fn delete_column(state: State<AppState>, col: String) -> Result<(), String> {
    unicel_lib::commands::delete_column_impl(&state, col)
}

#[tauri::command]
fn delete_row(state: State<AppState>, row: usize) -> Result<(), String> {
    unicel_lib::commands::delete_row_impl(&state, row)
}

fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "unicel=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Unicel application");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState::default())
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
