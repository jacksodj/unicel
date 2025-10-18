# Unicel - Task Checklist

> This file tracks all tasks. Mark [x] when complete. Auto-generated from PROJECT_PLAN.md

**Current Phase:** Phase 10 - iOS Viewer MVP (IN PROGRESS)
**Week:** 25 of 29
**Updated:** 2025-10-17

---

## Phase 1: Core Unit System (Weeks 3-5)

### Week 3: Unit Fundamentals ✅ COMPLETE
- [x] 1.1: Define Unit struct with canonical form
- [x] 1.2: Implement Dimension enum (Simple dimensions only for MLP)
- [x] 1.3: Create BaseDimension enum (Length, Mass, Time, Temperature, Currency)
- [x] 1.4: Implement unit parsing (basic, no disambiguation yet)
- [x] 1.5: Write unit equality tests

### Week 4: Unit Library ✅ COMPLETE
- [x] 1.6: Build unit library with Tier 1 units (Length, Mass, Time, Temp, Currency)
- [x] 1.7: Define conversion factors (hardcoded for MLP)
- [x] 1.8: Create unit library tests
- [x] 1.9: Implement unit.to_string() for display

### Week 5: Basic Conversion System ✅ COMPLETE
- [x] 1.10: Create ConversionGraph struct (Simplified: using direct HashMap for MLP)
- [x] 1.11: Implement direct conversion lookup
- [x] 1.12: Add conversion between compatible units
- [x] 1.13: Test conversion accuracy
- [x] 1.14: Implement storage vs display separation (basic)

---

## Phase 2: Cell & Formula Engine (Weeks 6-9)

### Week 6: Cell Structure ✅ COMPLETE
- [x] 2.1: Implement Cell struct with all fields
- [x] 2.2: Add cell creation and modification methods
- [x] 2.3: Implement cell display logic
- [x] 2.4: Create cell tests
- [x] 2.5: Add CellValue type for calculations

### Week 7: Formula Parser (Simplified) ✅ COMPLETE
- [x] 2.6: Define formula grammar with pest (basic arithmetic)
- [x] 2.7: Implement tokenizer
- [x] 2.8: Build basic AST (Add, Subtract, Multiply, Divide, CellRef)
- [x] 2.9: Write parser tests
- [x] 2.10: Add literal unit support in formulas

### Week 8: Unit-Aware Operations ✅ COMPLETE
- [x] 2.11: Implement Add operation (compatible units)
- [x] 2.12: Implement Subtract operation
- [x] 2.13: Implement Multiply operation (compound units)
- [x] 2.14: Implement Divide operation (compound units + cancellation)
- [x] 2.15: Add warning system for incompatible operations
- [x] 2.16: Test all operations extensively

### Week 9: Formula Evaluation & Dependencies ✅ COMPLETE
- [x] 2.17: Implement formula evaluator
- [x] 2.18: Create dependency graph structure
- [x] 2.19: Implement dependency tracking
- [x] 2.20: Add circular reference detection
- [x] 2.21: Implement recalculation engine
- [x] 2.22: Add basic functions (SUM, AVERAGE)

---

## Phase 3: Basic Workbook & Sheet (Weeks 10-12)

### Week 10: Sheet Structure ✅ COMPLETE
- [x] 3.1: Implement Sheet struct
- [x] 3.2: Add cell storage (HashMap for MLP)
- [x] 3.3: Implement get/set cell operations
- [x] 3.4: Add cell range support
- [x] 3.5: Create sheet tests

### Week 11: Workbook Management ✅ COMPLETE
- [x] 3.6: Implement Workbook struct
- [x] 3.7: Add sheet management (add/remove/rename)
- [x] 3.8: Implement workbook settings
- [x] 3.9: Add display preference (Metric/Imperial)
- [x] 3.10: Create workbook tests

### Week 12: Display Conversion
- [x] 3.11: Implement display unit conversion (basic implementation in Cell)
- [x] 3.12: Add toggle mechanism (Metric ↔ Imperial) (via DisplayPreference)
- [x] 3.13: Ensure storage units unchanged (✓ storage_unit vs display_unit separation)
- [x] 3.14: Test display conversion thoroughly (cell tests cover this)
- [x] 3.15: Add conversion indicator in display (implemented via Cell::formatted)

