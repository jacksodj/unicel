// Tests for CEILING formula function

use unicel_lib::core::formula::{parse_formula, Evaluator};
use unicel_lib::core::units::UnitLibrary;

#[test]
fn test_ceiling_basic_positive() {
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    // CEILING(4.3) should be 5
    let expr = parse_formula("CEILING(4.3)").unwrap();
    let result = eval.eval(&expr).unwrap();
    assert_eq!(result.as_number(), Some(5.0));
    assert!(result.unit.is_dimensionless());
}

#[test]
fn test_ceiling_basic_negative() {
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    // CEILING(-4.3) should be -4 (rounds toward zero for negative numbers)
    let expr = parse_formula("CEILING(-4.3)").unwrap();
    let result = eval.eval(&expr).unwrap();
    assert_eq!(result.as_number(), Some(-4.0));
    assert!(result.unit.is_dimensionless());
}

#[test]
fn test_ceiling_already_integer() {
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    // CEILING(5) should be 5
    let expr = parse_formula("CEILING(5)").unwrap();
    let result = eval.eval(&expr).unwrap();
    assert_eq!(result.as_number(), Some(5.0));
}

#[test]
fn test_ceiling_zero() {
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    // CEILING(0) should be 0
    let expr = parse_formula("CEILING(0)").unwrap();
    let result = eval.eval(&expr).unwrap();
    assert_eq!(result.as_number(), Some(0.0));
}

#[test]
fn test_ceiling_with_significance() {
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    // CEILING(4.3, 0.5) should be 4.5
    let expr = parse_formula("CEILING(4.3, 0.5)").unwrap();
    let result = eval.eval(&expr).unwrap();
    assert_eq!(result.as_number(), Some(4.5));
}

#[test]
fn test_ceiling_with_integer_significance() {
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    // CEILING(12, 5) should be 15
    let expr = parse_formula("CEILING(12, 5)").unwrap();
    let result = eval.eval(&expr).unwrap();
    assert_eq!(result.as_number(), Some(15.0));

    // CEILING(12.7, 5) should be 15
    let expr = parse_formula("CEILING(12.7, 5)").unwrap();
    let result = eval.eval(&expr).unwrap();
    assert_eq!(result.as_number(), Some(15.0));
}

#[test]
fn test_ceiling_with_negative_significance() {
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    // CEILING(4.3, -0.5) - negative significance works like positive abs value
    let expr = parse_formula("CEILING(4.3, -0.5)").unwrap();
    let result = eval.eval(&expr).unwrap();
    // ceil(4.3 / -0.5) * -0.5 = ceil(-8.6) * -0.5 = -8 * -0.5 = 4.0
    assert_eq!(result.as_number(), Some(4.0));
}

#[test]
fn test_ceiling_preserves_units() {
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    // CEILING(4.3m) should preserve meters
    let expr = parse_formula("CEILING(4.3m)").unwrap();
    let result = eval.eval(&expr).unwrap();
    assert_eq!(result.as_number(), Some(5.0));
    assert_eq!(result.unit.canonical(), "m");
}

#[test]
fn test_ceiling_with_units_and_dimensionless_significance() {
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    // CEILING(12.7m, 5) should preserve meters
    let expr = parse_formula("CEILING(12.7m, 5)").unwrap();
    let result = eval.eval(&expr).unwrap();
    assert_eq!(result.as_number(), Some(15.0));
    assert_eq!(result.unit.canonical(), "m");
}

#[test]
fn test_ceiling_with_compatible_units() {
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    // CEILING(100cm, 0.5m) should convert and preserve cm
    let expr = parse_formula("CEILING(100cm, 0.5m)").unwrap();
    let result = eval.eval(&expr).unwrap();
    // 100cm = 1m, ceil(1/0.5) * 0.5 = 1, result in cm = 100cm
    assert_eq!(result.as_number(), Some(100.0));
    assert_eq!(result.unit.canonical(), "cm");
}

