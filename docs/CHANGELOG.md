# Changelog

All notable changes to Unicel will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.1] - 2025-10-17

### Added
- **Excel Export: Implicit Conversion Factor Injection**
  - Excel export now captures implicit unit conversions from arithmetic operations
  - Formulas with cross-scale operations (e.g., `TB × $/GB`) export with explicit conversion factors
  - Conversion factors added as named ranges in Conversions sheet
  - Works seamlessly with existing CONVERT function expansion

### Technical
- Implicit conversion detection in formula evaluation
- Automatic injection of conversion factor named ranges during Excel export
- Formula rewriting to include conversion factors (e.g., `=C1*A2*TB_GB_m`)

### Example
- **Before:** Formula `=B1*A2` (B1=100 TB, A2=15 $/GB) exported as calculated value `1536000`
- **After:** Exports as `=C1*A2*TB_GB_m` where `TB_GB_m` is a named range = 1024

### Benefits
- Users can see exactly what conversions are being applied
- Conversion factors can be modified directly in Excel
- Formula structure preserved instead of losing logic to calculated values
- Makes exported spreadsheets more transparent and maintainable

## [0.4.2] - 2025-10-17

### Added
- **Resizable columns and rows**: Drag column/row borders to resize with visual feedback
  - Drag handles on column headers (right edge) and row headers (bottom edge)
  - Real-time visual feedback during resize
  - Minimum constraints: 50px width, 20px height
  - Sizes persisted in .usheet files and restored on load
- **Insert columns and rows**: Right-click context menus to insert before/after
  - Cell shifting preserves all data: values, units, formulas, metadata
  - Formula reference updating automatically shifts cell references
  - Right-click context menus with hover highlighting
- **Delete columns and rows**: Right-click to delete with confirmation dialog
  - Confirmation dialogs with keyboard support (Enter/Escape)
  - Formula references become #REF! errors when targets are deleted
  - Cell shifting handles edge cases correctly

### Technical
- Backend: 6 new Sheet methods (`insert_column_before`, `insert_column_after`, `delete_column`, `insert_row_before`, `insert_row_after`, `delete_row`)
- Added 18 comprehensive tests for column/row operations
- Frontend: 2 new React components (`GridContextMenu`, `DeleteConfirmDialog`)
- All 287 tests passing

## [0.4.1] - 2025-10-17

### Fixed
- Fixed percentage unit handling in SheetEvaluator (properly handles % units in formulas)
- Fixed CONVERT function to handle compound unit conversions (e.g., mi/hr to km/hr)
- Fixed period units dimension to enable quarter/year conversions (aligned time period dimensions)
- Added frontend version info to debug export for better troubleshooting

## [0.4.0] - 2025-10-16

### Added
- Named range UI and improved dependency tracking
- Frontend version information in debug exports

### Fixed
- Preserved units when editing cells with bare numbers
- Fixed metric conversion for long-form unit names
- Fixed Excel export tests to work on Windows CI

## [0.3.0] - 2025-10-16

### Added

**UI Enhancements:**
- Auto-scroll to keep selected cell visible during arrow key navigation
- Improved arrow key navigation in formula mode (stays in edit mode, immediate picker movement)
- Arrow key exit for non-formula cells (saves and exits edit mode)
- Interactive base units display in StatusBar with clickable navigation
- Formula Functions Showcase added to examples dialog

**Formula Engine:**
- String concatenation with + operator (e.g., "Hello" + " world")
- String literal support in formula parser with proper escaping
- CEILING(number) function for rounding up to nearest integer
- CEILING(number, significance) function for rounding up to nearest multiple
- Full unit handling for CEILING function with dimensional analysis

**Excel Export:**
- Auto-convert string + operator to CONCATENATE() for Excel compatibility
- Preserve operator precedence with proper parenthesization
- Case-insensitive cell reference support (A1, a1, A1 all work)

### Fixed
- Fixed formula cell highlight after math operator insertion
- Fixed cell highlight logic for picker/editing/selected states
- Fixed auto-edit mode when typing in selected cells
- Fixed cell highlight bug when navigating with arrow keys
- Fixed temperature round-trip conversion precision issues

### Changed
- Updated 180 test assertions for new enum structure
- Added comprehensive test coverage for new features
- All 286 tests passing

## [0.2.0] - 2025-10-16

### Added
- Initial MVP release with core unit-aware spreadsheet functionality
- Unit system with dimensional analysis
- Formula engine with unit-aware calculations
- Basic Tauri integration
- File I/O with .usheet format

[0.5.1]: https://github.com/jacksodj/unicel/compare/v0.4.2...v0.5.1
[0.4.2]: https://github.com/jacksodj/unicel/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/jacksodj/unicel/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/jacksodj/unicel/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/jacksodj/unicel/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/jacksodj/unicel/releases/tag/v0.2.0