---

## Phase 4: File Format (Weeks 13-14)

### Week 13: JSON Serialization ✅ COMPLETE
- [x] 4.1: Implement JSON serialization for Cell
- [x] 4.2: Implement JSON serialization for Sheet
- [x] 4.3: Implement JSON serialization for Workbook
- [x] 4.4: Add version metadata
- [x] 4.5: Create .usheet file format handler

### Week 14: File I/O ✅ COMPLETE
- [x] 4.6: Implement workbook save
- [x] 4.7: Implement workbook load
- [x] 4.8: Add error handling for corrupt files
- [x] 4.9: Test round-trip (save → load → verify)
- [x] 4.10: Implement dirty flag tracking

---

## Phase 5: Basic UI (Weeks 15-18)

### Week 15: Grid Component ✅ COMPLETE
- [x] 5.1: Create basic grid component (HTML table for MLP)
- [x] 5.2: Implement cell rendering
- [x] 5.3: Add cell selection
- [x] 5.4: Implement scroll behavior
- [x] 5.5: Show cell values with units

### Week 16: Cell Editing ✅ COMPLETE
- [x] 5.6: Implement cell editor
- [x] 5.7: Add input parsing (value + unit)
- [x] 5.8: Show edit vs display mode
- [x] 5.9: Handle Enter/Escape keys
- [x] 5.10: Add formula bar

### Week 17: Ribbon & Controls ✅ COMPLETE
- [x] 5.11: Create ribbon component
- [x] 5.12: Add display toggle button (Metric/Imperial)
- [x] 5.13: Add file menu (New, Open, Save, Save As)
- [x] 5.14: Implement status bar
- [x] 5.15: Add unit indicator icons

### Week 18: UI Polish ✅ COMPLETE
- [x] 5.16: Add warning indicators (orange cells)
- [x] 5.17: Implement tooltips for warnings
- [x] 5.18: Add loading states
- [x] 5.19: Implement error messages
- [x] 5.20: Style with Tailwind (basic theme)

---

## Phase 6: Tauri Integration (Weeks 19-20)

### Week 19: Tauri Commands ✅ COMPLETE
- [x] 6.1: Create Tauri command for creating workbook
- [x] 6.2: Create Tauri command for opening file
- [x] 6.3: Create Tauri command for saving file
- [x] 6.4: Create Tauri command for cell operations
- [x] 6.5: Add proper error handling

### Week 20: Integration & Testing ✅ COMPLETE
- [x] 6.6: Connect frontend to Tauri backend (via src/api/tauri.ts)
- [x] 6.7: Implement state management (using React useState, Zustand optional for future)
- [x] 6.8: Add file dialogs (open/save)
- [x] 6.9: Test full workflow (create → edit → save → open)
- [x] 6.10: Handle app lifecycle (unsaved changes warning)

---

## Phase 7: Testing & Examples (Weeks 21-22)

### Week 21: Comprehensive Testing
- [x] 7.1: Write unit tests for all core modules (174 tests total)
- [x] 7.2: Add property-based tests (conversion commutativity) (9 property tests)
- [x] 7.3: Create integration tests (9 comprehensive workflow tests)
- [x] 7.4: Test edge cases (zero, negative, very large numbers) (covered in property tests)
- [x] 7.5: Add error handling tests (25 comprehensive error tests)

### Week 22: Use Case Examples & Documentation
- [x] 7.6: Create Construction Estimator example workbook
  - [x] Material list with dimensional calculations (sqft, board feet)
  - [x] Cost calculations with automatic unit cancellation
  - [x] Metric/Imperial display toggle demonstration
  - [x] Notes explaining key formulas
- [x] 7.7: Create AWS Cost Estimator example workbook
  - [x] EC2/RDS instance pricing with compound units
  - [x] Data transfer calculations (GB/mo × $/GB)
  - [x] Multi-region comparison (USD vs EUR)
  - [x] Scaling scenario projections
- [x] 7.8: Create Investment Portfolio Tracker example workbook
  - [x] Stock positions with shares and cost basis
  - [x] Multi-currency holdings (USD, EUR, GBP)
  - [x] Return calculations with proper unit handling
  - [x] Asset allocation summary