#[test]
fn test_ceiling_with_incompatible_units() {
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    // CEILING(10m, 5kg) should error
    let expr = parse_formula("CEILING(10m, 5kg)").unwrap();
    let result = eval.eval(&expr);
    assert!(result.is_err());
}

#[test]
fn test_ceiling_zero_significance_error() {
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    // CEILING(10, 0) should error
    let expr = parse_formula("CEILING(10, 0)").unwrap();
    let result = eval.eval(&expr);
    assert!(result.is_err());
}

#[test]
fn test_ceiling_no_arguments_error() {
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    // CEILING() should error
    let expr = parse_formula("CEILING()").unwrap();
    let result = eval.eval(&expr);
    assert!(result.is_err());
}

#[test]
fn test_ceiling_too_many_arguments_error() {
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    // CEILING(1, 2, 3) should error
    let expr = parse_formula("CEILING(1, 2, 3)").unwrap();
    let result = eval.eval(&expr);
    assert!(result.is_err());
}

#[test]
fn test_ceiling_text_argument_error() {
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    // CEILING("hello") should error
    let expr = parse_formula("CEILING(\"hello\")").unwrap();
    let result = eval.eval(&expr);
    assert!(result.is_err());
}

#[test]
fn test_ceiling_very_small_numbers() {
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    // CEILING(0.0001) should be 1
    let expr = parse_formula("CEILING(0.0001)").unwrap();
    let result = eval.eval(&expr).unwrap();
    assert_eq!(result.as_number(), Some(1.0));

    // CEILING(0.0001, 0.001) should be 0.001
    let expr = parse_formula("CEILING(0.0001, 0.001)").unwrap();
    let result = eval.eval(&expr).unwrap();
    assert_eq!(result.as_number(), Some(0.001));
}

#[test]
fn test_ceiling_large_numbers() {
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    // CEILING(1234567.89) should be 1234568
    let expr = parse_formula("CEILING(1234567.89)").unwrap();
    let result = eval.eval(&expr).unwrap();
    assert_eq!(result.as_number(), Some(1234568.0));

    // CEILING(1234567.89, 100) should be 1234600
    let expr = parse_formula("CEILING(1234567.89, 100)").unwrap();
    let result = eval.eval(&expr).unwrap();
    assert_eq!(result.as_number(), Some(1234600.0));
}

#[test]
fn test_ceiling_in_expression() {
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    // CEILING(4.3) + 5 should be 10
    let expr = parse_formula("CEILING(4.3) + 5").unwrap();
    let result = eval.eval(&expr).unwrap();
    assert_eq!(result.as_number(), Some(10.0));

    // CEILING(4.3 + 0.5) should be 5
    let expr = parse_formula("CEILING(4.3 + 0.5)").unwrap();
    let result = eval.eval(&expr).unwrap();
    assert_eq!(result.as_number(), Some(5.0));
}

#[test]
fn test_ceiling_nested() {
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    // CEILING(CEILING(4.1)) should be 5
    let expr = parse_formula("CEILING(CEILING(4.1))").unwrap();
    let result = eval.eval(&expr).unwrap();
    assert_eq!(result.as_number(), Some(5.0));
}

#[test]
fn test_ceiling_currency_units() {
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    // CEILING(12.34USD, 0.25USD) should be 12.50USD
    let expr = parse_formula("CEILING(12.34USD, 0.25USD)").unwrap();
    let result = eval.eval(&expr).unwrap();
    assert_eq!(result.as_number(), Some(12.5));
    assert_eq!(result.unit.canonical(), "USD");
}

#[test]
fn test_ceiling_temperature_units() {
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    // CEILING(25.3C) should preserve Celsius
    let expr = parse_formula("CEILING(25.3C)").unwrap();
    let result = eval.eval(&expr).unwrap();
    assert_eq!(result.as_number(), Some(26.0));
    assert_eq!(result.unit.canonical(), "C");
}
