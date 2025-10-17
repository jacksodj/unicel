use unicel_lib::core::formula::evaluator::Evaluator;
use unicel_lib::core::formula::parser::parse_formula;
use unicel_lib::core::units::UnitLibrary;

#[test]
fn test_string_concatenation() {
    let library = UnitLibrary::new();
    let evaluator = Evaluator::new(&library);

    // Test 1: String + String
    let expr = parse_formula(r#""Hello" + " world""#).unwrap();
    let result = evaluator.eval(&expr).unwrap();
    assert!(result.is_text());
    assert_eq!(result.as_text(), Some("Hello world"));

    // Test 2: String + Number
    let expr = parse_formula(r#""Count: " + 42"#).unwrap();
    let result = evaluator.eval(&expr).unwrap();
    assert!(result.is_text());
    assert_eq!(result.as_text(), Some("Count: 42"));

    // Test 3: Number + String
    let expr = parse_formula(r#"42 + " items""#).unwrap();
    let result = evaluator.eval(&expr).unwrap();
    assert!(result.is_text());
    assert_eq!(result.as_text(), Some("42 items"));
}

#[test]
fn test_string_with_units() {
    let library = UnitLibrary::new();
    let evaluator = Evaluator::new(&library);

    // Number with unit + String
    let expr = parse_formula(r#"100m + " long""#).unwrap();
    let result = evaluator.eval(&expr).unwrap();
    assert!(result.is_text());
    assert_eq!(result.as_text(), Some("100 m long"));
}

#[test]
fn test_number_addition_still_works() {
    let library = UnitLibrary::new();
    let evaluator = Evaluator::new(&library);

    // Number + Number should still add
    let expr = parse_formula("10 + 20").unwrap();
    let result = evaluator.eval(&expr).unwrap();
    assert!(result.is_number());
    assert_eq!(result.as_number(), Some(30.0));
}
