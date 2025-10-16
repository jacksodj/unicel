// MCP Tool Definitions and Handlers for Unicel

use super::types::*;
use crate::core::{
    cell::{Cell, CellValue},
    table::CellAddr,
    units::{parse_unit, Unit, UnitLibrary},
    workbook::Workbook,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Registry of all available tools
pub fn get_tool_definitions() -> Vec<ToolDefinition> {
    vec![
        // Read tools
        define_read_cell(),
        define_read_range(),
        define_query_table(),
        define_get_sheet_structure(),
        define_list_tables(),
        // Write tools
        define_write_cell(),
        define_write_range(),
        define_append_row(),
        // Conversion tools
        define_convert_value(),
        define_get_conversion_rate(),
        define_list_compatible_units(),
        define_validate_unit(),
        // Schema tools
        define_get_workbook_metadata(),
    ]
}

// ============================================================================
// Tool Definitions
// ============================================================================

fn define_read_cell() -> ToolDefinition {
    ToolDefinition {
        name: "read_cell".to_string(),
        description:
            "Read a single cell with full metadata including value, unit, formula, and warnings"
                .to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "cell_ref": {
                    "type": "string",
                    "description": "Cell reference (e.g., 'A1', 'B5')"
                },
                "sheet_name": {
                    "type": "string",
                    "description": "Sheet name (optional, defaults to active sheet)"
                }
            },
            "required": ["cell_ref"]
        }),
    }
}

fn define_read_range() -> ToolDefinition {
    ToolDefinition {
        name: "read_range".to_string(),
        description: "Read a range of cells efficiently in a single operation".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "range": {
                    "type": "string",
                    "description": "Cell range (e.g., 'A1:B10')"
                },
                "sheet_name": {
                    "type": "string",
                    "description": "Sheet name (optional, defaults to active sheet)"
                }
            },
            "required": ["range"]
        }),
    }
}

fn define_query_table() -> ToolDefinition {
    ToolDefinition {
        name: "query_table".to_string(),
        description: "Query a table using SQL with unit-aware operations. WHERE clauses automatically handle unit conversions.".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "table_name": {
                    "type": "string",
                    "description": "Name of the table (typically the sheet name)"
                },
                "sql": {
                    "type": "string",
                    "description": "SQL SELECT statement (e.g., 'SELECT InstanceType, RAM FROM EC2Instances WHERE RAM > 16GB')"
                }
            },
            "required": ["table_name", "sql"]
        }),
    }
}

fn define_get_sheet_structure() -> ToolDefinition {
    ToolDefinition {
        name: "get_sheet_structure".to_string(),
        description:
            "Get the structure of a sheet including dimensions, used ranges, and table metadata"
                .to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "sheet_name": {
                    "type": "string",
                    "description": "Sheet name (optional, defaults to active sheet)"
                }
            }
        }),
    }
}

fn define_list_tables() -> ToolDefinition {
    ToolDefinition {
        name: "list_tables".to_string(),
        description: "List all available tables (sheets) in the workbook with their metadata"
            .to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    }
}

fn define_write_cell() -> ToolDefinition {
    ToolDefinition {
        name: "write_cell".to_string(),
        description: "Write a value to a single cell with optional unit and validation".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "cell_ref": {
                    "type": "string",
                    "description": "Cell reference (e.g., 'A1', 'B5')"
                },
                "value": {
                    "description": "Cell value (number, string, or formula)"
                },
                "unit": {
                    "type": "string",
                    "description": "Unit for the value (optional)"
                },
                "sheet_name": {
                    "type": "string",
                    "description": "Sheet name (optional, defaults to active sheet)"
                },
                "validate": {
                    "type": "boolean",
                    "description": "Whether to validate the unit and value (default: true)"
                }
            },
            "required": ["cell_ref", "value"]
        }),
    }
}

fn define_write_range() -> ToolDefinition {
    ToolDefinition {
        name: "write_range".to_string(),
        description: "Write multiple cells in a single operation for better performance"
            .to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "cells": {
                    "type": "array",
                    "description": "Array of cell writes",
                    "items": {
                        "type": "object",
                        "properties": {
                            "cell_ref": { "type": "string" },
                            "value": {},
                            "unit": { "type": "string" }
                        },
                        "required": ["cell_ref", "value"]
                    }
                },
                "sheet_name": {
                    "type": "string",
                    "description": "Sheet name (optional, defaults to active sheet)"
                }
            },
            "required": ["cells"]
        }),
    }
}

