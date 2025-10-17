# Changelog

All notable changes to Unicel will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[0.4.1]: https://github.com/jacksodj/unicel/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/jacksodj/unicel/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/jacksodj/unicel/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/jacksodj/unicel/releases/tag/v0.2.0