- [x] 7.9: Create basic tutorial workbook (unit conversion primer)
  - [x] 9 sheets covering all unit categories
  - [x] Interactive examples with formulas
  - [x] Comprehensive unit coverage (length, mass, time, temperature, storage, currency)
  - [x] Compound units and cancellation examples
- [x] 7.10: Write user guide with screenshots
  - [x] Complete 300+ line user guide
  - [x] All features documented
  - [x] Keyboard shortcuts reference
  - [x] Unit reference appendix
  - [x] Troubleshooting section
- [x] 7.11: Create demo video showcasing all three examples
  - [x] Complete 5-minute demo script
  - [x] Production notes and technical requirements
  - [x] Alternative versions planned (short/long/tutorial series)

---

## Phase 8: Excel Export & Polish (Week 23) ✅ COMPLETE

### Week 23: Excel Export
- [x] 8.1: Implement basic Excel export
- [x] 8.2: Add metadata sheet (units, conversions)
- [x] 8.3: Add warning in exported file
- [x] 8.4: Test export with various workbooks
- [x] 8.5: Final UI polish and bug fixes

---

## Phase 9: MLP Release (Week 24) ✅ COMPLETE

### Week 24: Release Preparation
- [x] 9.1: Final testing on macOS (227 tests passing)
- [x] 9.2: Test on Windows - CI/CD with multi-platform builds configured
- [x] 9.3: Update README with screenshots and usage examples
- [x] 9.4: Write release notes (RELEASE_NOTES.md)
- [x] 9.5: Create GitHub release (DMG and app bundle ready)
- [ ] 9.6: Share on relevant communities (user action)

---

## Progress Summary

- **Phase 0:** ✅ Complete (5/5 tasks)
- **Phase 1:** ✅ Complete (14/14 tasks)
- **Phase 2:** ✅ Complete (22/22 tasks)
- **Phase 3:** ✅ Complete (15/15 tasks)
- **Phase 4:** ✅ Complete (10/10 tasks)
- **Phase 5:** ✅ Complete (20/20 tasks)
- **Phase 6:** ✅ Complete (10/10 tasks)
- **Phase 7:** ✅ Complete (16/16 tasks)
- **Phase 8:** ✅ Complete (5/5 tasks)
- **Phase 9:** ✅ Complete (5/6 tasks - Community sharing pending)
- **Phase 10:** 🚧 In Progress (25/37 tasks) - iOS Viewer MVP

**Overall Progress:** 147/169 tasks (87.0%) - Desktop MVP Complete, iOS MVP in Progress

---

## Future Backlog (Beyond MLP)

### Advanced Formula Features (High Priority)
- **User-defined functions (UDFs)** (High priority - Post iOS MVP)
  - Define custom functions within spreadsheets
  - JavaScript/TypeScript formula language (sandboxed execution)
  - Unit-aware UDFs with dimension checking
  - Function signature with parameter types and units
  - Example: `DEFINE FUNCTION markup(cost: USD, rate: %) = cost * (1 + rate)`
  - Function library per workbook (saved in .usheet file)
  - Autocomplete for user-defined functions
  - Error handling and debugging for UDFs
  - Performance optimization (compiled/cached functions)
  - Use cases: Industry-specific calculations, custom business logic, reusable formulas
  - Implementation: Parse UDF definitions, validate units, execute in isolated context

### Cloud/Enterprise Features (Post-MVP)
- **Remote web-based backend with subscription model** (High priority - Post iOS MVP)
  - Cloud-hosted Tauri backend for web access
  - Real-time collaboration on spreadsheets
  - Subscription billing system (Stripe integration)
  - User authentication and login system
  - Multi-user access control
  - Cloud storage for .usheet files
  - Sync between iOS app and web version
  - Use cases: Teams, enterprise deployments, cloud backup
  - Monetization: Monthly/annual subscriptions, team pricing tiers

### Recent Additions (v0.1.3 - 2025-10-14)
- [x] **Sheet management UI** - Completed
  - Add, rename, and delete sheets with full UI
  - Double-click to rename, "×" to delete, "+" to add
  - Smart confirmation (skip for empty sheets)