fn define_append_row() -> ToolDefinition {
    ToolDefinition {
        name: "append_row".to_string(),
        description: "Append a new row to a table, respecting column schema and units".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "table_name": {
                    "type": "string",
                    "description": "Name of the table (sheet)"
                },
                "row_data": {
                    "type": "object",
                    "description": "Object mapping column names to values with optional units"
                }
            },
            "required": ["table_name", "row_data"]
        }),
    }
}

fn define_convert_value() -> ToolDefinition {
    ToolDefinition {
        name: "convert_value".to_string(),
        description: "Convert a value from one unit to another with metadata about the conversion"
            .to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "value": {
                    "type": "number",
                    "description": "Value to convert"
                },
                "from_unit": {
                    "type": "string",
                    "description": "Source unit (e.g., 'USD', 'm', 'GB')"
                },
                "to_unit": {
                    "type": "string",
                    "description": "Target unit (e.g., 'EUR', 'ft', 'TB')"
                },
                "include_path": {
                    "type": "boolean",
                    "description": "Whether to include conversion path metadata (default: false)"
                }
            },
            "required": ["value", "from_unit", "to_unit"]
        }),
    }
}

fn define_get_conversion_rate() -> ToolDefinition {
    ToolDefinition {
        name: "get_conversion_rate".to_string(),
        description: "Get the current conversion rate between two units with metadata".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "from_unit": {
                    "type": "string",
                    "description": "Source unit"
                },
                "to_unit": {
                    "type": "string",
                    "description": "Target unit"
                }
            },
            "required": ["from_unit", "to_unit"]
        }),
    }
}

fn define_list_compatible_units() -> ToolDefinition {
    ToolDefinition {
        name: "list_compatible_units".to_string(),
        description: "List all units compatible with a given unit (same dimension)".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "unit": {
                    "type": "string",
                    "description": "Unit to check compatibility for"
                }
            },
            "required": ["unit"]
        }),
    }
}

fn define_validate_unit() -> ToolDefinition {
    ToolDefinition {
        name: "validate_unit".to_string(),
        description: "Validate if a unit is recognized and get its canonical form".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "unit": {
                    "type": "string",
                    "description": "Unit string to validate"
                }
            },
            "required": ["unit"]
        }),
    }
}

fn define_get_workbook_metadata() -> ToolDefinition {
    ToolDefinition {
        name: "get_workbook_metadata".to_string(),
        description:
            "Get metadata about the entire workbook including sheets, settings, and units in use"
                .to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    }
}

// ============================================================================
// Tool Handlers
// ============================================================================

pub struct ToolHandler {
    workbook: Arc<Mutex<Workbook>>,
    unit_library: Arc<UnitLibrary>,
}

impl ToolHandler {
    pub fn new(workbook: Arc<Mutex<Workbook>>, unit_library: Arc<UnitLibrary>) -> Self {
        Self {
            workbook,
            unit_library,
        }
    }

    pub fn handle_tool_call(
        &self,
        name: &str,
        arguments: Option<HashMap<String, Value>>,
    ) -> CallToolResult {
        let args = arguments.unwrap_or_default();

        let result = match name {
            "read_cell" => self.handle_read_cell(args),
            "read_range" => self.handle_read_range(args),
            "query_table" => self.handle_query_table(args),
            "get_sheet_structure" => self.handle_get_sheet_structure(args),
            "list_tables" => self.handle_list_tables(args),
            "write_cell" => self.handle_write_cell(args),
            "write_range" => self.handle_write_range(args),
            "append_row" => self.handle_append_row(args),
            "convert_value" => self.handle_convert_value(args),
            "get_conversion_rate" => self.handle_get_conversion_rate(args),
            "list_compatible_units" => self.handle_list_compatible_units(args),
            "validate_unit" => self.handle_validate_unit(args),
            "get_workbook_metadata" => self.handle_get_workbook_metadata(args),
            _ => Err(format!("Unknown tool: {}", name)),
        };

        match result {
            Ok(content) => CallToolResult {
                content: vec![ToolContent::Text { text: content }],
                is_error: Some(false),
            },
            Err(error) => CallToolResult {
                content: vec![ToolContent::Text { text: error }],
                is_error: Some(true),
            },
        }
    }

