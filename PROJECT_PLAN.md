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

#### Week 22: Examples & Documentation
- [ ] 7.6: Create simple example workbook (basic conversions)
- [ ] 7.7: Create formula examples workbook
- [ ] 7.8: Create mixed units workbook (imperial + metric)
- [ ] 7.9: Write user guide (basic)
- [ ] 7.10: Create demo video

**Deliverable:** Well-tested app with examples

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
