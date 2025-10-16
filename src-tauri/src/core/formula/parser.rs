// Formula parser using pest

use super::ast::Expr;
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "core/formula/formula.pest"]
struct FormulaParser;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Parse error: {0}")]
    PestError(Box<pest::error::Error<Rule>>),

    #[error("Invalid cell reference: {0}")]
    InvalidCellRef(String),

    #[error("Invalid number: {0}")]
    InvalidNumber(String),

    #[error("Unexpected rule: {0:?}")]
    UnexpectedRule(Rule),
}

impl From<pest::error::Error<Rule>> for ParseError {
    fn from(err: pest::error::Error<Rule>) -> Self {
        ParseError::PestError(Box::new(err))
    }
}

/// Parse a formula string into an AST
pub fn parse_formula(input: &str) -> Result<Expr, ParseError> {
    let pairs = FormulaParser::parse(Rule::formula, input)?;

    for pair in pairs {
        // The first pair should be the formula
        if pair.as_rule() == Rule::formula {
            let inner = pair.into_inner().next().unwrap();
            return parse_expr(inner);
        }
    }

    Err(ParseError::UnexpectedRule(Rule::formula))
}

fn parse_expr(pair: pest::iterators::Pair<Rule>) -> Result<Expr, ParseError> {
    match pair.as_rule() {
        Rule::expr => {
            let mut pairs = pair.into_inner();
            let mut left = parse_expr(pairs.next().unwrap())?;

            while let Some(op) = pairs.next() {
                let right = parse_expr(pairs.next().unwrap())?;

                left = match op.as_str() {
                    "+" => Expr::new_add(left, right),
                    "-" => Expr::new_subtract(left, right),
                    _ => return Err(ParseError::UnexpectedRule(op.as_rule())),
                };
            }

            Ok(left)
        }

        Rule::term => {
            let mut pairs = pair.into_inner();
            let mut left = parse_expr(pairs.next().unwrap())?;

            while let Some(op) = pairs.next() {
                let right = parse_expr(pairs.next().unwrap())?;

                left = match op.as_str() {
                    "*" => Expr::new_multiply(left, right),
                    "/" => Expr::new_divide(left, right),
                    _ => return Err(ParseError::UnexpectedRule(op.as_rule())),
                };
            }

            Ok(left)
        }

        Rule::factor => {
            let mut pairs = pair.into_inner();
            let first = pairs.next().unwrap();

            if first.as_rule() == Rule::unary_op {
                let expr = parse_expr(pairs.next().unwrap())?;
                if first.as_str() == "-" {
                    Ok(Expr::negate(expr))
                } else {
                    Ok(expr) // Unary + is a no-op
                }
            } else {
                parse_expr(first)
            }
        }

        Rule::number => {
            let num_str = pair.as_str();
            let value = num_str
                .parse::<f64>()
                .map_err(|_| ParseError::InvalidNumber(num_str.to_string()))?;
            Ok(Expr::number(value))
        }

        Rule::currency_with_number => {
            let mut pairs = pair.into_inner();
            let num_str = pairs.next().unwrap().as_str();
            let value = num_str
                .parse::<f64>()
                .map_err(|_| ParseError::InvalidNumber(num_str.to_string()))?;
            // Currency symbol $ is before the number
            Ok(Expr::number_with_unit(value, "$"))
        }

        Rule::number_with_unit => {
            let mut pairs = pair.into_inner();
            let num_str = pairs.next().unwrap().as_str();
            let value = num_str
                .parse::<f64>()
                .map_err(|_| ParseError::InvalidNumber(num_str.to_string()))?;
            let unit = pairs.next().unwrap().as_str();
            Ok(Expr::number_with_unit(value, unit))
        }

        Rule::cell_ref => {
            let cell_str = pair.as_str();
            parse_cell_ref(cell_str)
        }

        Rule::named_ref => {
            let name = pair.as_str();
            Ok(Expr::named_ref(name))
        }

        Rule::function_call => {
            let mut pairs = pair.into_inner();
            let name = pairs.next().unwrap().as_str();

            let args = if let Some(arg_list) = pairs.next() {
                match arg_list.as_rule() {
                    Rule::arg_list => {
                        let mut args = Vec::new();
                        for arg in arg_list.into_inner() {
                            match arg.as_rule() {
                                Rule::arg => {
                                    let inner = arg.into_inner().next().unwrap();
                                    if inner.as_rule() == Rule::range {
                                        args.push(parse_range(inner)?);
                                    } else {
                                        args.push(parse_expr(inner)?);
                                    }
                                }
                                Rule::range => args.push(parse_range(arg)?),
                                _ => args.push(parse_expr(arg)?),
                            }
                        }
                        args
                    }
                    _ => Vec::new(),
                }
            } else {
                Vec::new()
            };

            Ok(Expr::function(name, args))
        }

        Rule::range => parse_range(pair),

        _ => Err(ParseError::UnexpectedRule(pair.as_rule())),
    }
}

