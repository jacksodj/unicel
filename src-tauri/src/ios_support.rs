//! iOS-specific filesystem helpers for handling shared documents and bundled resources.

use std::path::{Path, PathBuf};

#[cfg(target_os = "ios")]
use std::{collections::VecDeque, ffi::OsStr, fmt, fs, sync::Mutex};

#[cfg(target_os = "ios")]
use once_cell::sync::Lazy;
#[cfg(not(target_os = "ios"))]
use tauri::AppHandle;
#[cfg(target_os = "ios")]
use tauri::{path::BaseDirectory, AppHandle, Manager};
#[cfg(target_os = "ios")]
use tracing::{
    field::{Field, Visit},
    Event, Subscriber,
};
#[cfg(target_os = "ios")]
use tracing_subscriber::{
    layer::{Context, Layer},
    registry::LookupSpan,
};

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
const LOG_CAPACITY: usize = 500;

#[cfg(target_os = "ios")]
static LOG_BUFFER: Lazy<Mutex<VecDeque<String>>> =
    Lazy::new(|| Mutex::new(VecDeque::with_capacity(LOG_CAPACITY)));

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
fn push_log_line(line: String) {
    let mut buffer = LOG_BUFFER.lock().unwrap();
    if buffer.len() == LOG_CAPACITY {
        buffer.pop_front();
    }
    buffer.push_back(line);
}

#[cfg(target_os = "ios")]
#[derive(Default)]
struct EventFieldCollector {
    message: Option<String>,
    fields: Vec<String>,
}

#[cfg(target_os = "ios")]
impl EventFieldCollector {
    fn push_formatted(&mut self, field: &Field, value: String) {
        if field.name() == "message" {
            if self.message.is_none() {
                self.message = Some(value);
            } else {
                // Preserve additional message-like fields alongside other key-value pairs.
                self.fields.push(format!("{}={value}", field.name()));
            }
        } else {
            self.fields.push(format!("{}={value}", field.name()));
        }
    }
}

#[cfg(target_os = "ios")]
impl Visit for EventFieldCollector {
    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "message" {
            self.message = Some(value.to_string());
        } else {
            self.push_formatted(field, format!("\"{value}\""));
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.push_formatted(field, format!("{value:?}"));
    }
}

#[cfg(target_os = "ios")]
fn formatted_event_string<S>(event: &Event<'_>, ctx: Context<'_, S>) -> Option<String>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let metadata = event.metadata();
    let mut collector = EventFieldCollector::default();
    event.record(&mut collector);

    let mut line = String::new();
    line.push_str(metadata.level().as_str());
    line.push(' ');
    line.push_str(metadata.target());
    line.push(':');

    if let Some(scope) = ctx.event_scope(event) {
        let mut first = true;
        line.push(' ');
        for span in scope.from_root() {
            if !first {
                line.push_str("::");
            }
            first = false;
            line.push_str(span.name());
        }
        if first {
            // No spans were written, remove the extra space.
            line.pop();
        } else {
            line.push(' ');
        }
    } else {
        line.push(' ');
    }

    let message = collector.message;
    if let Some(message_text) = &message {
        line.push_str(message_text);
    }

    let had_message = message.is_some();
    if !collector.fields.is_empty() {
        if had_message {
            line.push(' ');
        }
        line.push_str(&collector.fields.join(" "));
    }

    let trimmed = line.trim_end().to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

#[cfg(target_os = "ios")]
#[derive(Default)]
pub struct LogCaptureLayer;

#[cfg(target_os = "ios")]
impl<S> Layer<S> for LogCaptureLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        if let Some(line) = formatted_event_string(event, ctx) {
            push_log_line(line);
        }
    }
}

#[cfg(target_os = "ios")]
pub fn collect_recent_logs(limit: usize) -> String {
    let buffer = LOG_BUFFER.lock().unwrap();
    let count = buffer.len().min(limit);
    buffer
        .iter()
        .rev()
        .take(count)
        .cloned()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(not(target_os = "ios"))]
pub fn collect_recent_logs(_limit: usize) -> String {
    String::new()
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
