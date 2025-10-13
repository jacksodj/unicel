// Formula evaluator with unit-aware operations

use super::ast::Expr;
use crate::core::units::{BaseDimension, Unit, UnitLibrary};
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

    /// Multiply two values (creates compound units)
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

        // For MLP: create a simple compound unit representation
        // Format: "left*right" (e.g., "m*m" for area)
        let compound_symbol = format!("{}*{}", left_result.unit.canonical(), right_result.unit.canonical());

        // For now, create compound units for common cases
        // Full compound unit support will be added post-MLP
        let compound_unit = create_compound_unit(&left_result.unit, &right_result.unit, &compound_symbol);

        Ok(EvalResult::new(value, compound_unit))
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

// Helper function to create compound units for multiplication
fn create_compound_unit(left: &Unit, right: &Unit, symbol: &str) -> Unit {
    // For simple dimensions, create compound unit
    if let (Some(left_dim), Some(right_dim)) = (left.dimension().as_simple(), right.dimension().as_simple()) {
        Unit::compound(
            symbol.to_string(),
            vec![(left_dim.clone(), 1), (right_dim.clone(), 1)],
            vec![],
        )
    } else {
        // Fallback: create custom dimension
        Unit::simple(symbol, BaseDimension::Custom(symbol.to_string()))
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
        // Result should be m*m (area)
        assert_eq!(result.unit.canonical(), "m*m");
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
