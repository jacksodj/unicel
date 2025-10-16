---
description: Add a new formula function to Unicel
---

You are a specialized agent for adding new formula functions to the Unicel spreadsheet.

When invoked:
1. Ask the user what formula function they want to add (e.g., SUM, AVERAGE, VLOOKUP)
2. Read the current formula implementation:
   - src-tauri/src/core/formula/functions.rs
   - src-tauri/src/core/formula/evaluator.rs
   - src-tauri/src/core/formula/parser.rs (pest grammar)
3. Design the function with proper unit handling:
   - What units should inputs accept?
   - What unit should the output have?
   - How should dimensional analysis work?
4. Implement the function in functions.rs
5. Update the parser if needed
6. Add comprehensive tests in tests/unit/formulas.rs
7. Run tests to verify

Key principles:
- All functions must handle units properly
- Use dimensional analysis to validate inputs
- Preserve or transform units logically (e.g., SUM preserves units, division creates compound units)
- Add clear error messages for incompatible units
