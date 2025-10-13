# Unicel - Project Plan & Task Tracking

## MLP (Minimum Lovable Product) Strategy

The MLP focuses on delivering the core innovation with enough polish to be genuinely useful and demonstrate the value proposition.

### MLP Success Criteria

**User can:**
1. Create a workbook with unit-aware cells
2. Enter values with units (e.g., "100 USD", "5 meters")
3. See automatic unit conversion (Metric ↔ Imperial toggle)
4. Write formulas that preserve units (e.g., `=A1*2` where A1=100ft → 200ft)
5. Experience automatic unit cancellation (e.g., `100mi / 2hr → 50 mi/hr`)
6. Save and load workbooks
7. Get helpful warnings for incompatible operations
8. Export to Excel for sharing

**NOT included in MLP:**
- SQL queries (can be added later)
- MCP integration (Phase 2)
- Complex UI features (Phase 2)
- Advanced conversion modes (As of Date, etc.)
- Custom units (Phase 2)

---

## Implementation Phases

### ✅ Phase 0: Foundation (COMPLETED)
- [x] Project setup
- [x] Technology stack selection
- [x] Folder structure
- [x] Build configuration
- [x] Git repository and GitHub setup

### 🚧 Phase 1: Core Unit System (Current Phase - Weeks 3-5)

**Goal:** Basic unit representation and conversion

#### Week 3: Unit Fundamentals
- [ ] 1.1: Define Unit struct with canonical form
- [ ] 1.2: Implement Dimension enum (Simple dimensions only for MLP)
- [ ] 1.3: Create BaseDimension enum (Length, Mass, Time, Temperature, Currency)
- [ ] 1.4: Implement unit parsing (basic, no disambiguation yet)
- [ ] 1.5: Write unit equality tests

#### Week 4: Unit Library
- [ ] 1.6: Build unit library with Tier 1 units:
  - [ ] Length: m, cm, km, ft, yd, mi
  - [ ] Mass: g, kg, lb, oz
  - [ ] Time: s, min, hr, day
  - [ ] Temperature: C, F, K
  - [ ] Currency: USD, EUR, GBP
- [ ] 1.7: Define conversion factors (hardcoded for MLP)
- [ ] 1.8: Create unit library tests
- [ ] 1.9: Implement unit.to_string() for display

#### Week 5: Basic Conversion System
- [ ] 1.10: Create ConversionGraph struct
- [ ] 1.11: Implement direct conversion lookup
- [ ] 1.12: Add conversion between compatible units
- [ ] 1.13: Test conversion accuracy
- [ ] 1.14: Implement storage vs display separation (basic)

**Deliverable:** Can convert between basic units, tested and accurate

---

### Phase 2: Cell & Formula Engine (Weeks 6-9)

**Goal:** Unit-aware calculations

#### Week 6: Cell Structure
- [ ] 2.1: Implement Cell struct with all fields
- [ ] 2.2: Add cell creation and modification methods
- [ ] 2.3: Implement cell display logic
- [ ] 2.4: Create cell tests
- [ ] 2.5: Add CellValue type for calculations

#### Week 7: Formula Parser (Simplified)
- [ ] 2.6: Define formula grammar with pest (basic arithmetic)
- [ ] 2.7: Implement tokenizer
- [ ] 2.8: Build basic AST (Add, Subtract, Multiply, Divide, CellRef)
- [ ] 2.9: Write parser tests
- [ ] 2.10: Add literal unit support in formulas

#### Week 8: Unit-Aware Operations
- [ ] 2.11: Implement Add operation (compatible units)
- [ ] 2.12: Implement Subtract operation
- [ ] 2.13: Implement Multiply operation (compound units)
- [ ] 2.14: Implement Divide operation (compound units + cancellation)
- [ ] 2.15: Add warning system for incompatible operations
- [ ] 2.16: Test all operations extensively

