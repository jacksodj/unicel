// Tests for conversion-aware arithmetic
// Tests the critical TB/GB cross-scale multiplication and GB cancellation issues

use unicel_lib::core::formula::ast::Expr;
use unicel_lib::core::formula::evaluator::{EvalValue, Evaluator};
use unicel_lib::core::units::UnitLibrary;

#[test]
fn test_tb_times_dollar_per_gb() {
    // Bug case: 100 TB * 15 $/GB should be 1,536,000 $ (100 * 1024 * 15)
    // NOT 1,500 $ (which would be 100 * 15 ignoring conversion)
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    let expr = Expr::new_multiply(
        Expr::number_with_unit(100.0, "TB"),
        Expr::number_with_unit(15.0, "$/GB"),
    );

    let result = eval.eval(&expr).unwrap();

    // Expected: 100 TB = 102,400 GB
    // 102,400 GB * 15 $/GB = 1,536,000 $
    assert_eq!(result.value, EvalValue::Number(1_536_000.0));
    assert!(
        result.unit.canonical() == "$" || result.unit.canonical() == "USD",
        "Expected $ or USD, got: {}",
        result.unit.canonical()
    );
}

#[test]
fn test_gb_times_dollar_per_gb() {
    // Control case: 100 GB * 15 $/GB = 1,500 $
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    let expr = Expr::new_multiply(
        Expr::number_with_unit(100.0, "GB"),
        Expr::number_with_unit(15.0, "$/GB"),
    );

    let result = eval.eval(&expr).unwrap();

    assert_eq!(result.value, EvalValue::Number(1_500.0));
    assert!(
        result.unit.canonical() == "$" || result.unit.canonical() == "USD",
        "Expected $ or USD, got: {}",
        result.unit.canonical()
    );
}

#[test]
fn test_gb_times_dollar_per_gb_month() {
    // Bug case: 100 GB * 0.005 $/GB·Month should give 0.5 $/Month
    // GB should cancel out completely
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    // Build the rate: $/GB/month = $ / (GB * month)
    let dollars = Expr::number_with_unit(0.005, "$");
    let per_gb = Expr::new_divide(Expr::number(1.0), Expr::number_with_unit(1.0, "GB"));
    let per_month = Expr::new_divide(Expr::number(1.0), Expr::number_with_unit(1.0, "month"));
    let rate = Expr::new_multiply(Expr::new_multiply(dollars, per_gb), per_month);

    // 100 GB * rate
    let expr = Expr::new_multiply(Expr::number_with_unit(100.0, "GB"), rate);

    let result = eval.eval(&expr).unwrap();

    assert_eq!(result.value, EvalValue::Number(0.5));

    // The result should be $/month (GB should cancel)
    let canonical = result.unit.canonical();
    assert!(
        !canonical.contains("GB") && !canonical.contains("B"),
        "GB should cancel out, got: {}",
        canonical
    );
    assert!(
        canonical.contains("$") || canonical.contains("USD"),
        "Should contain currency, got: {}",
        canonical
    );
    assert!(
        canonical.contains("month"),
        "Should contain month, got: {}",
        canonical
    );
}

#[test]
fn test_minute_minus_seconds() {
    // Addition/subtraction should align to finer unit
    // 1 min - 15 sec = 45 sec (not 0.75 min)
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    let expr = Expr::new_subtract(
        Expr::number_with_unit(1.0, "min"),
        Expr::number_with_unit(15.0, "s"),
    );

    let result = eval.eval(&expr).unwrap();

    // Expected: converted to seconds (finer unit)
    // 1 min = 60 sec, 60 - 15 = 45 sec
    assert_eq!(result.value, EvalValue::Number(45.0));
    assert_eq!(result.unit.canonical(), "s");
}

#[test]
fn test_cross_scale_cancellation_mb_to_kb() {
    // 100 MB * $/MB should work even if price is in different scale
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    let expr = Expr::new_multiply(
        Expr::number_with_unit(100.0, "MB"),
        Expr::number_with_unit(0.001, "$/KB"),
    );

    let result = eval.eval(&expr).unwrap();

    // 100 MB = 102,400 KB
    // 102,400 KB * 0.001 $/KB = 102.4 $
    assert_eq!(result.value, EvalValue::Number(102.4));
    assert!(
        result.unit.canonical() == "$" || result.unit.canonical() == "USD",
        "Expected $ or USD, got: {}",
        result.unit.canonical()
    );
}

#[test]
fn test_exponent_handling_in_cancellation() {
    // ft^2 * $/ft should leave $/ft (one ft cancels)
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    let expr = Expr::new_multiply(
        Expr::new_multiply(
            Expr::number_with_unit(10.0, "ft"),
            Expr::number_with_unit(10.0, "ft"),
        ),
        Expr::number_with_unit(5.0, "$/ft"),
    );

    let result = eval.eval(&expr).unwrap();

    // 10 ft * 10 ft = 100 ft^2
    // 100 ft^2 * 5 $/ft = 500 $/ft (one ft cancels from ft^2)
    assert_eq!(result.value, EvalValue::Number(500.0));

    let canonical = result.unit.canonical();
    assert!(
        canonical.contains("$") || canonical.contains("USD"),
        "Should contain currency, got: {}",
        canonical
    );
    assert!(
        canonical.contains("ft") || canonical.contains("m"),
        "Should contain length unit, got: {}",
        canonical
    );
}

#[test]
fn test_complete_gb_squared_cancellation() {
    // GB^2 * 1/GB^2 should give dimensionless
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    let gb_squared = Expr::new_multiply(
        Expr::number_with_unit(10.0, "GB"),
        Expr::number_with_unit(5.0, "GB"),
    );

    let per_gb_squared = Expr::new_divide(
        Expr::new_divide(Expr::number(2.0), Expr::number_with_unit(1.0, "GB")),
        Expr::number_with_unit(1.0, "GB"),
    );

    let expr = Expr::new_multiply(gb_squared, per_gb_squared);

    let result = eval.eval(&expr).unwrap();

    // 50 GB^2 * 2 GB^-2 = 100 (dimensionless)
    assert_eq!(result.value, EvalValue::Number(100.0));
    assert!(
        result.unit.is_dimensionless(),
        "Should be dimensionless, got: {}",
        result.unit.canonical()
    );
}

#[test]
fn test_hour_times_gb_per_hour() {
    // 24 hr * 50 GB/hr = 1200 GB (time cancels)
    let library = UnitLibrary::new();
    let eval = Evaluator::new(&library);

    let expr = Expr::new_multiply(
        Expr::number_with_unit(24.0, "hr"),
        Expr::number_with_unit(50.0, "GB/hr"),
    );

    let result = eval.eval(&expr).unwrap();

    assert_eq!(result.value, EvalValue::Number(1200.0));

    let canonical = result.unit.canonical();
    assert!(
        !canonical.contains("hr"),
        "hr should cancel out, got: {}",
        canonical
    );
    assert!(
        canonical.contains("GB") || canonical.contains("B"),
        "Should have storage unit, got: {}",
        canonical
    );
}