fn parse_cell_ref(cell_str: &str) -> Result<Expr, ParseError> {
    // Split into column and row parts
    let mut col = String::new();
    let mut row = String::new();

    for ch in cell_str.chars() {
        if ch.is_ascii_alphabetic() {
            col.push(ch.to_ascii_uppercase());
        } else if ch.is_ascii_digit() {
            row.push(ch);
        }
    }

    if col.is_empty() || row.is_empty() {
        return Err(ParseError::InvalidCellRef(cell_str.to_string()));
    }

    let row_num = row
        .parse::<usize>()
        .map_err(|_| ParseError::InvalidCellRef(cell_str.to_string()))?;

    Ok(Expr::cell_ref(col, row_num))
}

fn parse_range(pair: pest::iterators::Pair<Rule>) -> Result<Expr, ParseError> {
    let mut pairs = pair.into_inner();
    let start_str = pairs.next().unwrap().as_str();
    let end_str = pairs.next().unwrap().as_str();

    let start = parse_cell_ref(start_str)?;
    let end = parse_cell_ref(end_str)?;

    Ok(Expr::range(start, end))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let expr = parse_formula("42").unwrap();
        assert!(matches!(expr, Expr::Number(42.0)));

        let expr = parse_formula("3.14").unwrap();
        assert!(matches!(expr, Expr::Number(v) if (v - 3.14).abs() < 0.001));
    }

    #[test]
    fn test_parse_number_with_unit() {
        let expr = parse_formula("100m").unwrap();
        assert!(matches!(
            expr,
            Expr::NumberWithUnit { value: 100.0, ref unit } if unit == "m"
        ));

        let expr = parse_formula("5.5kg").unwrap();
        assert!(matches!(
            expr,
            Expr::NumberWithUnit { value: 5.5, ref unit } if unit == "kg"
        ));
    }

    #[test]
    fn test_parse_cell_ref() {
        let expr = parse_formula("A1").unwrap();
        assert!(matches!(
            expr,
            Expr::CellRef { ref col, row: 1 } if col == "A"
        ));

        let expr = parse_formula("B12").unwrap();
        assert!(matches!(
            expr,
            Expr::CellRef { ref col, row: 12 } if col == "B"
        ));

        let expr = parse_formula("AA100").unwrap();
        assert!(matches!(
            expr,
            Expr::CellRef { ref col, row: 100 } if col == "AA"
        ));
    }

    #[test]
    fn test_parse_arithmetic() {
        let expr = parse_formula("1 + 2").unwrap();
        assert!(matches!(expr, Expr::Add(_, _)));

        let expr = parse_formula("10 - 5").unwrap();
        assert!(matches!(expr, Expr::Subtract(_, _)));

        let expr = parse_formula("3 * 4").unwrap();
        assert!(matches!(expr, Expr::Multiply(_, _)));

        let expr = parse_formula("8 / 2").unwrap();
        assert!(matches!(expr, Expr::Divide(_, _)));
    }

    #[test]
    fn test_parse_precedence() {
        // 1 + 2 * 3 should be 1 + (2 * 3), not (1 + 2) * 3
        let expr = parse_formula("1 + 2 * 3").unwrap();
        match expr {
            Expr::Add(left, right) => {
                assert!(matches!(*left, Expr::Number(1.0)));
                assert!(matches!(*right, Expr::Multiply(_, _)));
            }
            _ => panic!("Expected Add expression"),
        }
    }

    #[test]
    fn test_parse_parentheses() {
        let expr = parse_formula("(1 + 2) * 3").unwrap();
        match expr {
            Expr::Multiply(left, right) => {
                assert!(matches!(*left, Expr::Add(_, _)));
                assert!(matches!(*right, Expr::Number(3.0)));
            }
            _ => panic!("Expected Multiply expression"),
        }
    }

    #[test]
    fn test_parse_unary_minus() {
        let expr = parse_formula("-5").unwrap();
        assert!(matches!(expr, Expr::Negate(_)));

        let expr = parse_formula("-(3 + 4)").unwrap();
        assert!(matches!(expr, Expr::Negate(_)));
    }

    #[test]
    fn test_parse_formula_with_equals() {
        let expr = parse_formula("=A1 + B1").unwrap();
        assert!(matches!(expr, Expr::Add(_, _)));
    }

    #[test]
    fn test_parse_function() {
        let expr = parse_formula("SUM(A1:A10)").unwrap();
        match expr {
            Expr::Function { name, args } => {
                assert_eq!(name, "SUM");
                assert_eq!(args.len(), 1);
                assert!(matches!(args[0], Expr::Range { .. }));
            }
            _ => panic!("Expected Function expression"),
        }

        let expr = parse_formula("AVERAGE(1, 2, 3)").unwrap();
        match expr {
            Expr::Function { name, args } => {
                assert_eq!(name, "AVERAGE");
                assert_eq!(args.len(), 3);
            }
            _ => panic!("Expected Function expression"),
        }
    }

    #[test]
    fn test_parse_unit_arithmetic() {
        let expr = parse_formula("100m + 50m").unwrap();
        match expr {
            Expr::Add(left, right) => {
                assert!(matches!(*left, Expr::NumberWithUnit { .. }));
                assert!(matches!(*right, Expr::NumberWithUnit { .. }));
            }
            _ => panic!("Expected Add expression"),
        }

        let expr = parse_formula("100mi / 2hr").unwrap();
        assert!(matches!(expr, Expr::Divide(_, _)));
    }

    #[test]
    fn test_parse_complex_expression() {
        let expr = parse_formula("=A1 * 2 + B1 / C1").unwrap();
        // Should parse as (A1 * 2) + (B1 / C1) due to precedence
        assert!(matches!(expr, Expr::Add(_, _)));
    }

    #[test]
    fn test_parse_currency_first() {
        // Test $15 format
        let expr = parse_formula("=$15").unwrap();
        match expr {
            Expr::NumberWithUnit { value, unit } => {
                assert_eq!(value, 15.0);
                assert_eq!(unit, "$");
            }
            _ => panic!("Expected NumberWithUnit, got: {:?}", expr),
        }

        // Test $15.50 format
        let expr = parse_formula("=$15.50").unwrap();
        match expr {
            Expr::NumberWithUnit { value, unit } => {
                assert_eq!(value, 15.5);
                assert_eq!(unit, "$");
            }
            _ => panic!("Expected NumberWithUnit, got: {:?}", expr),
        }
    }

    #[test]
    fn test_parse_currency_expression() {
        // Test 2ft * $15/ft
        let expr = parse_formula("=2ft * $15").unwrap();
        assert!(matches!(expr, Expr::Multiply(_, _)));
    }

    #[test]
    fn test_parse_named_ref() {
        // Test lowercase named reference
        let expr = parse_formula("revenue").unwrap();
        match expr {
            Expr::NamedRef { name } => {
                assert_eq!(name, "revenue");
            }
            _ => panic!("Expected NamedRef, got: {:?}", expr),
        }

        // Test with underscores
        let expr = parse_formula("tax_rate").unwrap();
        assert!(matches!(expr, Expr::NamedRef { ref name } if name == "tax_rate"));

        // Test starting with underscore
        let expr = parse_formula("_private").unwrap();
        assert!(matches!(expr, Expr::NamedRef { ref name } if name == "_private"));

        // Test with numbers
        let expr = parse_formula("value123").unwrap();
        assert!(matches!(expr, Expr::NamedRef { ref name } if name == "value123"));
    }

    #[test]
    fn test_parse_named_ref_in_expression() {
        // Test revenue * tax_rate
        let expr = parse_formula("=revenue * tax_rate").unwrap();
        match expr {
            Expr::Multiply(left, right) => {
                assert!(matches!(*left, Expr::NamedRef { ref name } if name == "revenue"));
                assert!(matches!(*right, Expr::NamedRef { ref name } if name == "tax_rate"));
            }
            _ => panic!("Expected Multiply expression"),
        }
    }

    #[test]
    fn test_parse_named_ref_vs_cell_ref() {
        // Cell references should still work (uppercase + digits)
        let expr = parse_formula("A1").unwrap();
        assert!(matches!(expr, Expr::CellRef { .. }));

        // Named refs (lowercase start) should parse as NamedRef
        let expr = parse_formula("a1").unwrap();
        assert!(matches!(expr, Expr::NamedRef { .. }));

        // All uppercase should be cell ref
        let expr = parse_formula("AA100").unwrap();
        assert!(matches!(expr, Expr::CellRef { .. }));
    }

    #[test]
    fn test_parse_function_vs_named_ref() {
        // Function calls have parentheses
        let expr = parse_formula("SUM(A1:A10)").unwrap();
        assert!(matches!(expr, Expr::Function { .. }));

        // Named refs don't have parentheses
        let expr = parse_formula("sum_value").unwrap();
        assert!(matches!(expr, Expr::NamedRef { .. }));
    }
}
