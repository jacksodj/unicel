//! iOS-specific filesystem helpers for handling shared documents and bundled resources.

use std::path::{Path, PathBuf};

#[cfg(target_os = "ios")]
use std::{ffi::OsStr, fs};

#[cfg(target_os = "ios")]
use once_cell::sync::Lazy;
#[cfg(not(target_os = "ios"))]
use tauri::AppHandle;
#[cfg(target_os = "ios")]
use tauri::{path::BaseDirectory, AppHandle, Manager};

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
const ICLOUD_CONTAINER_ID: &str = "iCloud.com.unicel.app";

#[cfg(target_os = "ios")]
fn transform_container_id(id: &str) -> String {
    let mut parts = id.split('.');
    let mut name = String::new();
    if let Some(first) = parts.next() {
        name.push_str(first);
    }
    for part in parts {
        name.push('~');
        name.push_str(part);
    }
    name
}

#[cfg(target_os = "ios")]
fn document_root() -> Result<PathBuf, String> {
    dirs::document_dir()
        .map(PathBuf::from)
        .ok_or_else(|| "Failed to resolve iOS documents directory".to_string())
}

#[cfg(target_os = "ios")]
fn app_root() -> Result<PathBuf, String> {
    Ok(document_root()?.join(APP_FOLDER_NAME))
}

#[cfg(target_os = "ios")]
fn icloud_documents_root() -> Result<Option<PathBuf>, String> {
    let home = match dirs::home_dir() {
        Some(path) => path,
        None => return Ok(None),
    };

    let relative = transform_container_id(ICLOUD_CONTAINER_ID);
    let container_path = home
        .join("Library")
        .join("Mobile Documents")
        .join(relative)
        .join("Documents");

    match fs::create_dir_all(&container_path) {
        Ok(_) => Ok(Some(container_path)),
        Err(err) => {
            tracing::warn!(
                "iCloud container unavailable ({}): {err}",
                container_path.display()
            );
            Ok(None)
        }
    }
}

#[cfg(target_os = "ios")]
fn icloud_app_root() -> Result<Option<PathBuf>, String> {
    if let Some(docs) = icloud_documents_root()? {
        let app_dir = docs.join(APP_FOLDER_NAME);
        if let Err(err) = ensure_dir(&app_dir) {
            tracing::warn!(
                "Failed to prepare iCloud app directory {}: {err}",
                app_dir.display()
            );
            return Ok(None);
        }
        Ok(Some(app_dir))
    } else {
        Ok(None)
    }
}

#[cfg(target_os = "ios")]
fn storage_roots() -> Result<Vec<PathBuf>, String> {
    let mut roots = vec![app_root()?];
    if let Some(icloud) = icloud_app_root()? {
        roots.push(icloud);
    }
    Ok(roots)
}

#[cfg(target_os = "ios")]
fn copy_examples_to_roots(app: &AppHandle, roots: &[PathBuf]) -> Result<(), String> {
    for filename in EXAMPLE_FILES.iter() {
        let mut source_path: Option<PathBuf> = None;
        let resource_candidates = [
            format!("ExampleSpreadsheets/{filename}"),
            format!("assets/ExampleSpreadsheets/{filename}"),
            format!("assets/examples/{filename}"),
            format!("examples/{filename}"),
        ];

        for candidate in resource_candidates {
            if let Ok(path) = app.path().resolve(&candidate, BaseDirectory::Resource) {
                if path.exists() {
                    source_path = Some(path);
                    break;
                }
            }
        }

        let Some(source) = source_path else {
            tracing::warn!("Failed to locate bundled example {filename}");
            continue;
        };

        for root in roots {
            let dest = root.join(EXAMPLES_FOLDER_NAME).join(filename);
            if dest.exists() {
                continue;
            }
            tracing::info!("Seeding example {} into {}", filename, dest.display());
            if let Err(err) = fs::copy(&source, &dest) {
                tracing::warn!(
                    "Failed to copy example {} -> {}: {err}",
                    source.display(),
                    dest.display()
                );
            }
        }
    }

    Ok(())
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

    let roots = storage_roots()?;
    let primary_imports = roots
        .first()
        .ok_or_else(|| "Missing primary storage root".to_string())?
        .join(IMPORT_FOLDER_NAME);
    ensure_dir(&primary_imports)?;

    let destination = unique_destination(&primary_imports, filename);

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

    // Mirror into any additional storage roots (iCloud).
    for root in roots.iter().skip(1) {
        let mirror_dir = root.join(IMPORT_FOLDER_NAME);
        if ensure_dir(&mirror_dir).is_ok() {
            let mirror_dest = unique_destination(&mirror_dir, filename);
            if let Err(err) = fs::copy(&destination, &mirror_dest) {
                tracing::warn!(
                    "Failed to mirror workbook into {}: {err}",
                    mirror_dest.display()
                );
            }
        }
    }

    Ok(destination)
}

#[cfg(target_os = "ios")]
pub fn initialize_environment(app: &AppHandle) -> Result<(), String> {
    tracing::info!("Initializing iOS filesystem environment");

    let roots = storage_roots()?;
    for root in &roots {
        ensure_dir(root)?;
        ensure_dir(&root.join(IMPORT_FOLDER_NAME))?;
        ensure_dir(&root.join(EXAMPLES_FOLDER_NAME))?;
    }

    copy_examples_to_roots(app, &roots)?;

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
    let mut imported = Vec::new();
    let mut inbox_dirs = Vec::new();
    inbox_dirs.push(document_root()?.join("Inbox"));
    if let Some(icloud_docs) = icloud_documents_root()? {
        inbox_dirs.push(icloud_docs.join("Inbox"));
    }

    for inbox_dir in inbox_dirs {
        if !inbox_dir.exists() {
            continue;
        }
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
                        tracing::warn!(
                            "Failed to import shared workbook {}: {err}",
                            path.display()
                        );
                    }
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
    let roots = storage_roots()?;
    for root in roots {
        let candidate = root.join(EXAMPLES_FOLDER_NAME).join(filename);
        if candidate.exists() {
            return Ok(Some(candidate));
        }
    }

    Ok(None)
}

#[cfg(not(target_os = "ios"))]
pub fn example_document_path(_filename: &str) -> Result<Option<PathBuf>, String> {
    Ok(None)
}
