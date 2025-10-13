# Unit-Aware Spreadsheet - Unified Design Document

## Executive Summary

A next-generation open-source spreadsheet application that treats units as first-class data types, enabling dimensional analysis, automatic unit conversion, and type-safe calculations. Built with Rust and Tauri for native performance, with full AI integration via MCP (Model Context Protocol).

**Core Innovation:** Units are data, not formatting. Values stored as `(number, unit)` tuples enable operations like `$100/hr × 720hr/month → $72,000/month` to work naturally with automatic unit cancellation.

**Target Users:**
- Financial analysts working with multi-currency data
- Engineers managing mixed imperial/metric measurements
- Data scientists standardizing heterogeneous datasets
- Cloud infrastructure teams analyzing costs and resources
- Any user collaborating internationally across measurement systems

**Key Differentiators:**
- Units preserved through all operations (copy/paste, formulas, export)
- Non-destructive display conversion (Metric ↔ Imperial toggle)
- SQL-queryable tables with entity-aware operations
- AI-native via MCP protocol
- Open source and LLM-friendly file format

---

## Table of Contents

1. [Architecture Overview](#1-architecture-overview)
2. [Data Model](#2-data-model)
3. [Unit System](#3-unit-system)
4. [Conversion System](#4-conversion-system)
5. [Formula Engine](#5-formula-engine)
6. [Table System](#6-table-system)
7. [User Interface](#7-user-interface)
8. [MCP Integration](#8-mcp-integration)
9. [File Format](#9-file-format)
10. [Testing Strategy](#10-testing-strategy)
11. [MVP Scope](#11-mvp-scope)
12. [Implementation Phases](#12-implementation-phases)
13. [Performance Requirements](#13-performance-requirements)
14. [Open Source Strategy](#14-open-source-strategy)

---

## 1. Architecture Overview

### 1.1 Technology Stack

**Frontend:**
- **Framework:** Tauri (Rust-native, lightweight)
- **UI Library:** TBD (React/Svelte/Yew)
- **Rationale:** Native performance, small bundle size, Rust integration

**Backend (Calculation Engine):**
- **Language:** Rust
- **Database:** SQLite (in-memory for runtime)
- **Rationale:** Performance-critical calculations, memory safety, WASM potential

**MCP Server:**
- **Language:** Rust
- **Protocol:** MCP (Model Context Protocol)
- **Rationale:** AI-native integration, standard protocol

**File Format:**
- **MVP:** Pure JSON (LLM-friendly, human-readable)
- **Phase 2:** SQLite hybrid (performance optimization)

### 1.2 System Architecture

```
┌─────────────────────────────────────────────────────────┐
│                  Tauri Frontend                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐    │
│  │ Ribbon UI   │  │ Cell Editor │  │ Query Panel │    │
│  └─────────────┘  └─────────────┘  └─────────────┘    │
└────────────────────────┬────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────┐
│              Rust Calculation Engine                     │
│  ┌──────────────────────────────────────────────────┐  │
│  │ In-Memory SQLite (Runtime Performance)           │  │
│  │  - cells table (value, unit, formula)            │  │
│  │  - dependencies table (recalc graph)             │  │
│  │  - Fast SQL queries for table operations         │  │
│  └──────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────┐  │
│  │ Unit Type System                                 │  │
│  │  - Canonical unit representation                 │  │
│  │  - Dimensional analysis                          │  │
│  │  - Conversion graph pathfinding                  │  │
│  └──────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────┐  │
│  │ Formula Parser & Evaluator                       │  │
│  │  - Unit-aware operations                         │  │
│  │  - Automatic unit cancellation                   │  │
│  │  - Dependency tracking                           │  │
│  └──────────────────────────────────────────────────┘  │
└────────────────────────┬────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────┐
│                  MCP Server (Rust)                       │
│  - Exposes spreadsheet via MCP protocol                 │
│  - AI tools can read/write/query                        │
│  - Unit-aware operations for AI agents                  │
└────────────┬──────────────────────────┬─────────────────┘
             │                          │
             ▼                          ▼
┌──────────────────────┐    ┌──────────────────────┐
│ External MCP Servers │    │ Custom MCP Servers   │
│ - Currency (ECB)     │    │ - User-defined       │
│ - Stocks (Yahoo)     │    │ - Enterprise data    │
└──────────────────────┘    └──────────────────────┘
```

### 1.3 Runtime Data Flow

**On Workbook Open:**
```
1. Load JSON file from disk
2. Create in-memory SQLite database
3. Initialize schema (cells, dependencies, formats)
4. Populate SQLite from JSON
5. Build conversion graph
6. Ready for operations
```

**On Cell Edit:**
```
1. Parse user input (value + unit)
2. Disambiguate if needed (autocomplete)
3. Store in SQLite (UPDATE/INSERT)
4. Identify dependent cells (query dependency graph)
5. Recalculate affected formulas
6. Update display (apply conversion if needed)
7. Mark workbook as dirty
```

**On Save:**
```
1. Query all cells from SQLite
2. Reconstruct JSON structure
3. Write to disk
4. Clear dirty flag
```

**On Display Toggle (Metric ↔ Imperial):**
```
1. Update sheet display preference
2. For each visible cell:
   - Keep stored unit unchanged
   - Apply conversion via graph
   - Update display only
3. No recalculation needed (formulas use storage units)
```

---

## 2. Data Model

### 2.1 Cell Structure

Each cell stores:

```rust
pub struct Cell {
    /// Numeric value
    value: f64,
    
    /// Unit as stored (never changes unless user edits or CONVERT() used)
    stored_unit: Unit,
    
    /// Optional formula expression
    formula: Option<String>,
    
    /// Optional display unit override (from column setting)
    display_override: Option<Unit>,
    
    /// Last modified timestamp
    modified_at: DateTime<Utc>,
}
```

**Key Principle:** `stored_unit` is what the user entered. Display conversions are separate and non-destructive.

### 2.2 Unit Representation

```rust
pub struct Unit {
    /// Canonical form (normalized for comparison)
    /// "mph" = "mi/hr" = "miles per hour" all normalize to same canonical
    canonical: String,
    
    /// Original as entered (for exact round-trip)
    original: String,
    
    /// Parsed components for compound units
    /// "mi/hr" → [(mile, 1), (hour, -1)]
    components: Vec<(BaseUnit, i32)>,
    
    /// Dimension for compatibility checking
    dimension: Dimension,
}

pub enum Dimension {
    Dimensionless,
    Simple(BaseDimension),  // Length, Mass, Time, etc.
    Compound {
        numerator: Vec<(BaseDimension, i32)>,
        denominator: Vec<(BaseDimension, i32)>,
    },
}

pub enum BaseDimension {
    Length,
    Mass,
    Time,
    Currency,
    Temperature,
    DigitalStorage,
    Custom(String),
}
```

### 2.3 Column Metadata

```rust
pub struct ColumnMetadata {
    /// Column identifier (A, B, C, ...)
    index: String,
    
    /// Display name
    name: String,
    
    /// Default unit when user types bare number
    default_unit: Option<Unit>,
    
    /// How to display values (conversion applied)
    display_as: Option<Unit>,
    
    /// Semantic type (helps with suggestions)
    value_type: ValueType,
    
    /// Validation rules
    validation: ValidationRules,
    
    /// Column width (UI)
    width: u32,
}

pub enum ValueType {
    Temperature,
    Length,
    Mass,
    Volume,
    StorageSize,
    Currency,
    Duration,
    DateTime,
    Count,
    Rate,
    Custom(String),
}

pub struct ValidationRules {
    /// Is this field required?
    required: bool,
    
    /// Valid values restriction
    valid_values: Option<ValidValuesSource>,
}

pub enum ValidValuesSource {
    /// Manual list: ["us-east-1", "us-west-2"]
    ManualList(Vec<String>),
    
    /// SQL query: "SELECT DISTINCT Region FROM AvailableRegions"
    SqlQuery(String),
}
```

### 2.4 Table Metadata

```rust
pub struct Table {
    /// Table name
    name: String,
    
    /// Cell range (e.g., "A1:F100")
    range: String,
    
    /// Entity type this table represents
    entity_type: String,  // "EC2Instance", "User", "Transaction"
    
    /// Unit for row counts
    row_unit: String,  // "instances", "users", "transactions"
    
    /// Header row index
    header_row: u32,
    
    /// Column metadata
    columns: Vec<ColumnMetadata>,
}
```

**Context-Aware Operations:**
```sql
-- When counting rows in EC2Instances table (row_unit = "instances"):
SELECT COUNT(*) FROM EC2Instances WHERE RAM > 16GB
-- Returns: 45 instances (inherits row_unit from table metadata)
```

### 2.5 Workbook Structure

```rust
pub struct Workbook {
    /// Workbook-level settings
    settings: WorkbookSettings,
    
    /// All sheets
    sheets: HashMap<String, Sheet>,
    
    /// In-memory SQLite for runtime queries
    runtime_db: Connection,
    
    /// Conversion rate history and configuration
    conversions: ConversionConfig,
    
    /// Custom units defined in this workbook
    custom_units: Vec<CustomUnit>,
    
    /// Undo stack (unlimited until save/exit)
    undo_stack: Vec<WorkbookState>,
    
    /// Current dirty state
    dirty: bool,
}

pub struct WorkbookSettings {
    /// Default unit system (Metric or Imperial)
    unit_preference: UnitSystem,
    
    /// Conversion rate mode
    conversion_mode: ConversionMode,
    
    /// Enabled unit domains
    enabled_domains: HashSet<String>,
    
    /// MCP server configurations
    mcp_servers: Vec<MCPServerConfig>,
}

pub enum UnitSystem {
    Metric,
    Imperial,
}

pub enum ConversionMode {
    LiveAuto,
    PromptOnOpen,
    AsOfDate(DateTime<Utc>),
    Manual,
}
```

---

## 3. Unit System

### 3.1 Built-in Unit Library

**Comprehensive library with domain organization:**

#### Physical Measurements

**Length:**
- Metric: mm, cm, m, km
- Imperial: in, ft, yd, mi
- Nautical: nmi

**Mass:**
- Metric: mg, g, kg, tonne
- Imperial: oz, lb, ton

**Time:**
- Basic: s, min, hr, day, week, month, year
- Business: quarter, fiscal_year

**Temperature:**
- C (Celsius), F (Fahrenheit), K (Kelvin)

**Volume:**
- Metric: mL, L
- Imperial: fl oz, cup, pt, qt, gal

**Area (Derived):**
- m², cm², km², ft², yd², mi², acre, hectare

**Speed (Derived):**
- mi/hr (mph), km/hr (kph), m/s, ft/s, knot

#### Digital/Data

**Digital Storage:**
- Decimal: B, KB, MB, GB, TB, PB
- Binary: KiB, MiB, GiB, TiB, PiB

**Data Rate:**
- bps, Kbps, Mbps, Gbps, Tbps

#### Energy & Power

**Energy:**
- J (joule), kJ, kWh, cal, kcal, BTU

**Power:**
- W (watt), kW, MW, hp

**Pressure:**
- Pa, kPa, bar, psi, atm, mmHg

**Force:**
- N (newton), kN, lbf

**Frequency:**
- Hz, kHz, MHz, GHz

**Angle:**
- deg (degree), rad (radian), grad

#### Financial

**Currency:**
- Major: USD, EUR, GBP, JPY, CNY
- Common: CAD, AUD, CHF, INR, BRL, MXN, ZAR
- Full ISO 4217 support (150+ currencies)

### 3.2 Unit Domain Management

**Workbook-Level Domain Enable/Disable:**

Users can enable/disable entire domains to keep autocomplete clean:

```rust
pub struct UnitDomain {
    id: String,              // "length", "energy", "currency"
    name: String,            // "Length", "Energy", "Currency"
    description: String,
    builtin: bool,           // App-provided vs custom
    units: Vec<Unit>,
}

// In workbook settings:
enabled_domains: ["length", "mass", "time", "currency", "digital_storage"]
```

**Benefits:**
- Cleaner autocomplete (only relevant units shown)
- Faster unit search
- Domain-specific workbooks (e.g., financial analysis doesn't need pressure units)

**UI:**
- Settings → Manage Unit Domains
- Checkbox list with unit counts
- Preview of units in each domain

### 3.3 Custom Units

Users can define custom units with conversions:

```rust
pub struct CustomUnit {
    name: String,                    // "widgets", "API_calls", "sprints"
    domain: String,                  // "BusinessMetrics", "ProjectTime"
    dimension: Dimension,            // Dimensionless or maps to standard
    conversions: Vec<Conversion>,    // To other units
    created_by: String,
    created_at: DateTime<Utc>,
}

pub struct Conversion {
    to_unit: String,
    rate: f64,
    conversion_type: ConversionType,
}

pub enum ConversionType {
    Manual,      // User assumption
    Fixed,       // Never changes (12 in/ft)
    MCP(String), // From MCP server
}
```

**Examples:**
```rust
// Business domain
CustomUnit {
    name: "widgets",
    domain: "BusinessMetrics",
    dimension: Dimensionless,
    conversions: [
        Conversion { to: "USD", rate: 5.50, type: Manual }
    ]
}

// Time domain
CustomUnit {
    name: "sprint",
    domain: "ProjectTime", 
    dimension: Time,
    conversions: [
        Conversion { to: "weeks", rate: 2.0, type: Fixed }
    ]
}
```

**Storage:**
- Embedded in workbook JSON (travels with file)
- Optionally in user's personal library (not embedded)

### 3.4 Unit Parsing & Disambiguation

**Autocomplete System:**

```rust
pub fn parse_unit_input(input: &str) -> Result<Unit, Disambiguation> {
    let candidates = find_matching_units(input);
    
    match candidates.len() {
        0 => Err("Unknown unit"),
        1 => Ok(candidates[0]),
        _ => Err(Disambiguation { candidates })
    }
}
```

**Disambiguation UI:**
```
User types: "100 m"

Autocomplete shows:
┌──────────────────────────────────┐
│ m - meters (Length)          ✓   │
│ m - minutes (Time)               │
│ mi - miles (Length)              │
│ mm - millimeters (Length)        │
└──────────────────────────────────┘

Arrow keys to navigate, Tab/Enter to select
```

**Smart Suggestions:**
- Filter by enabled domains
- Prioritize recently used
- Show category for clarity
- Fuzzy matching support

---

## 4. Conversion System

### 4.1 Display vs Storage Separation

**Critical Design Principle:**

**Storage Unit:** What the cell actually contains (immutable unless explicitly converted)

**Display Unit:** How the value is shown (non-destructive, can toggle freely)

**Example:**
```
Cell A1 stored: 100 feet
Sheet display: Metric
Cell A1 displays: 30.48 m

Formula =A1*2 calculates: 200 feet (uses storage unit)
Result displays: 60.96 m (applies display conversion)
```

### 4.2 Conversion Graph

**Pathfinding System:**

```rust
pub struct ConversionGraph {
    /// Direct conversions (built-in + manual)
    edges: HashMap<(Unit, Unit), ConversionFactor>,
    
    /// Find conversion path between units
    fn find_path(&self, from: &Unit, to: &Unit) -> Option<ConversionPath>,
    
    /// For compound units: convert each component
    fn convert_compound(&self, from: &CompoundUnit, to: &CompoundUnit) 
        -> Option<CompoundConversionPath>,
}

pub struct ConversionPath {
    steps: Vec<ConversionStep>,
    total_factor: f64,
}

pub struct ConversionStep {
    from: Unit,
    to: Unit,
    factor: f64,
    source: ConversionSource,
}

pub enum ConversionSource {
    Builtin,           // Fixed conversion (12 in/ft)
    Manual,            // User-defined
    MCP(String),       // From MCP server
    Chained(Vec<Unit>), // Multi-hop path
}
```

**Compound Unit Conversion:**

```rust
// Convert $/hour to k$/month
Source: [(USD, 1), (hour, -1)]
Target: [(kUSD, 1), (month, -1)]

Steps:
1. Numerator: USD → kUSD (÷1000)
2. Denominator: hour → month (×720)
3. Compose: value ÷ 1000 × 720
```

### 4.3 Conversion Rate Modes

**Four modes for dynamic conversions (e.g., currencies, stocks):**

#### Mode 1: Live Auto
- Continuous updates from MCP server
- Configurable interval (default: 1 hour)
- Shows last update time
- Formulas recalculate on rate change

#### Mode 2: Prompt on Open
- Dialog when workbook opens
- Shows current vs latest rate
- User chooses: Update, Keep Current, or Manual Entry
- Good for reproducible analysis

#### Mode 3: As of Specific Date
- User selects historical date
- Fetches rate from that date
- Locks to historical rate
- Perfect for "reproduce Q2 analysis with Q2 rates"

#### Mode 4: Manual Entry
- User types assumed rate
- Displays as "Assumption: 0.95"
- Good for forecast scenarios

**Settings Hierarchy:**

```rust
pub struct ConversionConfig {
    /// Default mode for workbook
    default_mode: ConversionMode,
    
    /// Per-pair overrides
    manual_overrides: HashMap<(Unit, Unit), ManualRate>,
    
    /// Rate history (for auditing)
    history: Vec<ConversionRateSnapshot>,
}
```

**Example:**
- Workbook default: Live Auto
- USD→EUR: Uses Live Auto (inherits default)
- Widget→USD: Manual override = $5.50 (forecast assumption)

### 4.4 Conversion Chain Trust System

**Problem:** EUR→GBP conversion when only EUR→USD and USD→GBP exist.

**Solution:** Automatic chaining with trust approval.

**Behavior:**
```
First use of EUR→GBP:
┌─────────────────────────────────────────────────┐
│ ⚠️  Conversion Chain Required                   │
├─────────────────────────────────────────────────┤
│                                                 │
│ No direct EUR→GBP rate available.               │
│                                                 │
│ Available chain:                                │
│   EUR → USD → GBP                               │
│   (0.92 × 1.15 = 1.058)                         │
│   1 hop                                         │
│                                                 │
│ Options:                                        │
│ [Calculate Once]  - Use this time only          │
│ [Trust This Chain] - Remember and auto-use      │
│ [Add Direct Rate] - Enter EUR→GBP manually      │
│ [Cancel]                                        │
└─────────────────────────────────────────────────┘

After "Trust This Chain":
- Workbook remembers: EUR→GBP via USD is trusted
- Future calculations silent (no warning)
- Manageable in "Manage Conversion Rates" dialog
```

**Warning Threshold:**
- 1+ hops triggers warning (even simple chains)
- Ensures user awareness of indirect conversions
- Good for data quality

### 4.5 Conversion Factor Detection

**From Feature Request Doc - Unit Cancellation:**

System automatically identifies conversion factors in formulas:

```
Formula: $10/hr * 720hr/month

System recognizes:
- 720hr/month is a conversion factor (ratio format, causes cancellation)
- hr in denominator cancels with hr in numerator
- Result: $/month

Visual feedback:
- Blue highlighting on 720hr/month
- Tooltip: "Conversion factor: hours per month"
- ⚡ icon showing cancellation occurred
```

**Conversion Factor Library:**

Users can define named conversion factors:

```rust
pub struct ConversionFactor {
    name: String,                // "MonthlyHours", "WorkWeek"
    value: f64,                  // 720, 40
    unit: CompoundUnit,          // hr/month, hr/week
    confidence: Confidence,      // High (in library) or Medium (custom)
    description: String,
}
```

**Benefits:**
- Self-documenting formulas
- Centralized management
- Easy updates across workbook

---

## 5. Formula Engine

### 5.1 Unit-Aware Operations

**Arithmetic Operations:**

```rust
impl Add for CellValue {
    fn add(self, other: CellValue) -> CellValue {
        // Check unit compatibility
        if self.unit.dimension() == other.unit.dimension() {
            // Compatible: auto-convert to workbook preference
            let converted = convert_to_preference(other, workbook.unit_preference);
            CellValue {
                value: self.value + converted.value,
                unit: self.unit,  // Result takes first operand's unit
                warnings: vec![],
            }
        } else {
            // Incompatible: calculate anyway, warn, result dimensionless
            CellValue {
                value: self.value + other.value,
                unit: Unit::dimensionless(),
                warnings: vec![
                    Warning::IncompatibleUnits {
                        op: "addition",
                        lhs: self.unit,
                        rhs: other.unit,
                    }
                ],
            }
        }
    }
}
```

**Multiplication/Division (Compound Units):**

```rust
impl Mul for CellValue {
    fn mul(self, other: CellValue) -> CellValue {
        CellValue {
            value: self.value * other.value,
            unit: Unit::compound_multiply(self.unit, other.unit),
            warnings: vec![],
        }
    }
}

// Examples:
// 5m * 3m → 15 m²
// 100USD / 5 users → 20 USD/user
// 50 mi / 2 hr → 25 mi/hr
```

**Unit Cancellation:**

```rust
impl Div for CellValue {
    fn div(self, other: CellValue) -> CellValue {
        let result_unit = Unit::compound_divide(self.unit, other.unit);
        
        // If units perfectly cancel, result is dimensionless
        if result_unit.is_dimensionless() {
            CellValue {
                value: self.value / other.value,
                unit: Unit::dimensionless(),
                warnings: vec![],
            }
        } else {
            CellValue {
                value: self.value / other.value,
                unit: result_unit,
                warnings: vec![],
            }
        }
    }
}

// Examples:
// 100m / 50m → 2 (dimensionless, units cancel)
// 100m / 50s → 2 m/s
```

### 5.2 Functions

**Aggregation Functions:**

```rust
fn SUM(range: Vec<CellValue>, workbook: &Workbook) -> CellValue {
    // Auto-convert all to workbook preference unit
    let first_unit = range[0].unit;
    let converted: Vec<f64> = range.iter()
        .map(|cell| convert_to(cell, &first_unit, workbook).value)
        .collect();
    
    CellValue {
        value: converted.iter().sum(),
        unit: first_unit,
        warnings: vec![],
    }
}

// Example: =SUM(5m, 10ft, 200cm) on Metric workbook → 10.048m
```

**Statistical Functions:**

```rust
fn STDEV(range: Vec<CellValue>) -> CellValue {
    // Standard deviation maintains unit
    let mean = AVERAGE(range.clone());
    // ... calculate stdev ...
    CellValue {
        value: stdev_value,
        unit: mean.unit,  // Same unit as inputs
        warnings: vec![],
    }
}
```

**Context-Aware COUNT:**

```rust
fn COUNT(range: Vec<CellValue>, context: &TableContext) -> CellValue {
    CellValue {
        value: range.len() as f64,
        unit: context.row_unit.clone(),  // Inherits from table
        warnings: vec![],
    }
}

// Example: =COUNT(RAM_column) in EC2Instances table → 50 instances
```

**Trigonometric Functions:**

```rust
fn SIN(angle: CellValue) -> CellValue {
    let radians = match angle.unit {
        Unit::Degree => angle.value.to_radians(),
        Unit::Radian => angle.value,
        Unit::Dimensionless => angle.value, // Assume radians
        _ => {
            // Warning: unusual unit for trig function
            angle.value
        }
    };
    
    CellValue {
        value: radians.sin(),
        unit: Unit::dimensionless(),
        warnings: vec![],
    }
}
```

**Unit Conversion Function:**

```rust
fn CONVERT(value: CellValue, target_unit: Unit) -> Result<CellValue> {
    // DESTRUCTIVE: Permanently changes stored unit
    // Shows warning dialog before execution
    
    let converted = conversion_graph.convert(&value, &target_unit)?;
    
    // Lock display to target unit
    Ok(CellValue {
        value: converted.value,
        unit: target_unit.clone(),
        display_override: Some(target_unit),
        warnings: vec![
            Warning::PermanentConversion {
                from: value.unit,
                to: target_unit,
            }
        ],
    })
}
```

### 5.3 Formula Parsing

**Parser Architecture:**

```rust
pub struct FormulaParser {
    tokenizer: Tokenizer,
    unit_library: UnitLibrary,
}

pub enum Token {
    Number(f64),
    Unit(String),
    Operator(Operator),
    Function(String),
    CellRef(String),
    RangeRef(String, String),
}

impl FormulaParser {
    pub fn parse(&self, formula: &str) -> Result<AST> {
        // 1. Tokenize: "=A1*2+3m" → [CellRef(A1), *, Number(2), +, Number(3), Unit(m)]
        // 2. Build AST with unit annotations
        // 3. Validate dimensional consistency
        // 4. Return executable AST
    }
}
```

**Literal Units in Formulas:**

```
=5m * 3m → 15 m²
=100 USD + 50 EUR  (auto-converts per workbook setting)
```

**Autocomplete in formula bar:**
- Type `=5 m` → dropdown shows meters/minutes
- Context-aware suggestions

### 5.4 Dependency Tracking

**Dependency Graph:**

```rust
pub struct DependencyGraph {
    /// Map of cell → cells that depend on it
    dependents: HashMap<CellRef, HashSet<CellRef>>,
    
    /// Map of cell → cells it depends on
    dependencies: HashMap<CellRef, HashSet<CellRef>>,
}

impl DependencyGraph {
    pub fn add_formula(&mut self, cell: CellRef, formula: &AST) {
        let deps = extract_dependencies(formula);
        for dep in deps {
            self.dependents.entry(dep).or_default().insert(cell);
            self.dependencies.entry(cell).or_default().insert(dep);
        }
    }
    
    pub fn get_recalc_order(&self, changed: CellRef) -> Vec<CellRef> {
        // Topological sort of dependents
        // Returns order to recalculate
    }
}
```

**Recalculation:**

```rust
pub fn recalculate(&mut self, changed: CellRef) {
    let order = self.dep_graph.get_recalc_order(changed);
    
    for cell in order {
        let formula = self.get_formula(cell);
        let result = self.formula_engine.evaluate(formula);
        self.set_value(cell, result);
    }
}
```

**Circular Reference Detection:**

```rust
pub fn check_circular(&self, cell: CellRef, formula: &AST) -> Result<()> {
    let deps = extract_dependencies(formula);
    
    for dep in deps {
        if self.reaches(dep, cell) {
            return Err(CircularReference { cell, via: dep });
        }
    }
    
    Ok(())
}
```

---

## 6. Table System

### 6.1 Table Definition

Tables are structured collections of entities with metadata:

```rust
pub struct Table {
    name: String,
    range: CellRange,          // A1:F100
    entity_type: String,       // "EC2Instance", "User"
    row_unit: String,          // "instances", "users"
    header_row: u32,
    columns: Vec<ColumnMetadata>,
}
```

**Benefits:**
- SQL-queryable structure
- Context-aware COUNT operations
- Validation rules
- Entity-type semantics

### 6.2 SQL Query System

**Query Language:**

Familiar SQL-like syntax adapted for spreadsheets:

```sql
SELECT [fields]
FROM [table_name]
WHERE [conditions]
ORDER BY [field] [ASC|DESC]
LIMIT [number]
```

**Unit-Aware Queries:**

```sql
-- Automatic unit conversion in comparisons
SELECT * FROM EC2Instances 
WHERE Storage > 1TB AND Price < 0.50 USD/hr

-- System converts: 1TB → 1024GB (if Storage stored in GB)
-- Compares correctly with units
```

**Aggregate Operations:**

```sql
-- Count with entity awareness
SELECT Region, COUNT(*) as InstanceCount
FROM EC2Instances
WHERE RAM >= 16GB
GROUP BY Region

-- Returns: "3 instances in us-east-1" (inherits row_unit)
```

**Compound Unit Calculations:**

```sql
SELECT InstanceType, Price, (Price * 730 hr/month) as MonthlyCost
FROM EC2Instances
WHERE Region = 'us-east-1'
ORDER BY MonthlyCost ASC
```

### 6.3 Query Execution

**In-Memory SQLite:**

Since workbook uses in-memory SQLite at runtime, queries are native SQL:

```rust
pub fn execute_query(&self, query: &str) -> Result<QueryResult> {
    // Parse SQL with unit annotations
    let parsed = parse_unit_aware_sql(query)?;
    
    // Execute on in-memory SQLite
    let rows = self.runtime_db.query(parsed.sql_string)?;
    
    // Apply unit conversions for display
    let converted_rows = rows.iter()
        .map(|row| apply_display_conversions(row, &parsed))
        .collect();
    
    Ok(QueryResult {
        rows: converted_rows,
        column_units: parsed.column_units,
    })
}
```

**Query Result Handling:**

```rust
pub struct QueryResult {
    rows: Vec<Row>,
    column_units: HashMap<String, Unit>,
    row_count: usize,
    execution_time_ms: u64,
}

pub struct Row {
    cells: HashMap<String, CellValue>,
}
```

### 6.4 Validation System

**Column-Level Validation:**

```rust
pub struct ValidationRules {
    required: bool,
    valid_values: Option<ValidValuesSource>,
}

pub enum ValidValuesSource {
    /// Manual list: ["us-east-1", "us-west-2"]
    ManualList(Vec<String>),
    
    /// SQL query generates dropdown:
    /// "SELECT DISTINCT Region FROM AvailableRegions ORDER BY Region"
    SqlQuery(String),
}
```

**Validation Execution:**

```rust
pub fn validate_cell(&self, cell: &Cell, rules: &ValidationRules) -> Vec<ValidationError> {
    let mut errors = vec![];
    
    // Required check
    if rules.required && cell.value.is_none() {
        errors.push(ValidationError::Required);
    }
    
    // Enum check
    if let Some(ValidValuesSource::ManualList(values)) = &rules.valid_values {
        if !values.contains(&cell.value.to_string()) {
            errors.push(ValidationError::InvalidValue {
                value: cell.value,
                allowed: values.clone(),
            });
        }
    }
    
    // SQL-driven enum check
    if let Some(ValidValuesSource::SqlQuery(query)) = &rules.valid_values {
        let valid_values = self.execute_query(query)?;
        if !valid_values.contains(&cell.value) {
            errors.push(ValidationError::NotInQueryResult);
        }
    }
    
    errors
}
```

**UI Feedback:**

- Red cell border for validation errors
- Tooltip with error message
- Validation panel shows all errors

**Data Entry Assistance:**

- Dropdown for enum fields (manual or SQL-sourced)
- Default unit application (from column metadata)
- Real-time validation as user types

---

## 7. User Interface

### 7.1 Cell Entry & Editing

**Text Entry with Unit Autocomplete:**

```
User types: "100 m"

Autocomplete appears:
┌──────────────────────────────────┐
│ m - meters (Length)          ✓   │
│ m - minutes (Time)               │
│ mi - miles (Length)              │
│ mm - millimeters (Length)        │
└──────────────────────────────────┘

Category shown for disambiguation
Arrow keys navigate, Tab/Enter accepts
ESC cancels
```

**Disambiguation Flow:**

```
1. User types unit abbreviation
2. System finds matches across enabled domains
3. If multiple matches:
   - Show dropdown with categories
   - Highlight most common
   - Wait for user selection
4. Store chosen unit with cell
```

**Edit Behavior:**

```
User clicks cell showing "30.48 m" (stored as 100 ft, Metric display)
Cell edit mode shows: "100 ft" (actual storage unit)
User edits to: "50 m"
System stores: 50 m (storage unit changes)
```

**Key principle:** Edit the storage unit directly. Display conversions only apply in view mode.

### 7.2 Display Unit Toggle

**Ribbon Toggle Button:**

```
┌──────────────────────────────────┐
│ Display: [Metric ⇄ Imperial]    │
└──────────────────────────────────┘

Click to switch entire sheet display
Non-destructive (storage unchanged)
Instant visual conversion
```

**Behavior:**

```
Sheet in Metric mode:
- Cell with 100 ft displays: 30.48 m
- Cell with 5 mi displays: 8.05 km
- Cell with 50 USD displays: 50 USD (no change)

Toggle to Imperial:
- Cell with 100 ft displays: 100 ft (no conversion needed)
- Cell with 8047 m displays: 5 mi (converts for display)
- Formulas still calculate with storage units
```

### 7.3 Warning & Error Display

**Soft Warnings (Orange):**

```
Cell shows: 7.236 (dimensionless)
Background: Light orange
Icon: ⚠️ in top-right corner
Tooltip: "Warning: Adding incompatible units (m + s). Result is dimensionless."

Calculation proceeds
User can acknowledge and continue
No blocking
```

**Warning Types:**

- Incompatible unit operations (`5m + 10s`)
- Fractional exponents (`(5m)^0.5 → m^0.5`)
- Dimensioned + dimensionless mixing
- Temperature ratios
- Unusual unit combinations

**Conversion Rate Indicators:**

```
Cell displays: "94 EUR"

Indicator icons:
🟢 - Live rate (auto-updating)
📅 - Historical rate (as of date)
✏️ - Manual assumption
⚠️ - Stale/connection issue
```

### 7.4 Unit Domain Management

**Settings → Manage Unit Domains:**

```
┌─────────────────────────────────────────────────────────┐
│ Unit Domains for This Workbook                         │
├─────────────────────────────────────────────────────────┤
│                                                         │
│ Built-in Domains:                                       │
│ ☑ Length (23 units: m, ft, mi, ...)                    │
│ ☑ Mass (15 units: kg, lb, oz, ...)                     │
│ ☑ Time (12 units: s, min, hr, day, ...)                │
│ ☑ Currency (50+ units: USD, EUR, GBP, ...)             │
│ ☑ Temperature (3 units: °C, °F, K)                      │
│ ☑ Digital Storage (10 units: B, KB, MB, GB, ...)       │
│ ☑ Data Rate (8 units: bps, Kbps, Mbps, ...)            │
│ ☑ Area (8 units: m², ft², acre, ...)                   │
│ ☑ Volume (9 units: L, mL, gal, ...)                    │
│ ☑ Speed (6 units: mph, kph, m/s, ...)                  │
│                                                         │
│ ☐ Energy (7 units: J, kWh, cal, BTU, ...)              │
│ ☐ Power (4 units: W, kW, hp, ...)                      │
│ ☐ Pressure (6 units: Pa, bar, psi, ...)                │
│ ☐ Force (3 units: N, kN, lbf)                          │
│ ☐ Frequency (4 units: Hz, kHz, MHz, GHz)               │
│ ☐ Angle (3 units: deg, rad, grad)                      │
│                                                         │
│ Custom Domains (2):                                     │
│ ☑ Business Metrics (widgets, API_calls, seats)         │
│ ☑ Project Time (sprints = 2 weeks)                     │
│                                                         │
│ [Select All] [Deselect All] [Reset to Defaults]        │
│ [Manage Custom Units...]                                │
│                                                         │
│ Note: Disabled domains won't appear in autocomplete    │
│       but existing cells retain their units.           │
│                                                         │
│ [Cancel]                                       [Apply]  │
└─────────────────────────────────────────────────────────┘
```

### 7.5 Conversion Rate Management

**Ribbon → Manage Conversion Rates:**

```
┌─────────────────────────────────────────────────────────┐
│ Conversion Rates                                        │
├─────────────────────────────────────────────────────────┤
│                                                         │
│ Workbook Default Mode: [Live Auto ▼]                   │
│ Update Interval: [1 hour ▼]                            │
│ MCP Server: [mcp-server-currency (ECB) ▼]              │
│                                                         │
│ Active Conversions (8):                                 │
│ ┌───────────────────────────────────────────────────┐ │
│ │ From → To  │ Mode      │ Rate   │ Updated        │ │
│ ├───────────────────────────────────────────────────┤ │
│ │ USD → EUR  │ Live Auto │ 0.94   │ 2 min ago   🟢│ │
│ │ EUR → GBP  │ Live Auto │ 0.86   │ 2 min ago   🟢│ │
│ │ USD → JPY  │ Live Auto │ 149.50 │ 2 min ago   🟢│ │
│ │ Widget→USD │ Manual    │ 5.50   │ User set    ✏️│ │
│ │ EUR → GBP  │ Chained   │ 1.058  │ via USD     📊│ │
│ └───────────────────────────────────────────────────┘ │
│                                                         │
│ [Add Manual Conversion]                                 │
│ [Update All Rates Now]                                  │
│ [View Rate History...]                                  │
│                                                         │
│ Trusted Chains (2):                                     │
│ - EUR → GBP via USD (1 hop) [Remove Trust]             │
│ - JPY → GBP via USD (1 hop) [Remove Trust]             │
│                                                         │
│ [Close]                                                 │
└─────────────────────────────────────────────────────────┘
```

### 7.6 Query Builder

**Visual query builder for non-SQL users:**

```
┌─────────────────────────────────────────────────────────┐
│ Query Builder                                           │
├─────────────────────────────────────────────────────────┤
│                                                         │
│ Table: [EC2Instances ▼]                                 │
│                                                         │
│ Select Fields:                                          │
│ ☑ InstanceType  ☑ RAM  ☑ Price  ☐ Storage              │
│                                                         │
│ Filters:                                                │
│ ┌───────────────────────────────────────────────────┐ │
│ │ [Region     ▼] [=  ▼] [us-east-1  ▼]       [×]   │ │
│ │ [AND ▼]                                           │ │
│ │ [RAM        ▼] [>  ▼] [16        ] GB        [×]   │ │
│ │ [AND ▼]                                           │ │
│ │ [Price      ▼] [<  ▼] [0.50      ] USD/hr    [×]   │ │
│ └───────────────────────────────────────────────────┘ │
│ [+ Add Filter]                                          │
│                                                         │
│ Sort By:                                                │
│ [Price ▼] [Ascending ▼]  [×]                           │
│ [+ Add Sort]                                            │
│                                                         │
│ Limit: [50 ▼]                                           │
│                                                         │
│ Generated SQL:                                          │
│ ┌───────────────────────────────────────────────────┐ │
│ │ SELECT InstanceType, RAM, Price                   │ │
│ │ FROM EC2Instances                                 │ │
│ │ WHERE Region = 'us-east-1'                        │ │
│ │   AND RAM > 16 GB                                 │ │
│ │   AND Price < 0.50 USD/hr                         │ │
│ │ ORDER BY Price ASC                                │ │
│ │ LIMIT 50                                          │ │
│ └───────────────────────────────────────────────────┘ │
│                                                         │
│ [Preview] [Run Query] [Save as View]                   │
└─────────────────────────────────────────────────────────┘
```

### 7.7 Column Configuration

**Right-click column header → Configure Column:**

```
┌─────────────────────────────────────────────────────────┐
│ Configure Column: RAM                                   │
├─────────────────────────────────────────────────────────┤
│                                                         │
│ Basic Properties:                                       │
│ Column Name: [RAM                                    ]  │
│ Value Type:  [StorageSize           ▼]                 │
│ Default Unit: [GB                   ▼]                  │
│ Display As:   [GB                   ▼]                  │
│                                                         │
│ Validation:                                             │
│ ☑ Required field                                        │
│ ☐ Valid values (dropdown)                               │
│   Source: ○ Manual list  ○ SQL query                    │
│                                                         │
│ Description:                                            │
│ ┌───────────────────────────────────────────────────┐ │
│ │ Amount of RAM in gigabytes. Typical range is     │ │
│ │ 0.5 GB to 384 GB for cloud instances.            │ │
│ └───────────────────────────────────────────────────┘ │
│                                                         │
│ [Cancel]                              [Apply]    [OK]   │
└─────────────────────────────────────────────────────────┘
```

---

## 8. MCP Integration

### 8.1 Internal MCP Server

**Exposes spreadsheet operations to AI tools:**

```rust
pub struct SpreadsheetMCPServer {
    workbook: Arc<RwLock<Workbook>>,
    server: MCPServer,
}

impl SpreadsheetMCPServer {
    pub fn new(workbook: Workbook) -> Self {
        let server = MCPServer::new("spreadsheet");
        
        // Register tools
        server.register_tool("read_cell", read_cell_handler);
        server.register_tool("write_cell", write_cell_handler);
        server.register_tool("query_table", query_table_handler);
        server.register_tool("get_conversion_rate", get_conversion_rate_handler);
        
        // Register resources
        server.register_resource("spreadsheet://workbook/{id}/cell/{ref}");
        server.register_resource("spreadsheet://workbook/{id}/table/{name}");
        
        Self { workbook, server }
    }
}
```

**MCP Tools:**

```rust
// Read cell with full unit information
pub fn read_cell(cell_ref: String) -> MCPResponse {
    let cell = workbook.get_cell(&cell_ref)?;
    
    json!({
        "value": cell.value,
        "unit": cell.unit.to_string(),
        "formula": cell.formula,
        "display_unit": cell.display_override,
        "dimension": cell.unit.dimension()
    })
}

// Write cell with unit awareness
pub fn write_cell(cell_ref: String, value: f64, unit: String) -> MCPResponse {
    let parsed_unit = parse_unit(&unit)?;
    workbook.set_cell(&cell_ref, Cell {
        value,
        unit: parsed_unit,
        formula: None,
    })?;
    
    json!({ "success": true })
}

// Query table with SQL
pub fn query_table(table_name: String, sql: String) -> MCPResponse {
    let result = workbook.execute_query(&sql)?;
    
    json!({
        "rows": result.rows,
        "column_units": result.column_units,
        "row_count": result.row_count
    })
}
```

**MCP Resources:**

```
spreadsheet://workbook/{workbook_id}/cell/{cell_ref}
spreadsheet://workbook/{workbook_id}/table/{table_name}
spreadsheet://workbook/{workbook_id}/conversions
spreadsheet://workbook/{workbook_id}/units/custom
```

### 8.2 External MCP Servers

**Built-in MCP Clients:**

```rust
pub struct MCPClientManager {
    clients: HashMap<String, MCPClient>,
}

impl MCPClientManager {
    pub fn add_server(&mut self, config: MCPServerConfig) {
        let client = MCPClient::connect(&config.url)?;
        self.clients.insert(config.name, client);
    }
    
    pub fn get_conversion_rate(&self, from: &Unit, to: &Unit) -> Result<f64> {
        // Try currency server
        if let Some(client) = self.clients.get("currency") {
            if let Ok(rate) = client.call_tool("get_rate", json!({
                "from": from.to_string(),
                "to": to.to_string()
            })) {
                return Ok(rate["rate"].as_f64()?);
            }
        }
        
        Err("No conversion available")
    }
}
```

**Supported External Servers (MVP):**

1. **mcp-server-currency** (ECB, Federal Reserve)
   - Currency conversion rates
   - Historical rates (for "As of Date" mode)
   - Live updates

2. **mcp-server-stocks** (Yahoo Finance, Alpha Vantage)
   - Stock prices as units
   - Real-time quotes
   - Historical data

### 8.3 Custom MCP Servers

**User Configuration UI:**

```
Settings → MCP Servers → Add Custom Server

┌─────────────────────────────────────────────────────────┐
│ Add Custom MCP Server                                   │
├─────────────────────────────────────────────────────────┤
│                                                         │
│ Name: [Company Widget Pricing                       ]  │
│ URL:  [mcp://internal.company.com/widget-pricing    ]  │
│                                                         │
│ Server Type:                                            │
│ ○ Conversion Rate Provider                              │
│ ○ Data Source                                           │
│ ○ Custom                                                │
│                                                         │
│ Provides:                                               │
│ ☑ Unit conversions (widget → USD)                       │
│ ☐ Table data                                            │
│ ☐ Other operations                                      │
│                                                         │
│ Authentication (optional):                              │
│ API Key: [••••••••••••••••••••                      ]  │
│                                                         │
│ [Test Connection]                                       │
│                                                         │
│ [Cancel]                                          [Add]  │
└─────────────────────────────────────────────────────────┘
```

**Custom Server Protocol:**

```json
// Example: Company's widget pricing server
{
  "server": {
    "name": "widget-pricing",
    "version": "1.0"
  },
  "tools": [
    {
      "name": "get_widget_price",
      "description": "Get current widget price in USD",
      "parameters": {
        "widget_sku": "string",
        "quantity": "number"
      }
    }
  ],
  "conversions": [
    {
      "from": "widgets",
      "to": "USD",
      "rate": 5.50,
      "updated_at": "2025-01-15T10:00:00Z"
    }
  ]
}
```

### 8.4 AI Assistant Integration

**Natural Language Query:**

```
User: "Show me cost-effective EC2 instances in us-east-1 with at least 32GB RAM"

AI via MCP:
1. Calls get_table_schema("EC2Instances")
2. Generates SQL:
   SELECT InstanceType, RAM, Price
   FROM EC2Instances
   WHERE Region = 'us-east-1' AND RAM >= 32GB
   ORDER BY Price ASC
   LIMIT 10
3. Calls query_table() with SQL
4. Formats results for user
5. Suggests follow-ups
```

**AI-Assisted Formula Creation:**

```
User: "Calculate monthly cost from hourly rate in cell B2"

AI via MCP:
1. Reads cell B2 (finds: 0.192 USD/hr)
2. Suggests formula: =B2*730hr/month
3. Explains unit cancellation
4. Offers to insert formula
```

---

## 9. File Format

### 9.1 JSON Structure (MVP)

**File Extension:** `.usheet` or `.usheet.json`

**Top-Level Structure:**

```json
{
  "version": "1.0",
  "created": "2025-10-05T14:30:00Z",
  "modified": "2025-10-05T16:45:00Z",
  "application": "unit-aware-spreadsheet",
  
  "workbook_settings": {
    "unit_preference": "Metric",
    "conversion_mode": "LiveAuto",
    "conversion_update_interval": 3600,
    "enabled_domains": [
      "length", "mass", "time", "currency", 
      "digital_storage", "data_rate"
    ],
    "mcp_servers": [
      {
        "name": "currency",
        "url": "mcp://localhost:3001/currency",
        "enabled": true
      }
    ]
  },
  
  "sheets": [
    {
      "name": "Sheet1",
      "display_settings": {
        "unit_display": "Metric",
        "frozen_rows": 1,
        "frozen_cols": 0
      },
      "columns": [...],
      "tables": [...],
      "cells": {...},
      "named_ranges": {...}
    }
  ],
  
  "conversions": {
    "history": [...],
    "manual_overrides": [...],
    "trusted_chains": [...]
  },
  
  "custom_units": [...],
  "custom_domains": [...]
}
```

**Sheet Structure:**

```json
{
  "name": "EC2Pricing",
  "display_settings": {
    "unit_display": "Metric",
    "frozen_rows": 1,
    "frozen_cols": 0
  },
  
  "columns": [
    {
      "index": "A",
      "name": "Region",
      "width": 120,
      "default_unit": null,
      "display_as": null,
      "value_type": "Text",
      "validation": {
        "required": false,
        "valid_values": {
          "type": "manual_list",
          "values": ["us-east-1", "us-west-2", "eu-central-1"]
        }
      }
    },
    {
      "index": "B",
      "name": "RAM",
      "width": 80,
      "default_unit": "GB",
      "display_as": "GB",
      "value_type": "StorageSize",
      "validation": {
        "required": true
      }
    }
  ],
  
  "tables": [
    {
      "name": "EC2Instances",
      "range": "A1:F100",
      "entity_type": "EC2Instance",
      "row_unit": "instances",
      "header_row": 1
    }
  ],
  
  "cells": {
    "A1": {
      "value": 100.0,
      "unit": "USD",
      "formula": null,
      "display_override": null,
      "modified_at": "2025-10-05T16:30:00Z"
    },
    "A2": {
      "value": 85.0,
      "unit": "EUR",
      "formula": null,
      "display_override": null,
      "modified_at": "2025-10-05T16:31:00Z"
    },
    "B1": {
      "value": 150.0,
      "unit": "USD",
      "formula": "=A1*1.5",
      "display_override": null,
      "modified_at": "2025-10-05T16:32:00Z"
    }
  },
  
  "named_ranges": {
    "PriceList": "C2:C100"
  }
}
```

**Conversion Rates:**

```json
{
  "conversions": {
    "history": [
      {
        "from": "USD",
        "to": "EUR",
        "rate": 0.94,
        "source": "mcp://localhost:3001/currency",
        "timestamp": "2025-10-05T16:30:00Z",
        "mode": "LiveAuto"
      }
    ],
    "manual_overrides": [
      {
        "from": "widgets",
        "to": "USD",
        "rate": 5.50,
        "description": "Widget sales price FY2025 forecast",
        "created_by": "user@example.com",
        "created_at": "2025-01-15T10:00:00Z"
      }
    ],
    "trusted_chains": [
      {
        "from": "EUR",
        "to": "GBP",
        "via": ["USD"],
        "trusted_at": "2025-10-01T12:00:00Z"
      }
    ]
  }
}
```

**Custom Units:**

```json
{
  "custom_units": [
    {
      "name": "widgets",
      "domain": "BusinessMetrics",
      "dimension": "dimensionless",
      "conversions": [
        {
          "to": "USD",
          "rate": 5.50,
          "type": "manual",
          "description": "Forecast assumption"
        }
      ],
      "created_by": "user@example.com",
      "created_at": "2025-01-15T10:00:00Z"
    },
    {
      "name": "sprint",
      "domain": "ProjectTime",
      "dimension": "time",
      "equivalent_to": "2 weeks",
      "created_by": "user@example.com",
      "created_at": "2025-02-01T14:00:00Z"
    }
  ],
  
  "custom_domains": [
    {
      "id": "custom_business_metrics",
      "name": "Business Metrics",
      "description": "Company-specific business units",
      "units": ["widgets", "API_calls", "seats"]
    }
  ]
}
```

### 9.2 Excel Export (One-Way)

**Purpose:** Viewing in Excel, not editing. Excel users should switch to this tool for unit-aware editing.

**Export Structure:**

**Main Sheets:**
- Side-by-side layout:
  - Original columns with values
  - Adjacent columns with unit labels (as text)
  - Formulas preserved where possible

**Metadata Sheets:**

```
Sheet: __units_metadata
┌────────┬──────┬─────┬─────────────────┐
│ Sheet  │ Cell │ Unit│ Display_Override│
├────────┼──────┼─────┼─────────────────┤
│ Sheet1 │ A1   │ USD │ null            │
│ Sheet1 │ B1   │ EUR │ EUR             │
│ Sheet1 │ C1   │ m²  │ null            │
└────────┴──────┴─────┴─────────────────┘
```

```
Sheet: __conversions
┌──────────────────┬──────┬───────┬────────────────┐
│ Conversion Pair  │ Mode │ Value │ Timestamp      │
├──────────────────┼──────┼───────┼────────────────┤
│ USD→EUR          │ Live │ 0.94  │ 2025-01-15 14:30│
│ Widget→USD       │Manual│ 5.50  │ User assumption│
└──────────────────┴──────┴───────┴────────────────┘
```

```
Sheet: __README
┌─────────────────────────────────────────────────┐
│ ⚠️ EXPORTED FROM UNIT-AWARE SPREADSHEET         │
│                                                 │
│ This file was exported on 2025-10-05.           │
│ For full unit-aware editing, open in           │
│ Unit-Aware Spreadsheet application.            │
│                                                 │
│ Editing in Excel may break unit relationships. │
│                                                 │
│ Visit https://github.com/... to download       │
└─────────────────────────────────────────────────┘
```

**Warning Cell (A1 of first sheet):**
- Red/orange background
- "⚠️ EXPORTED - Open in Unit-Aware Spreadsheet for editing"

### 9.3 Copy/Paste Behavior

**Within Same Workbook:**
- Units fully preserved
- Formulas maintain unit awareness

**To External Apps (Excel, Google Sheets):**
- Paste as "100 USD" (value + unit as text)
- Or "Paste Values Only" (strips units, number only)

**From External Apps:**
- Parse common patterns: "100 USD", "5m", "3.5 kg"
- Ambiguous cases prompt user
- Recognize and convert to unit-aware cells

---

## 10. Testing Strategy

### 10.1 Golden Workbook

**Cloud Cost Analysis Workbook** exercises all core features:

**Sheet 1: EC2 Instances (Main Data Table)**
- Table with entity_type and row_unit
- Mixed currencies (USD, EUR)
- Column validation (enums, SQL-driven)
- Display unit conversions

**Sheet 2: Cost Projections (Formulas)**
- Unit cancellation: $/hr × hr/month → $/month
- Custom conversion factors (730 hr/month)
- Compound units
- Display_as override (k$/month)

**Sheet 3: Multi-Region Analysis (Display Toggle)**
- Mixed imperial/metric
- Bandwidth (Mbps/Gbps)
- Storage (GB/TB)
- Distance (mi/km)

**Sheet 4: Temperature Monitoring (Warnings)**
- Incompatible unit operations
- Temperature conversions
- Warning display and tooltips

**Sheet 5: Custom Units (Business Domain)**
- User-defined units (widgets, API_calls)
- Manual conversion factors
- Business metrics

**Sheet 6: Validation Examples**
- Required fields
- Manual enum (dropdown)
- SQL-driven enum
- Value types

**Sheet 7: Query Examples (SQL)**
- Context-aware COUNT
- Unit-aware filtering
- Compound unit calculations in SQL
- GROUP BY with aggregates

### 10.2 Unit Test Coverage

**Critical Areas:**

```rust
#[cfg(test)]
mod unit_tests {
    // Unit conversion correctness
    #[test]
    fn test_length_conversions() {
        assert_eq!(convert(1.0, "m", "ft"), 3.28084);
        assert_eq!(convert(5280.0, "ft", "mi"), 1.0);
        assert_eq!(convert(1.0, "km", "m"), 1000.0);
    }
    
    // Dimensional analysis
    #[test]
    fn test_unit_compatibility() {
        assert!(units_compatible("m", "ft")); // Both length
        assert!(!units_compatible("m", "s")); // Length vs time
    }
    
    // Compound units
    #[test]
    fn test_compound_multiplication() {
        let result = multiply_units("5 m", "3 m");
        assert_eq!(result.value, 15.0);
        assert_eq!(result.unit, "m²");
    }
    
    #[test]
    fn test_unit_cancellation() {
        let result = divide_units("100 m", "50 m");
        assert_eq!(result.value, 2.0);
        assert_eq!(result.unit, Unit::Dimensionless);
    }
    
    // Conversion graph pathfinding
    #[test]
    fn test_conversion_chain() {
        // EUR → GBP via USD
        let path = graph.find_path("EUR", "GBP");
        assert_eq!(path.hops(), 1);
        assert_eq!(path.via(), vec!["USD"]);
    }
    
    // Formula evaluation
    #[test]
    fn test_formula_with_units() {
        let formula = "=A1*B1";
        let a1 = Cell { value: 10.0, unit: "USD/hr" };
        let b1 = Cell { value: 730.0, unit: "hr/month" };
        
        let result = evaluate_formula(formula, &cells);
        assert_eq!(result.value, 7300.0);
        assert_eq!(result.unit, "USD/month");
    }
}
```

### 10.3 Property-Based Testing

```rust
use proptest::prelude::*;

proptest! {
    // Conversion commutativity
    #[test]
    fn test_conversion_round_trip(value in 0.0..1000.0) {
        let converted = convert(value, "m", "ft");
        let back = convert(converted, "ft", "m");
        assert!((value - back).abs() < 0.0001);
    }
    
    // Associativity of addition with compatible units
    #[test]
    fn test_addition_associativity(
        a in 0.0..100.0,
        b in 0.0..100.0,
        c in 0.0..100.0
    ) {
        let ab_c = add(add(a, "m", b, "m"), c, "m");
        let a_bc = add(a, "m", add(b, "m", c, "m"));
        assert!((ab_c.value - a_bc.value).abs() < 0.0001);
    }
    
    // Dimensional consistency
    #[test]
    fn test_dimension_preservation(
        value in 0.0..1000.0,
        unit in length_units()
    ) {
        let cell = Cell { value, unit };
        assert_eq!(cell.dimension(), Dimension::Length);
    }
}
```

### 10.4 Integration Tests

```rust
#[test]
fn test_workbook_round_trip() {
    // Create workbook
    let mut wb = Workbook::new();
    wb.set_cell("A1", Cell { value: 100.0, unit: "USD" });
    wb.set_cell("A2", Cell { value: 85.0, unit: "EUR" });
    
    // Save to JSON
    wb.save("test.usheet")?;
    
    // Load from JSON
    let wb2 = Workbook::open("test.usheet")?;
    
    // Verify preservation
    assert_eq!(wb2.get_cell("A1").value, 100.0);
    assert_eq!(wb2.get_cell("A1").unit, "USD");
    assert_eq!(wb2.get_cell("A2").unit, "EUR");
}

#[test]
fn test_mcp_integration() {
    let wb = create_test_workbook();
    let mcp = SpreadsheetMCPServer::new(wb);
    
    // Test read operation
    let response = mcp.handle_request(json!({
        "method": "tools/call",
        "params": {
            "name": "read_cell",
            "arguments": { "cell_ref": "A1" }
        }
    }))?;
    
    assert_eq!(response["value"], 100.0);
    assert_eq!(response["unit"], "USD");
}

#[test]
fn test_sql_query_execution() {
    let wb = create_test_workbook_with_table();
    
    let result = wb.execute_query(
        "SELECT COUNT(*) FROM EC2Instances WHERE RAM > 16GB"
    )?;
    
    assert_eq!(result.rows.len(), 1);
    assert_eq!(result.rows[0]["COUNT(*)"].unit, "instances");
}
```

### 10.5 UI Testing

**Deferred to post-MVP for full E2E, but basic interaction tests:**

```rust
#[test]
fn test_cell_edit_interaction() {
    // Simulate: User types "100 m" in cell
    let input = "100 m";
    let candidates = autocomplete(input);
    
    // Should disambiguate meters vs minutes
    assert_eq!(candidates.len(), 2);
    assert!(candidates.contains(&Unit::Meters));
    assert!(candidates.contains(&Unit::Minutes));
    
    // User selects meters
    let selected = Unit::Meters;
    let cell = create_cell_from_input(100.0, selected);
    
    assert_eq!(cell.value, 100.0);
    assert_eq!(cell.unit, Unit::Meters);
}

#[test]
fn test_display_toggle() {
    let mut sheet = Sheet::new();
    sheet.add_cell("A1", Cell { value: 100.0, unit: Unit::Feet });
    
    // Initially in Imperial (no conversion)
    assert_eq!(sheet.display_value("A1"), "100 ft");
    
    // Toggle to Metric
    sheet.set_display_mode(UnitSystem::Metric);
    assert_eq!(sheet.display_value("A1"), "30.48 m");
    
    // Storage unchanged
    assert_eq!(sheet.get_cell("A1").unit, Unit::Feet);
}
```

---

## 11. MVP Scope

### 11.1 In-Scope Features

**Core Functionality:**
- ✅ Unit-aware cell storage (value, unit, formula)
- ✅ Built-in unit library (all domains: Tier 1 + Tier 2)
- ✅ Unit domain management (enable/disable per workbook)
- ✅ Custom units (embedded in workbook)
- ✅ Display toggle (Metric ↔ Imperial, workbook-level)
- ✅ Conversion graph with pathfinding
- ✅ All 4 conversion rate modes (Live Auto, Prompt on Open, As of Date, Manual)
- ✅ Conversion chain trust system
- ✅ CONVERT() function with warnings
- ✅ Soft warning system (orange cells, tooltips)
- ✅ Unit autocomplete with disambiguation

**Formula Engine:**
- ✅ Basic arithmetic with unit operations
- ✅ Automatic unit cancellation
- ✅ Compound unit creation (multiplication/division)
- ✅ Aggregation functions (SUM, AVERAGE, COUNT, MIN, MAX)
- ✅ Statistical functions (STDEV, VAR)
- ✅ Trigonometric functions (SIN, COS, TAN with angle units)
- ✅ Dependency tracking and recalculation
- ✅ Circular reference detection

**Table System:**
- ✅ Table definition with entity types and row units
- ✅ Column metadata (default_unit, display_as, value_type)
- ✅ Context-aware COUNT operations
- ✅ SQL query system (SELECT, WHERE, ORDER BY, LIMIT)
- ✅ Unit-aware filtering and comparisons
- ✅ GROUP BY and aggregates
- ✅ Query builder UI (visual + SQL editor)

**Validation:**
- ✅ Required fields
- ✅ Valid values (manual list)
- ✅ Valid values (SQL-driven dropdown)
- ✅ Value type classification
- ✅ Real-time validation feedback

**MCP Integration:**
- ✅ Internal MCP server (exposes spreadsheet to AI)
- ✅ Basic MCP tools (read_cell, write_cell, query_table)
- ✅ External MCP servers (currency, stocks)
- ✅ Custom MCP server configuration

**File Format:**
- ✅ Pure JSON (LLM-friendly, human-readable)
- ✅ In-memory SQLite (runtime performance)
- ✅ Excel export (one-way, with metadata sheets)
- ✅ Copy/paste with unit preservation

**UI:**
- ✅ Cell editor with autocomplete
- ✅ Display toggle button (ribbon)
- ✅ Warning indicators (orange cells, icons, tooltips)
- ✅ Domain management dialog
- ✅ Conversion rate management dialog
- ✅ Column configuration dialog
- ✅ Query builder
- ✅ Conversion rate indicators (🟢📅✏️⚠️)

**Documentation:**
- ✅ Code comments
- ✅ README with architecture
- ✅ Golden Workbook (self-documenting examples)
- ✅ Error tooltips only (no general tooltips)

**Localization:**
- ✅ i18n infrastructure (rust-i18n)
- ✅ English strings only
- ✅ Number/date formatting via locale

**Testing:**
- ✅ Unit tests (conversion correctness, dimensional analysis)
- ✅ Property-based tests (commutativity, associativity)
- ✅ Integration tests (round-trip, MCP, SQL)
- ✅ Golden Workbook validation

### 11.2 Out-of-Scope (Future Phases)

**Phase 2 (Post-MVP):**
- Sheet-level unit preferences (workbook-level only in MVP)
- SQLite hybrid file format (performance optimization)
- Excel import (reverse of export)
- Range constraints validation (min/max)
- Pattern matching validation (regex)
- Cross-column dependency validation
- Advanced MCP resource types
- Inline help panel
- Comprehensive user guide
- Video tutorials

**Phase 3 (Future):**
- Web-based multi-user version
- Real-time collaboration
- Shared unit library marketplace
- Natural language queries (full AI-powered)
- Advanced charting with unit-aware axes
- Solver with unit constraints
- Macro/scripting with unit awareness
- Database connectivity
- Mobile apps
- Plugin system

### 11.3 MVP Success Criteria

**Functional:**
- ✅ Can create workbooks with unit-aware cells
- ✅ Display toggle works without data loss
- ✅ Formulas calculate correctly with units
- ✅ Unit cancellation automatic and visible
- ✅ SQL queries work on tables
- ✅ MCP server exposes operations to AI
- ✅ Excel export produces viewable files
- ✅ Golden Workbook demonstrates all features

**Performance:**
- ✅ 10,000 cells load in <1 second
- ✅ Formula recalculation <200ms for 1000 cells
- ✅ Display toggle <100ms
- ✅ SQL queries <500ms for 10,000 rows
- ✅ MCP operations <1 second

**Quality:**
- ✅ No data loss on save/load
- ✅ Unit conversions accurate to machine precision
- ✅ Warnings display correctly
- ✅ No crashes on invalid input
- ✅ Test coverage >80% for core engine

**Usability:**
- ✅ New user can enter unit-aware data within 1 minute
- ✅ Display toggle discoverable (ribbon button)
- ✅ Autocomplete helps with unit entry
- ✅ Warnings understandable (clear tooltips)
- ✅ Golden Workbook teaches by example

---

## 12. Implementation Phases

### 12.1 Phase 0: Foundation (Weeks 1-2)

**Goal:** Project setup and core data structures

**Tasks:**
- Set up Tauri project
- Initialize Rust workspace structure
- Define core data models (Cell, Unit, Workbook)
- Set up i18n framework
- Create project README
- Set up CI/CD pipeline

**Deliverable:** Compilable project skeleton

### 12.2 Phase 1: Core Unit System (Weeks 3-5)

**Goal:** Unit representation and conversion

**Tasks:**
- Implement Unit struct with canonical form
- Build comprehensive unit library (all domains)
- Create dimension system
- Implement conversion graph
- Build pathfinding algorithm
- Unit parsing and disambiguation
- Property-based tests for conversions

**Deliverable:** Unit conversion engine with tests

### 12.3 Phase 2: Cell & Formula Engine (Weeks 6-9)

**Goal:** Unit-aware calculations

**Tasks:**
- Implement Cell struct with units
- Build formula parser
- Implement AST for formulas
- Unit-aware arithmetic operations
- Compound unit creation/cancellation
- Dependency graph tracking
- Recalculation engine
- Circular reference detection
- Basic functions (SUM, AVERAGE, COUNT)
- Warning system (incompatible units)

**Deliverable:** Working formula engine

### 12.4 Phase 3: Display & Conversion Modes (Weeks 10-12)

**Goal:** Display separation and conversion rates

**Tasks:**
- Display vs storage separation
- Display toggle implementation
- Conversion rate modes (all 4)
- Conversion chain trust system
- MCP client for currency/stock servers
- Conversion rate management UI
- Rate history tracking

**Deliverable:** Full conversion system

### 12.5 Phase 4: Table System & SQL (Weeks 13-16)

**Goal:** Structured data and queries

**Tasks:**
- In-memory SQLite setup
- Table metadata system
- Column validation rules
- SQL query parser (unit-aware)
- Query execution engine
- Context-aware COUNT
- Query builder UI
- Validation UI (dropdowns, warnings)

**Deliverable:** Queryable tables

### 12.6 Phase 5: UI Implementation (Weeks 17-21)

**Goal:** User interface

**Tasks:**
- Cell editor with autocomplete
- Ribbon with display toggle
- Warning display (orange cells, icons)
- Domain management dialog
- Column configuration dialog
- Conversion rate management
- Query builder
- Settings panels

**Deliverable:** Functional UI

### 12.7 Phase 6: File Format & I/O (Weeks 22-24)

**Goal:** Save/load and export

**Tasks:**
- JSON serialization
- Workbook save/load
- In-memory SQLite population
- Excel export implementation
- Copy/paste handling
- Undo/redo system
- File format tests

**Deliverable:** Persistent storage

### 12.8 Phase 7: MCP Integration (Weeks 25-27)

**Goal:** AI integration

**Tasks:**
- Internal MCP server setup
- Expose spreadsheet operations
- Resource URIs
- External MCP client
- Custom MCP configuration
- MCP tool implementations
- Integration tests

**Deliverable:** MCP-enabled spreadsheet

### 12.9 Phase 8: Testing & Polish (Weeks 28-30)

**Goal:** Quality and stability

**Tasks:**
- Create Golden Workbook
- Comprehensive testing
- Bug fixes
- Performance optimization
- Documentation
- Example workbooks
- Release preparation

**Deliverable:** MVP release candidate

### 12.10 Phase 9: Release (Week 31)

**Goal:** Public MVP

**Tasks:**
- Final testing
- Documentation review
- Create release notes
- Package for distribution
- Open source release
- Community announcement

**Deliverable:** MVP 1.0 released

---

## 13. Performance Requirements

### 13.1 Target Benchmarks

**Workbook Size:**
- MVP: 10,000 cells comfortable
- Graceful: 50,000 cells (slower but functional)
- Warning: >25,000 cells

**Calculation Speed:**
- Simple formulas: <50ms for 1,000 dependent cells
- Complex formulas: <200ms for 1,000 dependent cells
- Incremental recalc only

**Query Performance:**
- Simple SELECT: <100ms for 10,000 rows
- Complex GROUP BY: <500ms for 10,000 rows
- Timeout: 5 seconds

**UI Responsiveness:**
- Cell edit to display: <16ms (60 FPS)
- Sheet switch: <200ms
- Display toggle: <100ms
- Autocomplete: <50ms

**MCP Operations:**
- Conversion rate fetch: <2 seconds
- Cell read/write: <100ms
- Table query: <1 second

**File Operations:**
- Open: <1 second (1,000 cells)
- Save: <500ms
- Excel export: <2 seconds

### 13.2 Optimization Strategies

**Memory:**
- In-memory SQLite for runtime queries
- Lazy loading of non-visible cells
- Dependency graph pruning
- Conversion rate caching

**Computation:**
- Incremental recalculation (only changed)
- Parallel formula evaluation (where safe)
- Conversion path caching
- Compiled formula AST

**I/O:**
- Streaming JSON parsing
- Async file operations
- Incremental save (dirty cells only)
- Background autosave

---

## 14. Open Source Strategy

### 14.1 License

**Recommendation:** MIT or Apache 2.0
- Permissive for commercial use
- Encourages adoption
- Compatible with most ecosystems

### 14.2 Repository Structure

```
unit-aware-spreadsheet/
├── README.md
├── LICENSE
├── CONTRIBUTING.md
├── docs/
│   ├── architecture.md
│   ├── unit-system.md
│   ├── formula-language.md
│   └── mcp-integration.md
├── src/
│   ├── core/          # Rust calculation engine
│   ├── ui/            # Tauri frontend
│   ├── mcp/           # MCP server
│   └── formats/       # File I/O
├── tests/
│   ├── unit/
│   ├── integration/
│   └── golden_workbook/
├── examples/
│   └── cloud-cost-analysis.usheet
└── scripts/
    └── build.sh
```

### 14.3 Community Building

**Initial Launch:**
- GitHub release with MVP
- Blog post explaining innovation
- Demo video with Golden Workbook
- Hacker News/Reddit posts
- AI community outreach (MCP integration)

**Documentation:**
- Getting started guide
- Architecture overview
- API reference (MCP)
- Contributing guidelines
- Example workbooks

**Communication:**
- GitHub Discussions for Q&A
- Discord/Slack for community
- Regular blog updates
- Issue templates
- PR guidelines

### 14.4 Contribution Guidelines

**Welcome Contributions:**
- Bug fixes
- New unit definitions
- Formula functions
- UI improvements
- Documentation
- Example workbooks
- MCP server integrations

**Code Standards:**
- Rust formatting (rustfmt)
- Linting (clippy)
- Tests required for new features
- Documentation for public APIs
- Semantic versioning

---

## 15. Future Roadmap

### 15.1 Phase 2 Features (Post-MVP)

**Performance Optimization:**
- SQLite hybrid file format
- Incremental save
- Streaming large datasets
- WASM compilation for web

**Enhanced Validation:**
- Range constraints with units
- Pattern matching (regex)
- Cross-column dependencies
- Custom validation functions

**Advanced Queries:**
- JOIN across tables
- Subqueries
- Window functions
- Natural language queries (AI-powered)

**Collaboration:**
- Conflict resolution
- User permissions
- Shared workbook links

### 15.2 Phase 3 Features (Future)

**Web Version:**
- Browser-based application
- WebSocket for real-time sync
- Cloud storage integration
- Multi-user editing

**Advanced Features:**
- Charting with unit-aware axes
- Solver with unit constraints
- Pivot tables with units
- Conditional formatting with units

**Ecosystem:**
- Shared unit library marketplace
- Plugin system for extensions
- Integration with other tools
- Mobile apps (iOS/Android)

**Enterprise:**
- SSO/SAML authentication
- Audit logging
- Compliance features
- On-premise deployment

---

## 16. Conclusion

This unified design document provides a comprehensive blueprint for implementing a revolutionary unit-aware spreadsheet application. The system treats units as first-class data types, enabling intelligent calculations, seamless international collaboration, and AI-native integration.

**Key Innovations:**
1. **Units as Data:** Values stored as (number, unit) tuples
2. **Non-Destructive Display:** Conversion separate from storage
3. **Automatic Unit Cancellation:** Intelligent formula operations
4. **SQL-Queryable Tables:** Entity-aware structured data
5. **MCP Integration:** AI-native from the start
6. **Open Source & LLM-Friendly:** JSON format, comprehensive APIs

**Implementation Path:**
- 31-week development timeline
- Clear phase separation
- Testable milestones
- Golden Workbook for validation

**Success Metrics:**
- Performance targets met
- Feature completeness
- User adoption
- Community engagement

This project has the potential to transform how people work with quantitative data, bridging measurement systems and enabling new workflows through AI integration.

---

**Document Version:** 1.0  
**Last Updated:** 2025-10-05  
**Status:** Ready for Implementation  
**Next Steps:** Phase 0 - Foundation Setup

---

**Appendix A: Glossary**

- **Unit:** A measurement standard (meter, USD, hour)
- **Compound Unit:** Combination of units (mi/hr, USD/user)
- **Dimension:** Physical quantity type (Length, Mass, Time)
- **Conversion Graph:** Network of unit conversion paths
- **Display Unit:** How value is shown (non-destructive)
- **Storage Unit:** How value is stored (immutable)
- **Entity Type:** What table rows represent (EC2Instance, User)
- **Row Unit:** Unit for counting rows (instances, users)
- **MCP:** Model Context Protocol (AI integration standard)
- **Golden Workbook:** Test workbook exercising all features

---

**Appendix B: References**

- MCP Protocol: https://modelcontextprotocol.io
- ISO 4217 Currency Codes: https://www.iso.org/iso-4217-currency-codes.html
- SI Units: https://www.bipm.org/en/measurement-units
- Tauri: https://tauri.app
- SQLite: https://www.sqlite.org
- rust-i18n: https://github.com/longbridgeapp/rust-i18n

---

**End of Document**