    fn handle_read_cell(&self, args: HashMap<String, Value>) -> Result<String, String> {
        let cell_ref = args
            .get("cell_ref")
            .and_then(|v| v.as_str())
            .ok_or("Missing cell_ref")?;

        let addr = CellAddr::from_string(cell_ref)
            .map_err(|e| format!("Invalid cell reference: {}", e))?;

        let workbook = self.workbook.lock().unwrap();
        let sheet = workbook.active_sheet();

        let cell = sheet
            .get(&addr)
            .ok_or_else(|| format!("Cell {} is empty or does not exist", cell_ref))?;

        let result = json!({
            "cell_ref": cell_ref,
            "value": match cell.value() {
                CellValue::Number(n) => json!(n),
                CellValue::Text(t) => json!(t),
                CellValue::Error(e) => json!({"error": e}),
                CellValue::Empty => json!(null),
            },
            "unit": {
                "canonical": cell.storage_unit().to_string(),
                "dimension": format!("{:?}", cell.storage_unit().dimension()),
                "display": cell.display_unit().to_string(),
            },
            "formula": cell.formula().map(|f| f.to_string()),
            "warnings": cell.warning().map(|w| vec![w.clone()]).unwrap_or_default(),
            "is_empty": cell.is_empty(),
            "is_number": cell.is_number(),
            "is_text": cell.is_text(),
            "is_error": cell.is_error(),
        });

        Ok(serde_json::to_string_pretty(&result).unwrap())
    }

    fn handle_read_range(&self, _args: HashMap<String, Value>) -> Result<String, String> {
        // TODO: Implement range reading
        Err("read_range not yet implemented".to_string())
    }

    fn handle_query_table(&self, _args: HashMap<String, Value>) -> Result<String, String> {
        // TODO: Implement SQL query support
        Err("query_table not yet implemented".to_string())
    }

    fn handle_get_sheet_structure(&self, _args: HashMap<String, Value>) -> Result<String, String> {
        let workbook = self.workbook.lock().unwrap();
        let sheet = workbook.active_sheet();

        let cell_addrs = sheet.cell_addresses();
        let used_cells: Vec<String> = cell_addrs.iter().map(|addr| format!("{}", addr)).collect();

        let result = json!({
            "sheet_name": sheet.name(),
            "used_cells": used_cells.len(),
            "cell_references": used_cells,
        });

        Ok(serde_json::to_string_pretty(&result).unwrap())
    }

    fn handle_list_tables(&self, _args: HashMap<String, Value>) -> Result<String, String> {
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

        let result = json!({
            "tables": sheets,
            "active_sheet": workbook.active_sheet().name(),
        });

        Ok(serde_json::to_string_pretty(&result).unwrap())
    }

    fn handle_write_cell(&self, args: HashMap<String, Value>) -> Result<String, String> {
        let cell_ref = args
            .get("cell_ref")
            .and_then(|v| v.as_str())
            .ok_or("Missing cell_ref")?;

        let addr = CellAddr::from_string(cell_ref)
            .map_err(|e| format!("Invalid cell reference: {}", e))?;

        let value = args.get("value").ok_or("Missing value")?;

        let unit_str = args.get("unit").and_then(|v| v.as_str());

        let mut workbook = self.workbook.lock().unwrap();
        let sheet = workbook.active_sheet_mut();

        // Determine cell type and create appropriate cell
        let cell = if let Some(s) = value.as_str() {
            if s.starts_with('=') {
                // Formula
                Cell::with_formula(s)
            } else {
                // Text
                Cell::with_text(s)
            }
        } else if let Some(n) = value.as_f64() {
            // Number with optional unit
            if let Some(unit_str) = unit_str {
                let unit = parse_unit(unit_str, &self.unit_library)
                    .map_err(|e| format!("Invalid unit '{}': {}", unit_str, e))?;
                Cell::new(n, unit)
            } else {
                Cell::new(n, Unit::dimensionless())
            }
        } else {
            return Err("Invalid value type".to_string());
        };

        sheet
            .set(addr.clone(), cell)
            .map_err(|e| format!("Error writing cell: {}", e))?;

        let result = json!({
            "success": true,
            "cell_ref": cell_ref,
        });

        Ok(serde_json::to_string_pretty(&result).unwrap())
    }

    fn handle_write_range(&self, _args: HashMap<String, Value>) -> Result<String, String> {
        // TODO: Implement batch write
        Err("write_range not yet implemented".to_string())
    }

    fn handle_append_row(&self, _args: HashMap<String, Value>) -> Result<String, String> {
        // TODO: Implement append row
        Err("append_row not yet implemented".to_string())
    }

