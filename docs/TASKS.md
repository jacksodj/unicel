# Unicel - Task Checklist

> This file tracks all tasks. Mark [x] when complete. Auto-generated from PROJECT_PLAN.md

**Current Phase:** Phase 7 - Testing & Examples
**Week:** 21 of 24
**Updated:** 2025-10-13

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
- [ ] 7.1: Write unit tests for all core modules
- [ ] 7.2: Add property-based tests (conversion commutativity)
- [ ] 7.3: Create integration tests
- [ ] 7.4: Test edge cases (zero, negative, very large numbers)
- [ ] 7.5: Add error handling tests

### Week 22: Use Case Examples & Documentation
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

---

## Phase 8: Excel Export & Polish (Week 23)

### Week 23: Excel Export
- [ ] 8.1: Implement basic Excel export
- [ ] 8.2: Add metadata sheet (units, conversions)
- [ ] 8.3: Add warning in exported file
- [ ] 8.4: Test export with various workbooks
- [ ] 8.5: Final UI polish and bug fixes

---

## Phase 9: MLP Release (Week 24)

### Week 24: Release Preparation
- [ ] 9.1: Final testing on macOS
- [ ] 9.2: Test on Windows (if possible)
- [ ] 9.3: Update README with screenshots
- [ ] 9.4: Write release notes
- [ ] 9.5: Create GitHub release
- [ ] 9.6: Share on relevant communities (HN, Reddit)

---

## Progress Summary

- **Phase 0:** ✅ Complete (5/5 tasks)
- **Phase 1:** ✅ Complete (14/14 tasks)
- **Phase 2:** ✅ Complete (22/22 tasks)
- **Phase 3:** ✅ Complete (15/15 tasks)
- **Phase 4:** ✅ Complete (10/10 tasks)
- **Phase 5:** ✅ Complete (20/20 tasks)
- **Phase 6:** ✅ Complete (10/10 tasks)
- **Phase 7:** ⏸ Not Started (0/16 tasks)
- **Phase 8:** ⏸ Not Started (0/5 tasks)
- **Phase 9:** ⏸ Not Started (0/6 tasks)

**Overall Progress:** 96/132 tasks (72.7%)

---

## Future Backlog (Beyond MLP)

### Enhancements
- **Make "units in use" panel interactive** (Medium priority)
  - Allow users to interact with the units listed in the "units in use" panel in the Unit Preferences dialog
  - Possible features:
    - Click on a unit to set it as the preferred unit for that dimension
    - Show which units are currently set as preferred
    - Allow quick conversion tests by clicking units

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
