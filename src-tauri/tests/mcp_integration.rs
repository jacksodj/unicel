// MCP Server Integration Tests

use serde_json::json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use unicel_lib::core::{
    cell::Cell,
    table::CellAddr,
    units::{BaseDimension, Unit, UnitLibrary},
    workbook::Workbook,
};
use unicel_lib::mcp::{
    get_tool_definitions,
    McpServer, ToolHandler,
};

#[test]
fn test_tool_definitions() {
    let tools = get_tool_definitions();

    // Verify we have the expected number of tools
    assert!(tools.len() >= 13, "Should have at least 13 tools defined");

    // Verify key tools are present
    let tool_names: Vec<String> = tools.iter().map(|t| t.name.clone()).collect();

    assert!(tool_names.contains(&"read_cell".to_string()));
    assert!(tool_names.contains(&"write_cell".to_string()));
    assert!(tool_names.contains(&"convert_value".to_string()));
    assert!(tool_names.contains(&"list_tables".to_string()));
    assert!(tool_names.contains(&"get_workbook_metadata".to_string()));

    // Verify each tool has required fields
    for tool in &tools {
        assert!(!tool.name.is_empty(), "Tool name should not be empty");
        assert!(
            !tool.description.is_empty(),
            "Tool description should not be empty"
        );
        // input_schema should be a valid JSON object
        assert!(
            tool.input_schema.is_object(),
            "Tool input_schema should be an object"
        );
    }
}

#[test]
fn test_read_cell_tool() {
    let mut workbook = Workbook::new("Test");
    let sheet = workbook.active_sheet_mut();

    // Set up test data
    let addr = CellAddr::new("A", 1);
    let cell = Cell::new(100.0, Unit::simple("USD", BaseDimension::Currency));
    sheet.set(addr, cell).unwrap();

    // Create tool handler
    let workbook = Arc::new(Mutex::new(workbook));
    let unit_library = Arc::new(UnitLibrary::new());
    let handler = ToolHandler::new(workbook, unit_library);

    // Call read_cell tool
    let mut args = HashMap::new();
    args.insert("cell_ref".to_string(), json!("A1"));

    let result = handler.handle_tool_call("read_cell", Some(args));

    assert_eq!(result.is_error, Some(false), "Should not be an error");
    assert_eq!(result.content.len(), 1, "Should have one content item");

    // Parse the JSON response
    if let unicel_lib::mcp::ToolContent::Text { text } = &result.content[0] {
        let response: serde_json::Value = serde_json::from_str(text).unwrap();

        assert_eq!(response["cell_ref"], "A1");
        assert_eq!(response["value"], 100.0);
        assert_eq!(response["unit"]["canonical"], "USD");
        assert_eq!(response["is_number"], true);
        assert_eq!(response["is_empty"], false);
    } else {
        panic!("Expected text content");
    }
}

#[test]
fn test_write_cell_tool() {
    let workbook = Workbook::new("Test");
    let workbook = Arc::new(Mutex::new(workbook));
    let unit_library = Arc::new(UnitLibrary::new());
    let handler = ToolHandler::new(Arc::clone(&workbook), unit_library);

    // Write a number with unit
    let mut args = HashMap::new();
    args.insert("cell_ref".to_string(), json!("B2"));
    args.insert("value".to_string(), json!(42.5));
    args.insert("unit".to_string(), json!("m"));

    let result = handler.handle_tool_call("write_cell", Some(args));

    assert_eq!(result.is_error, Some(false), "Should not be an error");

    // Verify the cell was written
    let workbook = workbook.lock().unwrap();
    let sheet = workbook.active_sheet();
    let addr = CellAddr::new("B", 2);
    let cell = sheet.get(&addr).unwrap();

    assert_eq!(cell.as_number(), Some(42.5));
    assert_eq!(cell.storage_unit().canonical(), "m");
}

#[test]
fn test_write_text_cell() {
    let workbook = Workbook::new("Test");
    let workbook = Arc::new(Mutex::new(workbook));
    let unit_library = Arc::new(UnitLibrary::new());
    let handler = ToolHandler::new(Arc::clone(&workbook), unit_library);

    // Write text
    let mut args = HashMap::new();
    args.insert("cell_ref".to_string(), json!("C3"));
    args.insert("value".to_string(), json!("Hello, World!"));

    let result = handler.handle_tool_call("write_cell", Some(args));

    assert_eq!(result.is_error, Some(false));

    // Verify the text was written
    let workbook = workbook.lock().unwrap();
    let sheet = workbook.active_sheet();
    let addr = CellAddr::new("C", 3);
    let cell = sheet.get(&addr).unwrap();

    assert_eq!(cell.as_text(), Some("Hello, World!"));
    assert!(cell.is_text());
}

