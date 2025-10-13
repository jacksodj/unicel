// Formula evaluator with unit-aware operations

use super::ast::Expr;
use crate::core::units::{BaseDimension, Dimension, Unit, UnitLibrary};
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
    /// The computed value
    pub value: f64,

    /// The unit of the result
    pub unit: Unit,

    /// Optional warning message (for incompatible but allowed operations)
    pub warning: Option<String>,
}

impl EvalResult {
    pub fn new(value: f64, unit: Unit) -> Self {
        Self {
            value,
            unit,
            warning: None,
        }
    }

    pub fn with_warning(mut self, warning: String) -> Self {
        self.warning = Some(warning);
        self
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
                // Parse the unit
                if !self.library.contains(unit) {
                    return Err(EvalError::UnknownUnit(unit.clone()));
                }
                let unit_obj = self.library.get(unit).unwrap().clone();
                Ok(EvalResult::new(*value, unit_obj))
            }

            Expr::Add(left, right) => self.eval_add(left, right),
            Expr::Subtract(left, right) => self.eval_subtract(left, right),
            Expr::Multiply(left, right) => self.eval_multiply(left, right),
            Expr::Divide(left, right) => self.eval_divide(left, right),

            Expr::Negate(expr) => {
                let result = self.eval(expr)?;
                Ok(EvalResult::new(-result.value, result.unit))
            }

            Expr::CellRef { .. } => {
                Err(EvalError::CellNotFound("Cell references not supported in standalone evaluation".to_string()))
            }

            Expr::Range { .. } => {
                Err(EvalError::InvalidOperation("Ranges can only be used in functions".to_string()))
            }

            Expr::Function { name, .. } => {
                Err(EvalError::FunctionNotImplemented(name.clone()))
            }
        }
    }

    /// Add two values (requires compatible units)
    fn eval_add(&self, left: &Expr, right: &Expr) -> Result<EvalResult, EvalError> {
        let left_result = self.eval(left)?;
        let right_result = self.eval(right)?;

        // Both dimensionless - simple addition
        if left_result.unit.is_dimensionless() && right_result.unit.is_dimensionless() {
            return Ok(EvalResult::new(
                left_result.value + right_result.value,
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
                left_result.value + right_result.value,
                left_result.unit.clone(),
            ));
        }

        // Units are compatible but different - convert right to left's unit
        let right_value_converted = self.library.convert(
            right_result.value,
            right_result.unit.canonical(),
            left_result.unit.canonical(),
        ).ok_or_else(|| EvalError::IncompatibleUnits {
            operation: "add".to_string(),
            left: left_result.unit.to_string(),
            right: right_result.unit.to_string(),
        })?;

        Ok(EvalResult::new(
            left_result.value + right_value_converted,
            left_result.unit.clone(),
        ))
    }

    /// Subtract two values (requires compatible units)
    fn eval_subtract(&self, left: &Expr, right: &Expr) -> Result<EvalResult, EvalError> {
        let left_result = self.eval(left)?;
        let right_result = self.eval(right)?;

        // Both dimensionless - simple subtraction
        if left_result.unit.is_dimensionless() && right_result.unit.is_dimensionless() {
            return Ok(EvalResult::new(
                left_result.value - right_result.value,
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
                left_result.value - right_result.value,
                left_result.unit.clone(),
            ));
        }

        // Units are compatible but different - convert right to left's unit
        let right_value_converted = self.library.convert(
            right_result.value,
            right_result.unit.canonical(),
            left_result.unit.canonical(),
        ).ok_or_else(|| EvalError::IncompatibleUnits {
            operation: "subtract".to_string(),
            left: left_result.unit.to_string(),
            right: right_result.unit.to_string(),
        })?;

        Ok(EvalResult::new(
            left_result.value - right_value_converted,
            left_result.unit.clone(),
        ))
    }

    /// Multiply two values (creates compound units with cancellation)
    fn eval_multiply(&self, left: &Expr, right: &Expr) -> Result<EvalResult, EvalError> {
        let left_result = self.eval(left)?;
        let right_result = self.eval(right)?;

        let value = left_result.value * right_result.value;

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

        // Multiply units with cancellation
        let result_unit = multiply_units_with_cancellation(&left_result.unit, &right_result.unit);

        Ok(EvalResult::new(value, result_unit))
    }

    /// Divide two values (creates compound units with cancellation)
    fn eval_divide(&self, left: &Expr, right: &Expr) -> Result<EvalResult, EvalError> {
        let left_result = self.eval(left)?;
        let right_result = self.eval(right)?;

        if right_result.value == 0.0 {
            return Err(EvalError::DivisionByZero);
        }

        let value = left_result.value / right_result.value;

        // If both dimensionless, result is dimensionless
        if left_result.unit.is_dimensionless() && right_result.unit.is_dimensionless() {
            return Ok(EvalResult::new(value, Unit::dimensionless()));
        }

        // If left is dimensionless, result is 1/right_unit
        if left_result.unit.is_dimensionless() {
            let compound_symbol = format!("1/{}", right_result.unit.canonical());
            let compound_unit = Unit::compound(
                compound_symbol.clone(),
                vec![],
                vec![(right_result.unit.dimension().as_simple().unwrap().clone(), 1)],
            );
            return Ok(EvalResult::new(value, compound_unit));
        }

        // If right is dimensionless, result has left's unit
        if right_result.unit.is_dimensionless() {
            return Ok(EvalResult::new(value, left_result.unit.clone()));
        }

        // If units are the same, they cancel out
        if left_result.unit.is_equal(&right_result.unit) {
            return Ok(EvalResult::new(value, Unit::dimensionless()));
        }

        // For MLP: create a simple compound unit representation
        // Format: "left/right" (e.g., "m/s" for velocity)
        let compound_symbol = format!("{}/{}", left_result.unit.canonical(), right_result.unit.canonical());

        let compound_unit = create_division_unit(&left_result.unit, &right_result.unit, &compound_symbol);

        Ok(EvalResult::new(value, compound_unit))
    }
}

