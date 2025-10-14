// MCP Server Implementation
// Handles JSON-RPC requests over STDIO

use super::super::tools::{get_tool_definitions, ToolHandler};
use super::super::types::*;
use crate::core::{units::UnitLibrary, workbook::Workbook};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::sync::{Arc, Mutex};
use tracing::{debug, error, info, warn};

pub struct McpServer {
    workbook: Arc<Mutex<Workbook>>,
    unit_library: Arc<UnitLibrary>,
    tool_handler: ToolHandler,
    initialized: bool,
}

impl McpServer {
    pub fn new(workbook: Workbook, unit_library: UnitLibrary) -> Self {
        let workbook = Arc::new(Mutex::new(workbook));
        let unit_library = Arc::new(unit_library);
        let tool_handler = ToolHandler::new(Arc::clone(&workbook), Arc::clone(&unit_library));

        Self {
            workbook,
            unit_library,
            tool_handler,
            initialized: false,
        }
    }

    /// Start the MCP server, reading from stdin and writing to stdout
    pub fn run(&mut self) -> io::Result<()> {
        info!("Starting Unicel MCP Server");
        info!("Protocol: Model Context Protocol (JSON-RPC 2.0)");
        info!("Transport: STDIO");

        let stdin = io::stdin();
        let mut stdout = io::stdout();

        for line in stdin.lock().lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }

            debug!("Received request: {}", line);