#[test]
fn test_write_formula_cell() {
    let workbook = Workbook::new("Test");
    let workbook = Arc::new(Mutex::new(workbook));
    let unit_library = Arc::new(UnitLibrary::new());
    let handler = ToolHandler::new(Arc::clone(&workbook), unit_library);

    // Write formula
    let mut args = HashMap::new();
    args.insert("cell_ref".to_string(), json!("D4"));
    args.insert("value".to_string(), json!("=A1 + B2"));

    let result = handler.handle_tool_call("write_cell", Some(args));

    assert_eq!(result.is_error, Some(false));

    // Verify the formula was written
    let workbook = workbook.lock().unwrap();
    let sheet = workbook.active_sheet();
    let addr = CellAddr::new("D", 4);
    let cell = sheet.get(&addr).unwrap();

    assert_eq!(cell.formula(), Some("=A1 + B2"));
}

#[test]
fn test_convert_value_tool() {
    let workbook = Workbook::new("Test");
    let workbook = Arc::new(Mutex::new(workbook));
    let unit_library = Arc::new(UnitLibrary::new());
    let handler = ToolHandler::new(workbook, unit_library);

    // Convert meters to feet
    let mut args = HashMap::new();
    args.insert("value".to_string(), json!(10.0));
    args.insert("from_unit".to_string(), json!("m"));
    args.insert("to_unit".to_string(), json!("ft"));

    let result = handler.handle_tool_call("convert_value", Some(args));

    assert_eq!(result.is_error, Some(false));

    if let unicel_lib::mcp::ToolContent::Text { text } = &result.content[0] {
        let response: serde_json::Value = serde_json::from_str(text).unwrap();

        assert_eq!(response["original"]["value"], 10.0);
        assert_eq!(response["original"]["unit"], "m");
        assert_eq!(response["converted"]["unit"], "ft");

        // 10 meters ≈ 32.8084 feet
        let converted = response["converted"]["value"].as_f64().unwrap();
        assert!(
            (converted - 32.8084).abs() < 0.001,
            "Conversion should be approximately 32.8084 feet"
        );
    }
}

#[test]
fn test_get_conversion_rate() {
    let workbook = Workbook::new("Test");
    let workbook = Arc::new(Mutex::new(workbook));
    let unit_library = Arc::new(UnitLibrary::new());
    let handler = ToolHandler::new(workbook, unit_library);

    // Get USD to EUR rate
    let mut args = HashMap::new();
    args.insert("from_unit".to_string(), json!("USD"));
    args.insert("to_unit".to_string(), json!("EUR"));

    let result = handler.handle_tool_call("get_conversion_rate", Some(args));

    assert_eq!(result.is_error, Some(false));

    if let unicel_lib::mcp::ToolContent::Text { text } = &result.content[0] {
        let response: serde_json::Value = serde_json::from_str(text).unwrap();

        assert_eq!(response["from_unit"], "USD");
        assert_eq!(response["to_unit"], "EUR");
        assert!(response["rate"].is_number());
        assert!(response["formula"].is_string());
    }
}

#[test]
fn test_list_compatible_units() {
    let workbook = Workbook::new("Test");
    let workbook = Arc::new(Mutex::new(workbook));
    let unit_library = Arc::new(UnitLibrary::new());
    let handler = ToolHandler::new(workbook, unit_library);

    // List units compatible with meters
    let mut args = HashMap::new();
    args.insert("unit".to_string(), json!("m"));

    let result = handler.handle_tool_call("list_compatible_units", Some(args));

    assert_eq!(result.is_error, Some(false));

    if let unicel_lib::mcp::ToolContent::Text { text } = &result.content[0] {
        let response: serde_json::Value = serde_json::from_str(text).unwrap();

        assert_eq!(response["unit"], "m");
        assert!(response["compatible_units"].is_array());

        let compatible = response["compatible_units"].as_array().unwrap();
        // Should include m, cm, mm, km, in, ft, yd, mi
        assert!(
            compatible.len() >= 8,
            "Should have at least 8 compatible length units"
        );

        // Verify some expected units are present
        let unit_strings: Vec<String> = compatible
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();

        assert!(unit_strings.contains(&"m".to_string()));
        assert!(unit_strings.contains(&"cm".to_string()));
        assert!(unit_strings.contains(&"ft".to_string()));
        assert!(unit_strings.contains(&"km".to_string()));
    }
}

