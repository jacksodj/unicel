# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Unicel is a unit-aware spreadsheet application that treats units as first-class data types. Values are stored as `(number, unit)` tuples enabling dimensional analysis, automatic unit conversion, and type-safe calculations. Built with Rust and Tauri for native performance with AI integration via MCP (Model Context Protocol).

**Core Innovation:** Units are data, not formatting. Operations like `$100/hr × 720hr/month → $72,000/month` work naturally with automatic unit cancellation.

## Technology Stack

**Backend (Rust):**
- Tauri 2.x for native desktop application
- SQLite (in-memory) for runtime performance
- Custom unit system with dimensional analysis
- pest parser for formula parsing
- serde for JSON serialization

**Frontend:**
- React 18+ with TypeScript
- Vite for build tooling
- Tailwind CSS + shadcn/ui for UI components
- Zustand for state management
- Custom canvas-based grid renderer for performance

**MCP Integration:**
- Internal MCP server exposes spreadsheet operations to AI
- External MCP clients for currency/stock data

## Architecture

### Core Components

**1. Unit System (`src/core/units/`)**
- Unit representation with canonical forms and dimensional analysis
- Conversion graph with pathfinding for multi-hop conversions
- Built-in library covering: length, mass, time, temperature, currency, digital storage, energy, power
- Custom unit definitions embedded in workbooks

**2. Cell & Formula Engine (`src/core/cell/`, `src/core/formula/`)**
- Cells store `(value, unit, formula)` tuples
- Formula parser builds AST with unit annotations
- Unit-aware operations: addition (compatible units), multiplication/division (compound units), automatic cancellation
- Dependency tracking for incremental recalculation
- Circular reference detection

**3. Conversion System (`src/core/conversion/`)**
- **Storage vs Display separation**: Storage unit is immutable (what user entered), display unit is non-destructive (toggle Metric ↔ Imperial)
- Four conversion rate modes:
  - Live Auto: continuous updates from MCP
  - Prompt on Open: user chooses rates when opening
  - As of Date: historical rates locked to specific date
  - Manual: user-provided assumptions
- Conversion chain trust system for multi-hop paths

**4. Table System (`src/core/table/`)**
- Tables represent structured entities with metadata
- Entity-aware operations: `COUNT(*)` inherits row_unit from table (e.g., "45 instances")
- SQL queries with unit-aware filtering and comparisons
- Column validation: required fields, enum values (manual or SQL-driven)

**5. Workbook & File Format (`src/core/workbook/`, `src/formats/`)**
- Runtime: In-memory SQLite for fast queries
- Storage: Pure JSON (MVP) - LLM-friendly, human-readable
- File extension: `.usheet` or `.usheet.json`
- Excel export (one-way) with metadata sheets

**6. MCP Integration (`src/mcp/`)**
- Internal server exposes: read_cell, write_cell, query_table
- External clients: currency rates (ECB), stock prices
- Custom MCP servers for enterprise data

### Data Flow

**On Cell Edit:**
1. Parse user input (value + unit) with autocomplete/disambiguation
2. Store in SQLite
3. Identify dependent cells via dependency graph
4. Recalculate affected formulas
5. Update display (apply conversion if needed)

**On Display Toggle (Metric ↔ Imperial):**
1. Update sheet display preference
2. Apply conversion via graph (storage unchanged)
3. No recalculation needed (formulas use storage units)

**On Save:**
1. Query all cells from SQLite
2. Reconstruct JSON structure
3. Write to disk

## Commands

### Build and Run

```bash
# Install dependencies (first time)
npm install

# Run development server with hot reload
npm run tauri dev

# Build for production
npm run tauri build

# Run only frontend (without Tauri)
npm run dev
```

### Testing

```bash
# Run all Rust tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test --test unit_conversions

# Run property-based tests
cargo test --test prop_tests

# Run frontend tests
npm run test

# Run tests in watch mode
npm run test:watch

# Run integration tests (requires golden workbook)
cargo test --test integration
```

### Linting and Formatting

```bash
# Format Rust code
cargo fmt

# Lint Rust code
cargo clippy -- -D warnings

# Format TypeScript/React
npm run format

# Lint TypeScript/React
npm run lint
```

### Development

```bash
# Run single Rust test file
cargo test --test <test_name>

# Check Rust code without building
cargo check

# Build only Rust backend
cargo build

# Clean build artifacts
cargo clean
npm run clean
```

## Key Design Principles

### 1. Units Are Immutable (Unless Explicitly Converted)
- **Storage unit**: What the cell contains (never changes unless CONVERT() used)
- **Display unit**: How it's shown (non-destructive, toggleable)
- Formula: `=A1*2` where A1=100ft calculates as 200ft, displays as 60.96m in Metric mode

### 2. Dimensional Analysis First
- Operations check dimension compatibility
- Incompatible units (5m + 10s) produce warnings, not errors
- Result becomes dimensionless with orange warning indicator

### 3. Automatic Unit Cancellation
- `100mi / 2hr → 50 mi/hr` (compound unit)
- `100m / 50m → 2` (dimensionless, units cancel)
- Visual feedback shows cancellation occurred

### 4. Soft Warnings, Not Errors
- Orange cells with ⚠️ icon
- Tooltips explain issue
- Calculation proceeds (fail-soft behavior)