// Helper function to create compound units for division
fn create_division_unit(left: &Unit, right: &Unit, symbol: &str) -> Unit {
    // For simple dimensions, create compound unit
    if let (Some(left_dim), Some(right_dim)) = (left.dimension().as_simple(), right.dimension().as_simple()) {
        Unit::compound(
            symbol.to_string(),
            vec![(left_dim.clone(), 1)],
            vec![(right_dim.clone(), 1)],
        )
    } else {
        // Fallback: create custom dimension
        Unit::simple(symbol, BaseDimension::Custom(symbol.to_string()))
    }
}

// Helper function to multiply units with dimensional cancellation
fn multiply_units_with_cancellation(left: &Unit, right: &Unit) -> Unit {
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

    // Build result unit
    build_unit_from_dimensions(num_dims, den_dims)
}

// Extract dimensions from a unit into numerator and denominator maps
fn extract_dimensions(unit: &Unit) -> (HashMap<BaseDimension, i32>, HashMap<BaseDimension, i32>) {
    let mut numerator = HashMap::new();
    let mut denominator = HashMap::new();

    match unit.dimension() {
        Dimension::Dimensionless => {},
        Dimension::Simple(base) => {
            numerator.insert(base.clone(), 1);
        },
        Dimension::Compound { numerator: num, denominator: den } => {
            for (base, power) in num {
                *numerator.entry(base.clone()).or_insert(0) += power;
            }
            for (base, power) in den {
                *denominator.entry(base.clone()).or_insert(0) += power;
            }
        },
    }

    (numerator, denominator)
}