#### Week 9: Formula Evaluation & Dependencies
- [ ] 2.17: Implement formula evaluator
- [ ] 2.18: Create dependency graph structure
- [ ] 2.19: Implement dependency tracking
- [ ] 2.20: Add circular reference detection
- [ ] 2.21: Implement recalculation engine
- [ ] 2.22: Add basic functions (SUM, AVERAGE)

**Deliverable:** Formulas work with units, automatic cancellation

---

### Phase 3: Basic Workbook & Sheet (Weeks 10-12)

**Goal:** Multi-cell structure with display toggle

#### Week 10: Sheet Structure
- [ ] 3.1: Implement Sheet struct
- [ ] 3.2: Add cell storage (HashMap for MLP)
- [ ] 3.3: Implement get/set cell operations
- [ ] 3.4: Add cell range support
- [ ] 3.5: Create sheet tests

#### Week 11: Workbook Management
- [ ] 3.6: Implement Workbook struct
- [ ] 3.7: Add sheet management (add/remove/rename)
- [ ] 3.8: Implement workbook settings
- [ ] 3.9: Add display preference (Metric/Imperial)
- [ ] 3.10: Create workbook tests

#### Week 12: Display Conversion
- [ ] 3.11: Implement display unit conversion
- [ ] 3.12: Add toggle mechanism (Metric ↔ Imperial)
- [ ] 3.13: Ensure storage units unchanged
- [ ] 3.14: Test display conversion thoroughly
- [ ] 3.15: Add conversion indicator in display

**Deliverable:** Working workbook with display toggle

---

### Phase 4: File Format (Weeks 13-14)

**Goal:** Save/load workbooks

#### Week 13: JSON Serialization
- [ ] 4.1: Implement JSON serialization for Cell
- [ ] 4.2: Implement JSON serialization for Sheet
- [ ] 4.3: Implement JSON serialization for Workbook
- [ ] 4.4: Add version metadata
- [ ] 4.5: Create .usheet file format handler

#### Week 14: File I/O
- [ ] 4.6: Implement workbook save
- [ ] 4.7: Implement workbook load
- [ ] 4.8: Add error handling for corrupt files
- [ ] 4.9: Test round-trip (save → load → verify)
- [ ] 4.10: Implement dirty flag tracking

**Deliverable:** Can save and load workbooks

---

### Phase 5: Basic UI (Weeks 15-18)

**Goal:** Functional spreadsheet interface

#### Week 15: Grid Component
- [ ] 5.1: Create basic grid component (HTML table for MLP)
- [ ] 5.2: Implement cell rendering
- [ ] 5.3: Add cell selection
- [ ] 5.4: Implement scroll behavior
- [ ] 5.5: Show cell values with units

#### Week 16: Cell Editing
- [ ] 5.6: Implement cell editor
- [ ] 5.7: Add input parsing (value + unit)
- [ ] 5.8: Show edit vs display mode
- [ ] 5.9: Handle Enter/Escape keys
- [ ] 5.10: Add formula bar

#### Week 17: Ribbon & Controls
- [ ] 5.11: Create ribbon component
- [ ] 5.12: Add display toggle button (Metric/Imperial)
- [ ] 5.13: Add file menu (New, Open, Save, Save As)
- [ ] 5.14: Implement status bar
- [ ] 5.15: Add unit indicator icons

#### Week 18: UI Polish
- [ ] 5.16: Add warning indicators (orange cells)
- [ ] 5.17: Implement tooltips for warnings
- [ ] 5.18: Add loading states
- [ ] 5.19: Implement error messages
- [ ] 5.20: Style with Tailwind (basic theme)

**Deliverable:** Usable spreadsheet UI

---

### Phase 6: Tauri Integration (Weeks 19-20)

**Goal:** Desktop app with file system access

#### Week 19: Tauri Commands
- [ ] 6.1: Create Tauri command for creating workbook
- [ ] 6.2: Create Tauri command for opening file
- [ ] 6.3: Create Tauri command for saving file
- [ ] 6.4: Create Tauri command for cell operations
- [ ] 6.5: Add proper error handling