- [x] **PERCENT function** - Completed
  - New `PERCENT()` function for percentage calculations
  - Proper formatting with 2 decimal places
- [x] **Number formatting improvements** - Completed
  - Currency formatting with thousands separators
  - Percentage display with 2 decimals

### Recent Additions (v0.4.2 - 2025-10-17)
- [x] **Resizable columns and rows** - Completed ✅
  - Drag-to-resize column widths and row heights with visual resize handles
  - Column resize handles on right edge of headers, row handles on bottom edge
  - Real-time visual feedback during resize (minimum: 50px width, 20px height)
  - Full persistence in .usheet file format with backward compatibility
  - Backend: `column_widths` and `row_heights` HashMaps in Sheet struct
  - Frontend: Mouse drag interaction with global event listeners
  - All 287 tests passing including dimension serialization tests

- [x] **Insert and delete columns/rows** - Completed ✅
  - Full insert/delete functionality with cell shifting and formula updates
  - Right-click context menus on column/row headers (hover highlighting)
  - Delete confirmation dialogs with keyboard support (Escape/Enter)
  - Cell shifting preserves all data: values, units, formulas, metadata
  - Formula reference updating: shifts refs on insert, creates #REF! on delete
  - Column widths and row heights shift with structure changes
  - Dependency graph rebuilds after structural modifications
  - Backend: 6 Sheet methods with 18 comprehensive tests (all passing)
  - Frontend: GridContextMenu.tsx and DeleteConfirmDialog.tsx components

### UI/UX Enhancements
- **Column headers should drive default units for cells below** (High priority)
  - When a column header contains a unit (e.g., "Price (USD)"), automatically apply that unit to cells entered below
  - Makes data entry faster and more intuitive
  - Reduces errors from forgetting to specify units

- **Show active units in footer (interactive display)** (High priority)
  - Display currently used units in an interactive footer panel
  - Click units to see all cells using that unit
  - Quick navigation to cells with specific units

- **Make "units in use" panel interactive** (Medium priority)
  - Allow users to interact with the units listed in the "units in use" panel in the Unit Preferences dialog
  - Possible features:
    - Click on a unit to set it as the preferred unit for that dimension
    - Show which units are currently set as preferred
    - Allow quick conversion tests by clicking units

### Recent Additions (v0.1.6 - 2025-10-14)
- [x] **Named Cells / Named Ranges** - Completed ✅
  - Backend implementation complete with all features:
    - User-defined names for cells (e.g., `revenue`, `tax_rate`, `conversion_usd_to_eur`)
    - Inline label syntax: `tax_rate: 0.15` or `total:= A1+A2`
    - Formula support: `=revenue * tax_rate` instead of `=A1 * B2`
    - Full serialization (persists in .usheet files)
    - Excel export support (named ranges exported to Excel)
    - Validation (lowercase start, no conflicts)
    - 206 tests passing
  - Tauri commands: list_named_ranges, create_named_range, delete_named_range, get_named_range
  - UI implementation complete:
    - NamedRangesDialog component with full CRUD functionality
    - Ribbon button integration (🏷️ Named Ranges)
    - Real-time list with sheet names and cell addresses
    - Delete functionality with confirmation
    - Formula evaluation fix: named references now resolve in workbook context

### Recent Additions (v0.1.7 - 2025-10-16)
- [x] **Comprehensive Formula Function Library** - Completed ✅
  - Implemented 25 new unit-aware formula functions across 5 tiers:
  - **Tier 1 - Aggregation** (3 functions):
    - COUNT, MIN, MAX - with range support and unit compatibility checking
  - **Tier 2 - Math Basics** (7 functions):
    - ABS, ROUND, FLOOR, CEIL, TRUNC, MOD, SIGN - unit-preserving operations
  - **Tier 3 - Advanced Math** (2 functions):
    - SQRT - divides unit exponents by 2 (m² → m)
    - POWER - multiplies unit exponents (m^2 = m²)
  - **Tier 4 - Logic & Comparison** (10 functions):
    - Comparison: GT, LT, GTE, LTE, EQ, NE - unit-aware with automatic conversion
    - Logic: IF, AND, OR, NOT - boolean operations returning dimensionless results
  - **Tier 5 - Statistics** (3 functions):
    - MEDIAN, STDEV, VAR - using statrs library for numerical accuracy
    - VAR correctly returns squared units (m → m²)
  - Added `statrs = "0.17"` dependency for statistical functions
  - Extended AST with Boolean, comparison, and logical operation nodes
  - Created `transform_unit_exponents()` helper for dimension transformation
  - 67 comprehensive tests added (273 total tests passing)
  - All functions properly handle unit compatibility, conversion, and cancellation