// Build a unit from dimension maps
fn build_unit_from_dimensions(
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
fn build_unit_symbol(numerator: &HashMap<BaseDimension, i32>, denominator: &HashMap<BaseDimension, i32>) -> String {
    let mut parts = Vec::new();

    // Build numerator
    let mut num_symbols: Vec<_> = numerator.iter().map(|(d, p)| (get_standard_symbol(d), p)).collect();
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
    let mut den_symbols: Vec<_> = denominator.iter().map(|(d, p)| (get_standard_symbol(d), p)).collect();
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
        assert_eq!(result.value, 42.0);
        assert!(result.unit.is_dimensionless());
    }

    #[test]
    fn test_eval_number_with_unit() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        let expr = Expr::number_with_unit(100.0, "m");
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, 100.0);
        assert_eq!(result.unit.canonical(), "m");
    }

    #[test]
    fn test_add_same_units() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        let expr = Expr::add(
            Expr::number_with_unit(100.0, "m"),
            Expr::number_with_unit(50.0, "m"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, 150.0);
        assert_eq!(result.unit.canonical(), "m");
    }

    #[test]
    fn test_add_compatible_units() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        let expr = Expr::add(
            Expr::number_with_unit(100.0, "m"),
            Expr::number_with_unit(50.0, "cm"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, 100.5); // 100m + 0.5m
        assert_eq!(result.unit.canonical(), "m");
    }

    #[test]
    fn test_add_incompatible_units() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        let expr = Expr::add(
            Expr::number_with_unit(100.0, "m"),
            Expr::number_with_unit(50.0, "kg"),
        );
        let result = eval.eval(&expr);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EvalError::IncompatibleUnits { .. }));
    }

    #[test]
    fn test_subtract_same_units() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        let expr = Expr::subtract(
            Expr::number_with_unit(100.0, "m"),
            Expr::number_with_unit(30.0, "m"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, 70.0);
        assert_eq!(result.unit.canonical(), "m");
    }

    #[test]
    fn test_multiply_creates_compound() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        let expr = Expr::multiply(
            Expr::number_with_unit(10.0, "m"),
            Expr::number_with_unit(5.0, "m"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, 50.0);
        // Result should be m^2 (area, simplified)
        assert_eq!(result.unit.canonical(), "m^2");
    }

    #[test]
    fn test_multiply_with_dimensionless() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        let expr = Expr::multiply(
            Expr::number_with_unit(10.0, "m"),
            Expr::number(2.0),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, 20.0);
        assert_eq!(result.unit.canonical(), "m");
    }

    #[test]
    fn test_divide_same_units() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        let expr = Expr::divide(
            Expr::number_with_unit(100.0, "m"),
            Expr::number_with_unit(10.0, "m"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, 10.0);
        assert!(result.unit.is_dimensionless()); // Units cancel
    }

    #[test]
    fn test_divide_creates_compound() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        let expr = Expr::divide(
            Expr::number_with_unit(100.0, "m"),
            Expr::number_with_unit(2.0, "s"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, 50.0);
        // Result should be m/s (velocity)
        assert_eq!(result.unit.canonical(), "m/s");
    }

    #[test]
    fn test_divide_by_zero() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        let expr = Expr::divide(
            Expr::number(10.0),
            Expr::number(0.0),
        );
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
        assert_eq!(result.value, -50.0);
        assert_eq!(result.unit.canonical(), "m");
    }

    #[test]
    fn test_complex_expression() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // (100m + 50m) * 2
        let expr = Expr::multiply(
            Expr::add(
                Expr::number_with_unit(100.0, "m"),
                Expr::number_with_unit(50.0, "m"),
            ),
            Expr::number(2.0),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, 300.0);
        assert_eq!(result.unit.canonical(), "m");
    }

    #[test]
    fn test_velocity_calculation() {
        let library = UnitLibrary::new();
        let eval = Evaluator::new(&library);
        // 100mi / 2hr = 50 mi/hr
        let expr = Expr::divide(
            Expr::number_with_unit(100.0, "mi"),
            Expr::number_with_unit(2.0, "hr"),
        );
        let result = eval.eval(&expr).unwrap();
        assert_eq!(result.value, 50.0);
        assert_eq!(result.unit.canonical(), "mi/hr");
    }
}
