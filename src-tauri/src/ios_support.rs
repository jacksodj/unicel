//! iOS-specific filesystem helpers for handling shared documents and bundled resources.

use std::path::{Path, PathBuf};

#[cfg(target_os = "ios")]
use std::{ffi::OsStr, fs};

#[cfg(target_os = "ios")]
use once_cell::sync::Lazy;
#[cfg(target_os = "ios")]
use tauri::path::BaseDirectory;
use tauri::AppHandle;

#[cfg(target_os = "ios")]
static EXAMPLE_FILES: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "AWS_Cost_Estimator.usheet",
        "Construction_Estimator.usheet",
        "Investment_Portfolio.usheet",
        "Formula_Functions_Showcase.usheet",
    ]
});

#[cfg(target_os = "ios")]
const APP_FOLDER_NAME: &str = "Unicel";
#[cfg(target_os = "ios")]
const IMPORT_FOLDER_NAME: &str = "Imports";
#[cfg(target_os = "ios")]
const EXAMPLES_FOLDER_NAME: &str = "Examples";

#[cfg(target_os = "ios")]
fn document_root() -> Result<PathBuf, String> {
    tauri::api::path::document_dir()
        .ok_or_else(|| "Failed to resolve iOS documents directory".to_string())
        .map(PathBuf::from)
}

#[cfg(target_os = "ios")]
fn app_root() -> Result<PathBuf, String> {
    Ok(document_root()?.join(APP_FOLDER_NAME))
}

#[cfg(target_os = "ios")]
fn ensure_dir(path: &Path) -> Result<(), String> {
    if !path.exists() {
        tracing::info!("Creating iOS directory: {}", path.display());
        fs::create_dir_all(path)
            .map_err(|e| format!("Failed to create directory {}: {e}", path.display()))?;
    }
    Ok(())
}

#[cfg(target_os = "ios")]
fn is_usheet(path: &Path) -> bool {
    path.extension()
        .and_then(OsStr::to_str)
        .map(|ext| ext.eq_ignore_ascii_case("usheet"))
        .unwrap_or(false)
}

#[cfg(target_os = "ios")]
fn unique_destination(base: &Path, filename: &str) -> PathBuf {
    let mut candidate = base.join(filename);
    if !candidate.exists() {
        return candidate;
    }

    let stem = Path::new(filename)
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or("Workbook");
    let ext = Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or_default();

    let mut counter = 1;
    loop {
        let candidate_name = if ext.is_empty() {
            format!("{stem} ({counter})")
        } else {
            format!("{stem} ({counter}).{ext}")
        };
        candidate = base.join(candidate_name);
        if !candidate.exists() {
            break candidate;
        }
        counter += 1;
    }
}

#[cfg(target_os = "ios")]
fn copy_into_app_dir(source: &Path) -> Result<PathBuf, String> {
    let filename = source
        .file_name()
        .and_then(OsStr::to_str)
        .ok_or_else(|| format!("Invalid filename for {}", source.display()))?;

    let imports_dir = app_root()?.join(IMPORT_FOLDER_NAME);
    ensure_dir(&imports_dir)?;

    let destination = unique_destination(&imports_dir, filename);

    tracing::info!(
        "Copying workbook into sandbox: {} -> {}",
        source.display(),
        destination.display()
    );

    fs::copy(source, &destination).map_err(|e| {
        format!(
            "Failed to copy workbook {} -> {}: {e}",
            source.display(),
            destination.display()
        )
    })?;

    Ok(destination)
}

