# Unicel MCP Server

The Unicel MCP Server exposes Unicel workbook operations via the Model Context Protocol (MCP), allowing LLMs and AI agents to interact with unit-aware spreadsheets.

## Overview

The MCP server implements the JSON-RPC 2.0 protocol over STDIO, making it compatible with MCP clients like Claude Desktop, IDEs with MCP support, or custom MCP clients.

## Building

```bash
cd src-tauri
cargo build --bin unicel-mcp-server
```

The binary will be located at: `target/debug/unicel-mcp-server`

## Usage

### Starting the Server

```bash
# Start with an empty workbook
./target/debug/unicel-mcp-server

# Start with an existing workbook
./target/debug/unicel-mcp-server path/to/workbook.usheet
```

The server reads JSON-RPC requests from stdin and writes responses to stdout. Logs are written to stderr.

### Environment Variables

- `RUST_LOG` - Set logging level (e.g., `RUST_LOG=debug`)

## Available Tools

### Read Tools

#### `read_cell`
Read a single cell with full metadata.

**Parameters:**
- `cell_ref` (string, required): Cell reference (e.g., "A1", "B5")
- `sheet_name` (string, optional): Sheet name (defaults to active sheet)

**Returns:**
```json
{
  "cell_ref": "A1",
  "value": 100.0,
  "unit": {
    "canonical": "USD",
    "dimension": "Currency",
    "display": "USD"
  },
  "formula": null,
  "warnings": [],
  "is_empty": false,
  "is_number": true,
  "is_text": false,
  "is_error": false
}
```

#### `get_sheet_structure`
Get the structure of a sheet including used cells.

**Parameters:**
- `sheet_name` (string, optional): Sheet name (defaults to active sheet)

**Returns:**
```json
{
  "sheet_name": "Sheet1",
  "used_cells": 10,
  "cell_references": ["A1", "A2", "A3", ...]
}
```

#### `list_tables`
List all sheets (tables) in the workbook.

**Parameters:** None

**Returns:**
```json
{
  "tables": [
    {"name": "Sheet1", "cell_count": 10},
    {"name": "Sheet2", "cell_count": 5}
  ],
  "active_sheet": "Sheet1"
}
```

### Write Tools

#### `write_cell`
Write a value to a single cell.

**Parameters:**
- `cell_ref` (string, required): Cell reference (e.g., "A1")
- `value` (number/string, required): Cell value (number, text, or formula starting with "=")
- `unit` (string, optional): Unit for numeric values (e.g., "USD", "m", "GB")
- `sheet_name` (string, optional): Sheet name (defaults to active sheet)
- `validate` (boolean, optional): Validate unit and value (default: true)

**Examples:**
```json
// Write a number with unit
{"cell_ref": "A1", "value": 100, "unit": "USD"}

// Write text
{"cell_ref": "B1", "value": "Hello"}

// Write formula
{"cell_ref": "C1", "value": "=A1 + B1"}
```

**Returns:**
```json
{
  "success": true,
  "cell_ref": "A1"
}
```

### Conversion Tools

#### `convert_value`
Convert a value from one unit to another.

**Parameters:**
- `value` (number, required): Value to convert
- `from_unit` (string, required): Source unit (e.g., "USD", "m", "GB")
- `to_unit` (string, required): Target unit (e.g., "EUR", "ft", "TB")
- `include_path` (boolean, optional): Include conversion path metadata

**Returns:**
```json
{
  "original": {"value": 100, "unit": "USD"},
  "converted": {"value": 94.0, "unit": "EUR"},
  "conversion_rate": 0.94
}
```

#### `get_conversion_rate`
Get the conversion rate between two units.

**Parameters:**
- `from_unit` (string, required): Source unit
- `to_unit` (string, required): Target unit

**Returns:**
```json
{
  "from_unit": "USD",
  "to_unit": "EUR",
  "rate": 0.94,
  "formula": "1 USD = 0.94 EUR"
}
```

#### `list_compatible_units`
List all units compatible with a given unit.

**Parameters:**
- `unit` (string, required): Unit to check compatibility for

**Returns:**
```json
{
  "unit": "m",
  "dimension": "Simple(Length)",
  "compatible_units": ["m", "cm", "mm", "km", "in", "ft", "yd", "mi"],
  "count": 8
}
```

