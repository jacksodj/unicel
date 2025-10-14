# Unicel User Guide

**Version 0.1.0** | **Unit-Aware Spreadsheet Application**

---

## Table of Contents

1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Understanding Units](#understanding-units)
4. [Working with Cells](#working-with-cells)
5. [Formulas and Functions](#formulas-and-functions)
6. [Unit Conversions](#unit-conversions)
7. [Working with Sheets](#working-with-sheets)
8. [File Operations](#file-operations)
9. [MCP Integration (AI Access)](#mcp-integration-ai-access)
10. [Advanced Features](#advanced-features)
11. [Troubleshooting](#troubleshooting)

---

## Introduction

### What is Unicel?

Unicel is a **unit-aware spreadsheet** application that treats units as first-class data types. Unlike traditional spreadsheets where units are just formatting or text, Unicel understands units and performs automatic dimensional analysis.

### Key Features

- **🔢 Units as Data**: Values are stored as `(number, unit)` tuples, not just numbers
- **🔄 Automatic Conversions**: Convert between compatible units seamlessly
- **⚡ Dimensional Analysis**: Prevents unit mismatch errors automatically
- **🧮 Smart Calculations**: Operations understand unit math (e.g., `m/s × s = m`)
- **🌐 Multi-Currency**: Mix USD, EUR, GBP in the same calculation
- **🤖 AI Integration**: Access via Model Context Protocol (MCP)

### What Makes Unicel Different?

| Traditional Spreadsheet | Unicel |
|------------------------|--------|
| `100` with "USD" formatting | `100 USD` (actual unit type) |
| Manual conversion formulas | Automatic unit conversion |
| No unit validation | Dimensional analysis prevents errors |
| `mi/hr` as text | Compound unit with operations |

---

## Getting Started

### Installation

#### From Source

```bash
# Clone the repository
git clone https://github.com/anthropics/unicel.git
cd unicel

# Install dependencies
npm install

# Run development build
npm run tauri dev
```

#### Building for Production

```bash
# Create production build
npm run tauri build

# Binary will be in src-tauri/target/release/
```

### First Launch

1. Launch Unicel
2. You'll see a blank workbook with one sheet ("Sheet1")
3. Click on any cell to start entering data

### Quick Tour

Open the tutorial workbook to see Unicel in action:

```bash
# Open the tutorial
File → Open → examples/unit_conversion_tutorial.usheet
```

This workbook contains 9 sheets demonstrating all unit conversion features.

---

## Understanding Units

### What Are Units in Unicel?

In Unicel, every cell value can have a unit attached. Units are not just labels—they're part of the data type.

**Example:**
- Cell A1: `100 USD`
- Cell A2: `50 EUR`
- Cell A3: `=A1 + A2` → Automatically converts to common currency

### Unit Categories

#### Length
- **Metric**: m (meters), cm, mm, km
- **Imperial**: in (inches), ft, yd, mi
- **Conversions**: Automatic between metric and imperial

#### Mass/Weight
- **Metric**: g (grams), kg, mg
- **Imperial**: oz (ounces), lb (pounds)
- **Conversions**: Automatic with proper scaling

#### Time
- **Units**: s (seconds), min, hr (hour), day, month, year
- **Conversions**: 60s = 1min, 24hr = 1day, etc.

#### Temperature
- **Units**: C (Celsius), F (Fahrenheit), K (Kelvin)
- **Special**: Uses offset conversion (not just scaling)
- **Example**: 0°C = 32°F = 273.15K

#### Currency
- **Units**: USD, EUR, GBP
- **Note**: Exchange rates are currently hardcoded
- **Future**: Live rates via MCP external servers

#### Digital Storage
- **Bytes**: B, KB, MB, GB, TB, PB
- **Bits**: b, Kb, Mb, Gb, Tb, Pb
- **Tokens**: Tok (tokens), MTok (million tokens)
- **Conversion**: 1 KB = 1024 B (binary) or 1000 B (decimal)

### Dimensionless Values

Numbers without units are **dimensionless**:
- Counts: `45` items
- Ratios: `2.5` (ratio)
- Percentages: `0.85` (85%)

---

## Working with Cells

### Entering Data

#### Numbers with Units

Simply type the number followed by the unit:

```
100 USD
50 m
2.5 kg
32 F
```

#### Text

Type any text without a recognized unit pattern:

```
Product Name
Status: Active
Notes: Check this later
```

#### Formulas

Start with `=` to create a formula:

```
=A1 + A2
=B5 * C5
=SUM(A1:A10)
```

### Cell Display

Each cell shows:
- **Value**: The numeric value or text
- **Unit**: The storage unit (what you entered)
- **Display Unit**: Optional different display (e.g., show km as mi)
- **Warning**: Yellow ⚠️ if unit mismatch detected

### Editing Cells

1. **Single-click**: Select cell
2. **Double-click** or **F2**: Edit mode
3. **Enter**: Confirm and move down
4. **Tab**: Confirm and move right
5. **Esc**: Cancel edit

### Cell Types

Unicel automatically detects cell types:

| Input | Type | Storage |
|-------|------|---------|
| `100 USD` | Number with unit | `(100.0, USD)` |
| `Hello` | Text | `"Hello"` |
| `=A1+A2` | Formula | Formula AST + result |
| (empty) | Empty | No data |

---

## Formulas and Functions

### Formula Basics

All formulas start with `=`:

```
=A1 + B1
=C5 * D5
=100 * 2
```

### Unit-Aware Operations

#### Addition/Subtraction
Units must be compatible (same dimension):

```
=100m + 50m      → 150m
=5ft + 3in       → 5.25ft (auto-converted)
=10USD + 5EUR    → Converted to common currency
=5m + 10s        → ⚠️ Warning (incompatible)
```

#### Multiplication/Division
Creates compound units:

```
=100mi / 2hr     → 50 mi/hr (speed)
=50USD * 40hr    → 2000 USD (USD/hr × hr → USD cancels)
=1000GB / 100MB  → 10 (dimensionless, units cancel)
```

### Cell References

#### Absolute vs Relative

```
=A1              # Relative (moves when copied)
=$A$1            # Absolute (stays fixed)
=$A1             # Column fixed, row relative
=A$1             # Row fixed, column relative
```

#### Cross-Sheet References

```
='Sheet2'!A1
='Sales Data'!B5
```

### Available Functions

#### SUM
Add a range of cells:

```
=SUM(A1:A10)           # Sum of A1 through A10
=SUM(A1:A5, A10:A15)   # Sum of multiple ranges
```

#### COUNT
Count non-empty cells:

```
=COUNT(A1:A10)         # Count of values
```

#### AVERAGE
Average of values (unit-aware):

```
=AVERAGE(A1:A10)       # Average with unit preserved
```

#### IF
Conditional logic:

```
=IF(A1 > 100, "High", "Low")
=IF(B5 > 0, B5, 0)
```

#### MIN/MAX
Find minimum or maximum:

```
=MIN(A1:A10)           # Smallest value (with unit)
=MAX(A1:A10)           # Largest value (with unit)
```

### Formula Examples

#### Revenue Calculation
```
# A1: Price per unit (USD)
# A2: Quantity (dimensionless)
# A3: =A1 * A2  → Total revenue (USD)
```

#### Speed Calculation
```
# A1: Distance (100 mi)
# A2: Time (2 hr)
# A3: =A1 / A2  → Speed (50 mi/hr)
```

#### Storage Utilization
```
# A1: Used storage (500 GB)
# A2: Total storage (2 TB)
# A3: =A1 / A2  → Ratio (0.25, dimensionless)
# A4: =A3 * 100 → Percentage (25%)
```

---

## Unit Conversions

### Automatic Conversion

Unicel automatically converts between compatible units in operations:

```
Cell A1: 100 m
Cell A2: 50 ft
Cell A3: =A1 + A2  → Converts ft to m, shows 115.24 m
```

### Display Unit vs Storage Unit

**Key Concept**: Storage unit is immutable, display unit is for viewing only.

- **Storage Unit**: What you entered, never changes
- **Display Unit**: How it's shown (can toggle Metric ↔ Imperial)

**Example**:
- You enter: `100 mi`
- Storage: `100 mi` (never changes)
- Display (Metric mode): `160.93 km` (non-destructive)
- Toggle back: Shows `100 mi` again

### Manual Conversion (CONVERT function)

```
=CONVERT(100, "m", "ft")      → Converts 100 m to feet
=CONVERT(A1, "USD", "EUR")    → Converts A1 from USD to EUR
```

### Display Preferences

Three display modes:

1. **As Entered**: Show units exactly as entered
2. **Metric**: Prefer metric units (m, kg, km)
3. **Imperial**: Prefer imperial units (ft, lb, mi)

Change via: `File → Preferences → Display Units`

### Conversion Rates

#### Static Conversions
These are hardcoded and always accurate:
- Length: 1 mi = 1.60934 km
- Mass: 1 lb = 0.453592 kg
- Time: 1 hr = 3600 s
- Temperature: Proper offset conversion

#### Currency Conversions
Currently uses hardcoded rates (demonstration mode):
- 1 USD = 0.94 EUR
- 1 USD = 0.79 GBP
- 1 EUR = 0.84 GBP

**Future**: Live rates via MCP external servers

### Unit Compatibility

#### Compatible Units (Can Convert)
- All lengths: m, cm, km, in, ft, mi
- All masses: g, kg, oz, lb
- All times: s, min, hr, day
- All temperatures: C, F, K
- All currencies: USD, EUR, GBP
- All storage: B, KB, MB, GB, TB

#### Incompatible Units (Warning)
- Length + Time: `5m + 10s` → ⚠️ Warning
- Mass + Money: `100kg + 50USD` → ⚠️ Warning
- Temperature + Length: `32F + 100m` → ⚠️ Warning

When incompatible units are combined, Unicel:
- Shows a warning (yellow cell, ⚠️ icon)
- Still performs the operation (fail-soft)
- Result becomes dimensionless

---

## Working with Sheets

### Sheet Management

#### Creating Sheets
- Click the `+` button next to sheet tabs
- Or: `Right-click sheet tab → Insert Sheet`

#### Renaming Sheets
- Double-click the sheet tab
- Type new name
- Press Enter

#### Deleting Sheets
- Right-click sheet tab → Delete Sheet
- Note: Cannot delete the last sheet

#### Reordering Sheets
- Drag and drop sheet tabs to reorder

### Sheet Navigation

- Click sheet tabs to switch between sheets
- Keyboard shortcuts:
  - `Ctrl+PageDown`: Next sheet
  - `Ctrl+PageUp`: Previous sheet

### Cross-Sheet Formulas

Reference cells in other sheets:

```
='Sheet2'!A1
='Data'!B5:B10
=SUM('Q1 Sales'!A:A)
```

---

## File Operations

### File Format

Unicel uses `.usheet` format (JSON-based):
- Human-readable JSON structure
- LLM-friendly for AI interactions
- Embeds all data: cells, formulas, units, settings

**File Extension**: `.usheet` or `.usheet.json`

### Creating a New Workbook

```
File → New
```

Creates a blank workbook with one sheet.

### Opening Files

```
File → Open → Select .usheet file
```

#### Example Files

Unicel includes example workbooks:

1. **unit_conversion_tutorial.usheet** - Learn all conversion features
2. **construction_estimator.usheet** - Dimensional calculations
3. **aws_cost_estimator.usheet** - Cloud cost analysis
4. **investment_portfolio.usheet** - Multi-currency tracking

### Saving Files

```
File → Save (Ctrl+S)
File → Save As... (Ctrl+Shift+S)
```

Files are saved as JSON with the following structure:

```json
{
  "version": "1.0",
  "workbook_settings": {
    "unit_preference": "AsEntered",
    "auto_recalculate": true
  },
  "sheets": [
    {
      "name": "Sheet1",
      "cells": {
        "A1": {
          "value": 100.0,
          "unit": "USD",
          "formula": null
        }
      }
    }
  ]
}
```

### Exporting

#### Excel Export (Planned)
```
File → Export → Excel (.xlsx)
```

Exports to Excel format with:
- Main data sheet
- Metadata sheet (units, formulas)
- One-way export (cannot import back)

---

## MCP Integration (AI Access)

### What is MCP?

**Model Context Protocol (MCP)** is a standard protocol that allows AI assistants like Claude to interact with external data sources and tools.

Unicel includes an MCP server that exposes workbook operations to AI agents.

### Starting the MCP Server

```bash
# Start with a workbook file
./target/debug/unicel-mcp-server path/to/workbook.usheet

# Start with empty workbook
./target/debug/unicel-mcp-server
```

The server reads JSON-RPC requests from stdin and writes responses to stdout.

### Integration with Claude Desktop

Add to your Claude Desktop configuration (`claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "unicel": {
      "command": "/path/to/unicel-mcp-server",
      "args": ["/path/to/workbook.usheet"]
    }
  }
}
```

**Location**:
- macOS: `~/Library/Application Support/Claude/claude_desktop_config.json`
- Windows: `%APPDATA%\Claude\claude_desktop_config.json`
- Linux: `~/.config/Claude/claude_desktop_config.json`

### Available MCP Tools

Once configured, Claude can use these tools:

#### read_cell
Read a single cell with full metadata:
```
"Read cell A1 from the spreadsheet"
```

Returns: value, unit, formula, warnings, type flags

#### write_cell
Write values to cells:
```
"Write 100 USD to cell B5"
"Set cell C10 to the formula =A1+A2"
```

#### convert_value
Convert between units:
```
"Convert 50 meters to feet"
"What is 100 USD in EUR?"
```

#### list_tables
List all sheets:
```
"What sheets are in this workbook?"
```

#### get_workbook_metadata
Get workbook information:
```
"Show me the workbook structure"
```

See [MCP_SERVER.md](./MCP_SERVER.md) for complete tool documentation.

### Use Cases

- **Data Analysis**: "Analyze the cost data in Sheet2"
- **Data Entry**: "Add the Q4 sales figures to the workbook"
- **Conversions**: "Convert all distances to metric"
- **Reporting**: "Summarize the budget by currency"
- **Validation**: "Check if any cells have unit mismatches"

---

## Advanced Features

### Compound Units

Compound units are created automatically through operations:

#### Speed (Length / Time)
```
Distance: 100 mi
Time: 2 hr
Speed: =Distance / Time  → 50 mi/hr
```

#### Rates (Money / Time)
```
Hourly Rate: 75 USD/hr
Hours: 40 hr
Payment: =Rate * Hours  → 3000 USD (units cancel)
```

#### Density (Mass / Volume)
```
Mass: 500 kg
Volume: 2 m³
Density: =Mass / Volume  → 250 kg/m³
```

### Unit Cancellation

When dividing same dimensions, units cancel to dimensionless:

```
Distance1: 100 m
Distance2: 50 m
Ratio: =Distance1 / Distance2  → 2 (dimensionless)
```

This is **dimensional analysis** in action—ensuring calculations are physically meaningful.

### Storage vs Display Separation

**Example Scenario**:

1. You enter hotel costs in EUR: `150 EUR`
2. Storage: Always remains `150 EUR`
3. Display (USD mode): Shows `~$160` (converted for viewing)
4. Toggle back to EUR: Shows `150 EUR` (original)
5. Formulas: Always use storage unit (EUR)

**Benefits**:
- Non-destructive viewing
- Toggle display without losing precision
- Formulas use accurate storage values

### Custom Units (Future)

Future versions will support custom business units:

```
# Define custom units
1 instance = 1 EC2Instance
1 request = 1 APIRequest

# Use in formulas
Cost: 100 USD/instance
Instances: 45 instances
Total: =Cost * Instances  → 4500 USD
```

### Tables and Entities (Future)

Tables represent entities with metadata:

```
Table: EC2Instances
Entity Type: AWS::EC2::Instance
Row Unit: instance

COUNT(*) → Returns "45 instances" (not just "45")
```

---

## Troubleshooting

### Common Issues

#### "Unit mismatch" warning
**Problem**: Adding incompatible units (e.g., `5m + 10s`)
**Solution**: Check that units are compatible or intentionally convert

#### Formula shows #ERROR
**Problem**: Invalid formula syntax or circular reference
**Solution**: Check formula syntax, verify no circular references

#### Conversion seems wrong
**Problem**: Unexpected conversion result
**Solution**:
- Check storage vs display unit
- Verify units are compatible dimension
- Temperature uses offset conversion (not linear)

#### Can't save file
**Problem**: Permission or path error
**Solution**: Check file path exists and you have write permission

### Debug Mode

Enable debug logging:

```bash
RUST_LOG=debug ./target/debug/unicel
```

Shows detailed logs for:
- Formula evaluation
- Unit conversions
- File operations
- MCP requests

### Getting Help

- **Documentation**: `docs/` directory
- **Issues**: https://github.com/anthropics/unicel/issues
- **Examples**: `examples/` directory
- **Tests**: `src-tauri/tests/` for usage examples

---

## Keyboard Shortcuts

### Navigation
- `Arrow Keys`: Move between cells
- `Tab`: Move right
- `Shift+Tab`: Move left
- `Enter`: Move down
- `Shift+Enter`: Move up
- `Ctrl+Home`: Go to A1
- `Ctrl+End`: Go to last used cell

### Editing
- `F2`: Edit current cell
- `Esc`: Cancel edit
- `Ctrl+C`: Copy
- `Ctrl+V`: Paste
- `Ctrl+X`: Cut
- `Ctrl+Z`: Undo
- `Ctrl+Y`: Redo

### File Operations
- `Ctrl+N`: New workbook
- `Ctrl+O`: Open file
- `Ctrl+S`: Save
- `Ctrl+Shift+S`: Save As

### Sheets
- `Ctrl+PageDown`: Next sheet
- `Ctrl+PageUp`: Previous sheet

---

## Tips & Best Practices

### 1. Always Specify Units

```
✓ Good:  100 USD, 50 m, 2.5 kg
✗ Bad:   100, 50, 2.5 (ambiguous)
```

### 2. Use Dimensional Analysis

Let Unicel catch unit errors:
```
=Distance / Time  → Speed (automatic unit)
=Price * Quantity → Cost (automatic unit)
```

### 3. Leverage Storage vs Display

- Enter data in one unit system
- Toggle display for different audiences
- Formulas always use storage (accurate)

### 4. Document with Text Cells

Use text cells for headers and labels:
```
A1: "Monthly Budget"
A2: "Revenue"
A3: "Expenses"
```

### 5. Use Example Workbooks

Learn by exploring the example workbooks:
- Tutorial for learning
- Estimators for templates
- Portfolio for multi-currency patterns

---

## What's Next?

### Current Status (Phase 6)

✅ Core unit system with automatic conversions
✅ Formula engine with dimensional analysis
✅ Multi-sheet workbooks
✅ JSON file format
✅ MCP server for AI integration
✅ Example workbooks

### Coming Soon

**Phase 7** (Current):
- 🚧 User interface improvements
- 🚧 Visual design polish
- 🚧 Tutorial content

**Phase 8** (Next):
- Excel export
- Enhanced error messages
- Performance optimizations

**Future Features**:
- Live currency rates (via MCP external servers)
- Custom unit definitions
- Table entities with metadata
- SQL queries on tables
- Collaboration features
- Cloud sync

---

## Appendix: Unit Reference

### Length Units

| Symbol | Name | Metric? |
|--------|------|---------|
| m | meter | Yes |
| cm | centimeter | Yes |
| mm | millimeter | Yes |
| km | kilometer | Yes |
| in | inch | No |
| ft | foot | No |
| yd | yard | No |
| mi | mile | No |

**Conversions**:
- 1 km = 1000 m
- 1 m = 100 cm = 1000 mm
- 1 mi = 1760 yd = 5280 ft
- 1 ft = 12 in
- 1 mi ≈ 1.609 km

### Mass Units

| Symbol | Name | Metric? |
|--------|------|---------|
| g | gram | Yes |
| mg | milligram | Yes |
| kg | kilogram | Yes |
| oz | ounce | No |
| lb | pound | No |

**Conversions**:
- 1 kg = 1000 g
- 1 g = 1000 mg
- 1 lb = 16 oz
- 1 kg ≈ 2.205 lb

### Time Units

| Symbol | Name |
|--------|------|
| s | second |
| min | minute |
| hr, h, hour | hour |
| day | day |
| month | month (30 days) |
| year | year (365.25 days) |

**Conversions**:
- 1 min = 60 s
- 1 hr = 60 min = 3600 s
- 1 day = 24 hr
- 1 year ≈ 365.25 days

### Temperature Units

| Symbol | Name | Notes |
|--------|------|-------|
| C | Celsius | Water freezes at 0°C |
| F | Fahrenheit | Water freezes at 32°F |
| K | Kelvin | Absolute scale, 0K = absolute zero |

**Conversions** (with offsets):
- °F = (°C × 9/5) + 32
- K = °C + 273.15
- °C = (°F - 32) × 5/9

### Currency Units

| Symbol | Name |
|--------|------|
| USD | US Dollar |
| EUR | Euro |
| GBP | British Pound |

**Note**: Rates are currently hardcoded for demonstration.

### Digital Storage Units

| Symbol | Name | Value |
|--------|------|-------|
| B | Byte | 8 bits |
| KB | Kilobyte | 1024 B |
| MB | Megabyte | 1024 KB |
| GB | Gigabyte | 1024 MB |
| TB | Terabyte | 1024 GB |
| PB | Petabyte | 1024 TB |

**Bits**:
| Symbol | Name | Value |
|--------|------|-------|
| b | bit | 1/8 byte |
| Kb | Kilobit | 1024 b |
| Mb | Megabit | 1024 Kb |
| Gb | Gigabit | 1024 Mb |
| Tb | Terabit | 1024 Tb |
| Pb | Petabit | 1024 Pb |

**Tokens** (LLM context):
| Symbol | Name | Value |
|--------|------|-------|
| Tok | Token | 1 token |
| MTok | Million tokens | 1,000,000 tokens |

---

**© 2025 Anthropic. All rights reserved.**

*Unicel is under active development. Features and interfaces may change.*