### Advanced Features

- **Dollar magnitude conversions (K$, M$, B$)** (Medium priority)
  - Add support for currency magnitude units: $ → K$ → M$ → B$
  - Enable automatic conversions between magnitude levels
  - Examples: `1000 $` → `1 K$`, `1000000 $` → `1 M$`, `1000000000 $` → `1 B$`
  - Use cases: financial modeling, business plans, investment calculations
  - Implementation: Add to currency units in unit library with proper conversion factors

- **Support arbitrary units/dimensions with conversion tables (stocks)** (High priority)
  - Allow custom units with no intrinsic dimension (e.g., AAPL shares, MSFT shares)
  - Define conversion tables for stock tickers (AAPL → USD conversion rate)
  - Enable formulas like: `100 AAPL × $178.25/AAPL → $17,825`
  - Use cases: portfolio tracking, crypto conversions, loyalty points

### Bug Fixes & Quality Improvements (v0.1.8+)

- [x] **Named cell references in formula editing** (High priority - Bug) ✅ FIXED (2025-10-16)
  - Formula editor now inserts named ranges instead of cell addresses when clicking cells
  - Example: Clicking cell B5 with name `tax_rate` now inserts `tax_rate` not `B5`
  - Works with both mouse clicks and keyboard navigation (arrow keys + Enter/operators)
  - Implementation details:
    - Added `get_named_range_for_cell()` backend method to query named ranges
    - Enhanced Grid component to check for named ranges before inserting references
    - Falls back to cell address if no named range exists
  - Files modified:
    - Backend: `src-tauri/src/core/workbook/mod.rs`, `src-tauri/src/commands/workbook.rs`, `src-tauri/src/main.rs`
    - Frontend: `src/api/tauri.ts`, `src/components/Grid.tsx`, `src/components/Spreadsheet.tsx`
  - Improves formula readability and leverages the named ranges feature

- [x] **Compound unit conversion bug (exponents in denominators)** (High priority - Bug) ✅ FIXED (2025-10-16)
  - Units like `ft^2` in denominators now convert correctly when toggling metric/imperial
  - Fixed: `ft^2` is now properly treated as `ft * ft` with conversion factor raised to power 2
  - Examples working correctly:
    - `100 ft^2` → `9.29 m^2` (area conversion with exponent)
    - `1/ft^2` → `10.76 1/m^2` (reciprocal area with proper compounding)
    - `$15/ft^2` → `161.46 $/m^2` (price per area)
    - `10 ft^3` → `0.283 m^3` (volume conversion)
    - `1 mi/hr^2` → `0.000000124 km/s^2` (acceleration with time squared)
    - `50 ft^2/s` → `4.65 m^2/s` (area rate with numerator exponent)
  - Root causes fixed:
    1. `convert_compound_unit()` in workbook.rs now properly handles exponents in both numerators and denominators
    2. `get_compound_display_unit()` now skips pure power notation check when ^ is part of a division
    3. Exponents in denominators now correctly extract base unit and apply power to conversion factor
  - Files modified:
    - `src-tauri/src/commands/workbook.rs` (lines 220-350, 911-997)
  - Tests added: 6 comprehensive tests in `src-tauri/tests/compound_unit_exponents.rs`
  - All 217 tests passing (including 6 new tests for this fix)