#### Week 20: Integration & Testing
- [ ] 6.6: Connect frontend to Tauri backend
- [ ] 6.7: Implement state management with Zustand
- [ ] 6.8: Add file dialogs (open/save)
- [ ] 6.9: Test full workflow (create → edit → save → open)
- [ ] 6.10: Handle app lifecycle (unsaved changes warning)

**Deliverable:** Working desktop application

---

### Phase 7: Testing & Examples (Weeks 21-22)

**Goal:** Quality assurance and examples

#### Week 21: Comprehensive Testing
- [ ] 7.1: Write unit tests for all core modules
- [ ] 7.2: Add property-based tests (conversion commutativity)
- [ ] 7.3: Create integration tests
- [ ] 7.4: Test edge cases (zero, negative, very large numbers)
- [ ] 7.5: Add error handling tests

#### Week 22: Use Case Examples & Documentation
- [ ] 7.6: Create Construction Estimator example workbook
  - [ ] Material list with dimensional calculations (sqft, board feet)
  - [ ] Cost calculations with automatic unit cancellation
  - [ ] Metric/Imperial display toggle demonstration
  - [ ] Notes explaining key formulas
- [ ] 7.7: Create AWS Cost Estimator example workbook
  - [ ] EC2/RDS instance pricing with compound units
  - [ ] Data transfer calculations (GB/mo × $/GB)
  - [ ] Multi-region comparison (USD vs EUR)
  - [ ] Scaling scenario projections
- [ ] 7.8: Create Investment Portfolio Tracker example workbook
  - [ ] Stock positions with shares and cost basis
  - [ ] Multi-currency holdings (USD, EUR, GBP)
  - [ ] Return calculations with proper unit handling
  - [ ] Asset allocation summary
- [ ] 7.9: Create basic tutorial workbook (unit conversion primer)
- [ ] 7.10: Write user guide with screenshots
- [ ] 7.11: Create demo video showcasing all three examples

**Deliverable:** Well-tested app with professional example workbooks

---

### Phase 8: Excel Export & Polish (Week 23)

**Goal:** Interoperability and final touches

#### Week 23: Excel Export
- [ ] 8.1: Implement basic Excel export
- [ ] 8.2: Add metadata sheet (units, conversions)
- [ ] 8.3: Add warning in exported file
- [ ] 8.4: Test export with various workbooks
- [ ] 8.5: Final UI polish and bug fixes

**Deliverable:** Can export to Excel

---

### Phase 9: MLP Release (Week 24)

**Goal:** Public release

#### Week 24: Release Preparation
- [ ] 9.1: Final testing on macOS
- [ ] 9.2: Test on Windows (if possible)
- [ ] 9.3: Update README with screenshots
- [ ] 9.4: Write release notes
- [ ] 9.5: Create GitHub release
- [ ] 9.6: Share on relevant communities (HN, Reddit)

**Deliverable:** MLP v0.1.0 released

---

## Post-MLP Features (Phase 10+)

These are valuable features for subsequent releases:

### Phase 10: Enhanced Conversions
- Multi-hop conversion pathfinding
- Conversion chain trust system
- Custom units
- MCP integration for live rates

### Phase 11: Table System
- Table metadata
- SQL queries
- Column validation
- Context-aware COUNT

### Phase 12: Advanced Formula Functions
- Statistical functions
- Date/time functions
- Lookup functions
- Text functions

### Phase 13: UI Enhancements
- Unit autocomplete with disambiguation
- Domain management
- Conversion rate management
- Query builder

### Phase 14: MCP Integration
- Internal MCP server
- External MCP clients
- AI assistant integration

---

## Example Workbooks for MLP Launch

The three use case example workbooks will demonstrate real-world applications of the unit-aware system to new users. These simplified versions focus on core functionality (no MCP, no live data) but show the key value propositions.

### 1. Construction Estimator (`construction-estimator.usheet`)

**Purpose:** Show dimensional analysis and automatic unit handling

