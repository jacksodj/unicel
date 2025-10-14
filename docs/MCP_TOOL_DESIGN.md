# Unicel MCP Tool Design
## Making Unicel an LLM-Native Unit-Aware Datastore

**Version:** 1.0
**Date:** 2025-10-13
**Status:** Design Specification

---

## Executive Summary

This document specifies the Model Context Protocol (MCP) tools that expose Unicel's unit-aware spreadsheet functionality to LLMs and AI agents. The goal is to make Unicel a powerful, queryable datastore that understands physical units, dimensions, and conversions—enabling sophisticated data analysis through natural language.

**Key Innovation:** LLMs can work with quantitative data while respecting units, dimensional analysis, and automatic conversions—something not possible with traditional spreadsheets or databases.

---

## Table of Contents

1. [Design Principles](#1-design-principles)
2. [Tool Categories](#2-tool-categories)
3. [Tool Specifications](#3-tool-specifications)
4. [Resource URIs](#4-resource-uris)
5. [Use Case Examples](#5-use-case-examples)
6. [Error Handling](#6-error-handling)
7. [Implementation Notes](#7-implementation-notes)

---

## 1. Design Principles

### 1.1 LLM-Friendly Design

**Natural Semantics:**
- Tool names describe intent clearly ("read_cell", not "get_cell_data")
- Parameters use common terminology
- Results include context (units, dimensions, metadata)
- Errors are descriptive and actionable

**Rich Context:**
- Every numeric value includes its unit
- Results explain dimensional analysis
- Warnings flag potential unit issues
- Schema information helps LLMs understand structure

**Composability:**
- Simple tools combine for complex operations
- Query results can feed other tools
- Unit conversion integrated throughout
- Batch operations for efficiency

### 1.2 Unit-Aware by Default

**All Operations Respect Units:**
- Read operations return values with units
- Write operations validate units
- Queries handle unit conversions automatically
- Aggregations preserve dimensional analysis

**Conversion Intelligence:**
- Automatic unit compatibility checking
- Conversion rate transparency (live, manual, historical)
- Warning on incompatible operations
- Compound unit support (mi/hr, USD/user, GB/month)

### 1.3 SQL-Queryable

**Familiar Query Language:**
- Standard SQL SELECT syntax
- Unit-aware WHERE clauses
- Automatic conversions in comparisons
- Context-aware aggregations (COUNT inherits row_unit)

**Table-First Design:**
- Tables represent entities (EC2Instance, User, Transaction)
- Rows have semantic meaning (instances, users, transactions)
- Columns have metadata (units, types, validation)
- Entity-aware operations

---

## 2. Tool Categories

### 2.1 Read Tools (Get Data Out)
- `read_cell` - Single cell with full metadata
- `read_range` - Batch read cell range
- `query_table` - SQL SELECT with units
- `get_sheet_structure` - Understand layout
- `list_tables` - Discover available tables

### 2.2 Write Tools (Put Data In)
- `write_cell` - Single cell with unit
- `write_range` - Batch write cells
- `append_row` - Add to table (respects schema)
- `update_row` - Modify existing row
- `delete_row` - Remove from table

### 2.3 Conversion Tools (Unit Operations)
- `convert_value` - Convert between units
- `get_conversion_rate` - Current rate with metadata
- `list_compatible_units` - What can convert to
- `validate_unit` - Check if unit is recognized

### 2.4 Schema Tools (Understand Structure)
- `get_workbook_metadata` - Sheets, tables, settings
- `get_table_schema` - Entity type, columns, units
- `get_column_metadata` - Unit, type, validation rules
- `list_unit_domains` - Available unit categories

### 2.5 Validation Tools (Data Quality)
- `validate_cell` - Check against rules
- `get_valid_values` - Dropdown options (SQL or manual)
- `check_formula` - Validate formula syntax
- `check_unit_compatibility` - Can add/multiply/divide

---

## 3. Tool Specifications

### 3.1 Read Tools

#### `read_cell`
**Purpose:** Read a single cell with full unit and formula metadata.

**Parameters:**
```json
{
  "cell_ref": "string",      // e.g., "A1", "Sheet2!B5"
  "include_formula": boolean, // Include formula text (default: true)
  "display_unit": "string"    // Optional: convert for display
}
```

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
  "formula": "=B1*C1",           // If cell has formula
  "formula_result_unit": "USD",  // Result unit of formula
  "display_value": 94.0,         // If display_unit requested
  "display_unit": "EUR",
  "warnings": [],                // Unit warnings if any
  "modified_at": "2025-10-13T10:30:00Z"
}
```

**Example Use Cases:**
- "What's the value in cell A1?"
- "Show me the formula for cell B5"
- "Get cell C10 in euros instead of dollars"

---

#### `read_range`
**Purpose:** Batch read a range of cells efficiently.

**Parameters:**
```json
{
  "range": "string",          // e.g., "A1:C10", "Sheet2!A:A"
  "include_formulas": boolean,
  "include_empty": boolean,   // Include empty cells (default: false)
  "convert_to_unit": "string" // Optional: uniform unit conversion
}
```

**Returns:**
```json
{
  "range": "A1:C3",
  "cells": [
    {
      "ref": "A1",
      "value": 100.0,
      "unit": "USD",
      "formula": null
    },
    {
      "ref": "A2",
      "value": 50.0,
      "unit": "EUR",
      "formula": null
    },
    // ... more cells
  ],
  "row_count": 3,
  "col_count": 3,
  "unit_summary": {
    "USD": 5,        // 5 cells with USD
    "EUR": 3,
    "dimensionless": 1
  }
}
```

**Example Use Cases:**
- "Get all prices from column B"
- "Read the entire header row"
- "Fetch all data from the Price table"

---

#### `query_table`
**Purpose:** SQL SELECT query with unit-aware operations.

**Parameters:**
```json
{
  "table_name": "string",
  "sql": "string",           // SELECT statement
  "convert_units": boolean,  // Auto-convert compatible units (default: true)
  "limit": number            // Row limit (default: 100, max: 10000)
}
```

**Returns:**
```json
{
  "table_name": "EC2Instances",
  "query": "SELECT InstanceType, RAM, Price FROM EC2Instances WHERE RAM > 16GB",
  "columns": [
    {
      "name": "InstanceType",
      "type": "Text",
      "unit": null
    },
    {
      "name": "RAM",
      "type": "StorageSize",
      "unit": "GB"
    },
    {
      "name": "Price",
      "type": "Rate",
      "unit": "USD/hr"
    }
  ],
  "rows": [
    {
      "InstanceType": "m5.large",
      "RAM": {"value": 32.0, "unit": "GB"},
      "Price": {"value": 0.096, "unit": "USD/hr"}
    },
    // ... more rows
  ],
  "row_count": 15,
  "total_count": 15,
  "execution_time_ms": 23
}
```

**Example Use Cases:**
- "Show me all EC2 instances with more than 16GB RAM"
- "Find the cheapest instances in us-east-1"
- "Count how many users have premium subscriptions"

---

#### `get_sheet_structure`
**Purpose:** Understand sheet layout, tables, and ranges.

**Parameters:**
```json
{
  "sheet_name": "string"  // Optional: active sheet if omitted
}
```

**Returns:**
```json
{
  "sheet_name": "EC2Pricing",
  "dimensions": {
    "rows": 100,
    "cols": 10,
    "populated_cells": 450
  },
  "tables": [
    {
      "name": "EC2Instances",
      "range": "A1:F50",
      "entity_type": "EC2Instance",
      "row_unit": "instances",
      "column_count": 6,
      "row_count": 49  // Excluding header
    }
  ],
  "named_ranges": {
    "PriceList": "C2:C50",
    "TotalCost": "F51"
  },
  "display_settings": {
    "unit_display": "Metric",
    "frozen_rows": 1,
    "frozen_cols": 1
  }
}
```

**Example Use Cases:**
- "What tables are in this sheet?"
- "Show me the structure of the workbook"
- "What named ranges exist?"

---

#### `list_tables`
**Purpose:** Discover all tables across all sheets.

**Parameters:**
```json
{
  "include_schema": boolean  // Include column details (default: false)
}
```

**Returns:**
```json
{
  "tables": [
    {
      "name": "EC2Instances",
      "sheet": "Sheet1",
      "entity_type": "EC2Instance",
      "row_unit": "instances",
      "row_count": 49,
      "column_count": 6,
      "range": "A1:F50"
    },
    {
      "name": "RDSInstances",
      "sheet": "Sheet2",
      "entity_type": "RDSInstance",
      "row_unit": "instances",
      "row_count": 25,
      "column_count": 5,
      "range": "A1:E26"
    }
  ],
  "total_tables": 2
}
```

**Example Use Cases:**
- "What data is available in this workbook?"
- "List all tables"
- "Show me all entity types"

---

### 3.2 Write Tools

#### `write_cell`
**Purpose:** Write a single cell with unit validation.

**Parameters:**
```json
{
  "cell_ref": "string",
  "value": number | string,  // Numeric value or text
  "unit": "string",          // Optional: dimensionless if omitted
  "formula": "string",       // Alternative to value: write formula
  "validate": boolean        // Validate against column rules (default: true)
}
```

**Returns:**
```json
{
  "success": true,
  "cell_ref": "A1",
  "stored_value": 100.0,
  "stored_unit": "USD",
  "warnings": [],
  "validation_errors": []
}
```

**Errors:**
- Invalid unit
- Unit incompatible with column type
- Validation rule violation
- Formula syntax error

**Example Use Cases:**
- "Set cell A1 to 100 USD"
- "Write the formula =B1*C1 to cell D1"
- "Update the price in B5 to 50 euros"

---

#### `write_range`
**Purpose:** Batch write multiple cells efficiently.

**Parameters:**
```json
{
  "range": "string",  // e.g., "A1:C3"
  "values": [         // Row-major order
    [
      {"value": 100, "unit": "USD"},
      {"value": 50, "unit": "EUR"},
      {"value": 200, "unit": "GBP"}
    ],
    // ... more rows
  ],
  "validate": boolean
}
```

**Returns:**
```json
{
  "success": true,
  "cells_written": 9,
  "warnings": [],
  "validation_errors": []
}
```

**Example Use Cases:**
- "Paste this data into cells A1:C3"
- "Fill column B with these prices"
- "Update the entire price list"

---

#### `append_row`
**Purpose:** Add a row to a table (respects schema and validation).

**Parameters:**
```json
{
  "table_name": "string",
  "row_data": {
    "Region": "us-east-1",
    "InstanceType": "m5.large",
    "RAM": {"value": 32, "unit": "GB"},
    "Price": {"value": 0.096, "unit": "USD/hr"}
  },
  "validate": boolean
}
```

**Returns:**
```json
{
  "success": true,
  "table_name": "EC2Instances",
  "row_index": 50,
  "cell_range": "A50:F50",
  "validation_errors": []
}
```

**Example Use Cases:**
- "Add a new instance to the EC2Instances table"
- "Append this data as a new row"
- "Insert a new user record"

---

#### `update_row`
**Purpose:** Modify an existing table row.

**Parameters:**
```json
{
  "table_name": "string",
  "row_index": number,      // Or use WHERE clause
  "where": "string",        // Alternative: "InstanceType = 'm5.large'"
  "updates": {
    "Price": {"value": 0.10, "unit": "USD/hr"},
    "RAM": {"value": 64, "unit": "GB"}
  },
  "validate": boolean
}
```

**Returns:**
```json
{
  "success": true,
  "rows_updated": 1,
  "cell_refs": ["D5", "C5"]
}
```

**Example Use Cases:**
- "Update the price for m5.large instances"
- "Change RAM to 64GB for row 5"
- "Set the status to active for user johnsmith"

---

#### `delete_row`
**Purpose:** Remove a row from a table.

**Parameters:**
```json
{
  "table_name": "string",
  "row_index": number,    // Or use WHERE clause
  "where": "string"       // Alternative: "Status = 'inactive'"
}
```

**Returns:**
```json
{
  "success": true,
  "rows_deleted": 1,
  "backup": {             // Optional backup of deleted data
    "Region": "us-west-2",
    "InstanceType": "t3.micro",
    // ... full row data
  }
}
```

**Example Use Cases:**
- "Delete row 10 from the table"
- "Remove all inactive users"
- "Delete the instance with ID xyz"

---

### 3.3 Conversion Tools

#### `convert_value`
**Purpose:** Convert a value from one unit to another.

**Parameters:**
```json
{
  "value": number,
  "from_unit": "string",
  "to_unit": "string",
  "include_path": boolean  // Show conversion path (default: false)
}
```

**Returns:**
```json
{
  "original": {
    "value": 100,
    "unit": "USD"
  },
  "converted": {
    "value": 94.0,
    "unit": "EUR"
  },
  "conversion_rate": 0.94,
  "conversion_source": "LiveAuto",  // Live, Manual, Historical, Chained
  "conversion_path": ["USD", "EUR"],  // If include_path = true
  "updated_at": "2025-10-13T10:30:00Z",
  "warnings": []
}
```

**Example Use Cases:**
- "Convert 100 USD to EUR"
- "How many feet in 50 meters?"
- "What's 32 GB in TB?"

---

#### `get_conversion_rate`
**Purpose:** Get current conversion rate with metadata.

**Parameters:**
```json
{
  "from_unit": "string",
  "to_unit": "string",
  "include_history": boolean  // Last 10 rates (default: false)
}
```

**Returns:**
```json
{
  "from": "USD",
  "to": "EUR",
  "rate": 0.94,
  "mode": "LiveAuto",
  "source": "mcp-server-currency",
  "updated_at": "2025-10-13T10:30:00Z",
  "confidence": "high",
  "history": [  // If include_history = true
    {"rate": 0.94, "timestamp": "2025-10-13T10:30:00Z"},
    {"rate": 0.93, "timestamp": "2025-10-13T09:30:00Z"},
    // ... more
  ]
}
```

**Example Use Cases:**
- "What's the current USD to EUR conversion rate?"
- "Show me the exchange rate history"
- "Is this a live rate or manual?"

---

#### `list_compatible_units`
**Purpose:** Find units that can be converted to/from a given unit.

**Parameters:**
```json
{
  "unit": "string",
  "dimension_only": boolean  // Only same dimension (default: true)
}
```

**Returns:**
```json
{
  "unit": "USD",
  "dimension": "Currency",
  "compatible_units": [
    {
      "unit": "EUR",
      "conversion_available": true,
      "conversion_mode": "LiveAuto"
    },
    {
      "unit": "GBP",
      "conversion_available": true,
      "conversion_mode": "LiveAuto"
    },
    {
      "unit": "JPY",
      "conversion_available": true,
      "conversion_mode": "LiveAuto"
    }
  ],
  "total_count": 15
}
```

**Example Use Cases:**
- "What currencies can I convert USD to?"
- "Show me all length units"
- "What units are compatible with GB?"

---

#### `validate_unit`
**Purpose:** Check if a unit string is recognized and valid.

**Parameters:**
```json
{
  "unit": "string",
  "suggest_alternatives": boolean  // Fuzzy match (default: true)
}
```

**Returns:**
```json
{
  "valid": true,
  "unit": "USD",
  "canonical": "USD",
  "dimension": "Currency",
  "domain": "currency",
  "alternatives": []  // If invalid, suggests: ["USD", "EUR", "GBP"]
}
```

**Example Use Cases:**
- "Is 'dollars' a valid unit?"
- "Check if this unit exists: GigaByte"
- "Validate the unit string before writing"

---

### 3.4 Schema Tools

#### `get_workbook_metadata`
**Purpose:** Get workbook-level information.

**Parameters:**
```json
{
  "include_sheets": boolean,    // Sheet details (default: true)
  "include_tables": boolean,    // Table details (default: true)
  "include_settings": boolean   // Workbook settings (default: false)
}
```

**Returns:**
```json
{
  "name": "AWS Cost Analysis",
  "version": "1.0",
  "created_at": "2025-10-01T10:00:00Z",
  "modified_at": "2025-10-13T10:30:00Z",
  "sheets": [
    {
      "name": "EC2Pricing",
      "index": 0,
      "is_active": true,
      "table_count": 1,
      "cell_count": 450
    },
    {
      "name": "RDSPricing",
      "index": 1,
      "is_active": false,
      "table_count": 1,
      "cell_count": 200
    }
  ],
  "settings": {  // If include_settings = true
    "unit_preference": "Metric",
    "conversion_mode": "LiveAuto",
    "enabled_domains": ["length", "mass", "time", "currency", "digital_storage"]
  },
  "total_tables": 2,
  "total_sheets": 2
}
```

**Example Use Cases:**
- "What's in this workbook?"
- "Show me all sheets and tables"
- "What are the workbook settings?"

---

#### `get_table_schema`
**Purpose:** Get detailed table schema with units and validation.

**Parameters:**
```json
{
  "table_name": "string"
}
```

**Returns:**
```json
{
  "name": "EC2Instances",
  "sheet": "Sheet1",
  "entity_type": "EC2Instance",
  "row_unit": "instances",
  "range": "A1:F50",
  "row_count": 49,
  "columns": [
    {
      "index": "A",
      "name": "Region",
      "value_type": "Text",
      "default_unit": null,
      "display_unit": null,
      "validation": {
        "required": true,
        "valid_values": ["us-east-1", "us-west-2", "eu-central-1"]
      }
    },
    {
      "index": "B",
      "name": "InstanceType",
      "value_type": "Text",
      "validation": {
        "required": true
      }
    },
    {
      "index": "C",
      "name": "RAM",
      "value_type": "StorageSize",
      "default_unit": "GB",
      "display_unit": "GB",
      "validation": {
        "required": true
      }
    },
    {
      "index": "D",
      "name": "Price",
      "value_type": "Rate",
      "default_unit": "USD/hr",
      "display_unit": "USD/hr"
    }
  ]
}
```

**Example Use Cases:**
- "What columns are in the EC2Instances table?"
- "Show me the schema for this table"
- "What units are used in each column?"

---

#### `get_column_metadata`
**Purpose:** Get detailed metadata for a specific column.

**Parameters:**
```json
{
  "table_name": "string",
  "column_name": "string"  // Or column_index: "C"
}
```

**Returns:**
```json
{
  "name": "RAM",
  "index": "C",
  "table": "EC2Instances",
  "value_type": "StorageSize",
  "dimension": "DigitalStorage",
  "default_unit": "GB",
  "display_unit": "GB",
  "validation": {
    "required": true,
    "valid_values": null,
    "min_value": null,
    "max_value": null
  },
  "statistics": {
    "min": {"value": 0.5, "unit": "GB"},
    "max": {"value": 384, "unit": "GB"},
    "average": {"value": 32, "unit": "GB"},
    "unit_distribution": {
      "GB": 45,
      "TB": 4
    }
  }
}
```

**Example Use Cases:**
- "What are the validation rules for the RAM column?"
- "Show me statistics for the Price column"
- "What unit is used for this column?"

---

#### `list_unit_domains`
**Purpose:** List available unit domains and their units.

**Parameters:**
```json
{
  "enabled_only": boolean  // Only enabled domains (default: true)
}
```

**Returns:**
```json
{
  "domains": [
    {
      "id": "length",
      "name": "Length",
      "enabled": true,
      "unit_count": 8,
      "units": ["m", "cm", "mm", "km", "in", "ft", "yd", "mi"]
    },
    {
      "id": "currency",
      "name": "Currency",
      "enabled": true,
      "unit_count": 15,
      "units": ["USD", "EUR", "GBP", "JPY", "CNY", ...]
    },
    {
      "id": "digital_storage",
      "name": "Digital Storage",
      "enabled": true,
      "unit_count": 14,
      "units": ["B", "KB", "MB", "GB", "TB", "PB", "b", "Kb", "Mb", "Gb", "Tb", "Pb", "Tok", "MTok"]
    }
  ],
  "total_domains": 15,
  "total_units": 150
}
```

**Example Use Cases:**
- "What unit domains are available?"
- "List all currency units"
- "Show me enabled unit categories"

---

### 3.5 Validation Tools

#### `validate_cell`
**Purpose:** Validate a value against column rules.

**Parameters:**
```json
{
  "table_name": "string",
  "column_name": "string",
  "value": number | string,
  "unit": "string"
}
```

**Returns:**
```json
{
  "valid": true,
  "value": 32,
  "unit": "GB",
  "errors": [],
  "warnings": []
}
```

**Errors:**
```json
{
  "valid": false,
  "errors": [
    {
      "type": "RequiredFieldMissing",
      "message": "This field is required"
    },
    {
      "type": "InvalidValue",
      "message": "Value must be one of: [us-east-1, us-west-2, eu-central-1]"
    },
    {
      "type": "IncompatibleUnit",
      "message": "Unit 'seconds' is not compatible with StorageSize dimension"
    }
  ]
}
```

**Example Use Cases:**
- "Will this value be accepted?"
- "Check if I can write 32 GB to the RAM column"
- "Validate this data before inserting"

---

#### `get_valid_values`
**Purpose:** Get allowed values for a dropdown/enum column.

**Parameters:**
```json
{
  "table_name": "string",
  "column_name": "string"
}
```

**Returns:**
```json
{
  "column": "Region",
  "value_source": "ManualList",  // or "SqlQuery"
  "values": [
    "us-east-1",
    "us-west-2",
    "eu-central-1"
  ],
  "total_count": 3
}
```

**Example Use Cases:**
- "What regions are valid?"
- "Show me the dropdown options for this field"
- "What values can I use?"

---

#### `check_formula`
**Purpose:** Validate formula syntax and dimensional analysis.

**Parameters:**
```json
{
  "formula": "string",
  "context": {     // Optional: for cell references
    "sheet": "string",
    "cell_ref": "string"
  }
}
```

**Returns:**
```json
{
  "valid": true,
  "formula": "=B1*C1",
  "result_unit": "USD/hr",
  "result_dimension": "Rate",
  "dependencies": ["B1", "C1"],
  "operations": [
    {
      "op": "multiply",
      "lhs": {"ref": "B1", "unit": "USD"},
      "rhs": {"ref": "C1", "unit": "1/hr"},
      "result": "USD/hr"
    }
  ],
  "warnings": [],
  "errors": []
}
```

**Example Use Cases:**
- "Check if this formula is valid"
- "What unit will this formula produce?"
- "Validate the formula before writing"

---

#### `check_unit_compatibility`
**Purpose:** Check if two units can be used in an operation.

**Parameters:**
```json
{
  "unit1": "string",
  "unit2": "string",
  "operation": "add" | "subtract" | "multiply" | "divide"
}
```

**Returns:**
```json
{
  "compatible": true,
  "operation": "add",
  "unit1": "USD",
  "unit2": "EUR",
  "result_unit": "USD",
  "notes": "Units will be auto-converted to USD",
  "warnings": []
}
```

**Incompatible Example:**
```json
{
  "compatible": false,
  "operation": "add",
  "unit1": "m",
  "unit2": "s",
  "result_unit": "dimensionless",
  "warnings": [
    {
      "type": "IncompatibleUnits",
      "message": "Cannot add Length (m) and Time (s). Result will be dimensionless with warning."
    }
  ]
}
```

**Example Use Cases:**
- "Can I add meters and feet?"
- "What happens if I multiply USD by hours?"
- "Check compatibility before calculation"

---

## 4. Resource URIs

### 4.1 URI Scheme

**Format:** `unicel://{resource_type}/{identifier}[?params]`

### 4.2 Resource Types

#### Workbook Resources
```
unicel://workbook                      # Current workbook metadata
unicel://workbook/settings             # Workbook settings
unicel://workbook/conversions          # Conversion rates
unicel://workbook/units/custom         # Custom units
```

#### Sheet Resources
```
unicel://sheet/{sheet_name}            # Sheet metadata
unicel://sheet/{sheet_name}/structure  # Sheet structure
unicel://sheet/{sheet_name}/tables     # Tables in sheet
```

#### Table Resources
```
unicel://table/{table_name}            # Table data
unicel://table/{table_name}/schema     # Table schema
unicel://table/{table_name}/rows       # All rows
unicel://table/{table_name}/row/{id}   # Specific row
```

#### Cell Resources
```
unicel://cell/{cell_ref}               # Single cell
unicel://range/{range_ref}             # Cell range
```

#### Unit Resources
```
unicel://units/domains                 # All unit domains
unicel://units/domain/{domain_id}      # Specific domain
unicel://units/{unit_id}               # Unit definition
unicel://conversions/{from}/{to}       # Conversion rate
```

### 4.3 Resource Examples

```
unicel://workbook
unicel://sheet/EC2Pricing
unicel://table/EC2Instances
unicel://table/EC2Instances/schema
unicel://cell/A1
unicel://cell/Sheet2!B5
unicel://range/A1:C10
unicel://units/domains
unicel://units/domain/currency
unicel://units/USD
unicel://conversions/USD/EUR
```

---

## 5. Use Case Examples

### 5.1 Data Analysis Query

**User Prompt:** "Find cost-effective EC2 instances in us-east-1 with at least 32GB RAM"

**LLM Agent Workflow:**
1. `list_tables()` - Discover EC2Instances table
2. `get_table_schema("EC2Instances")` - Understand columns
3. `query_table()`:
   ```sql
   SELECT InstanceType, RAM, Price, (Price * 730 hr/month) as MonthlyCost
   FROM EC2Instances
   WHERE Region = 'us-east-1' AND RAM >= 32GB
   ORDER BY Price ASC
   LIMIT 10
   ```
4. Return formatted results with units

**Key Features:**
- Automatic unit conversion (>= 32GB works even if stored in different units)
- Compound unit calculation (Price × 730hr/month → USD/month)
- Context-aware results (prices with units)

---

### 5.2 Data Entry & Validation

**User Prompt:** "Add a new m5.xlarge instance in us-west-2 with 64GB RAM at $0.192/hr"

**LLM Agent Workflow:**
1. `get_table_schema("EC2Instances")` - Check schema
2. `get_valid_values("EC2Instances", "Region")` - Verify region is valid
3. `validate_cell()` - Check all values
4. `append_row()`:
   ```json
   {
     "table_name": "EC2Instances",
     "row_data": {
       "Region": "us-west-2",
       "InstanceType": "m5.xlarge",
       "RAM": {"value": 64, "unit": "GB"},
       "Price": {"value": 0.192, "unit": "USD/hr"}
     }
   }
   ```
5. Confirm success

**Key Features:**
- Schema-aware insertion
- Validation before write
- Unit preservation

---

### 5.3 Unit Conversion Analysis

**User Prompt:** "Convert all prices to EUR and show monthly costs"

**LLM Agent Workflow:**
1. `get_conversion_rate("USD", "EUR")` - Get current rate
2. `query_table()`:
   ```sql
   SELECT InstanceType, Price, (Price * 730 hr/month) as MonthlyCost
   FROM EC2Instances
   ```
3. For each row:
   - `convert_value(price, "USD/hr", "EUR/hr")`
   - Calculate monthly in EUR
4. Return formatted table

**Key Features:**
- Current conversion rates with transparency
- Compound unit conversion
- Rate metadata (live vs manual)

---

### 5.4 Schema Discovery

**User Prompt:** "What data do you have about cloud costs?"

**LLM Agent Workflow:**
1. `get_workbook_metadata()` - See all sheets/tables
2. `list_tables()` - Get table list
3. For each table:
   - `get_table_schema()` - Understand structure
4. Summarize:
   - "I have 2 tables: EC2Instances and RDSInstances"
   - "EC2Instances has 49 instances with columns: Region, Type, RAM, Storage, Price"
   - "Prices are in USD/hr, can calculate monthly costs"

**Key Features:**
- Self-describing schema
- Entity-aware tables
- Unit-aware columns

---

### 5.5 Validation & Error Handling

**User Prompt:** "Set RAM to 100 TerraBytes for instance m5.large"

**LLM Agent Workflow:**
1. `validate_cell("EC2Instances", "RAM", 100, "TB")` - Check validity
2. Result: Valid but unusual
3. Confirm with user: "Did you mean 100 GB instead of 100 TB? That's 100,000 GB, which is much larger than typical instance RAM."
4. User corrects: "Yes, 100 GB"
5. `update_row()` with correct value

**Key Features:**
- Validation catches errors
- Unit understanding prevents mistakes
- Graceful error handling

---

### 5.6 Aggregation & Statistics

**User Prompt:** "What's the average monthly cost across all instances?"

**LLM Agent Workflow:**
1. `query_table()`:
   ```sql
   SELECT AVG(Price * 730 hr/month) as AvgMonthlyCost,
          COUNT(*) as InstanceCount
   FROM EC2Instances
   ```
2. Result:
   ```json
   {
     "AvgMonthlyCost": {"value": 123.50, "unit": "USD/month"},
     "InstanceCount": {"value": 49, "unit": "instances"}
   }
   ```
3. Return: "The average monthly cost is $123.50/month across 49 instances"

**Key Features:**
- Aggregate functions preserve units
- Context-aware COUNT (returns "instances")
- Compound unit calculations

---

## 6. Error Handling

### 6.1 Error Types

**Unit Errors:**
```json
{
  "error": "InvalidUnit",
  "message": "Unit 'dollars' not recognized",
  "suggestions": ["USD", "EUR", "GBP"],
  "code": "UNIT_001"
}
```

**Validation Errors:**
```json
{
  "error": "ValidationFailed",
  "message": "Value does not meet column requirements",
  "details": [
    {"field": "Region", "error": "Must be one of: us-east-1, us-west-2"},
    {"field": "RAM", "error": "Unit must be StorageSize dimension"}
  ],
  "code": "VAL_001"
}
```

**Query Errors:**
```json
{
  "error": "QuerySyntaxError",
  "message": "SQL syntax error near 'FORM'",
  "query": "SELECT * FORM EC2Instances",
  "position": 10,
  "code": "SQL_001"
}
```

**Conversion Errors:**
```json
{
  "error": "ConversionNotAvailable",
  "message": "No conversion path found from 'm' to 'kg'",
  "from_unit": "m",
  "to_unit": "kg",
  "reason": "Incompatible dimensions: Length vs Mass",
  "code": "CONV_001"
}
```

### 6.2 Warning Types

**Unit Warnings:**
```json
{
  "warning": "IncompatibleUnits",
  "message": "Adding incompatible units (m + s). Result will be dimensionless.",
  "severity": "medium",
  "code": "WARN_UNIT_001"
}
```

**Conversion Warnings:**
```json
{
  "warning": "IndirectConversion",
  "message": "Using chained conversion: EUR → USD → GBP",
  "path": ["EUR", "USD", "GBP"],
  "severity": "low",
  "code": "WARN_CONV_001"
}
```

**Data Quality Warnings:**
```json
{
  "warning": "UnusualValue",
  "message": "Value 100TB is unusually large for RAM",
  "typical_range": "0.5GB to 384GB",
  "severity": "medium",
  "code": "WARN_DATA_001"
}
```

---

## 7. Implementation Notes

### 7.1 Performance Considerations

**Batching:**
- Use `read_range` instead of multiple `read_cell` calls
- Use `write_range` instead of multiple `write_cell` calls
- Query results capped at 10,000 rows (use LIMIT)

**Caching:**
- Conversion rates cached for 1 hour (Live mode)
- Schema cached until workbook modification
- Unit library static (no fetching)

**Timeouts:**
- Tool calls timeout after 30 seconds
- Queries timeout after 5 seconds
- Long operations return partial results

### 7.2 Security & Permissions

**Read-Only Mode:**
- Disable all write tools
- Enable all read/query tools
- Resource URIs still accessible

**Validation Mode:**
- All writes validated by default
- Can disable with `validate: false` (not recommended)
- Schema changes require elevated permissions

**Audit Log:**
- All write operations logged
- Query history tracked
- Conversion rate fetches logged

### 7.3 Extensibility

**Custom Tools:**
- Users can add custom MCP tools
- Access to full workbook API
- Can define new resource types

**Tool Composition:**
- Tools designed to compose
- Results can feed other tools
- Pipelines supported

**Future Enhancements:**
- Natural language query tool (AI-generated SQL)
- Batch operations tool (multiple writes)
- Export tool (to various formats)
- Chart generation tool (unit-aware)

---

## 8. Summary

This MCP tool design makes Unicel a powerful, LLM-accessible datastore with unique unit-awareness capabilities. Key strengths:

**For LLMs:**
- Clear, semantic tool names
- Rich context in all responses
- Unit metadata everywhere
- Validation before errors

**For Users:**
- Natural language queries → SQL
- Automatic unit conversions
- Validation prevents mistakes
- Self-describing schema

**For Developers:**
- Standard MCP protocol
- Composable tools
- Extensible design
- Well-defined errors

**Unique Value:**
- Only datastore with native unit support
- Dimensional analysis built-in
- Automatic conversions
- Perfect for quantitative AI agents

---

**Next Steps:**
1. Implement core read tools (read_cell, query_table)
2. Implement basic write tools (write_cell, append_row)
3. Add conversion tools (convert_value, get_conversion_rate)
4. Build schema tools (get_table_schema, list_tables)
5. Add validation tools (validate_cell, check_formula)
6. Register resource URIs
7. Write integration tests
8. Document with examples

---

**End of Document**