#### `validate_unit`
Validate if a unit is recognized.

**Parameters:**
- `unit` (string, required): Unit string to validate

**Returns:**
```json
{
  "valid": true,
  "input": "m",
  "canonical": "m",
  "dimension": "Simple(Length)"
}
```

### Schema Tools

#### `get_workbook_metadata`
Get metadata about the entire workbook.

**Parameters:** None

**Returns:**
```json
{
  "name": "My Workbook",
  "sheet_count": 2,
  "sheets": [
    {"name": "Sheet1", "cell_count": 10},
    {"name": "Sheet2", "cell_count": 5}
  ],
  "active_sheet": "Sheet1",
  "display_preference": "AsEntered"
}
```

## Resource URIs

The server exposes data via resource URIs:

- `unicel://workbook` - Workbook metadata
- `unicel://sheet/{sheet_name}` - Sheet data with all cells
- `unicel://units/domains` - All available units

## Supported Units

### Length
m, cm, mm, km, in, ft, yd, mi

### Mass
g, kg, mg, oz, lb

### Time
s, min, hr, h, hour, day, month, year

### Temperature
C, F, K (with offset conversion)

### Currency
USD, EUR, GBP (hardcoded rates)

### Digital Storage
- Bytes: B, KB, MB, GB, TB, PB
- Bits: b, Kb, Mb, Gb, Tb, Pb
- Tokens: Tok, MTok

## Integration with Claude Desktop

Add to your Claude Desktop MCP configuration (`~/Library/Application Support/Claude/claude_desktop_config.json` on macOS):

```json
{
  "mcpServers": {
    "unicel": {
      "command": "/path/to/unicel/target/debug/unicel-mcp-server",
      "args": ["/path/to/your/workbook.usheet"]
    }
  }
}
```

Then Claude can use tools like:
- "Read cell A1 from the spreadsheet"
- "Write 100 USD to cell B5"
- "Convert 50 meters to feet"
- "List all sheets in the workbook"

## Example Workflow

```bash
# Start server (in one terminal)
./target/debug/unicel-mcp-server examples/construction_estimator.usheet

# Send JSON-RPC requests (example using echo)
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}' | ./target/debug/unicel-mcp-server

# List available tools
echo '{"jsonrpc":"2.0","id":2,"method":"tools/list"}' | ./target/debug/unicel-mcp-server

# Read a cell
echo '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"read_cell","arguments":{"cell_ref":"A1"}}}' | ./target/debug/unicel-mcp-server
```

## Protocol Details

The server implements:
- **Protocol Version**: MCP 2024-11-05
- **Transport**: STDIO (JSON-RPC 2.0)
- **Capabilities**: Tools and Resources

### Initialization Sequence

1. Client sends `initialize` request
2. Server responds with capabilities
3. Client sends `initialized` notification
4. Client can now call tools and read resources

## Error Handling

Errors are returned as JSON-RPC error responses:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32602,
    "message": "Invalid params: Missing cell_ref"
  }
}
```

Common error codes:
- `-32700`: Parse error
- `-32600`: Invalid request
- `-32601`: Method not found
- `-32602`: Invalid params
- `-32603`: Internal error

## Future Enhancements

The following tools are stubbed for future implementation:

- `read_range` - Batch read cells in a range
- `query_table` - SQL queries with unit-aware WHERE clauses
- `write_range` - Batch write operations
- `append_row` - Add rows to tables with schema respect
- `update_row` - Modify existing rows
- `delete_row` - Remove rows

## Development

### Running Tests

```bash
cargo test --lib
cargo test --bin unicel-mcp-server
```

### Debugging

Set `RUST_LOG=debug` to see detailed logs:

```bash
RUST_LOG=debug ./target/debug/unicel-mcp-server
```

### Adding New Tools

1. Define tool in `src/mcp/tools.rs` using `define_*()` pattern
2. Add handler method `handle_*()` in `ToolHandler` impl
3. Register in `handle_tool_call()` match statement
4. Update this documentation

## See Also

- [MCP Tool Design](./MCP_TOOL_DESIGN.md) - Detailed tool specifications
- [Model Context Protocol](https://modelcontextprotocol.io/) - Official MCP documentation
- [Unit Aware Spreadsheet Design](../requirements/Unit%20Aware%20Spreadsheet%20Design.md) - Unicel design document
