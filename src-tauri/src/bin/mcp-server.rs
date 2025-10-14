// Unicel MCP Server Binary
// Exposes Unicel workbook operations via Model Context Protocol (JSON-RPC over STDIO)
//
// Usage:
//   unicel-mcp-server [workbook_file]
//
// If no workbook file is provided, starts with an empty workbook.
// Reads JSON-RPC requests from stdin and writes responses to stdout.

use std::env;
use std::path::PathBuf;
use unicel_lib::core::units::UnitLibrary;
use unicel_lib::core::workbook::Workbook;
use unicel_lib::formats::json::WorkbookFile;
use unicel_lib::mcp::McpServer;

fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_writer(std::io::stderr) // Log to stderr to keep stdout clean for JSON-RPC
        .init();

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    // Load or create workbook
    let workbook = if args.len() > 1 {
        let path = PathBuf::from(&args[1]);
        match WorkbookFile::load_from_file(&path) {
            Ok(file) => match file.to_workbook() {
                Ok(wb) => {
                    tracing::info!("Loaded workbook from: {:?}", path);
                    wb
                }
                Err(e) => {
                    tracing::error!("Failed to deserialize workbook from {:?}: {}", path, e);
                    tracing::info!("Starting with empty workbook");
                    Workbook::new("Untitled")
                }
            },
            Err(e) => {
                tracing::error!("Failed to load file {:?}: {}", path, e);
                tracing::info!("Starting with empty workbook");
                Workbook::new("Untitled")
            }
        }
    } else {
        tracing::info!("Starting with empty workbook");
        Workbook::new("Untitled")
    };

    // Create unit library
    let unit_library = UnitLibrary::new();

    // Create and run MCP server
    let mut server = McpServer::new(workbook, unit_library);

    if let Err(e) = server.run() {
        tracing::error!("Server error: {}", e);
        std::process::exit(1);
    }
}