### 5. Context-Aware Operations
- `COUNT(*)` in EC2Instances table returns "45 instances" (inherits row_unit)
- Column default_unit applied to bare numbers
- Display_as conversion per column

## Important Files

**Configuration:**
- `src-tauri/Cargo.toml` - Rust dependencies and workspace config
- `src-tauri/tauri.conf.json` - Tauri application configuration
- `package.json` - Frontend dependencies and scripts
- `tsconfig.json` - TypeScript configuration
- `tailwind.config.js` - Tailwind CSS configuration

**Core Engine:**
- `src/core/units/unit.rs` - Unit representation and dimensional analysis
- `src/core/units/conversion_graph.rs` - Conversion pathfinding
- `src/core/units/library.rs` - Built-in unit definitions
- `src/core/formula/parser.rs` - Formula parsing with pest
- `src/core/formula/evaluator.rs` - Unit-aware evaluation
- `src/core/cell/cell.rs` - Cell data structure
- `src/core/workbook/workbook.rs` - Workbook management

**Testing:**
- `tests/unit/` - Unit tests for core functionality
- `tests/integration/` - Integration tests
- `tests/golden_workbook/` - Golden workbook validation
- `examples/cloud-cost-analysis.usheet` - Golden workbook (comprehensive test)

## Common Patterns

### Adding a New Unit

1. Add to appropriate domain in `src/core/units/library.rs`
2. Define conversion factors to related units
3. Add dimension mapping
4. Add tests in `tests/unit/conversions.rs`

### Adding a New Formula Function

1. Implement in `src/core/formula/functions.rs`
2. Register in formula parser
3. Add unit-aware logic (check dimensions, preserve units)
4. Add tests in `tests/unit/formulas.rs`

### Adding a New Conversion Source

1. Implement MCP client in `src/mcp/client/`
2. Add configuration in `ConversionConfig`
3. Update conversion graph to use new source
4. Add integration test

## Testing Strategy

**Golden Workbook** (`examples/cloud-cost-analysis.usheet`) exercises all features:
- Sheet 1: EC2 Instances (main data table)
- Sheet 2: Cost Projections (formulas with unit cancellation)
- Sheet 3: Multi-Region Analysis (display toggle)
- Sheet 4: Temperature Monitoring (warnings)
- Sheet 5: Custom Units (business domain)
- Sheet 6: Validation Examples
- Sheet 7: Query Examples (SQL)

Run full validation: `cargo test --test golden_workbook`

## Performance Targets

- 10,000 cells load: <1 second
- Formula recalculation (1,000 cells): <200ms
- Display toggle: <100ms
- SQL queries (10,000 rows): <500ms
- Cell edit to display: <16ms (60 FPS)

## Code Style

**Rust:**
- Use rustfmt defaults
- Enable all clippy warnings
- Prefer `Result<T>` over panics
- Document all public APIs with `///` doc comments
- Use `anyhow` for error handling in application code, custom errors for library code

**TypeScript:**
- Use Prettier defaults
- Prefer functional components with hooks
- Use Zustand for shared state, local state for component-specific
- Prefer `const` over `let`
- Use descriptive variable names (no single letters except loop indices)

## File Format Specification

`.usheet` files are JSON with this structure:
```json
{
  "version": "1.0",
  "workbook_settings": { "unit_preference": "Metric", ... },
  "sheets": [{
    "name": "Sheet1",
    "cells": {
      "A1": { "value": 100.0, "unit": "USD", "formula": null }
    },
    "columns": [...],
    "tables": [...]
  }],
  "conversions": { "history": [...], "manual_overrides": [...] },
  "custom_units": [...]
}
```

## Implementation Phases

Current phase: **Phase 6 - Tauri Integration** (Week 19 of 24)

Project follows 24-week timeline outlined in PROJECT_PLAN.md:
- Phase 0: Foundation (Weeks 1-2) ✅
- Phase 1: Core Unit System (Weeks 3-5) ✅
- Phase 2: Cell & Formula Engine (Weeks 6-9) ✅
- Phase 3: Basic Workbook & Sheet (Weeks 10-12) ✅
- Phase 4: File Format (Weeks 13-14) ✅
- Phase 5: Basic UI (Weeks 15-18) ✅
- Phase 6: Tauri Integration (Weeks 19-20) 🚧 IN PROGRESS
- Phase 7: Testing & Examples (Weeks 21-22)
- Phase 8: Excel Export & Polish (Week 23)
- Phase 9: MLP Release (Week 24)

See `docs/PROJECT_PLAN.md` for full implementation roadmap.

## Task Tracking

**IMPORTANT:** Use `docs/TASKS.md` as the single source of truth for all task tracking.

**Guidelines:**
- ✅ **DO:** Update tasks in `docs/TASKS.md` as you complete them (change `[ ]` to `[x]`)
- ✅ **DO:** Add future feature ideas to the "Future Backlog" section in `docs/TASKS.md`
- ❌ **DON'T:** Create TODO files in the project root
- ❌ **DON'T:** Create separate task tracking files anywhere else in the repository

The TodoWrite tool should only be used for temporary session-level task tracking during active development. All permanent task tracking belongs in `docs/TASKS.md`.