    fn handle_convert_value(&self, args: HashMap<String, Value>) -> Result<String, String> {
        let value = args
            .get("value")
            .and_then(|v| v.as_f64())
            .ok_or("Missing or invalid value")?;

        let from_unit = args
            .get("from_unit")
            .and_then(|v| v.as_str())
            .ok_or("Missing from_unit")?;

        let to_unit = args
            .get("to_unit")
            .and_then(|v| v.as_str())
            .ok_or("Missing to_unit")?;

        let converted = self
            .unit_library
            .convert(value, from_unit, to_unit)
            .ok_or_else(|| format!("Cannot convert from {} to {}", from_unit, to_unit))?;

        let result = json!({
            "original": {
                "value": value,
                "unit": from_unit,
            },
            "converted": {
                "value": converted,
                "unit": to_unit,
            },
            "conversion_rate": converted / value,
        });

        Ok(serde_json::to_string_pretty(&result).unwrap())
    }

    fn handle_get_conversion_rate(&self, args: HashMap<String, Value>) -> Result<String, String> {
        let from_unit = args
            .get("from_unit")
            .and_then(|v| v.as_str())
            .ok_or("Missing from_unit")?;

        let to_unit = args
            .get("to_unit")
            .and_then(|v| v.as_str())
            .ok_or("Missing to_unit")?;

        // Convert 1.0 to get the rate
        let rate = self
            .unit_library
            .convert(1.0, from_unit, to_unit)
            .ok_or_else(|| format!("Cannot convert from {} to {}", from_unit, to_unit))?;

        let result = json!({
            "from_unit": from_unit,
            "to_unit": to_unit,
            "rate": rate,
            "formula": format!("1 {} = {} {}", from_unit, rate, to_unit),
        });

        Ok(serde_json::to_string_pretty(&result).unwrap())
    }

    fn handle_list_compatible_units(&self, args: HashMap<String, Value>) -> Result<String, String> {
        let unit_str = args
            .get("unit")
            .and_then(|v| v.as_str())
            .ok_or("Missing unit")?;

        let unit = parse_unit(unit_str, &self.unit_library)
            .map_err(|e| format!("Invalid unit '{}': {}", unit_str, e))?;

        // Known units by dimension (hardcoded for now)
        let all_known_units = vec![
            // Length
            "m", "cm", "mm", "km", "in", "ft", "yd", "mi", // Mass
            "g", "kg", "mg", "oz", "lb", // Time
            "s", "min", "hr", "h", "hour", "day", "month", "year", // Temperature
            "C", "F", "K", // Currency
            "USD", "EUR", "GBP", // Digital storage
            "B", "KB", "MB", "GB", "TB", "PB", "b", "Kb", "Mb", "Gb", "Tb", "Pb", "Tok", "MTok",
        ];

        // Filter for compatible units
        let compatible: Vec<String> = all_known_units
            .iter()
            .filter_map(|u| {
                if let Ok(other_unit) = parse_unit(u, &self.unit_library) {
                    if other_unit.is_compatible(&unit) {
                        Some(u.to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        let result = json!({
            "unit": unit_str,
            "dimension": format!("{:?}", unit.dimension()),
            "compatible_units": compatible,
            "count": compatible.len(),
        });

        Ok(serde_json::to_string_pretty(&result).unwrap())
    }

    fn handle_validate_unit(&self, args: HashMap<String, Value>) -> Result<String, String> {
        let unit_str = args
            .get("unit")
            .and_then(|v| v.as_str())
            .ok_or("Missing unit")?;

        match parse_unit(unit_str, &self.unit_library) {
            Ok(unit) => {
                let result = json!({
                    "valid": true,
                    "input": unit_str,
                    "canonical": unit.to_string(),
                    "dimension": format!("{:?}", unit.dimension()),
                });
                Ok(serde_json::to_string_pretty(&result).unwrap())
            }
            Err(e) => {
                let result = json!({
                    "valid": false,
                    "input": unit_str,
                    "error": format!("{}", e),
                });
                Ok(serde_json::to_string_pretty(&result).unwrap())
            }
        }
    }

    fn handle_get_workbook_metadata(
        &self,
        _args: HashMap<String, Value>,
    ) -> Result<String, String> {
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

        let result = json!({
            "name": workbook.name(),
            "sheet_count": sheets.len(),
            "sheets": sheets,
            "active_sheet": workbook.active_sheet().name(),
            "display_preference": format!("{:?}", workbook.settings().display_preference),
        });

        Ok(serde_json::to_string_pretty(&result).unwrap())
    }
}