**Key Features Demonstrated:**
- **Area calculations:** `18ft * 9ft = 162 sqft` (automatic unit multiplication)
- **Volume calculations:** `450 sqft * 4in = 150 cuft` (dimension handling)
- **Unit cancellation:** `935 sqft * $4.80/sqft = $4,488` (sqft cancels)
- **Waste factors:** `850 sqft * 1.10 = 935 sqft` (dimensionless multipliers)
- **Metric/Imperial toggle:** Show same materials in feet vs meters
- **Mixed units:** `18ft 6in` properly handled

**Sample Data:**
```
Sheet: Material Estimate
├─ 2x4 Studs: 145 pieces @ $4.25/piece = $616.25
├─ Drywall: 42 sheets @ $12.75/sheet = $535.50
├─ Flooring: 850 sqft @ $4.80/sqft = $4,080.00
├─ Concrete: 5.56 cuyd @ $120/cuyd = $667.20
└─ Total: $12,847.50

Sheet: Room Dimensions
├─ Living Room: 18ft × 14ft × 9ft = 576 sqft walls
├─ Kitchen: 12ft × 10ft × 9ft = 396 sqft walls
└─ Formulas show automatic dimensional analysis
```

**Formulas to Include:**
- `=Length * Width` → sqft
- `=Area * Depth` → cuft
- `=Volume / 27` → cuyd (cu ft to cu yd conversion)
- `=Price/sqft * Area` → total cost

### 2. AWS Cost Estimator (`aws-cost-estimator.usheet`)

**Purpose:** Show compound units and complex calculations

**Key Features Demonstrated:**
- **Compound units:** `GB/mo`, `$/GB`, `requests/mo`
- **Unit cancellation:** `2000 GB/mo * $0.085/GB = $170/mo` (GB cancels)
- **Nested compound units:** `50M requests/mo * $0.20/M requests = $10/mo`
- **Multi-region pricing:** EUR vs USD with conversion factor
- **Per-unit cost calculations:** `$550.66/mo / 10,000 users = $0.055/user/mo`
- **Scaling projections:** Show cost at 1x, 5x, 10x scale

**Sample Data:**
```
Sheet: Current Infrastructure
├─ EC2 (4× t3.medium): $121.48/mo
├─ RDS (db.r6g.large): $182.50/mo
├─ S3 Storage: 500GB @ $0.023/GB-mo = $11.50/mo
├─ CloudFront: 2000 GB/mo @ $0.085/GB = $170.00/mo
├─ Lambda: 50M requests/mo @ $0.20/M = $10.00/mo
└─ Total: $495.48/mo

Sheet: Scaling Scenarios
├─ Current (1,000 users): $495.48/mo = $0.50/user/mo
├─ 5x Scale (5,000 users): $1,247.40/mo = $0.25/user/mo
├─ 10x Scale (10,000 users): $2,482.50/mo = $0.25/user/mo
└─ Formulas show automatic scaling calculations

Sheet: Multi-Region Comparison
├─ US East (Virginia): $495.48/mo
├─ EU West (Ireland): €468.50/mo = $505.98/mo (@ 1.08 EUR/USD)
├─ Asia Pacific (Mumbai): $445.74/mo (-10% cheaper)
└─ Currency conversion formulas with exchange rates
```

**Formulas to Include:**
- `=Quantity * Price_per_unit` (with various compound units)
- `=Data_GB * Transfer_rate_$/GB`
- `=Requests * Price_per_million_requests / 1M`
- `=Total_cost / Number_of_users`

### 3. Investment Portfolio Tracker (`investment-portfolio.usheet`)

**Purpose:** Show financial calculations and multi-currency

**Key Features Demonstrated:**
- **Share arithmetic:** `150 shares * $180.23/share = $27,034.50`
- **Return calculations:** `(Current - Cost) / Cost = %`
- **Multi-currency:** EUR and GBP positions converted to USD
- **Compound return units:** `$/share`, `shares`, `%/year`
- **Dividend calculations:** `shares * $/share = $`
- **Asset allocation percentages:** `Position_value / Total_value`