#[test]
fn test_validate_unit() {
    let workbook = Workbook::new("Test");
    let workbook = Arc::new(Mutex::new(workbook));
    let unit_library = Arc::new(UnitLibrary::new());
    let handler = ToolHandler::new(workbook, unit_library);

    // Validate a valid unit
    let mut args = HashMap::new();
    args.insert("unit".to_string(), json!("USD"));

    let result = handler.handle_tool_call("validate_unit", Some(args));

    assert_eq!(result.is_error, Some(false));

    if let unicel_lib::mcp::ToolContent::Text { text } = &result.content[0] {
        let response: serde_json::Value = serde_json::from_str(text).unwrap();

        assert_eq!(response["valid"], true);
        assert_eq!(response["input"], "USD");
        assert_eq!(response["canonical"], "USD");
    }

    // Validate an invalid unit
    let mut args = HashMap::new();
    args.insert("unit".to_string(), json!("INVALID_UNIT"));

    let result = handler.handle_tool_call("validate_unit", Some(args));

    assert_eq!(result.is_error, Some(false)); // Tool succeeds but unit is invalid

    if let unicel_lib::mcp::ToolContent::Text { text } = &result.content[0] {
        let response: serde_json::Value = serde_json::from_str(text).unwrap();

        assert_eq!(response["valid"], false);
        assert_eq!(response["input"], "INVALID_UNIT");
        assert!(response["error"].is_string());
    }
}

#[test]
fn test_list_tables() {
    let mut workbook = Workbook::new("Test Workbook");
    workbook.add_sheet_with_name("Sales Data");
    workbook.add_sheet_with_name("Analytics");

    let workbook = Arc::new(Mutex::new(workbook));
    let unit_library = Arc::new(UnitLibrary::new());
    let handler = ToolHandler::new(workbook, unit_library);

    let result = handler.handle_tool_call("list_tables", None);

    assert_eq!(result.is_error, Some(false));

    if let unicel_lib::mcp::ToolContent::Text { text } = &result.content[0] {
        let response: serde_json::Value = serde_json::from_str(text).unwrap();

        assert!(response["tables"].is_array());
        let tables = response["tables"].as_array().unwrap();
        assert_eq!(tables.len(), 3); // Sheet1, Sales Data, Analytics

        // Verify sheet names
        let sheet_names: Vec<String> = tables
            .iter()
            .map(|t| t["name"].as_str().unwrap().to_string())
            .collect();

        assert!(sheet_names.contains(&"Sheet1".to_string()));
        assert!(sheet_names.contains(&"Sales Data".to_string()));
        assert!(sheet_names.contains(&"Analytics".to_string()));
    }
}

#[test]
fn test_get_workbook_metadata() {
    let mut workbook = Workbook::new("My Test Workbook");
    workbook.add_sheet_with_name("Data");

    // Add some cells
    {
        let sheet = workbook.active_sheet_mut();
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(10.0, Unit::dimensionless()),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(20.0, Unit::dimensionless()),
            )
            .unwrap();
    }

    let workbook = Arc::new(Mutex::new(workbook));
    let unit_library = Arc::new(UnitLibrary::new());
    let handler = ToolHandler::new(workbook, unit_library);

    let result = handler.handle_tool_call("get_workbook_metadata", None);

    assert_eq!(result.is_error, Some(false));

    if let unicel_lib::mcp::ToolContent::Text { text } = &result.content[0] {
        let response: serde_json::Value = serde_json::from_str(text).unwrap();

        assert_eq!(response["name"], "My Test Workbook");
        assert_eq!(response["sheet_count"], 2);
        assert!(response["sheets"].is_array());
        assert!(response["active_sheet"].is_string());
        assert!(response["display_preference"].is_string());
    }
}

