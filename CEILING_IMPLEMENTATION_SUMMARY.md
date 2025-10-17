# CEILING Function Implementation Summary

## Function Added: CEILING

### Signature
```
CEILING(number)
CEILING(number, significance)
```

### Behavior
The CEILING function rounds a number UP to the nearest integer or to the nearest multiple of significance.

**Examples:**
- `CEILING(4.3)` → 5
- `CEILING(-4.3)` → -4 (rounds toward zero)
- `CEILING(4.3, 0.5)` → 4.5
- `CEILING(12, 5)` → 15
- `CEILING(12.7, 5)` → 15

### Unit Handling

**Input Requirements:**
- First argument (`number`): Any numeric value with or without units
- Second argument (`significance`, optional): Numeric value, dimensionless or compatible units with first argument

**Output Units:**
- Result preserves the unit from the first argument (`number`)
- If `number` is dimensionless, result is dimensionless
- If `number` has units (e.g., meters, USD), result has the same units

**Unit Validation:**
- If both arguments have units, they must be compatible (same dimension)
  - ✓ Valid: `CEILING(100cm, 0.5m)` - both are length
  - ✗ Invalid: `CEILING(10m, 5kg)` - incompatible dimensions
- One argument can be dimensionless
  - ✓ Valid: `CEILING(12.7m, 5)` - significance is dimensionless
  - ✓ Valid: `CEILING(4.3, 0.5)` - both dimensionless

**Unit Conversion:**
- When significance has compatible units, it's automatically converted to match the number's units
- Example: `CEILING(100cm, 0.5m)` internally converts 0.5m to 50cm before calculation

### Implementation

**Files Modified:**

1. **src-tauri/src/core/formula/evaluator.rs**
   - Added `eval_function()` method to dispatch function calls
   - Added `eval_ceiling()` method implementing CEILING logic
   - Modified `Expr::Function` handling to call `eval_function()`

**Key Implementation Details:**

```rust
fn eval_ceiling(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
    // 1. Validate argument count (1 or 2)
    // 2. Evaluate number argument and extract value
    // 3. Get significance (default 1.0 if not provided)
    // 4. Check unit compatibility
    // 5. Convert significance to number's units if needed
    // 6. Perform ceiling: ceil(number / significance) * significance
    // 7. Return result with number's unit preserved
}
```

**Design Decisions:**
- Significance defaults to 1 for simple ceiling to integer
- Zero significance returns error (division by zero)
- Negative significance is allowed (mathematically correct)
- Unit compatibility checked before conversion
- Result always has the same unit as the input number

### Tests Added

**Test File:** `src-tauri/tests/ceiling_function.rs`

**Test Coverage (21 tests):**

1. **Basic Functionality:**
   - `test_ceiling_basic_positive` - Simple positive numbers
   - `test_ceiling_basic_negative` - Negative numbers (toward zero)
   - `test_ceiling_already_integer` - Numbers already at ceiling
   - `test_ceiling_zero` - Zero input

2. **With Significance:**
   - `test_ceiling_with_significance` - Decimal significance
   - `test_ceiling_with_integer_significance` - Integer significance
   - `test_ceiling_with_negative_significance` - Negative significance

3. **Unit Handling:**
   - `test_ceiling_preserves_units` - Units preserved in output
   - `test_ceiling_with_units_and_dimensionless_significance` - Mixed units
   - `test_ceiling_with_compatible_units` - Unit conversion
   - `test_ceiling_with_incompatible_units` - Error case
   - `test_ceiling_currency_units` - Currency-specific test
   - `test_ceiling_temperature_units` - Temperature units

4. **Error Cases:**
   - `test_ceiling_zero_significance_error` - Division by zero
   - `test_ceiling_no_arguments_error` - Missing arguments
   - `test_ceiling_too_many_arguments_error` - Too many arguments
   - `test_ceiling_text_argument_error` - Type mismatch

5. **Edge Cases:**
   - `test_ceiling_very_small_numbers` - Precision handling
   - `test_ceiling_large_numbers` - Large number handling

6. **Integration:**
   - `test_ceiling_in_expression` - Used in formulas
   - `test_ceiling_nested` - Nested function calls

### Verification

**All Tests Pass:**
```
running 21 tests
test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

**Code Quality:**
- ✓ All tests pass
- ✓ Clippy clean (no warnings with `-D warnings`)
- ✓ Properly formatted (`cargo fmt`)
- ✓ Unit handling correct
- ✓ Error messages clear
- ✓ Comprehensive test coverage

**Integration Verified:**
- ✓ Workbook integration tests pass
- ✓ Error handling tests pass
- ✓ Library builds successfully
- ✓ No regressions in existing functionality

### Example Usage

**Basic ceiling:**
```
=CEILING(4.3)           → 5
```

**Rounding to increments:**
```
=CEILING(12.34USD, 0.25USD)    → 12.50USD    (quarter rounding)
=CEILING(1.7km, 500m)          → 2km          (500m increments)
=CEILING(123.45USD, 100USD)    → 200USD       (hundred rounding)
```

**In formulas:**
```
=CEILING(A1 * B1, 0.5)         // Round product to nearest 0.5
=CEILING(price, 0.99USD)       // Price ending in .99
=CEILING(hours, 0.25hr)        // Quarter-hour billing
```

### Documentation

**Created:**
- `/Users/dennisjackson/Code/unicel/examples/ceiling_function_examples.md` - Comprehensive usage examples and documentation

**Includes:**
- Syntax reference
- Basic usage examples
- Unit-aware examples
- Error case documentation
- Practical use cases (budget planning, inventory, time rounding)
- Comparison with related functions (FLOOR, ROUND)

### Future Enhancements

**Potential additions:**
- FLOOR function (rounds down)
- ROUND function (rounds to nearest)
- ROUNDUP/ROUNDDOWN functions
- TRUNC function (truncates to integer)
- MROUND function (rounds to nearest multiple)

**Function framework established:**
The implementation creates a foundation for adding more formula functions:
1. `eval_function()` dispatcher is ready for new functions
2. Pattern established for unit-aware function implementation
3. Test patterns established for comprehensive coverage

### Notes

- Function names are case-insensitive (CEILING, Ceiling, ceiling all work)
- Compatible with Excel's CEILING function behavior
- Extends Excel with unit-aware capabilities
- Follows Unicel's principle of "units as data, not formatting"
- No breaking changes to existing code
- All existing tests continue to pass
