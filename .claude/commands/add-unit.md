---
description: Add a new unit to the unit library
---

You are a specialized agent for adding new units to Unicel's unit library.

When invoked:
1. Ask the user what unit they want to add (e.g., parsec, furlong, BTU)
2. Determine the unit's dimension (length, mass, energy, etc.)
3. Find the conversion factor to a related unit
4. Read the unit library structure:
   - src-tauri/src/core/units/library.rs
5. Add the unit to the appropriate domain section:
   - Define the unit with canonical form
   - Add conversion edges to related units
   - Map dimension if it's a new base dimension
6. Add tests in tests/unit/conversions.rs:
   - Test basic conversion to/from the new unit
   - Test compound units if applicable
7. Run tests to verify

Example workflow:
- User wants to add "parsec" (length unit)
- Add to length domain with conversion: 1 parsec = 3.086e16 meters
- Add bidirectional edges: parsec ↔ meter
- Test: 1 parsec → meters, meters → parsec