#[test]
fn test_get_sheet_structure() {
    let mut workbook = Workbook::new("Test");

    // Add several cells
    {
        let sheet = workbook.active_sheet_mut();
        sheet
            .set(CellAddr::new("A", 1), Cell::new(1.0, Unit::dimensionless()))
            .unwrap();
        sheet
            .set(CellAddr::new("A", 2), Cell::new(2.0, Unit::dimensionless()))
            .unwrap();
        sheet
            .set(CellAddr::new("B", 1), Cell::new(3.0, Unit::dimensionless()))
            .unwrap();
        sheet
            .set(CellAddr::new("C", 5), Cell::new(4.0, Unit::dimensionless()))
            .unwrap();
    }

    let workbook = Arc::new(Mutex::new(workbook));
    let unit_library = Arc::new(UnitLibrary::new());
    let handler = ToolHandler::new(workbook, unit_library);

    let result = handler.handle_tool_call("get_sheet_structure", None);

    assert_eq!(result.is_error, Some(false));

    if let unicel_lib::mcp::ToolContent::Text { text } = &result.content[0] {
        let response: serde_json::Value = serde_json::from_str(text).unwrap();

        assert!(response["sheet_name"].is_string());
        assert_eq!(response["used_cells"], 4);
        assert!(response["cell_references"].is_array());

        let cell_refs = response["cell_references"].as_array().unwrap();
        assert_eq!(cell_refs.len(), 4);
    }
}

#[test]
fn test_unknown_tool() {
    let workbook = Workbook::new("Test");
    let workbook = Arc::new(Mutex::new(workbook));
    let unit_library = Arc::new(UnitLibrary::new());
    let handler = ToolHandler::new(workbook, unit_library);

    let result = handler.handle_tool_call("non_existent_tool", None);

    assert_eq!(
        result.is_error,
        Some(true),
        "Should be an error for unknown tool"
    );

    if let unicel_lib::mcp::ToolContent::Text { text } = &result.content[0] {
        assert!(text.contains("Unknown tool"));
    }
}

#[test]
fn test_invalid_cell_reference() {
    let workbook = Workbook::new("Test");
    let workbook = Arc::new(Mutex::new(workbook));
    let unit_library = Arc::new(UnitLibrary::new());
    let handler = ToolHandler::new(workbook, unit_library);

    // Try to read with invalid cell reference
    let mut args = HashMap::new();
    args.insert("cell_ref".to_string(), json!("INVALID"));

    let result = handler.handle_tool_call("read_cell", Some(args));

    assert_eq!(
        result.is_error,
        Some(true),
        "Should be an error for invalid cell reference"
    );
}

#[test]
fn test_read_empty_cell() {
    let workbook = Workbook::new("Test");
    let workbook = Arc::new(Mutex::new(workbook));
    let unit_library = Arc::new(UnitLibrary::new());
    let handler = ToolHandler::new(workbook, unit_library);

    // Try to read an empty cell
    let mut args = HashMap::new();
    args.insert("cell_ref".to_string(), json!("Z99"));

    let result = handler.handle_tool_call("read_cell", Some(args));

    // Reading an empty cell should return an error since it doesn't exist
    assert_eq!(
        result.is_error,
        Some(true),
        "Should be an error for empty/non-existent cell"
    );
}

#[test]
fn test_server_creation() {
    let workbook = Workbook::new("Test");
    let unit_library = UnitLibrary::new();
    let _server = McpServer::new(workbook, unit_library);

    // Just verify we can create a server without panicking
}

#[test]
fn test_multi_sheet_operations() {
    let mut workbook = Workbook::new("Multi-Sheet Test");
    workbook.add_sheet_with_name("Sheet2");
    workbook.add_sheet_with_name("Sheet3");

    // Add cells to different sheets
    {
        let sheet1 = workbook.get_sheet_mut(0).unwrap();
        sheet1
            .set(
                CellAddr::new("A", 1),
                Cell::new(100.0, Unit::simple("USD", BaseDimension::Currency)),
            )
            .unwrap();
    }
    {
        let sheet2 = workbook.get_sheet_mut(1).unwrap();
        sheet2
            .set(
                CellAddr::new("B", 2),
                Cell::new(200.0, Unit::simple("EUR", BaseDimension::Currency)),
            )
            .unwrap();
    }

    let workbook = Arc::new(Mutex::new(workbook));
    let unit_library = Arc::new(UnitLibrary::new());
    let handler = ToolHandler::new(workbook, unit_library);

    // List tables should show all sheets
    let result = handler.handle_tool_call("list_tables", None);
    assert_eq!(result.is_error, Some(false));

    if let unicel_lib::mcp::ToolContent::Text { text } = &result.content[0] {
        let response: serde_json::Value = serde_json::from_str(text).unwrap();
        let tables = response["tables"].as_array().unwrap();
        assert_eq!(tables.len(), 3);
    }
}