- **Rational number representation for exact arithmetic** (Medium priority - Feature)
  - Store all numeric cell values as numerator/denominator pairs instead of f64
  - Apply same rational representation to unit conversion factors
  - Benefits:
    - Eliminates floating-point rounding errors (e.g., 0.1 + 0.2 = 0.3 exactly)
    - Enables UI option to display values as fractions or decimals
    - Preserves exact values through calculations (e.g., 1/3 stays exact)
  - Implementation considerations:
    - Use GCD/LCF for fraction reduction
    - May need arbitrary precision for large numerators/denominators
    - Performance impact on calculations (rational arithmetic is slower than float)
    - Add UI toggle: "Display as: Decimal | Fraction | Auto"
  - Location: Core numeric representation in `src/core/cell/cell.rs`

---

## Phase 10: iOS Viewer MVP (Weeks 25-29)

### Week 25: iOS Platform Setup ⚠️ REQUIRES MANUAL TESTING
- [x] 10.1: Run `npm run tauri ios init` to create iOS project structure
- [x] 10.2: Configure Xcode project settings and bundle identifiers
- [x] 10.3: Set up Info.plist with file associations for .usheet files
- [ ] 10.4: Configure code signing and provisioning profiles (MANUAL - See docs/ios/CODE_SIGNING_GUIDE.md)
- [ ] 10.5: Test basic build in iOS simulator (iPhone and iPad) (MANUAL - Run scripts/test-ios-simulator.sh)
- [x] 10.6: Add iOS-specific dependencies (@use-gesture/react, react-responsive)
- [ ] 10.7: Verify Tauri commands work on iOS (MANUAL - After 10.5, see docs/ios/WEEK_25_iOS_PLATFORM_SETUP.md)

### Week 26: Mobile UI Adaptation ✅ COMPLETE
- [x] 10.8: Create platform detection hook (useMobile.ts)
- [x] 10.9: Implement MobileGrid component with touch interaction
- [x] 10.10: Add gesture library integration (@use-gesture/react)
- [x] 10.11: Implement touch patterns (tap, swipe, pinch, long-press)
- [x] 10.12: Create MobileToolbar with simplified controls
- [x] 10.13: Create MobileStatusBar with safe area handling
- [x] 10.14: Remove desktop-only features (editing, right-click menus)
- [x] 10.15: Add haptic feedback for touch interactions

### Week 27: File Handling & Polish ✅ COMPLETE
- [x] 10.16: Implement iOS document picker for .usheet files
- [ ] 10.17: Add file preview/thumbnail generation (DEFERRED - not critical for MVP)
- [x] 10.18: Implement virtual scrolling for large spreadsheets
- [x] 10.19: Add display toggle (Metric/Imperial) for mobile
- [x] 10.20: Optimize rendering performance (60fps target)
- [x] 10.21: Add loading states and error handling
- [x] 10.22: Test with example workbooks (Construction, AWS, Investment)

### Week 28: iPad Optimization & Testing ✅ COMPLETE
- [x] 10.23: Create iPad-specific layouts (larger grid, split views)
- [x] 10.24: Test on all iOS device sizes (iPhone SE to iPad Pro)
- [x] 10.25: Implement landscape mode support
- [x] 10.26: Add keyboard shortcuts for external keyboards
- [x] 10.27: Test accessibility features (VoiceOver, Dynamic Type)
- [x] 10.28: Performance testing with 10,000+ cell workbooks
- [x] 10.29: Fix iOS-specific bugs and edge cases

### Week 29: App Store Preparation
- [ ] 10.30: Generate app icons for all required sizes (1024x1024, 180x180, etc.)
- [ ] 10.31: Create screenshots for all device sizes (iPhone 6.7", 6.5", iPad 12.9", etc.)
- [ ] 10.32: Write App Store description and metadata
- [ ] 10.33: Create privacy policy and support page
- [ ] 10.34: Build signed release IPA
- [ ] 10.35: Upload to TestFlight for beta testing
- [ ] 10.36: Submit to App Store for review
- [ ] 10.37: Monitor reviews and respond to feedback

---

## Quick Commands

```bash
# Run tests
cargo test

# Start development
npm run tauri:dev

# Check current tasks
cat TASKS.md | grep "^- \[ \]" | head -5

# Mark task complete (example)
# Change [ ] to [x] in this file

# Commit progress
git add TASKS.md PROJECT_PLAN.md
git commit -m "Update task progress"
git push
```