**Sample Data:**
```
Sheet: Holdings
├─ AAPL: 150 shares @ $180.23/share = $27,034.50 (+23.9%)
├─ MSFT: 85 shares @ $378.85/share = $32,202.25 (+35.3%)
├─ VOW3.DE: 100 shares @ €92.30/share = $9,968.40 (@ 1.08 USD/EUR)
├─ 7203.T: 500 shares @ ¥2,140/share = $7,163.33 (@ 150 JPY/USD)
└─ Total: $76,368.48 (+21.7%)

Sheet: Dashboard
├─ Total Value: $76,368.48
├─ Total Invested: $62,764.00
├─ Total Gain: $13,604.48 (+21.7%)
└─ Asset Allocation: 35.4% US, 10.4% Intl, 54.2% Crypto

Sheet: Performance
├─ Best Performer: MSFT +35.3%
├─ Worst Performer: GLD +4.0%
├─ YTD Return: +18.2%
└─ Formulas show proper return calculations
```

**Formulas to Include:**
- `=Shares * Price_per_share`
- `=(Current_value - Cost_basis) / Cost_basis` (return %)
- `=Foreign_price * Exchange_rate` (multi-currency)
- `=Position_value / Total_portfolio_value` (allocation %)
- `=Dividend_per_share * Shares` (dividend income)

### Example Workbook Implementation Notes

**For Week 22 Implementation:**
1. **Start simple:** Use hardcoded values (no live data, no MCP)
2. **Add notes/comments:** Explain what each formula demonstrates
3. **Visual polish:** Use cell colors to highlight key calculations
4. **Progressive complexity:** Start with simple formulas, build up
5. **Show errors:** Include 1-2 examples of incompatible unit warnings
6. **Documentation:** Each workbook should have a "README" sheet explaining it

**File Locations:**
```
examples/
├── construction-estimator.usheet
├── aws-cost-estimator.usheet
├── investment-portfolio.usheet
└── unit-conversion-tutorial.usheet
```

**README Sheet Template:**
```
=== Example: [Name] ===

Purpose: [What this demonstrates]

Key Features:
1. [Feature 1]
2. [Feature 2]
3. [Feature 3]

How to Use:
1. [Step 1]
2. [Step 2]
3. [Step 3]

Try This:
- [Exercise 1]
- [Exercise 2]
- [Exercise 3]
```

---

## Daily Development Workflow

1. **Start of day:**
   - Review PROJECT_PLAN.md
   - Pick 1-3 tasks from current week
   - Update task status

2. **During development:**
   - Focus on one task at a time
   - Write tests alongside code
   - Commit frequently with descriptive messages

3. **End of day:**
   - Mark completed tasks as done
   - Commit and push changes
   - Plan next day's tasks

4. **End of week:**
   - Review week's progress
   - Adjust timeline if needed
   - Celebrate completed milestones!

---

## Current Status

**Phase:** Phase 1 - Core Unit System (Week 3)
**Next Task:** 1.1 - Define Unit struct with canonical form
**Target Completion:** Week 24 (24 weeks from now)

---

## Risk Management

### Technical Risks
- **Risk:** Performance issues with large workbooks
  - **Mitigation:** Start with 1000 cell limit, optimize later

- **Risk:** Complex unit parsing edge cases
  - **Mitigation:** Use simple whitelist for MLP, expand later

- **Risk:** Cross-platform build issues
  - **Mitigation:** Focus on macOS first, Windows/Linux after MLP

### Scope Risks
- **Risk:** Feature creep
  - **Mitigation:** Strict MLP scope, defer everything else

- **Risk:** Over-engineering
  - **Mitigation:** Simple implementations first, refactor later

---

## Success Metrics

**MLP Success:**
- Can handle 1000+ cells
- All core unit operations work correctly
- Zero crashes in normal usage
- 80%+ test coverage on core engine
- 3+ users can use it without documentation
- Positive feedback on core innovation

---

## Notes

- Commit after every completed task
- Write tests BEFORE marking task complete
- Update CLAUDE.md if architecture changes
- Ask for help when stuck >2 hours
- Celebrate small wins!
