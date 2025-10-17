// Formula evaluator with unit-aware operations

use super::ast::Expr;
use crate::core::units::{parse_unit, BaseDimension, Dimension, Unit, UnitLibrary};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EvalError {
    #[error("Incompatible units: cannot {operation} {left} and {right}")]
    IncompatibleUnits {
        operation: String,
        left: String,
        right: String,
    },

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Cell reference not found: {0}")]
    CellNotFound(String),

    #[error("Named reference not found: {0}")]
    NamedRefNotFound(String),

    #[error("Unknown unit: {0}")]
    UnknownUnit(String),

    #[error("Function not implemented: {0}")]
    FunctionNotImplemented(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

/// Result of evaluating an expression
#[derive(Debug, Clone, PartialEq)]
pub struct EvalResult {
    /// The computed value (either number or text)
    pub value: EvalValue,

    /// The unit of the result (only applicable for numeric values)
    pub unit: Unit,

    /// Optional warning message (for incompatible but allowed operations)
    pub warning: Option<String>,
}

/// The value type for evaluation results
#[derive(Debug, Clone, PartialEq)]
pub enum EvalValue {
    Number(f64),
    Text(String),
}

impl EvalResult {
    pub fn new(value: f64, unit: Unit) -> Self {
        Self {
            value: EvalValue::Number(value),
            unit,
            warning: None,
        }
    }

    pub fn text(value: String) -> Self {
        Self {
            value: EvalValue::Text(value),
            unit: Unit::dimensionless(),
            warning: None,
        }
    }

    pub fn with_warning(mut self, warning: String) -> Self {
        self.warning = Some(warning);
        self
    }

    pub fn as_number(&self) -> Option<f64> {
        match self.value {
            EvalValue::Number(n) => Some(n),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&str> {
        match &self.value {
            EvalValue::Text(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_text(&self) -> bool {
        matches!(self.value, EvalValue::Text(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self.value, EvalValue::Number(_))
    }

    /// Get the numeric value, unwrapping if it's a number (for internal use where we validated it's a number)
    pub fn numeric_value(&self) -> f64 {
        match self.value {
            EvalValue::Number(n) => n,
            EvalValue::Text(_) => panic!("Expected number, got text"),
        }
    }
}

/// Unit-aware formula evaluator
pub struct Evaluator<'a> {
    library: &'a UnitLibrary,
}

impl<'a> Evaluator<'a> {
    pub fn new(library: &'a UnitLibrary) -> Self {
        Self { library }
    }

    /// Evaluate an expression (without cell references)
    pub fn eval(&self, expr: &Expr) -> Result<EvalResult, EvalError> {
        match expr {
            Expr::Number(n) => Ok(EvalResult::new(*n, Unit::dimensionless())),

            Expr::NumberWithUnit { value, unit } => {
                // Parse the unit (supports both simple and compound units)
                let unit_obj = parse_unit(unit, self.library)
                    .map_err(|_| EvalError::UnknownUnit(unit.clone()))?;
                Ok(EvalResult::new(*value, unit_obj))
            }

            Expr::String(s) => Ok(EvalResult::text(s.clone())),

            Expr::Add(left, right) => self.eval_add(left, right),
            Expr::Subtract(left, right) => self.eval_subtract(left, right),
            Expr::Multiply(left, right) => self.eval_multiply(left, right),
            Expr::Divide(left, right) => self.eval_divide(left, right),

            Expr::Negate(expr) => {
                let result = self.eval(expr)?;
                match result.value {
                    EvalValue::Number(n) => Ok(EvalResult::new(-n, result.unit)),
                    EvalValue::Text(_) => Err(EvalError::InvalidOperation(
                        "Cannot negate a text value".to_string(),
                    )),
                }
            }

            Expr::CellRef { .. } => Err(EvalError::CellNotFound(
                "Cell references not supported in standalone evaluation".to_string(),
            )),

            Expr::NamedRef { name } => Err(EvalError::NamedRefNotFound(format!(
                "Named reference '{}' not supported in standalone evaluation",
                name
            ))),

            Expr::Range { .. } => Err(EvalError::InvalidOperation(
                "Ranges can only be used in functions".to_string(),
            )),

            Expr::Function { name, args } => self.eval_function(name, args),

            Expr::Boolean(b) => Ok(EvalResult::new(
                if *b { 1.0 } else { 0.0 },
                Unit::dimensionless(),
            )),

            Expr::GreaterThan(_left, _right)
            | Expr::LessThan(_left, _right)
            | Expr::GreaterOrEqual(_left, _right)
            | Expr::LessOrEqual(_left, _right)
            | Expr::Equal(_left, _right)
            | Expr::NotEqual(_left, _right) => Err(EvalError::InvalidOperation(
                "Comparison operators not supported in standalone evaluation".to_string(),
            )),

            Expr::And(_left, _right) | Expr::Or(_left, _right) => Err(EvalError::InvalidOperation(
                "Logical operators not supported in standalone evaluation".to_string(),
            )),

            Expr::Not(_expr) => Err(EvalError::InvalidOperation(
                "NOT operator not supported in standalone evaluation".to_string(),
            )),
        }
    }

    /// Add two values (requires compatible units) or concatenate strings
    /// Uses finer unit alignment: converts both operands to the finer unit scale
    fn eval_add(&self, left: &Expr, right: &Expr) -> Result<EvalResult, EvalError> {
        let left_result = self.eval(left)?;
        let right_result = self.eval(right)?;

        // If either operand is text, perform string concatenation
        if left_result.is_text() || right_result.is_text() {
            let left_str = match &left_result.value {
                EvalValue::Text(s) => s.clone(),
                EvalValue::Number(n) => {
                    if left_result.unit.is_dimensionless() {
                        n.to_string()
                    } else {
                        format!("{} {}", n, left_result.unit)
                    }
                }
            };
            let right_str = match &right_result.value {
                EvalValue::Text(s) => s.clone(),
                EvalValue::Number(n) => {
                    if right_result.unit.is_dimensionless() {
                        n.to_string()
                    } else {
                        format!("{} {}", n, right_result.unit)
                    }
                }
            };
            return Ok(EvalResult::text(format!("{}{}", left_str, right_str)));
        }

        // Both are numbers - proceed with numeric addition
        let left_value = left_result.as_number().unwrap();
        let right_value = right_result.as_number().unwrap();

        // Both dimensionless - simple addition
        if left_result.unit.is_dimensionless() && right_result.unit.is_dimensionless() {
            return Ok(EvalResult::new(
                left_value + right_value,
                Unit::dimensionless(),
            ));
        }

        // Check if units are compatible
        if !left_result.unit.is_compatible(&right_result.unit) {
            return Err(EvalError::IncompatibleUnits {
                operation: "add".to_string(),
                left: left_result.unit.to_string(),
                right: right_result.unit.to_string(),
            });
        }

        // If units are exactly the same, just add
        if left_result.unit.is_equal(&right_result.unit) {
            return Ok(EvalResult::new(
                left_value + right_value,
                left_result.unit.clone(),
            ));
        }

        // Units are compatible but different - determine finer unit and convert both
        let left_canonical = left_result.unit.canonical();
        let right_canonical = right_result.unit.canonical();

        // Find finer unit
        let finer_unit = self
            .library
            .get_finer_unit(left_canonical, right_canonical)
            .ok_or_else(|| EvalError::IncompatibleUnits {
                operation: "add".to_string(),
                left: left_result.unit.to_string(),
                right: right_result.unit.to_string(),
            })?;

        // Convert both values to finer unit
        let left_converted = if left_canonical == finer_unit {
            left_value
        } else {
            self.library
                .convert(left_value, left_canonical, finer_unit)
                .ok_or_else(|| EvalError::IncompatibleUnits {
                    operation: "add".to_string(),
                    left: left_result.unit.to_string(),
                    right: right_result.unit.to_string(),
                })?
        };

        let right_converted = if right_canonical == finer_unit {
            right_value
        } else {
            self.library
                .convert(right_value, right_canonical, finer_unit)
                .ok_or_else(|| EvalError::IncompatibleUnits {
                    operation: "add".to_string(),
                    left: left_result.unit.to_string(),
                    right: right_result.unit.to_string(),
                })?
        };

        // Get the finer unit object
        let result_unit = self
            .library
            .get(finer_unit)
            .cloned()
            .unwrap_or_else(|| left_result.unit.clone());

        Ok(EvalResult::new(
            left_converted + right_converted,
            result_unit,
        ))
    }

    /// Subtract two values (requires compatible units)
    /// Uses finer unit alignment: converts both operands to the finer unit scale
    fn eval_subtract(&self, left: &Expr, right: &Expr) -> Result<EvalResult, EvalError> {
        let left_result = self.eval(left)?;
        let right_result = self.eval(right)?;

        // Subtraction requires both operands to be numbers
        let left_value = left_result.as_number().ok_or_else(|| {
            EvalError::InvalidOperation("Cannot subtract with text values".to_string())
        })?;
        let right_value = right_result.as_number().ok_or_else(|| {
            EvalError::InvalidOperation("Cannot subtract with text values".to_string())
        })?;

        // Both dimensionless - simple subtraction
        if left_result.unit.is_dimensionless() && right_result.unit.is_dimensionless() {
            return Ok(EvalResult::new(
                left_value - right_value,
                Unit::dimensionless(),
            ));
        }

        // Check if units are compatible
        if !left_result.unit.is_compatible(&right_result.unit) {
            return Err(EvalError::IncompatibleUnits {
                operation: "subtract".to_string(),
                left: left_result.unit.to_string(),
                right: right_result.unit.to_string(),
            });
        }

        // If units are exactly the same, just subtract
        if left_result.unit.is_equal(&right_result.unit) {
            return Ok(EvalResult::new(
                left_value - right_value,
                left_result.unit.clone(),
            ));
        }

        // Units are compatible but different - determine finer unit and convert both
        let left_canonical = left_result.unit.canonical();
        let right_canonical = right_result.unit.canonical();

        // Find finer unit
        let finer_unit = self
            .library
            .get_finer_unit(left_canonical, right_canonical)
            .ok_or_else(|| EvalError::IncompatibleUnits {
                operation: "subtract".to_string(),
                left: left_result.unit.to_string(),
                right: right_result.unit.to_string(),
            })?;

        // Convert both values to finer unit
        let left_converted = if left_canonical == finer_unit {
            left_value
        } else {
            self.library
                .convert(left_value, left_canonical, finer_unit)
                .ok_or_else(|| EvalError::IncompatibleUnits {
                    operation: "subtract".to_string(),
                    left: left_result.unit.to_string(),
                    right: right_result.unit.to_string(),
                })?
        };

        let right_converted = if right_canonical == finer_unit {
            right_value
        } else {
            self.library
                .convert(right_value, right_canonical, finer_unit)
                .ok_or_else(|| EvalError::IncompatibleUnits {
                    operation: "subtract".to_string(),
                    left: left_result.unit.to_string(),
                    right: right_result.unit.to_string(),
                })?
        };

        // Get the finer unit object
        let result_unit = self
            .library
            .get(finer_unit)
            .cloned()
            .unwrap_or_else(|| left_result.unit.clone());

        Ok(EvalResult::new(
            left_converted - right_converted,
            result_unit,
        ))
    }

    /// Multiply two values (creates compound units with symbol-aware cancellation and conversion)
    fn eval_multiply(&self, left: &Expr, right: &Expr) -> Result<EvalResult, EvalError> {
        let left_result = self.eval(left)?;
        let right_result = self.eval(right)?;

        // Multiplication requires both operands to be numbers
        let left_value = left_result.as_number().ok_or_else(|| {
            EvalError::InvalidOperation("Cannot multiply with text values".to_string())
        })?;
        let right_value = right_result.as_number().ok_or_else(|| {
            EvalError::InvalidOperation("Cannot multiply with text values".to_string())
        })?;

        let mut value = left_value * right_value;

        // Check if either operand is a percentage - treat as dimensionless multiplier
        let left_is_percent = is_percentage_unit(&left_result.unit);
        let right_is_percent = is_percentage_unit(&right_result.unit);

        // If one is percentage, result has the non-percentage unit (percentage gets removed)
        if left_is_percent && !right_is_percent {
            return Ok(EvalResult::new(value, right_result.unit.clone()));
        }
        if right_is_percent && !left_is_percent {
            return Ok(EvalResult::new(value, left_result.unit.clone()));
        }
        // If both are percentages, result is dimensionless
        if left_is_percent && right_is_percent {
            return Ok(EvalResult::new(value, Unit::dimensionless()));
        }

        // If both dimensionless, result is dimensionless
        if left_result.unit.is_dimensionless() && right_result.unit.is_dimensionless() {
            return Ok(EvalResult::new(value, Unit::dimensionless()));
        }

        // If one is dimensionless, result has the other's unit
        if left_result.unit.is_dimensionless() {
            return Ok(EvalResult::new(value, right_result.unit.clone()));
        }
        if right_result.unit.is_dimensionless() {
            return Ok(EvalResult::new(value, left_result.unit.clone()));
        }

        // Extract unit symbols with powers
        let (mut left_num, mut left_den) = extract_unit_symbols(&left_result.unit);
        let (right_num, right_den) = extract_unit_symbols(&right_result.unit);

        // Multiply: add right's numerator to left's numerator, right's denominator to left's denominator
        for (symbol, power) in right_num {
            *left_num.entry(symbol).or_insert(0) += power;
        }
        for (symbol, power) in right_den {
            *left_den.entry(symbol).or_insert(0) += power;
        }

        // Cancel symbols and apply conversions
        let (final_num, final_den, conversion_factor) =
            cancel_and_convert_units(left_num, left_den, self.library);

        // Apply conversion factor to value
        value *= conversion_factor;

        // Build result unit
        let result_unit = build_unit_from_symbols(final_num, final_den, self.library);

        Ok(EvalResult::new(value, result_unit))
    }

    /// Divide two values (creates compound units with symbol-aware cancellation and conversion)
    fn eval_divide(&self, left: &Expr, right: &Expr) -> Result<EvalResult, EvalError> {
        let left_result = self.eval(left)?;
        let right_result = self.eval(right)?;

        // Division requires both operands to be numbers
        let left_value = left_result.as_number().ok_or_else(|| {
            EvalError::InvalidOperation("Cannot divide with text values".to_string())
        })?;
        let right_value = right_result.as_number().ok_or_else(|| {
            EvalError::InvalidOperation("Cannot divide with text values".to_string())
        })?;

        if right_value == 0.0 {
            return Err(EvalError::DivisionByZero);
        }

        let mut value = left_value / right_value;

        // Check if either operand is a percentage - treat as dimensionless multiplier
        let left_is_percent = is_percentage_unit(&left_result.unit);
        let right_is_percent = is_percentage_unit(&right_result.unit);

        // If right is percentage, result has left's unit (percentage gets removed)
        if right_is_percent && !left_is_percent {
            return Ok(EvalResult::new(value, left_result.unit.clone()));
        }
        // If left is percentage and right is not, result is percentage/right_unit
        if left_is_percent && !right_is_percent {
            let (right_num, right_den) = extract_unit_symbols(&right_result.unit);
            let result_unit = build_unit_from_symbols(right_den, right_num, self.library);
            return Ok(EvalResult::new(value, result_unit));
        }
        // If both are percentages, result is dimensionless
        if left_is_percent && right_is_percent {
            return Ok(EvalResult::new(value, Unit::dimensionless()));
        }

        // If both dimensionless, result is dimensionless
        if left_result.unit.is_dimensionless() && right_result.unit.is_dimensionless() {
            return Ok(EvalResult::new(value, Unit::dimensionless()));
        }

        // If left is dimensionless, result is 1/right_unit
        if left_result.unit.is_dimensionless() {
            let (right_num, right_den) = extract_unit_symbols(&right_result.unit);
            let result_unit = build_unit_from_symbols(right_den, right_num, self.library);
            return Ok(EvalResult::new(value, result_unit));
        }

        // If right is dimensionless, result has left's unit
        if right_result.unit.is_dimensionless() {
            return Ok(EvalResult::new(value, left_result.unit.clone()));
        }

        // Extract unit symbols with powers
        let (mut left_num, mut left_den) = extract_unit_symbols(&left_result.unit);
        let (right_num, right_den) = extract_unit_symbols(&right_result.unit);

        // Divide: right's numerator goes to left's denominator, right's denominator goes to left's numerator
        for (symbol, power) in right_num {
            *left_den.entry(symbol).or_insert(0) += power;
        }
        for (symbol, power) in right_den {
            *left_num.entry(symbol).or_insert(0) += power;
        }

        // Cancel symbols and apply conversions
        let (final_num, final_den, conversion_factor) =
            cancel_and_convert_units(left_num, left_den, self.library);

        // Apply conversion factor to value
        value *= conversion_factor;

        // Build result unit
        let result_unit = build_unit_from_symbols(final_num, final_den, self.library);

        Ok(EvalResult::new(value, result_unit))
    }

    /// Evaluate a function call
    fn eval_function(&self, name: &str, args: &[Expr]) -> Result<EvalResult, EvalError> {
        match name.to_uppercase().as_str() {
            "CEILING" => self.eval_ceiling(args),
            _ => Err(EvalError::FunctionNotImplemented(name.to_string())),
        }
    }

    /// CEILING function: rounds a number up to the nearest integer or to the nearest multiple of significance
    /// CEILING(number) or CEILING(number, significance)
    fn eval_ceiling(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        // Validate argument count (1 or 2 arguments)
        if args.is_empty() || args.len() > 2 {
            return Err(EvalError::InvalidOperation(format!(
                "CEILING requires 1 or 2 arguments, got {}",
                args.len()
            )));
        }

        // Evaluate the number argument
        let number_result = self.eval(&args[0])?;
        let number = number_result.as_number().ok_or_else(|| {
            EvalError::InvalidOperation("CEILING requires a numeric first argument".to_string())
        })?;

        // Get significance (defaults to 1 if not provided)
        let significance = if args.len() == 2 {
            let sig_result = self.eval(&args[1])?;
            let sig_value = sig_result.as_number().ok_or_else(|| {
                EvalError::InvalidOperation("CEILING significance must be numeric".to_string())
            })?;

            // Check unit compatibility
            if !sig_result.unit.is_dimensionless()
                && !number_result.unit.is_dimensionless()
                && !sig_result.unit.is_compatible(&number_result.unit)
            {
                return Err(EvalError::IncompatibleUnits {
                    operation: "CEILING".to_string(),
                    left: number_result.unit.to_string(),
                    right: sig_result.unit.to_string(),
                });
            }

            // Convert significance to same unit as number if needed
            let converted_sig = if sig_result.unit.is_dimensionless()
                || number_result.unit.is_dimensionless()
                || sig_result.unit.is_equal(&number_result.unit)
            {
                sig_value
            } else {
                // Convert significance to number's unit
                self.library
                    .convert(
                        sig_value,
                        sig_result.unit.canonical(),
                        number_result.unit.canonical(),
                    )
                    .ok_or_else(|| EvalError::IncompatibleUnits {
                        operation: "CEILING".to_string(),
                        left: number_result.unit.to_string(),
                        right: sig_result.unit.to_string(),
                    })?
            };

            converted_sig
        } else {
            1.0
        };

        // Check for zero significance
        if significance == 0.0 {
            return Err(EvalError::InvalidOperation(
                "CEILING significance cannot be zero".to_string(),
            ));
        }

        // Perform ceiling operation
        let result_value = if significance == 1.0 {
            // Simple ceiling to nearest integer
            number.ceil()
        } else {
            // Ceiling to nearest multiple of significance
            // Formula: ceil(number / significance) * significance
            (number / significance).ceil() * significance
        };

        // Preserve the unit from the number argument
        Ok(EvalResult::new(result_value, number_result.unit.clone()))
    }
}

// Helper function to check if a unit is a percentage unit
fn is_percentage_unit(unit: &Unit) -> bool {
    // Check if the unit's canonical form is "%"
    unit.canonical() == "%"
}

// Helper function to create compound units for division
// Helper function to multiply units with dimensional cancellation
pub fn multiply_units_with_cancellation(left: &Unit, right: &Unit) -> Unit {
    // Extract dimensions from both units
    let (mut num_dims, mut den_dims) = extract_dimensions(left);
    let (right_num, right_den) = extract_dimensions(right);

    // Add right's numerator to our numerator
    for (dim, power) in right_num {
        *num_dims.entry(dim).or_insert(0) += power;
    }

    // Add right's denominator to our denominator
    for (dim, power) in right_den {
        *den_dims.entry(dim).or_insert(0) += power;
    }

    // Cancel out matching dimensions
    let keys: Vec<_> = num_dims.keys().cloned().collect();
    for dim in keys {
        let num_power = num_dims.get(&dim).copied().unwrap_or(0);
        let den_power = den_dims.get(&dim).copied().unwrap_or(0);

        if num_power > 0 && den_power > 0 {
            let cancel = num_power.min(den_power);
            let new_num = num_power - cancel;
            let new_den = den_power - cancel;

            if new_num == 0 {
                num_dims.remove(&dim);
            } else {
                num_dims.insert(dim.clone(), new_num);
            }

            if new_den == 0 {
                den_dims.remove(&dim);
            } else {
                den_dims.insert(dim.clone(), new_den);
            }
        }
    }

    // Remove zero powers
    num_dims.retain(|_, &mut p| p != 0);
    den_dims.retain(|_, &mut p| p != 0);

    // Build result unit, trying to preserve original symbols
    build_unit_from_dimensions_with_originals(num_dims, den_dims, left, right)
}

// Helper function to divide units with dimensional cancellation
pub fn divide_units_with_cancellation(left: &Unit, right: &Unit) -> Unit {
    // Extract dimensions from both units
    let (mut num_dims, mut den_dims) = extract_dimensions(left);
    let (right_num, right_den) = extract_dimensions(right);

    // When dividing, right's numerator goes to our denominator
    for (dim, power) in right_num {
        *den_dims.entry(dim).or_insert(0) += power;
    }

    // And right's denominator goes to our numerator
    for (dim, power) in right_den {
        *num_dims.entry(dim).or_insert(0) += power;
    }

    // Cancel out matching dimensions
    let keys: Vec<_> = num_dims.keys().cloned().collect();
    for dim in keys {
        let num_power = num_dims.get(&dim).copied().unwrap_or(0);
        let den_power = den_dims.get(&dim).copied().unwrap_or(0);

        if num_power > 0 && den_power > 0 {
            let cancel = num_power.min(den_power);
            let new_num = num_power - cancel;
            let new_den = den_power - cancel;

            if new_num == 0 {
                num_dims.remove(&dim);
            } else {
                num_dims.insert(dim.clone(), new_num);
            }

            if new_den == 0 {
                den_dims.remove(&dim);
            } else {
                den_dims.insert(dim.clone(), new_den);
            }
        }
    }

    // Remove zero powers
    num_dims.retain(|_, &mut p| p != 0);
    den_dims.retain(|_, &mut p| p != 0);

    // Build result unit
    build_unit_from_dimensions(num_dims, den_dims)
}

// Build a unit from dimension maps, preserving original symbols when possible
fn build_unit_from_dimensions_with_originals(
    numerator: HashMap<BaseDimension, i32>,
    denominator: HashMap<BaseDimension, i32>,
    left_unit: &Unit,
    right_unit: &Unit,
) -> Unit {
    // If no dimensions, return dimensionless
    if numerator.is_empty() && denominator.is_empty() {
        return Unit::dimensionless();
    }

    // If only one dimension in numerator and no denominator with power 1, try to use original symbol
    if numerator.len() == 1 && denominator.is_empty() {
        let (base, power) = numerator.iter().next().unwrap();
        if *power == 1 {
            // Try to find this dimension in the original units and use their symbol
            if let Some(symbol) = find_original_symbol_for_dimension(base, left_unit, right_unit) {
                return Unit::simple(symbol, base.clone());
            }
            // Fallback to standard symbol
            let symbol = get_standard_symbol(base);
            return Unit::simple(symbol, base.clone());
        }
    }

    // Build compound unit symbol using original symbols where possible
    let symbol = build_unit_symbol_with_originals(&numerator, &denominator, left_unit, right_unit);

    // Convert to Vec format for Dimension::Compound
    let num_vec: Vec<_> = numerator.into_iter().collect();
    let den_vec: Vec<_> = denominator.into_iter().collect();

    if den_vec.is_empty() && num_vec.len() == 1 {
        // Simple unit (possibly with power)
        let (base, power) = &num_vec[0];
        if *power == 1 {
            if let Some(symbol) = find_original_symbol_for_dimension(base, left_unit, right_unit) {
                Unit::simple(symbol, base.clone())
            } else {
                let symbol = get_standard_symbol(base);
                Unit::simple(symbol, base.clone())
            }
        } else {
            Unit::compound(symbol, num_vec, vec![])
        }
    } else {
        Unit::compound(symbol, num_vec, den_vec)
    }
}

// Find the original symbol for a dimension from the input units
fn find_original_symbol_for_dimension(
    dim: &BaseDimension,
    left: &Unit,
    right: &Unit,
) -> Option<String> {
    // Check if left unit has this dimension as a simple unit
    if let Dimension::Simple(left_dim) = left.dimension() {
        if left_dim == dim {
            return Some(left.canonical().to_string());
        }
    }

    // Check if right unit has this dimension as a simple unit
    if let Dimension::Simple(right_dim) = right.dimension() {
        if right_dim == dim {
            return Some(right.canonical().to_string());
        }
    }

    // Check compound units - extract the symbol for this specific dimension
    if let Dimension::Compound { numerator, .. } = left.dimension() {
        for (d, _) in numerator {
            if d == dim {
                // For compound units like "$/ft", try to extract the currency symbol
                return extract_symbol_from_compound(left.canonical(), dim);
            }
        }
    }

    if let Dimension::Compound { numerator, .. } = right.dimension() {
        for (d, _) in numerator {
            if d == dim {
                return extract_symbol_from_compound(right.canonical(), dim);
            }
        }
    }

    None
}

// Extract symbol for a specific dimension from a compound unit symbol
fn extract_symbol_from_compound(compound: &str, dim: &BaseDimension) -> Option<String> {
    // Parse compound unit to find the symbol for this dimension
    // Handles formats like "mi/hr", "USD/ft", "kg*m", etc.

    // Split by division first
    if let Some(div_pos) = compound.find('/') {
        let numerator_part = &compound[..div_pos];
        let denominator_part = &compound[div_pos + 1..];

        // Check numerator symbols
        if let Some(symbol) = extract_symbol_for_dimension_from_part(numerator_part, dim) {
            return Some(symbol);
        }

        // Check denominator symbols
        if let Some(symbol) = extract_symbol_for_dimension_from_part(denominator_part, dim) {
            return Some(symbol);
        }
    } else {
        // No division, check the whole string
        if let Some(symbol) = extract_symbol_for_dimension_from_part(compound, dim) {
            return Some(symbol);
        }
    }

    None
}

// Extract symbol from a unit part (handles multiplication and powers)
fn extract_symbol_for_dimension_from_part(part: &str, dim: &BaseDimension) -> Option<String> {
    // Split by multiplication
    let symbols: Vec<&str> = part.split('*').collect();

    for symbol in symbols {
        // Remove power notation if present (e.g., "ft^2" -> "ft")
        let base_symbol = if let Some(pow_pos) = symbol.find('^') {
            &symbol[..pow_pos]
        } else {
            symbol
        };

        // Check if this symbol matches the dimension
        if dimension_matches_symbol(dim, base_symbol) {
            return Some(base_symbol.to_string());
        }
    }

    None
}

// Check if a symbol matches a given dimension
fn dimension_matches_symbol(dim: &BaseDimension, symbol: &str) -> bool {
    match dim {
        BaseDimension::Length => {
            matches!(symbol, "m" | "cm" | "mm" | "km" | "in" | "ft" | "yd" | "mi")
        }
        BaseDimension::Mass => matches!(symbol, "g" | "kg" | "mg" | "oz" | "lb"),
        BaseDimension::Time => matches!(
            symbol,
            "s" | "min" | "hr" | "h" | "hour" | "day" | "month" | "year"
        ),
        BaseDimension::Temperature => matches!(symbol, "C" | "F" | "K"),
        BaseDimension::Currency => matches!(symbol, "USD" | "EUR" | "GBP" | "$" | "€" | "£"),
        BaseDimension::DigitalStorage => matches!(
            symbol,
            "B" | "KB"
                | "MB"
                | "GB"
                | "TB"
                | "PB"
                | "Kb"
                | "Mb"
                | "Gb"
                | "Tb"
                | "Pb"
                | "Tok"
                | "MTok"
        ),
        BaseDimension::Custom(name) => symbol == name,
    }
}

// Build unit symbol with original symbols when possible
fn build_unit_symbol_with_originals(
    numerator: &HashMap<BaseDimension, i32>,
    denominator: &HashMap<BaseDimension, i32>,
    left: &Unit,
    right: &Unit,
) -> String {
    let mut parts = Vec::new();

    // Build numerator
    let mut num_symbols: Vec<_> = numerator
        .iter()
        .map(|(d, p)| {
            let symbol = find_original_symbol_for_dimension(d, left, right)
                .unwrap_or_else(|| get_standard_symbol(d));
            (symbol, p)
        })
        .collect();
    num_symbols.sort();

    for (symbol, power) in num_symbols {
        if *power == 1 {
            parts.push(symbol);
        } else {
            parts.push(format!("{}^{}", symbol, power));
        }
    }

    let num_str = if parts.is_empty() {
        String::new()
    } else {
        parts.join("*")
    };

    // Build denominator
    let mut den_symbols: Vec<_> = denominator
        .iter()
        .map(|(d, p)| {
            let symbol = find_original_symbol_for_dimension(d, left, right)
                .unwrap_or_else(|| get_standard_symbol(d));
            (symbol, p)
        })
        .collect();
    den_symbols.sort();

    let mut den_parts = Vec::new();
    for (symbol, power) in den_symbols {
        if *power == 1 {
            den_parts.push(symbol);
        } else {
            den_parts.push(format!("{}^{}", symbol, power));
        }
    }

    if den_parts.is_empty() {
        num_str
    } else {
        let den_str = den_parts.join("*");
        if num_str.is_empty() {
            format!("1/{}", den_str)
        } else {
            format!("{}/{}", num_str, den_str)
        }
    }
}

// Extract dimensions from a unit into numerator and denominator maps
pub fn extract_dimensions(
    unit: &Unit,
) -> (HashMap<BaseDimension, i32>, HashMap<BaseDimension, i32>) {
    let mut numerator = HashMap::new();
    let mut denominator = HashMap::new();

    match unit.dimension() {
        Dimension::Dimensionless => {}
        Dimension::Simple(base) => {
            numerator.insert(base.clone(), 1);
        }
        Dimension::Compound {
            numerator: num,
            denominator: den,
        } => {
            for (base, power) in num {
                *numerator.entry(base.clone()).or_insert(0) += power;
            }
            for (base, power) in den {
                *denominator.entry(base.clone()).or_insert(0) += power;
            }
        }
    }

    (numerator, denominator)
}

/// Extract unit symbols with their powers from a unit
/// Returns (numerator_symbols, denominator_symbols)
/// where each map contains symbol -> power
///
/// Examples:
/// - "GB" → ({GB: 1}, {})
/// - "GB^2" → ({GB: 2}, {})
/// - "$/GB" → ({$: 1}, {GB: 1})
/// - "GB/Month" → ({GB: 1}, {Month: 1})
/// - "$/GB·Month" → ({$: 1}, {GB: 1, Month: 1})
pub fn extract_unit_symbols(unit: &Unit) -> (HashMap<String, i32>, HashMap<String, i32>) {
    let mut numerator = HashMap::new();
    let mut denominator = HashMap::new();

    let canonical = unit.canonical();

    if canonical.is_empty() {
        return (numerator, denominator);
    }

    // Split by division first
    let parts: Vec<&str> = canonical.split('/').collect();

    if parts.is_empty() {
        return (numerator, denominator);
    }

    // Parse numerator part
    if !parts[0].is_empty() {
        parse_unit_part(parts[0], &mut numerator);
    }

    // Parse denominator part (if exists)
    if parts.len() > 1 {
        // Join remaining parts (in case there are multiple / symbols)
        let denom_part = parts[1..].join("/");
        if !denom_part.is_empty() {
            parse_unit_part(&denom_part, &mut denominator);
        }
    }

    (numerator, denominator)
}

/// Parse a unit part (numerator or denominator) and extract symbols with powers
/// Handles multiplication (*) and exponents (^)
fn parse_unit_part(part: &str, map: &mut HashMap<String, i32>) {
    // Split by multiplication
    for term in part.split('*') {
        let term = term.trim();
        if term.is_empty() || term == "1" {
            continue;
        }

        // Check for exponent
        if let Some(caret_pos) = term.find('^') {
            let symbol = term[..caret_pos].trim();
            let power_str = term[caret_pos + 1..].trim();

            if let Ok(power) = power_str.parse::<i32>() {
                *map.entry(symbol.to_string()).or_insert(0) += power;
            } else {
                // Invalid power, treat as power 1
                *map.entry(symbol.to_string()).or_insert(0) += 1;
            }
        } else {
            // No exponent, power is 1
            *map.entry(term.to_string()).or_insert(0) += 1;
        }
    }
}

/// Cancel matching symbols between numerator and denominator with conversion-aware cancellation
/// Returns (final_numerator, final_denominator, conversion_factor)
///
/// Algorithm:
/// 1. Cancel exact symbol matches (e.g., GB/GB → 1)
/// 2. Convert and cancel compatible units (e.g., TB/GB → apply ratio 1024/1)
/// 3. Apply exponents to conversion ratios (e.g., ft^2/m^2 → (ft→m)^2)
pub fn cancel_and_convert_units(
    mut numerator: HashMap<String, i32>,
    mut denominator: HashMap<String, i32>,
    library: &UnitLibrary,
) -> (HashMap<String, i32>, HashMap<String, i32>, f64) {
    let mut conversion_factor = 1.0;

    // Step 1: Cancel exact symbol matches
    let num_keys: Vec<String> = numerator.keys().cloned().collect();
    for symbol in num_keys {
        if let Some(&num_power) = numerator.get(&symbol) {
            if let Some(&den_power) = denominator.get(&symbol) {
                let cancel_power = num_power.min(den_power);
                if cancel_power > 0 {
                    let new_num_power = num_power - cancel_power;
                    let new_den_power = den_power - cancel_power;

                    if new_num_power == 0 {
                        numerator.remove(&symbol);
                    } else {
                        numerator.insert(symbol.clone(), new_num_power);
                    }

                    if new_den_power == 0 {
                        denominator.remove(&symbol);
                    } else {
                        denominator.insert(symbol.clone(), new_den_power);
                    }
                }
            }
        }
    }

    // Step 2: Convert and cancel compatible units
    // Find all pairs of (num_symbol, den_symbol) that are compatible
    let num_symbols: Vec<String> = numerator.keys().cloned().collect();
    let den_symbols: Vec<String> = denominator.keys().cloned().collect();

    for num_symbol in &num_symbols {
        for den_symbol in &den_symbols {
            // Skip if symbols no longer exist (already processed)
            if !numerator.contains_key(num_symbol) || !denominator.contains_key(den_symbol) {
                continue;
            }

            // Check if units are compatible (can be converted)
            if library.can_convert(num_symbol, den_symbol) && num_symbol != den_symbol {
                // Get conversion ratio
                if let Some((ratio_num, ratio_den)) =
                    library.get_conversion_ratio(num_symbol, den_symbol)
                {
                    let num_power = *numerator.get(num_symbol).unwrap();
                    let den_power = *denominator.get(den_symbol).unwrap();

                    // Cancel as many as possible
                    let cancel_power = num_power.min(den_power);

                    if cancel_power > 0 {
                        // Apply conversion ratio raised to cancel_power
                        // ratio = (ratio_num / ratio_den)
                        // ratio^cancel_power = (ratio_num^cancel_power) / (ratio_den^cancel_power)
                        let ratio_factor = (ratio_num as f64).powi(cancel_power)
                            / (ratio_den as f64).powi(cancel_power);
                        conversion_factor *= ratio_factor;

                        // Update powers
                        let new_num_power = num_power - cancel_power;
                        let new_den_power = den_power - cancel_power;

                        if new_num_power == 0 {
                            numerator.remove(num_symbol);
                        } else {
                            numerator.insert(num_symbol.clone(), new_num_power);
                        }

                        if new_den_power == 0 {
                            denominator.remove(den_symbol);
                        } else {
                            denominator.insert(den_symbol.clone(), new_den_power);
                        }
                    }
                }
            }
        }
    }

    // Remove zero powers
    numerator.retain(|_, &mut p| p != 0);
    denominator.retain(|_, &mut p| p != 0);

    (numerator, denominator, conversion_factor)
}

/// Build a Unit from symbol maps (numerator and denominator with powers)
pub fn build_unit_from_symbols(
    numerator: HashMap<String, i32>,
    denominator: HashMap<String, i32>,
    library: &UnitLibrary,
) -> Unit {
    // If no units, return dimensionless
    if numerator.is_empty() && denominator.is_empty() {
        return Unit::dimensionless();
    }

    // Build symbol string
    let mut num_parts = Vec::new();
    for (symbol, power) in &numerator {
        if *power == 1 {
            num_parts.push(symbol.clone());
        } else {
            num_parts.push(format!("{}^{}", symbol, power));
        }
    }
    num_parts.sort();

    let mut den_parts = Vec::new();
    for (symbol, power) in &denominator {
        if *power == 1 {
            den_parts.push(symbol.clone());
        } else {
            den_parts.push(format!("{}^{}", symbol, power));
        }
    }
    den_parts.sort();

    let canonical_symbol = if den_parts.is_empty() {
        if num_parts.is_empty() {
            String::new()
        } else {
            num_parts.join("*")
        }
    } else {
        let num_str = if num_parts.is_empty() {
            "1".to_string()
        } else {
            num_parts.join("*")
        };
        format!("{}/{}", num_str, den_parts.join("*"))
    };

    // If single simple unit, return as simple
    if denominator.is_empty() && numerator.len() == 1 {
        let (symbol, power) = numerator.iter().next().unwrap();
        if *power == 1 {
            // Look up dimension from library
            if let Some(unit) = library.get(symbol) {
                return unit.clone();
            }
        }
    }

    // Build compound unit with dimensions
    let mut num_dims = Vec::new();
    let mut den_dims = Vec::new();

    for (symbol, power) in numerator {
        if let Some(unit) = library.get(&symbol) {
            if let Some(base_dim) = unit.dimension().as_simple() {
                num_dims.push((base_dim.clone(), power));
            }
        }
    }

    for (symbol, power) in denominator {
        if let Some(unit) = library.get(&symbol) {
            if let Some(base_dim) = unit.dimension().as_simple() {
                den_dims.push((base_dim.clone(), power));
            }
        }
    }

    if num_dims.is_empty() && den_dims.is_empty() {
        // Custom or unknown units - return as string unit
        Unit::simple(
            canonical_symbol.clone(),
            BaseDimension::Custom(canonical_symbol),
        )
    } else {
        Unit::compound(canonical_symbol, num_dims, den_dims)
    }
}

/// Transform unit by multiplying all dimension exponents by a factor
/// Used for SQRT (factor=0.5) and POWER (factor=exponent)
pub fn transform_unit_exponents(unit: &Unit, factor: f64) -> Unit {
    // Extract dimensions
    let (numerator, denominator) = extract_dimensions(unit);

    // Transform all exponents in numerator
    let transformed_numerator: HashMap<BaseDimension, i32> = numerator
        .into_iter()
        .map(|(base, power)| {
            let new_power = ((power as f64) * factor).round() as i32;
            (base, new_power)
        })
        .collect();

    // Transform all exponents in denominator
    let transformed_denominator: HashMap<BaseDimension, i32> = denominator
        .into_iter()
        .map(|(base, power)| {
            let new_power = ((power as f64) * factor).round() as i32;
            (base, new_power)
        })
        .collect();

    // Build the new unit
    build_unit_from_dimensions(transformed_numerator, transformed_denominator)
}

// Build a unit from dimension maps
pub fn build_unit_from_dimensions(
    numerator: HashMap<BaseDimension, i32>,
    denominator: HashMap<BaseDimension, i32>,
) -> Unit {
    // If no dimensions, return dimensionless
    if numerator.is_empty() && denominator.is_empty() {
        return Unit::dimensionless();
    }

    // If only one dimension in numerator and no denominator, return simple unit
    if numerator.len() == 1 && denominator.is_empty() {
        let (base, power) = numerator.iter().next().unwrap();
        if *power == 1 {
            // Get the standard symbol for this dimension
            let symbol = get_standard_symbol(base);
            return Unit::simple(symbol, base.clone());
        }
    }

    // Build compound unit symbol
    let symbol = build_unit_symbol(&numerator, &denominator);

    // Convert to Vec format for Dimension::Compound
    let num_vec: Vec<_> = numerator.into_iter().collect();
    let den_vec: Vec<_> = denominator.into_iter().collect();

    if den_vec.is_empty() && num_vec.len() == 1 {
        // Simple unit (possibly with power)
        let (base, power) = &num_vec[0];
        if *power == 1 {
            let symbol = get_standard_symbol(base);
            Unit::simple(symbol, base.clone())
        } else {
            Unit::compound(symbol, num_vec, vec![])
        }
    } else {
        Unit::compound(symbol, num_vec, den_vec)
    }
}

// Build unit symbol string from dimensions
fn build_unit_symbol(
    numerator: &HashMap<BaseDimension, i32>,
    denominator: &HashMap<BaseDimension, i32>,
) -> String {
    let mut parts = Vec::new();

    // Build numerator
    let mut num_symbols: Vec<_> = numerator
        .iter()
        .map(|(d, p)| (get_standard_symbol(d), p))
        .collect();
    num_symbols.sort();

    for (symbol, power) in num_symbols {
        if *power == 1 {
            parts.push(symbol);
        } else {
            parts.push(format!("{}^{}", symbol, power));
        }
    }

    let num_str = if parts.is_empty() {
        String::new()
    } else {
        parts.join("*")
    };

    // Build denominator
    let mut den_symbols: Vec<_> = denominator
        .iter()
        .map(|(d, p)| (get_standard_symbol(d), p))
        .collect();
    den_symbols.sort();

    let mut den_parts = Vec::new();
    for (symbol, power) in den_symbols {
        if *power == 1 {
            den_parts.push(symbol);
        } else {
            den_parts.push(format!("{}^{}", symbol, power));
        }
    }

    if den_parts.is_empty() {
        num_str
    } else {
        let den_str = den_parts.join("*");
        if num_str.is_empty() {
            format!("1/{}", den_str)
        } else {
            format!("{}/{}", num_str, den_str)
        }
    }
}

// Get standard symbol for a base dimension
fn get_standard_symbol(base: &BaseDimension) -> String {
    match base {
        BaseDimension::Length => "m".to_string(),
        BaseDimension::Mass => "kg".to_string(),
        BaseDimension::Time => "s".to_string(),
        BaseDimension::Currency => "USD".to_string(),
        BaseDimension::Temperature => "C".to_string(),
        BaseDimension::DigitalStorage => "B".to_string(),
        BaseDimension::Custom(name) => name.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_number() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        let expr = Expr::number(42.0);
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(42.0));
        assert!(result.unit.is_dimensionless());
    }

    #[test]
    fn test_eval_number_with_unit() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        let expr = Expr::number_with_unit(100.0, "m");
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(100.0));
        assert_eq!(result.unit.canonical(), "m");
    }

    #[test]
    fn test_add_same_units() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        let expr = Expr::new_add(
            Expr::number_with_unit(100.0, "m"),
            Expr::number_with_unit(50.0, "m"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(150.0));
        assert_eq!(result.unit.canonical(), "m");
    }

    #[test]
    fn test_add_compatible_units() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        let expr = Expr::new_add(
            Expr::number_with_unit(100.0, "m"),
            Expr::number_with_unit(50.0, "cm"),
        );
        let result = eval.eval(&expr).unwrap();
        // Updated: With finer unit alignment, result is in cm (finer than m)
        // 100 m = 10,000 cm, 10,000 cm + 50 cm = 10,050 cm
        assert_eq!(result.value, EvalValue::Number(10050.0));
        assert_eq!(result.unit.canonical(), "cm");
    }

    #[test]
    fn test_add_incompatible_units() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        let expr = Expr::new_add(
            Expr::number_with_unit(100.0, "m"),
            Expr::number_with_unit(50.0, "kg"),
        );
        let result = eval.eval(&expr);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            EvalError::IncompatibleUnits { .. }
        ));
    }

    #[test]
    fn test_subtract_same_units() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        let expr = Expr::new_subtract(
            Expr::number_with_unit(100.0, "m"),
            Expr::number_with_unit(30.0, "m"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(70.0));
        assert_eq!(result.unit.canonical(), "m");
    }

    #[test]
    fn test_multiply_creates_compound() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        let expr = Expr::new_multiply(
            Expr::number_with_unit(10.0, "m"),
            Expr::number_with_unit(5.0, "m"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(50.0));
        // Result should be m^2 (area, simplified)
        assert_eq!(result.unit.canonical(), "m^2");
    }

    #[test]
    fn test_multiply_with_dimensionless() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        let expr = Expr::new_multiply(Expr::number_with_unit(10.0, "m"), Expr::number(2.0));
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(20.0));
        assert_eq!(result.unit.canonical(), "m");
    }

    #[test]
    fn test_divide_same_units() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        let expr = Expr::new_divide(
            Expr::number_with_unit(100.0, "m"),
            Expr::number_with_unit(10.0, "m"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(10.0));
        assert!(result.unit.is_dimensionless()); // Units cancel
    }

    #[test]
    fn test_divide_creates_compound() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        let expr = Expr::new_divide(
            Expr::number_with_unit(100.0, "m"),
            Expr::number_with_unit(2.0, "s"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(50.0));
        // Result should be m/s (velocity)
        assert_eq!(result.unit.canonical(), "m/s");
    }

    #[test]
    fn test_divide_by_zero() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        let expr = Expr::new_divide(Expr::number(10.0), Expr::number(0.0));
        let result = eval.eval(&expr);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EvalError::DivisionByZero));
    }

    #[test]
    fn test_negate() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        let expr = Expr::negate(Expr::number_with_unit(50.0, "m"));
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(-50.0));
        assert_eq!(result.unit.canonical(), "m");
    }

    #[test]
    fn test_complex_expression() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // (100m + 50m) * 2
        let expr = Expr::new_multiply(
            Expr::new_add(
                Expr::number_with_unit(100.0, "m"),
                Expr::number_with_unit(50.0, "m"),
            ),
            Expr::number(2.0),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(300.0));
        assert_eq!(result.unit.canonical(), "m");
    }

    #[test]
    fn test_velocity_calculation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // 100mi / 2hr = 50 mi/hr
        let expr = Expr::new_divide(
            Expr::number_with_unit(100.0, "mi"),
            Expr::number_with_unit(2.0, "hr"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(50.0));
        assert_eq!(result.unit.canonical(), "mi/hr");
    }

    #[test]
    fn test_unit_cancellation_currency_per_length() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // 2 ft * 15 $/ft = 30 USD (ft should cancel)
        let expr = Expr::new_multiply(
            Expr::number_with_unit(2.0, "ft"),
            Expr::number_with_unit(15.0, "$/ft"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(30.0));
        // The unit should be USD or $ after cancellation
        assert!(result.unit.canonical() == "USD" || result.unit.canonical() == "$");
        assert!(
            !result.unit.canonical().contains("ft"),
            "Unit should not contain 'ft' after cancellation, got: {}",
            result.unit.canonical()
        );
    }

    #[test]
    fn test_parse_compound_unit_in_formula() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // Test that $/ft is parsed correctly as a compound unit
        let expr = Expr::number_with_unit(15.0, "$/ft");
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(15.0));
        assert_eq!(result.unit.canonical(), "$/ft");
    }

    #[test]
    fn test_unit_cancellation_with_division_result() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);

        // Simulate: A1=$15, A2=1ft, B2=A1/A2, A3=2ft, B4=A3*B2
        // B2 should be 15 $/ft
        let b2_expr = Expr::new_divide(
            Expr::number_with_unit(15.0, "$"),
            Expr::number_with_unit(1.0, "ft"),
        );
        let b2_result = eval.eval(&b2_expr).unwrap();
        assert_eq!(b2_result.value, EvalValue::Number(15.0));
        println!("B2 canonical: {}", b2_result.unit.canonical());

        // Now use B2's result in a multiplication: 2ft * (result from B2)
        // This should cancel to give us USD
        // We need to create an expression that multiplies ft by a compound unit
        let b4_expr = Expr::new_multiply(
            Expr::number_with_unit(2.0, "ft"),
            Expr::number_with_unit(15.0, "$/ft"),
        );
        let b4_result = eval.eval(&b4_expr).unwrap();

        assert_eq!(b4_result.value, EvalValue::Number(30.0));
        println!("B4 canonical: {}", b4_result.unit.canonical());
        println!("B4 dimension: {:?}", b4_result.unit.dimension());

        // The unit should be just currency, not containing "ft"
        assert!(
            b4_result.unit.canonical() == "USD" || b4_result.unit.canonical() == "$",
            "Expected USD or $, got: {}",
            b4_result.unit.canonical()
        );
        assert!(
            !b4_result.unit.canonical().to_lowercase().contains("ft"),
            "Unit should not contain 'ft' after cancellation, got: {}",
            b4_result.unit.canonical()
        );
    }

    // ===== COMPREHENSIVE UNIT CANCELLATION EDGE CASES =====

    // Test 1: Simple cancellation - Length
    #[test]
    fn test_cancel_simple_length() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // m * (1/m) = dimensionless
        let expr = Expr::new_multiply(
            Expr::number_with_unit(10.0, "m"),
            Expr::new_divide(Expr::number(5.0), Expr::number_with_unit(1.0, "m")),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(50.0));
        assert!(result.unit.is_dimensionless());
    }

    // Test 2: Simple cancellation - Mass
    #[test]
    fn test_cancel_simple_mass() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // kg * (USD/kg) = USD
        let expr = Expr::new_multiply(
            Expr::number_with_unit(5.0, "kg"),
            Expr::number_with_unit(20.0, "USD/kg"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(100.0));
        assert_eq!(result.unit.canonical(), "USD");
    }

    // Test 3: Simple cancellation - Time
    #[test]
    fn test_cancel_simple_time() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // hr * (m/hr) = m (note: using m not mi since mi isn't in library yet)
        let expr = Expr::new_multiply(
            Expr::number_with_unit(2.0, "hr"),
            Expr::number_with_unit(60.0, "m/hr"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(120.0));
        assert_eq!(result.unit.canonical(), "m");
    }

    // Test 4: Acceleration units - ft/s²
    #[test]
    fn test_acceleration_units() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // ft/s² is length/time²
        let expr = Expr::new_divide(
            Expr::number_with_unit(32.2, "ft"),
            Expr::number_with_unit(1.0, "s*s"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(32.2));
        // Should have ft in numerator, s² in denominator
        assert!(result.unit.canonical().contains("ft"));
        assert!(result.unit.canonical().contains("s"));
    }

    // Test 5: Multiply velocity by (1/s) to get acceleration
    #[test]
    fn test_acceleration_times_time() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // (ft/s) * (1/s) = ft/s²
        let expr = Expr::new_multiply(
            Expr::number_with_unit(32.2, "ft/s"),
            Expr::new_divide(Expr::number(1.0), Expr::number_with_unit(1.0, "s")),
        );
        let result = eval.eval(&expr).unwrap();
        // Should be ft/s² - note: ft might not be preserved based on find_original_symbol
        let canonical = result.unit.canonical();
        assert!(canonical.contains("ft") || canonical.contains("m")); // Either ft or converted to m
        assert!(canonical.contains("s"));
    }

    // Test 6: Currency per area (partial cancellation)
    #[test]
    fn test_currency_per_area() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // ft² * ($/ft) = ft*$ (one ft cancels, one remains)
        let expr = Expr::new_multiply(
            Expr::number_with_unit(100.0, "ft*ft"),
            Expr::number_with_unit(5.0, "$/ft"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(500.0));
        // One ft should cancel, leaving $/ft or USD/ft or ft*USD depending on order
        let canonical = result.unit.canonical();
        assert!(canonical.contains("$") || canonical.contains("USD"));
        // After cancellation should have one ft remaining
        assert!(canonical.contains("ft") || canonical.contains("m"));
    }

    // Test 7: Complete area cancellation using double multiplication
    #[test]
    fn test_currency_per_area_complete() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // Create ft² * $/ft² = $ by multiplying (ft * ft) * (($/ft) * (1/ft))
        let area = Expr::new_multiply(
            Expr::number_with_unit(10.0, "ft"),
            Expr::number_with_unit(10.0, "ft"),
        );
        // Create $/ft² by multiplying ($/ft) * (1/ft)
        let price_per_ft = Expr::number_with_unit(5.0, "$/ft");
        let per_ft = Expr::new_divide(Expr::number(1.0), Expr::number_with_unit(1.0, "ft"));
        let price_per_area = Expr::new_multiply(price_per_ft, per_ft);
        let expr = Expr::new_multiply(area, price_per_area);
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(500.0));
        // All ft should cancel leaving just currency
        let canonical = result.unit.canonical();
        assert!(canonical == "$" || canonical == "USD", "Got: {}", canonical);
    }

    // Test 8: Partial cancellation with multiple dimensions
    // Note: Division doesn't auto-cancel yet - only multiplication does cancellation
    #[test]
    fn test_partial_cancellation_multiple_dims() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // (kg*m) / kg = should cancel to m, but division doesn't do cancellation
        // So we get kg*m/kg which we then need to multiply by something to trigger cancel
        let numerator = Expr::new_multiply(
            Expr::number_with_unit(10.0, "kg"),
            Expr::number_with_unit(5.0, "m"),
        );
        let denominator = Expr::number_with_unit(2.0, "kg");
        let expr = Expr::new_divide(numerator, denominator);
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(25.0));
        // Currently division doesn't cancel, so kg will still be in there
        // This test documents that we need to add cancellation to division
        assert!(result.unit.canonical().contains("m"));
    }

    // Test 9: No cancellation - independent dimensions
    #[test]
    fn test_no_cancellation_independent() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // kg * m = kg*m (momentum-like)
        let expr = Expr::new_multiply(
            Expr::number_with_unit(5.0, "kg"),
            Expr::number_with_unit(10.0, "m"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(50.0));
        assert!(result.unit.canonical().contains("kg"));
        assert!(result.unit.canonical().contains("m"));
    }

    // Test 10: Currency with different length units
    #[test]
    fn test_currency_length_variations() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // yd * ($/yd) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(3.0, "yd"),
            Expr::number_with_unit(10.0, "$/yd"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(30.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
        assert!(!result.unit.canonical().contains("yd"));
    }

    // Test 11: Mile cancellation
    #[test]
    fn test_mile_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // mi * ($/mi) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(100.0, "mi"),
            Expr::number_with_unit(2.5, "$/mi"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(250.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 12: Time squared cancellation
    #[test]
    fn test_time_squared_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // s² * (1/s) * (1/s) = dimensionless
        let s_squared = Expr::new_multiply(
            Expr::number_with_unit(2.0, "s"),
            Expr::number_with_unit(2.0, "s"),
        );
        // Create (1/s) * (1/s) instead of (1/s²) to avoid compound unit division
        let per_s1 = Expr::new_divide(Expr::number(5.0), Expr::number_with_unit(1.0, "s"));
        let per_s2 = Expr::new_divide(Expr::number(2.0), Expr::number_with_unit(1.0, "s"));
        let per_s_squared = Expr::new_multiply(per_s1, per_s2);
        let expr = Expr::new_multiply(s_squared, per_s_squared);
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(40.0));
        assert!(result.unit.is_dimensionless());
    }

    // Test 13: Partial time cancellation
    #[test]
    fn test_partial_time_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // s³ * (1/s) * (1/s) = s
        let s_cubed = Expr::new_multiply(
            Expr::new_multiply(
                Expr::number_with_unit(1.0, "s"),
                Expr::number_with_unit(1.0, "s"),
            ),
            Expr::number_with_unit(1.0, "s"),
        );
        // Create (1/s) * (1/s) instead of (1/s²) to avoid compound unit division
        let per_s1 = Expr::new_divide(Expr::number(1.0), Expr::number_with_unit(1.0, "s"));
        let per_s2 = Expr::new_divide(Expr::number(1.0), Expr::number_with_unit(1.0, "s"));
        let per_s_squared = Expr::new_multiply(per_s1, per_s2);
        let expr = Expr::new_multiply(s_cubed, per_s_squared);
        let result = eval.eval(&expr).unwrap();
        // Should leave one s
        assert_eq!(result.unit.canonical(), "s");
    }

    // Test 14: Mass time product
    #[test]
    fn test_mass_time_product() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // kg * s = kg*s
        let expr = Expr::new_multiply(
            Expr::number_with_unit(5.0, "kg"),
            Expr::number_with_unit(2.0, "s"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(10.0));
        assert!(result.unit.canonical().contains("kg"));
        assert!(result.unit.canonical().contains("s"));
    }

    // Test 15: Compound cancellation chain
    #[test]
    fn test_compound_cancellation_chain() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // (m/s) * (s/kg) * kg = m
        let velocity = Expr::new_divide(
            Expr::number_with_unit(10.0, "m"),
            Expr::number_with_unit(1.0, "s"),
        );
        let time_per_mass = Expr::new_divide(
            Expr::number_with_unit(2.0, "s"),
            Expr::number_with_unit(1.0, "kg"),
        );
        let mass = Expr::number_with_unit(5.0, "kg");
        let temp = Expr::new_multiply(velocity, time_per_mass);
        let expr = Expr::new_multiply(temp, mass);
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(100.0));
        // s and kg should cancel, leaving only m
        assert_eq!(result.unit.canonical(), "m");
    }

    // Test 16: Currency with mass
    #[test]
    fn test_currency_per_mass() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // lb * ($/lb) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(10.0, "lb"),
            Expr::number_with_unit(3.5, "$/lb"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(35.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 17: Ounce cancellation
    #[test]
    fn test_ounce_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // oz * ($/oz) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(16.0, "oz"),
            Expr::number_with_unit(0.5, "$/oz"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(8.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 18: Minutes cancellation
    #[test]
    fn test_minutes_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // min * ($/min) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(30.0, "min"),
            Expr::number_with_unit(2.0, "$/min"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(60.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 19: Complex multi-dimension no-cancel
    #[test]
    fn test_complex_multi_dimension_no_cancel() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // (kg*m) / s² = kg*m/s² (force-like)
        let numerator = Expr::new_multiply(
            Expr::number_with_unit(10.0, "kg"),
            Expr::number_with_unit(5.0, "m"),
        );
        let denominator = Expr::new_multiply(
            Expr::number_with_unit(1.0, "s"),
            Expr::number_with_unit(1.0, "s"),
        );
        let expr = Expr::new_divide(numerator, denominator);
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(50.0));
        assert!(result.unit.canonical().contains("kg"));
        assert!(result.unit.canonical().contains("m"));
        assert!(result.unit.canonical().contains("s"));
    }

    // Test 20: Inch cancellation
    #[test]
    fn test_inch_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // in * ($/in) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(12.0, "in"),
            Expr::number_with_unit(5.0, "$/in"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(60.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 21: Day cancellation
    #[test]
    fn test_day_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // day * ($/day) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(7.0, "day"),
            Expr::number_with_unit(100.0, "$/day"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(700.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 22: Gram cancellation
    #[test]
    fn test_gram_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // g * ($/g) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(100.0, "g"),
            Expr::number_with_unit(0.1, "$/g"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(10.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 23: Millimeter cancellation
    #[test]
    fn test_millimeter_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // mm * (1/mm) = dimensionless
        let expr = Expr::new_multiply(
            Expr::number_with_unit(1000.0, "mm"),
            Expr::new_divide(Expr::number(2.0), Expr::number_with_unit(1.0, "mm")),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(2000.0));
        assert!(result.unit.is_dimensionless());
    }

    // Test 24: Kilometer cancellation
    #[test]
    fn test_kilometer_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // km * ($/km) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(50.0, "km"),
            Expr::number_with_unit(1.5, "$/km"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(75.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 25: Centimeter cancellation
    #[test]
    fn test_centimeter_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // cm * ($/cm) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(100.0, "cm"),
            Expr::number_with_unit(0.5, "$/cm"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(50.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 26: Milligram cancellation
    #[test]
    fn test_milligram_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // mg * ($/mg) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(1000.0, "mg"),
            Expr::number_with_unit(0.01, "$/mg"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(10.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 27: Complex three-way division (documents need for division cancellation)
    #[test]
    fn test_three_way_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // (m * s * kg) / (s * kg) = should be m, but division doesn't cancel yet
        let numerator = Expr::new_multiply(
            Expr::new_multiply(
                Expr::number_with_unit(10.0, "m"),
                Expr::number_with_unit(2.0, "s"),
            ),
            Expr::number_with_unit(5.0, "kg"),
        );
        let denominator = Expr::new_multiply(
            Expr::number_with_unit(2.0, "s"),
            Expr::number_with_unit(5.0, "kg"),
        );
        let expr = Expr::new_divide(numerator, denominator);
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(10.0));
        // Division doesn't automatically cancel, so we'll have compound unit
        assert!(result.unit.canonical().contains("m"));
    }

    // Test 28: Double division creates compound denominator
    #[test]
    fn test_double_division_compound() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // m / s / kg = m/(s*kg)
        let temp = Expr::new_divide(
            Expr::number_with_unit(10.0, "m"),
            Expr::number_with_unit(2.0, "s"),
        );
        let expr = Expr::new_divide(temp, Expr::number_with_unit(5.0, "kg"));
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(1.0));
        assert!(result.unit.canonical().contains("m"));
        assert!(result.unit.canonical().contains("s"));
        assert!(result.unit.canonical().contains("kg"));
    }

    // Test 29: Reverse cancellation (denominator with numerator)
    #[test]
    fn test_reverse_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // (1/m) * m = dimensionless
        let per_meter = Expr::new_divide(Expr::number(5.0), Expr::number_with_unit(1.0, "m"));
        let meters = Expr::number_with_unit(10.0, "m");
        let expr = Expr::new_multiply(per_meter, meters);
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(50.0));
        assert!(result.unit.is_dimensionless());
    }

    // Test 30: Mixed currency EUR (all currencies map to Currency dimension)
    #[test]
    fn test_eur_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // m * (EUR/m) = EUR (note: may be returned as USD due to dimension mapping)
        let expr = Expr::new_multiply(
            Expr::number_with_unit(5.0, "m"),
            Expr::number_with_unit(10.0, "EUR/m"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(50.0));
        // EUR, USD, GBP all map to Currency dimension, so we accept either
        let canonical = result.unit.canonical();
        assert!(
            canonical == "EUR" || canonical == "USD",
            "Got: {}",
            canonical
        );
    }

    // Test 31: GBP cancellation (all currencies map to Currency dimension)
    #[test]
    fn test_gbp_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // kg * (GBP/kg) = GBP (note: may be returned as USD due to dimension mapping)
        let expr = Expr::new_multiply(
            Expr::number_with_unit(10.0, "kg"),
            Expr::number_with_unit(5.0, "GBP/kg"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(50.0));
        // EUR, USD, GBP all map to Currency dimension, so we accept either
        let canonical = result.unit.canonical();
        assert!(
            canonical == "GBP" || canonical == "USD",
            "Got: {}",
            canonical
        );
    }

    // Test 32: Velocity times velocity = velocity squared
    #[test]
    fn test_velocity_squared() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // (m/s) * (m/s) = m²/s²
        let v1 = Expr::new_divide(
            Expr::number_with_unit(10.0, "m"),
            Expr::number_with_unit(1.0, "s"),
        );
        let v2 = Expr::new_divide(
            Expr::number_with_unit(5.0, "m"),
            Expr::number_with_unit(1.0, "s"),
        );
        let expr = Expr::new_multiply(v1, v2);
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(50.0));
        assert!(result.unit.canonical().contains("m"));
        assert!(result.unit.canonical().contains("s"));
    }

    // Test 33: Area divided by length (documents division without auto-cancellation)
    #[test]
    fn test_area_divided_by_length() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // (m * m) / m = should be m, but division doesn't auto-cancel compound units yet
        let area = Expr::new_multiply(
            Expr::number_with_unit(10.0, "m"),
            Expr::number_with_unit(5.0, "m"),
        );
        let length = Expr::number_with_unit(10.0, "m");
        let expr = Expr::new_divide(area, length);
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(5.0));
        // Division doesn't cancel m²/m to m yet, so we'll have a compound unit
        // This test documents that division needs cancellation support
        assert!(result.unit.canonical().contains("m"));
    }

    // Test 34: Volume divided by area (documents division without auto-cancellation)
    #[test]
    fn test_volume_divided_by_area() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // (m³) / (m²) = should be m, but division doesn't auto-cancel compound units yet
        let volume = Expr::new_multiply(
            Expr::new_multiply(
                Expr::number_with_unit(2.0, "m"),
                Expr::number_with_unit(3.0, "m"),
            ),
            Expr::number_with_unit(4.0, "m"),
        );
        let area = Expr::new_multiply(
            Expr::number_with_unit(2.0, "m"),
            Expr::number_with_unit(3.0, "m"),
        );
        let expr = Expr::new_divide(volume, area);
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(4.0));
        // Division doesn't cancel m³/m² to m yet, so we'll have a compound unit
        // This test documents that division needs cancellation support
        assert!(result.unit.canonical().contains("m"));
    }

    // Test 35: Power calculation (Watts = J/s = kg*m²/s³)
    #[test]
    fn test_power_units_no_cancel() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // (kg * m²) / s³
        let kg_m_squared = Expr::new_multiply(
            Expr::number_with_unit(1.0, "kg"),
            Expr::new_multiply(
                Expr::number_with_unit(10.0, "m"),
                Expr::number_with_unit(10.0, "m"),
            ),
        );
        let s_cubed = Expr::new_multiply(
            Expr::new_multiply(
                Expr::number_with_unit(1.0, "s"),
                Expr::number_with_unit(1.0, "s"),
            ),
            Expr::number_with_unit(1.0, "s"),
        );
        let expr = Expr::new_divide(kg_m_squared, s_cubed);
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(100.0));
        assert!(result.unit.canonical().contains("kg"));
        assert!(result.unit.canonical().contains("m"));
        assert!(result.unit.canonical().contains("s"));
    }

    // ===== DIGITAL STORAGE AND TOKEN UNIT TESTS =====

    // Test 36: Simple GB cancellation with currency
    #[test]
    fn test_gb_cancellation_currency() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // GB * ($/GB) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(100.0, "GB"),
            Expr::number_with_unit(0.05, "$/GB"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(5.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 37: MB cancellation with currency
    #[test]
    fn test_mb_cancellation_currency() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // MB * ($/MB) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(1000.0, "MB"),
            Expr::number_with_unit(0.001, "$/MB"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(1.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 38: TB cancellation with currency
    #[test]
    fn test_tb_cancellation_currency() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // TB * ($/TB) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(10.0, "TB"),
            Expr::number_with_unit(5.0, "$/TB"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(50.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 39: GB per hour (bandwidth)
    #[test]
    fn test_gb_per_hour() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // GB / hr
        let expr = Expr::new_divide(
            Expr::number_with_unit(100.0, "GB"),
            Expr::number_with_unit(2.0, "hr"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(50.0));
        assert!(result.unit.canonical().contains("GB"));
        assert!(result.unit.canonical().contains("hr"));
    }

    // Test 40: MB per month (data usage)
    #[test]
    fn test_mb_per_month() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // MB / month
        let expr = Expr::new_divide(
            Expr::number_with_unit(50000.0, "MB"),
            Expr::number_with_unit(1.0, "month"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(50000.0));
        assert!(result.unit.canonical().contains("MB"));
        assert!(result.unit.canonical().contains("month"));
    }

    // Test 41: Currency per GB per hour (cloud pricing)
    #[test]
    fn test_currency_per_gb_per_hour() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // GB * hr * ($/GB/hr) = $
        // Build $/GB/hr by: $ * (1/GB) * (1/hr)
        let gb_hours = Expr::new_multiply(
            Expr::number_with_unit(100.0, "GB"),
            Expr::number_with_unit(24.0, "hr"),
        );
        let dollars = Expr::number_with_unit(0.001, "$");
        let per_gb = Expr::new_divide(Expr::number(1.0), Expr::number_with_unit(1.0, "GB"));
        let per_hr = Expr::new_divide(Expr::number(1.0), Expr::number_with_unit(1.0, "hr"));
        let price_rate = Expr::new_multiply(Expr::new_multiply(dollars, per_gb), per_hr);
        let expr = Expr::new_multiply(gb_hours, price_rate);
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(2.4));
        assert!(
            result.unit.canonical() == "$" || result.unit.canonical() == "USD",
            "Expected $ or USD, got: {}",
            result.unit.canonical()
        );
    }

    // Test 42: Currency per TB per month (storage pricing)
    #[test]
    fn test_currency_per_tb_per_month() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // TB * month * ($/TB/month) = $
        // Build $/TB/month by: $ * (1/TB) * (1/month)
        let tb_months = Expr::new_multiply(
            Expr::number_with_unit(5.0, "TB"),
            Expr::number_with_unit(3.0, "month"),
        );
        let dollars = Expr::number_with_unit(10.0, "$");
        let per_tb = Expr::new_divide(Expr::number(1.0), Expr::number_with_unit(1.0, "TB"));
        let per_month = Expr::new_divide(Expr::number(1.0), Expr::number_with_unit(1.0, "month"));
        let price_rate = Expr::new_multiply(Expr::new_multiply(dollars, per_tb), per_month);
        let expr = Expr::new_multiply(tb_months, price_rate);
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(150.0));
        assert!(
            result.unit.canonical() == "$" || result.unit.canonical() == "USD",
            "Expected $ or USD, got: {}",
            result.unit.canonical()
        );
    }

    // Test 43: Token cancellation with currency
    #[test]
    fn test_tok_cancellation_currency() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // Tok * ($/Tok) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(1000.0, "Tok"),
            Expr::number_with_unit(0.00002, "$/Tok"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(0.02));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 44: KTok cancellation with currency
    #[test]
    fn test_ktok_cancellation_currency() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // KTok * ($/KTok) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(100.0, "KTok"),
            Expr::number_with_unit(0.02, "$/KTok"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(2.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 45: MTok cancellation with currency
    #[test]
    fn test_mtok_cancellation_currency() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // MTok * ($/MTok) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(10.0, "MTok"),
            Expr::number_with_unit(20.0, "$/MTok"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(200.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 46: MTok per hour (token usage rate)
    #[test]
    fn test_mtok_per_hour() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // MTok / hr
        let expr = Expr::new_divide(
            Expr::number_with_unit(5.0, "MTok"),
            Expr::number_with_unit(10.0, "hr"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(0.5));
        assert!(result.unit.canonical().contains("MTok"));
        assert!(result.unit.canonical().contains("hr"));
    }

    // Test 47: Currency per MTok per hour (API pricing)
    #[test]
    fn test_currency_per_mtok_per_hour() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // MTok * hr * ($/MTok/hr) = $
        // Build $/MTok/hr by: $ * (1/MTok) * (1/hr)
        let mtok_hours = Expr::new_multiply(
            Expr::number_with_unit(10.0, "MTok"),
            Expr::number_with_unit(24.0, "hr"),
        );
        let dollars = Expr::number_with_unit(0.5, "$");
        let per_mtok = Expr::new_divide(Expr::number(1.0), Expr::number_with_unit(1.0, "MTok"));
        let per_hr = Expr::new_divide(Expr::number(1.0), Expr::number_with_unit(1.0, "hr"));
        let price_rate = Expr::new_multiply(Expr::new_multiply(dollars, per_mtok), per_hr);
        let expr = Expr::new_multiply(mtok_hours, price_rate);
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(120.0));
        assert!(
            result.unit.canonical() == "$" || result.unit.canonical() == "USD",
            "Expected $ or USD, got: {}",
            result.unit.canonical()
        );
    }

    // Test 48: Mbits per hour (network bandwidth)
    #[test]
    fn test_mbits_per_hour() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // Mbits / hr
        let expr = Expr::new_divide(
            Expr::number_with_unit(1000.0, "Mbits"),
            Expr::number_with_unit(1.0, "hr"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(1000.0));
        assert!(result.unit.canonical().contains("Mbits"));
        assert!(result.unit.canonical().contains("hr"));
    }

    // Test 49: Gbits per month (bandwidth allocation)
    #[test]
    fn test_gbits_per_month() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // Gbits / month
        let expr = Expr::new_divide(
            Expr::number_with_unit(10000.0, "Gbits"),
            Expr::number_with_unit(1.0, "month"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(10000.0));
        assert!(result.unit.canonical().contains("Gbits"));
        assert!(result.unit.canonical().contains("month"));
    }

    // Test 50: Currency per Gbits (bandwidth pricing)
    #[test]
    fn test_currency_per_gbits() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // Gbits * ($/Gbits) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(500.0, "Gbits"),
            Expr::number_with_unit(0.1, "$/Gbits"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(50.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 51: GB squared (area-like, for testing powers)
    #[test]
    fn test_gb_squared() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // GB * GB = GB² (symbol is preserved when multiplying same units)
        let expr = Expr::new_multiply(
            Expr::number_with_unit(10.0, "GB"),
            Expr::number_with_unit(5.0, "GB"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(50.0));
        // System preserves original symbol when multiplying same units
        assert_eq!(result.unit.canonical(), "GB^2");
    }

    // Test 52: GB squared divided by GB (partial cancellation)
    #[test]
    fn test_gb_squared_divided_by_gb() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // GB² / GB = GB (note: division doesn't auto-cancel compound units)
        let gb_squared = Expr::new_multiply(
            Expr::number_with_unit(10.0, "GB"),
            Expr::number_with_unit(10.0, "GB"),
        );
        let expr = Expr::new_divide(gb_squared, Expr::number_with_unit(10.0, "GB"));
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(10.0));
        // Division doesn't auto-cancel, so result will contain GB
        assert!(result.unit.canonical().contains("GB") || result.unit.canonical().contains("B"));
    }

    // Test 53: MB squared cancellation
    #[test]
    fn test_mb_squared_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // MB² * (1/MB²) = dimensionless
        let mb_squared = Expr::new_multiply(
            Expr::number_with_unit(10.0, "MB"),
            Expr::number_with_unit(10.0, "MB"),
        );
        // Create (1/MB) * (1/MB) to get 1/MB²
        let per_mb1 = Expr::new_divide(Expr::number(1.0), Expr::number_with_unit(1.0, "MB"));
        let per_mb2 = Expr::new_divide(Expr::number(1.0), Expr::number_with_unit(1.0, "MB"));
        let per_mb_squared = Expr::new_multiply(per_mb1, per_mb2);
        let expr = Expr::new_multiply(mb_squared, per_mb_squared);
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(100.0));
        assert!(result.unit.is_dimensionless());
    }

    // Test 54: TB cubed (volume-like, for testing higher powers)
    #[test]
    fn test_tb_cubed() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // TB * TB * TB = TB³ (symbol is preserved when multiplying same units)
        let tb_times_tb = Expr::new_multiply(
            Expr::number_with_unit(2.0, "TB"),
            Expr::number_with_unit(3.0, "TB"),
        );
        let expr = Expr::new_multiply(tb_times_tb, Expr::number_with_unit(5.0, "TB"));
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(30.0));
        // System preserves original symbol when multiplying same units
        assert_eq!(result.unit.canonical(), "TB^3");
    }

    // Test 55: Bytes cancellation (lowercase b)
    #[test]
    fn test_bytes_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // b * ($/b) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(10000.0, "b"),
            Expr::number_with_unit(0.0001, "$/b"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(1.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 56: KB cancellation (kilobyte)
    #[test]
    fn test_kb_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // KB * ($/KB) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(5000.0, "KB"),
            Expr::number_with_unit(0.0002, "$/KB"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(1.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 57: PB cancellation (petabyte)
    #[test]
    fn test_pb_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // PB * ($/PB) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(1.0, "PB"),
            Expr::number_with_unit(500.0, "$/PB"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(500.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 58: Kbits cancellation
    #[test]
    fn test_kbits_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // Kbits * ($/Kbits) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(10000.0, "Kbits"),
            Expr::number_with_unit(0.0001, "$/Kbits"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(1.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 59: Tbits cancellation
    #[test]
    fn test_tbits_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // Tbits * ($/Tbits) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(10.0, "Tbits"),
            Expr::number_with_unit(5.0, "$/Tbits"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(50.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 60: Hour times GB/hr = B (time cancellation)
    #[test]
    fn test_hour_times_gb_per_hr() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // hr * (GB/hr) = B (note: all digital storage units map to DigitalStorage dimension with symbol "B")
        let expr = Expr::new_multiply(
            Expr::number_with_unit(24.0, "hr"),
            Expr::number_with_unit(50.0, "GB/hr"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(1200.0));
        // DigitalStorage dimension uses "B" as the standard symbol
        assert!(result.unit.canonical() == "B" || result.unit.canonical() == "GB");
    }

    // Test 61: Month times TB/month = B (time cancellation)
    #[test]
    fn test_month_times_tb_per_month() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // month * (TB/month) = B (note: all digital storage units map to DigitalStorage dimension with symbol "B")
        let expr = Expr::new_multiply(
            Expr::number_with_unit(3.0, "month"),
            Expr::number_with_unit(100.0, "TB/month"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(300.0));
        // DigitalStorage dimension uses "B" as the standard symbol
        assert!(result.unit.canonical() == "B" || result.unit.canonical() == "TB");
    }

    // Test 62: Hour times MTok/hr = B (time cancellation with tokens)
    #[test]
    fn test_hour_times_mtok_per_hr() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // hr * (MTok/hr) = B (note: all digital storage/token units map to DigitalStorage dimension with symbol "B")
        let expr = Expr::new_multiply(
            Expr::number_with_unit(10.0, "hr"),
            Expr::number_with_unit(5.0, "MTok/hr"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(50.0));
        // DigitalStorage dimension uses "B" as the standard symbol
        assert!(result.unit.canonical() == "B" || result.unit.canonical() == "MTok");
    }

    // Test 63: Complex multi-dimension: GB/hr/$ (GB per hour per dollar)
    #[test]
    fn test_gb_per_hr_per_dollar() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // GB * $ * hr * (GB/hr/$) = GB²
        // $ and hr cancel, but GB appears in both numerator and efficiency, so we get GB²
        // Build GB/hr/$ by: GB * (1/hr) * (1/$)
        let usage = Expr::new_multiply(
            Expr::new_multiply(
                Expr::number_with_unit(100.0, "GB"),
                Expr::number_with_unit(10.0, "$"),
            ),
            Expr::number_with_unit(24.0, "hr"),
        );
        let gb = Expr::number_with_unit(1.0, "GB");
        let per_hr = Expr::new_divide(Expr::number(1.0), Expr::number_with_unit(1.0, "hr"));
        let per_dollar = Expr::new_divide(Expr::number(1.0), Expr::number_with_unit(1.0, "$"));
        let efficiency = Expr::new_multiply(Expr::new_multiply(gb, per_hr), per_dollar);
        let expr = Expr::new_multiply(usage, efficiency);
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(24000.0));
        // $ and hr cancel, leaving GB²
        assert!(
            result.unit.canonical().contains("B"),
            "Expected units containing B (like B^2 or GB^2), got: {}",
            result.unit.canonical()
        );
    }

    // Test 64: GB²/hr (compound squared with time)
    #[test]
    fn test_gb_squared_per_hour() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // (GB²) / hr = GB²/hr
        let gb_squared = Expr::new_multiply(
            Expr::number_with_unit(10.0, "GB"),
            Expr::number_with_unit(10.0, "GB"),
        );
        let expr = Expr::new_divide(gb_squared, Expr::number_with_unit(2.0, "hr"));
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(50.0));
        assert!(result.unit.canonical().contains("B"));
        assert!(result.unit.canonical().contains("hr"));
    }

    // Test 65: MTok²/month (token squared per month)
    #[test]
    fn test_mtok_squared_per_month() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // (MTok²) / month = MTok²/month
        let mtok_squared = Expr::new_multiply(
            Expr::number_with_unit(5.0, "MTok"),
            Expr::number_with_unit(4.0, "MTok"),
        );
        let expr = Expr::new_divide(mtok_squared, Expr::number_with_unit(1.0, "month"));
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(20.0));
        assert!(result.unit.canonical().contains("MTok") || result.unit.canonical().contains("B"));
        assert!(result.unit.canonical().contains("month"));
    }

    // Test 66: Bytes (uppercase B) cancellation
    #[test]
    fn test_bytes_uppercase_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // B * ($/B) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(1000000.0, "B"),
            Expr::number_with_unit(0.00001, "$/B"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(10.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 67: bits (lowercase) cancellation
    #[test]
    fn test_bits_lowercase_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // bits * ($/bits) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(1000000.0, "bits"),
            Expr::number_with_unit(0.00001, "$/bits"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(10.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 68: tok (lowercase) cancellation
    #[test]
    fn test_tok_lowercase_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // tok * ($/tok) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(5000.0, "tok"),
            Expr::number_with_unit(0.00001, "$/tok"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(0.05));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 69: Ktok (lowercase k) cancellation
    #[test]
    fn test_ktok_lowercase_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // Ktok * ($/Ktok) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(200.0, "Ktok"),
            Expr::number_with_unit(0.01, "$/Ktok"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(2.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }

    // Test 70: Mtok (lowercase m) cancellation
    #[test]
    fn test_mtok_lowercase_cancellation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // Mtok * ($/Mtok) = $
        let expr = Expr::new_multiply(
            Expr::number_with_unit(50.0, "Mtok"),
            Expr::number_with_unit(10.0, "$/Mtok"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, EvalValue::Number(500.0));
        assert!(result.unit.canonical() == "$" || result.unit.canonical() == "USD");
    }
}
