// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tauri::State;
use unicel_lib::commands::{AppState, CellData, WorkbookInfo};

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
fn set_cell(
    state: State<AppState>,
    address: String,
    value: String,
) -> Result<CellData, String> {
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
