# Unicel v0.1.6 - Named Ranges Release

**Release Date:** 2025-10-14

This release adds Named Ranges functionality, allowing you to assign memorable names to cells and reference them in formulas.

## New Features

### Named Ranges (Named Cells)

You can now create user-defined names for cells and use them in formulas:

**Inline Label Syntax:**
```
tax_rate: 0.15          # Creates named range "tax_rate" with value 0.15
price: $100             # Creates named range "price" with value $100
total:= A1+A2          # Creates named range "total" with formula =A1+A2
```

**Formula References:**
```
A1: price: $100
A2: tax_rate: 0.15
A3: =price * tax_rate   # Result: $15
```

**Key Features:**
- User-friendly names for cells (e.g., `revenue`, `tax_rate`, `conversion_usd_to_eur`)
- Two inline syntaxes: `name: value` and `name:= formula`
- Reference named ranges in formulas instead of cell addresses
- Full serialization - named ranges persist in `.usheet` files
- Excel export support - named ranges exported to Excel workbooks
- Validation: names must start with lowercase letter or underscore, no conflicts
- UI: New "Named Ranges" button (🏷️) in ribbon opens management dialog
- Management dialog shows all named ranges with sheet names and addresses
- Delete functionality with confirmation dialog

**Benefits:**
- More readable formulas: `=revenue * tax_rate` vs `=A1 * B2`
- Self-documenting spreadsheets
- Easier maintenance when cell positions change
- Ideal for constants, assumptions, and key metrics

## Bug Fixes

- Fixed formula evaluation for named references - they now properly resolve in workbook context
- All 206 tests passing

## Technical Changes

- Modified `SheetEvaluator` to accept optional named ranges HashMap
- Added `resolve_named_ranges()` method to Workbook
- Added `evaluate_formula_with_named_refs()` and `recalculate_with_named_refs()` methods to Sheet
- Updated all recalculation call sites to resolve named ranges before evaluation
- Added Tauri commands: `list_named_ranges`, `create_named_range`, `delete_named_range`, `get_named_range`
- Added NamedRangesDialog React component
- Integrated Named Ranges button in Ribbon component

---

# Unicel v0.1.0 - MVP Release

**Release Date:** 2025-10-14

The first public release of Unicel, a unit-aware spreadsheet that treats units as first-class data types.

## What is Unicel?

Unicel is a next-generation spreadsheet where units are data, not formatting. Values are stored as `(number, unit)` tuples, enabling:
- Automatic unit conversion between metric and imperial systems
- Dimensional analysis that validates formula operations
- Unit cancellation (e.g., `$100/hr × 720hr/month → $72,000/month`)
- Type-safe calculations that preserve units through all operations

## Core Features

### 1. Unit-Aware Calculations

- **Store units with values**: Enter `100 m` or `50 USD` and the unit is preserved
- **Automatic unit cancellation**: `=A1/A2` where A1=100 USD and A2=5 hours → 20 USD/hour
- **Compound units**: Support for units like `mi/hr`, `USD/GB/month`, `kg*m/s^2`
- **Compatible unit addition**: `=100m + 200ft` automatically converts and adds compatible units
- **Soft warnings**: Orange indicator for incompatible operations (e.g., adding meters + seconds)

### 2. Non-Destructive Display Conversion

- **Three display modes**: As Entered, Metric, Imperial
- **Toggle without data loss**: Switch between `100 m` ↔ `328.08 ft` without changing storage
- **Formula preservation**: Formulas always use storage units for consistency
- **Visual indicators**: Blue background indicates formula cells

### 3. Spreadsheet Fundamentals

- **Grid interface**: Familiar spreadsheet with rows (1-50) and columns (A-Z)
- **Cell selection**: Click to select, double-click to edit
- **Formula support**: Full formula evaluation with cell references (=A1+B1, =SUM(A1:A10))
- **Formula bar**: Edit formulas in dedicated input above grid
- **Keyboard navigation**: Arrow keys, Enter, Escape
- **Cell indicators**:
  - Blue "ƒ" symbol for formula cells
  - Orange "⚠️" for cells with warnings
  - Right-aligned numeric values

### 4. File Operations