#[cfg(target_os = "ios")]
pub fn initialize_environment(app: &AppHandle) -> Result<(), String> {
    tracing::info!("Initializing iOS filesystem environment");

    let root = app_root()?;
    let imports_dir = root.join(IMPORT_FOLDER_NAME);
    let examples_dir = root.join(EXAMPLES_FOLDER_NAME);

    ensure_dir(&root)?;
    ensure_dir(&imports_dir)?;
    ensure_dir(&examples_dir)?;

    // Copy bundled examples into the sandbox for user visibility.
    for filename in EXAMPLE_FILES.iter() {
        let resource_paths = [
            format!("ExampleSpreadsheets/{filename}"),
            format!("assets/ExampleSpreadsheets/{filename}"),
            format!("assets/examples/{filename}"),
            format!("examples/{filename}"),
        ];

        let dest = examples_dir.join(filename);
        if dest.exists() {
            continue;
        }

        let mut copied = false;
        for resource in resource_paths {
            if let Ok(path) = app.path().resolve(&resource, BaseDirectory::Resource) {
                if path.exists() {
                    tracing::info!("Copying bundled example {filename} from {}", path.display());
                    if let Err(err) = fs::copy(&path, &dest) {
                        tracing::warn!(
                            "Failed to copy example {} -> {}: {err}",
                            path.display(),
                            dest.display()
                        );
                    } else {
                        copied = true;
                        break;
                    }
                }
            }
        }

        if !copied {
            tracing::warn!("Failed to locate bundled example {filename}");
        }
    }

    Ok(())
}

#[cfg(not(target_os = "ios"))]
pub fn initialize_environment(_app: &AppHandle) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "ios")]
pub fn prepare_workbook_path(path: &Path) -> Result<PathBuf, String> {
    tracing::info!("Preparing workbook path: {}", path.display());

    if !is_usheet(path) {
        return Err(format!(
            "File is not a .usheet workbook: {}",
            path.display()
        ));
    }

    let app_dir = app_root()?;
    if path.starts_with(&app_dir) {
        tracing::debug!("Workbook already inside sandbox");
        return Ok(path.to_path_buf());
    }

    let mut destination = copy_into_app_dir(path)?;

    // Remove original if it was inside the system Inbox directory.
    if let Some(parent) = path.parent() {
        if parent
            .file_name()
            .and_then(OsStr::to_str)
            .map(|name| name.eq_ignore_ascii_case("Inbox"))
            .unwrap_or(false)
        {
            tracing::info!("Removing source Inbox file: {}", path.display());
            if let Err(err) = fs::remove_file(path) {
                tracing::warn!("Failed to remove inbox file {}: {err}", path.display());
            }
        }
    }

    // Always return the canonical path stored in sandbox.
    if let Ok(canon) = destination.canonicalize() {
        destination = canon;
    }

    Ok(destination)
}

#[cfg(not(target_os = "ios"))]
pub fn prepare_workbook_path(path: &Path) -> Result<PathBuf, String> {
    Ok(path.to_path_buf())
}

#[cfg(target_os = "ios")]
pub fn import_pending_documents() -> Result<Vec<String>, String> {
    let inbox_dir = document_root()?.join("Inbox");
    if !inbox_dir.exists() {
        return Ok(Vec::new());
    }

    let mut imported = Vec::new();
    for entry in fs::read_dir(&inbox_dir).map_err(|e| {
        format!(
            "Failed to read iOS Inbox directory {}: {e}",
            inbox_dir.display()
        )
    })? {
        let entry = entry.map_err(|e| format!("Failed to iterate inbox entry: {e}"))?;
        let path = entry.path();
        if is_usheet(&path) {
            match prepare_workbook_path(&path) {
                Ok(dest) => {
                    tracing::info!("Imported shared workbook: {}", dest.display());
                    imported.push(dest.to_string_lossy().to_string());
                }
                Err(err) => {
                    tracing::warn!("Failed to import shared workbook {}: {err}", path.display());
                }
            }
        }
    }

    Ok(imported)
}

#[cfg(not(target_os = "ios"))]
pub fn import_pending_documents() -> Result<Vec<String>, String> {
    Ok(Vec::new())
}

#[cfg(target_os = "ios")]
pub fn example_document_path(filename: &str) -> Result<Option<PathBuf>, String> {
    let examples_dir = app_root()?.join(EXAMPLES_FOLDER_NAME).join(filename);
    if examples_dir.exists() {
        return Ok(Some(examples_dir));
    }

    Ok(None)
}

#[cfg(not(target_os = "ios"))]
pub fn example_document_path(_filename: &str) -> Result<Option<PathBuf>, String> {
    Ok(None)
}