            // Parse JSON-RPC request
            let request: JsonRpcRequest = match serde_json::from_str(&line) {
                Ok(req) => req,
                Err(e) => {
                    error!("Failed to parse request: {}", e);
                    let response = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: None,
                        result: None,
                        error: Some(JsonRpcError::parse_error()),
                    };
                    self.write_response(&mut stdout, &response)?;
                    continue;
                }
            };

            // Handle request
            let response = self.handle_request(request);

            // Write response
            self.write_response(&mut stdout, &response)?;
        }

        info!("MCP Server shutting down");
        Ok(())
    }

    fn handle_request(&mut self, request: JsonRpcRequest) -> JsonRpcResponse {
        if request.jsonrpc != "2.0" {
            return JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(JsonRpcError::invalid_request()),
            };
        }

        let result = match request.method.as_str() {
            "initialize" => self.handle_initialize(request.params),
            "initialized" => self.handle_initialized(),
            "tools/list" => self.handle_list_tools(),
            "tools/call" => self.handle_call_tool(request.params),
            "resources/list" => self.handle_list_resources(),
            "resources/read" => self.handle_read_resource(request.params),
            method => {
                warn!("Unknown method: {}", method);
                return JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: None,
                    error: Some(JsonRpcError::method_not_found(method)),
                };
            }
        };

        match result {
            Ok(result) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: Some(result),
                error: None,
            },
            Err(error) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(error),
            },
        }
    }

    fn handle_initialize(&mut self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let _params: InitializeParams = match params {
            Some(p) => serde_json::from_value(p)
                .map_err(|e| JsonRpcError::invalid_params(&format!("Invalid initialize params: {}", e)))?,
            None => return Err(JsonRpcError::invalid_params("Missing initialize params")),
        };

        self.initialized = true;

        let result = InitializeResult {
            protocol_version: "2024-11-05".to_string(),
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability {
                    list_changed: Some(false),
                }),
                resources: Some(ResourcesCapability {
                    list_changed: Some(false),
                    subscribe: Some(false),
                }),
            },
            server_info: ServerInfo {
                name: "unicel-mcp-server".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
        };

        info!("MCP Server initialized");
        serde_json::to_value(result).map_err(|e| JsonRpcError::internal_error(&e.to_string()))
    }

    fn handle_initialized(&self) -> Result<Value, JsonRpcError> {
        // Notification that client is ready - no response needed
        Ok(json!({}))
    }

    fn handle_list_tools(&self) -> Result<Value, JsonRpcError> {
        if !self.initialized {
            return Err(JsonRpcError::internal_error("Server not initialized"));
        }

        let result = ListToolsResult {
            tools: get_tool_definitions(),
        };

        debug!("Listed {} tools", result.tools.len());
        serde_json::to_value(result).map_err(|e| JsonRpcError::internal_error(&e.to_string()))
    }

    fn handle_call_tool(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        if !self.initialized {
            return Err(JsonRpcError::internal_error("Server not initialized"));
        }

        let params: CallToolParams = match params {
            Some(p) => serde_json::from_value(p)
                .map_err(|e| JsonRpcError::invalid_params(&format!("Invalid tool params: {}", e)))?,
            None => return Err(JsonRpcError::invalid_params("Missing tool params")),
        };

        info!("Calling tool: {}", params.name);

        let result = self.tool_handler.handle_tool_call(&params.name, params.arguments);

        serde_json::to_value(result).map_err(|e| JsonRpcError::internal_error(&e.to_string()))
    }

    fn handle_list_resources(&self) -> Result<Value, JsonRpcError> {
        if !self.initialized {
            return Err(JsonRpcError::internal_error("Server not initialized"));
        }

        let workbook = self.workbook.lock().unwrap();

        let mut resources = vec![
            ResourceDefinition {
                uri: "unicel://workbook".to_string(),
                name: "Workbook Metadata".to_string(),
                description: Some("Current workbook metadata including sheets and settings".to_string()),
                mime_type: Some("application/json".to_string()),
            },
        ];

        // Add resources for each sheet
        for i in 0..workbook.sheet_count() {
            if let Some(sheet) = workbook.get_sheet(i) {
                resources.push(ResourceDefinition {
                    uri: format!("unicel://sheet/{}", sheet.name()),
                    name: format!("Sheet: {}", sheet.name()),
                    description: Some(format!("Data from sheet '{}'", sheet.name())),
                    mime_type: Some("application/json".to_string()),
                });
            }
        }

        resources.push(ResourceDefinition {
            uri: "unicel://units/domains".to_string(),
            name: "Unit Domains".to_string(),
            description: Some("All available unit domains and units".to_string()),
            mime_type: Some("application/json".to_string()),
        });

        let result = ListResourcesResult { resources };

        debug!("Listed {} resources", result.resources.len());
        serde_json::to_value(result).map_err(|e| JsonRpcError::internal_error(&e.to_string()))
    }

    fn handle_read_resource(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        if !self.initialized {
            return Err(JsonRpcError::internal_error("Server not initialized"));
        }

        let params: ReadResourceParams = match params {
            Some(p) => serde_json::from_value(p)
                .map_err(|e| JsonRpcError::invalid_params(&format!("Invalid resource params: {}", e)))?,
            None => return Err(JsonRpcError::invalid_params("Missing resource params")),
        };

        info!("Reading resource: {}", params.uri);

        let content = self.read_resource_content(&params.uri)?;

        let result = ReadResourceResult {
            contents: vec![content],
        };

        serde_json::to_value(result).map_err(|e| JsonRpcError::internal_error(&e.to_string()))
    }

    fn read_resource_content(&self, uri: &str) -> Result<ResourceContent, JsonRpcError> {
        if uri == "unicel://workbook" {
            let workbook = self.workbook.lock().unwrap();

            let mut sheets = Vec::new();
            for i in 0..workbook.sheet_count() {
                if let Some(sheet) = workbook.get_sheet(i) {
                    sheets.push(json!({
                        "name": sheet.name(),
                        "cell_count": sheet.cell_count(),
                    }));
                }
            }

            let data = json!({
                "name": workbook.name(),
                "sheet_count": sheets.len(),
                "sheets": sheets,
                "active_sheet": workbook.active_sheet().name(),
                "display_preference": format!("{:?}", workbook.settings().display_preference),
            });

            Ok(ResourceContent {
                uri: uri.to_string(),
                mime_type: Some("application/json".to_string()),
                text: Some(serde_json::to_string_pretty(&data).unwrap()),
            })
        } else if uri.starts_with("unicel://sheet/") {
            let sheet_name = uri.strip_prefix("unicel://sheet/").unwrap();
            let workbook = self.workbook.lock().unwrap();

            let sheet = workbook.get_sheet_by_name(sheet_name)
                .ok_or_else(|| JsonRpcError::invalid_params(&format!("Sheet not found: {}", sheet_name)))?;

            let cells: Vec<Value> = sheet
                .cell_addresses()
                .iter()
                .filter_map(|addr| {
                    sheet.get(addr).map(|cell| {
                        json!({
                            "cell_ref": format!("{}", addr),
                            "value": match cell.value() {
                                crate::core::cell::CellValue::Number(n) => json!(n),
                                crate::core::cell::CellValue::Text(t) => json!(t),
                                crate::core::cell::CellValue::Error(e) => json!({"error": e}),
                                crate::core::cell::CellValue::Empty => json!(null),
                            },
                            "unit": cell.storage_unit().to_string(),
                        })
                    })
                })
                .collect();

            let data = json!({
                "sheet_name": sheet.name(),
                "cell_count": cells.len(),
                "cells": cells,
            });

            Ok(ResourceContent {
                uri: uri.to_string(),
                mime_type: Some("application/json".to_string()),
                text: Some(serde_json::to_string_pretty(&data).unwrap()),
            })
        } else if uri == "unicel://units/domains" {
            // Hardcoded list of known units (same as in tools.rs)
            let all_units = vec![
                // Length
                "m", "cm", "mm", "km", "in", "ft", "yd", "mi",
                // Mass
                "g", "kg", "mg", "oz", "lb",
                // Time
                "s", "min", "hr", "h", "hour", "day", "month", "year",
                // Temperature
                "C", "F", "K",
                // Currency
                "USD", "EUR", "GBP",
                // Digital storage
                "B", "KB", "MB", "GB", "TB", "PB",
                "b", "Kb", "Mb", "Gb", "Tb", "Pb",
                "Tok", "MTok",
            ];

            let data = json!({
                "unit_count": all_units.len(),
                "units": all_units,
            });

            Ok(ResourceContent {
                uri: uri.to_string(),
                mime_type: Some("application/json".to_string()),
                text: Some(serde_json::to_string_pretty(&data).unwrap()),
            })
        } else {
            Err(JsonRpcError::invalid_params(&format!("Unknown resource URI: {}", uri)))
        }
    }

    fn write_response<W: Write>(&self, writer: &mut W, response: &JsonRpcResponse) -> io::Result<()> {
        let json = serde_json::to_string(response)?;
        debug!("Sending response: {}", json);
        writeln!(writer, "{}", json)?;
        writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let workbook = Workbook::new("Test");
        let unit_library = UnitLibrary::new();
        let _server = McpServer::new(workbook, unit_library);
    }
}