- **Native file format**: `.usheet` files in JSON format (LLM-friendly)
- **Save/Load**: Full workbook persistence with all formulas and units
- **Excel export**: One-way export to `.xlsx` with:
  - Values exported as plain numbers (Excel-compatible)
  - Unit metadata sheet documenting original units
  - Warning sheet explaining limitations
  - Formula cells show calculated results (not formulas, as Excel can't recalculate with units)

### 5. User Interface

- **Modern desktop app**: Native application built with Tauri
- **Ribbon interface**: File menu, display mode toggle, unit settings
- **Status bar**: Shows selected cell, cell count, and unit of selected cell
- **Toast notifications**: User feedback for operations (save, load, export)
- **Loading overlay**: Visual feedback during long operations
- **Responsive design**: Tailwind CSS with hover effects and transitions

### 6. Keyboard Shortcuts

- **Ctrl/Cmd+N**: New workbook
- **Ctrl/Cmd+O**: Open workbook
- **Ctrl/Cmd+S**: Save workbook
- **Ctrl/Cmd+Shift+S**: Save As
- **Enter**: Commit edit and move to next row
- **Escape**: Cancel edit
- **Arrow Keys**: Navigate cells (when not editing)

## Supported Units

### Length
- Metric: mm, cm, m, km
- Imperial: in, ft, yd, mi

### Mass
- Metric: mg, g, kg
- Imperial: oz, lb

### Time
- Seconds, minutes, hours, days, weeks, months, years

### Temperature
- Celsius, Fahrenheit, Kelvin

### Digital Storage
- Bits: bit, kbit, mbit, gbit, tbit, pbit
- Bytes: B, KB, MB, GB, TB, PB

### Currency
- USD, EUR, GBP, JPY (manual rates for MVP)

### Tokens (AI/ML)
- tok, ktok, mtok (for LLM pricing calculations)

### Energy & Power
- J (joules), kWh (kilowatt-hours)
- W (watts), kW, MW

## Technical Highlights

- **227 tests passing**: Comprehensive test coverage including unit tests, integration tests, and property-based tests
- **Rust backend**: Fast, memory-safe calculation engine
- **React frontend**: Modern, responsive UI with TypeScript
- **SQLite runtime**: In-memory database for fast queries during session
- **JSON storage**: Human-readable, version-controllable file format
- **MCP integration**: Model Context Protocol server for AI tool integration (read_cell, write_cell, query_table)

## Example Use Cases

### Financial Calculations
```
A1: 50000 USD        # Annual salary
A2: 12 months        # Months per year
A3: =A1/A2           # Result: 4166.67 USD/month
```

### Unit Conversion
```
A1: 100 mi           # Distance in miles
Display Mode: Metric
                     # Shows: 160.93 km
```

### Cloud Cost Analysis
```
A1: 0.10 USD         # Cost per GB
A2: 1000 GB          # Storage used
A3: 720 hours        # Hours per month
A4: =A1*A2           # Cost: 100 USD
A5: =A4/A3           # Rate: 0.139 USD/hour
```

### Construction Estimating
```
A1: 100 ft           # Length
A2: 50 ft            # Width
A3: =A1*A2           # Area: 5000 ft^2
A4: 5 USD            # Cost per square foot
A5: =A3*A4           # Total: 25000 USD
```

## Known Limitations

- **Single sheet**: Multi-sheet support planned for future release
- **Limited functions**: Only SUM() and AVERAGE() implemented (more planned)
- **No undo/redo**: Planned for next release
- **Manual currency rates**: Live rates via MCP planned for future
- **No tables**: SQL-queryable tables planned for future
- **No charts**: Visualization planned for future
- **Excel export is one-way**: Cannot import Excel files back to Unicel
- **Formula mode cell picker**: Arrow key navigation in formulas for cell picking

## Breaking Changes

This is the first release, so no breaking changes from previous versions.

## Bug Fixes

- Fixed false positive warnings on cells with empty warning strings
- Fixed false positive formula indicators on cells with empty formula strings
- Formula indicator positioning now preserves numeric cell right-alignment

## Installation

### macOS

Download `Unicel_0.1.0_x64.dmg` from the releases page:
1. Open the DMG file
2. Drag Unicel to Applications folder
3. Launch from Applications

### Building from Source

```bash
git clone https://github.com/jacksodj/unicel.git
cd unicel
npm install
npm run tauri:build
```

Binary will be in `src-tauri/target/release/bundle/`

## What's Next?

Planned for future releases:
- Multi-sheet workbooks
- Undo/redo functionality
- More formula functions (COUNT, MIN, MAX, IF, etc.)
- Live currency rates via MCP
- SQL-queryable tables with entity-aware operations
- Charts and visualizations
- Custom unit definitions
- Column validation and default units
- Conversion rate history and trust chains
- Template workbooks for common use cases

## Feedback and Support

- **Issues**: https://github.com/jacksodj/unicel/issues
- **Discussions**: https://github.com/jacksodj/unicel/discussions
- **Documentation**: See README.md and docs/ folder

## Credits

Built with:
- Tauri 2.x for native desktop framework
- Rust for calculation engine
- React + TypeScript for UI
- SQLite for runtime database
- pest for formula parsing
- rust_xlsxwriter for Excel export

## License

Dual-licensed under MIT OR Apache-2.0. See LICENSE-MIT and LICENSE-APACHE for details.

---

Thank you for trying Unicel! We're excited to see what you build with unit-aware spreadsheets.
