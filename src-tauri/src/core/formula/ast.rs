// Abstract Syntax Tree for formulas

use serde::{Deserialize, Serialize};

/// A parsed formula expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    /// A numeric literal (e.g., 42, 3.14)
    Number(f64),

    /// A number with a unit literal (e.g., 100m, 5.5kg)
    NumberWithUnit {
        value: f64,
        unit: String,
    },

    /// A string literal (e.g., "Hello", "world")
    String(String),

    /// A cell reference (e.g., A1, B12)
    CellRef {
        col: String,
        row: usize,
    },

    /// A named cell/range reference (e.g., revenue, tax_rate)
    NamedRef {
        name: String,
    },

    /// A cell range (e.g., A1:B10)
    Range {
        start: Box<Expr>,
        end: Box<Expr>,
    },

    /// Addition
    Add(Box<Expr>, Box<Expr>),

    /// Subtraction
    Subtract(Box<Expr>, Box<Expr>),

    /// Multiplication
    Multiply(Box<Expr>, Box<Expr>),

    /// Division
    Divide(Box<Expr>, Box<Expr>),

    /// Unary negation
    Negate(Box<Expr>),

    /// Function call (e.g., SUM(A1:A10))
    Function {
        name: String,
        args: Vec<Expr>,
    },

    /// Boolean literal (TRUE or FALSE)
    Boolean(bool),

    /// Comparison operators
    GreaterThan(Box<Expr>, Box<Expr>),
    LessThan(Box<Expr>, Box<Expr>),
    GreaterOrEqual(Box<Expr>, Box<Expr>),
    LessOrEqual(Box<Expr>, Box<Expr>),
    Equal(Box<Expr>, Box<Expr>),
    NotEqual(Box<Expr>, Box<Expr>),

    /// Logical operators
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
}

impl Expr {
    /// Create a number expression
    pub fn number(value: f64) -> Self {
        Self::Number(value)
    }

    /// Create a number with unit expression
    pub fn number_with_unit(value: f64, unit: impl Into<String>) -> Self {
        Self::NumberWithUnit {
            value,
            unit: unit.into(),
        }
    }

    /// Create a string literal expression
    pub fn string(value: impl Into<String>) -> Self {
        Self::String(value.into())
    }

    /// Create a cell reference
    pub fn cell_ref(col: impl Into<String>, row: usize) -> Self {
        Self::CellRef {
            col: col.into(),
            row,
        }
    }

    /// Create a named reference
    pub fn named_ref(name: impl Into<String>) -> Self {
        Self::NamedRef { name: name.into() }
    }

    /// Create a range
    pub fn range(start: Expr, end: Expr) -> Self {
        Self::Range {
            start: Box::new(start),
            end: Box::new(end),
        }
    }

    /// Create an addition
    pub fn new_add(left: Expr, right: Expr) -> Self {
        Self::Add(Box::new(left), Box::new(right))
    }

    /// Create a subtraction
    pub fn new_subtract(left: Expr, right: Expr) -> Self {
        Self::Subtract(Box::new(left), Box::new(right))
    }

    /// Create a multiplication
    pub fn new_multiply(left: Expr, right: Expr) -> Self {
        Self::Multiply(Box::new(left), Box::new(right))
    }

    /// Create a division
    pub fn new_divide(left: Expr, right: Expr) -> Self {
        Self::Divide(Box::new(left), Box::new(right))
    }

    /// Create a negation
    pub fn negate(expr: Expr) -> Self {
        Self::Negate(Box::new(expr))
    }

    /// Create a function call
    pub fn function(name: impl Into<String>, args: Vec<Expr>) -> Self {
        Self::Function {
            name: name.into(),
            args,
        }
    }

    /// Create a boolean literal
    pub fn boolean(value: bool) -> Self {
        Self::Boolean(value)
    }

    /// Create a greater than comparison
    pub fn greater_than(left: Expr, right: Expr) -> Self {
        Self::GreaterThan(Box::new(left), Box::new(right))
    }

    /// Create a less than comparison
    pub fn less_than(left: Expr, right: Expr) -> Self {
        Self::LessThan(Box::new(left), Box::new(right))
    }

    /// Create a greater or equal comparison
    pub fn greater_or_equal(left: Expr, right: Expr) -> Self {
        Self::GreaterOrEqual(Box::new(left), Box::new(right))
    }

    /// Create a less or equal comparison
    pub fn less_or_equal(left: Expr, right: Expr) -> Self {
        Self::LessOrEqual(Box::new(left), Box::new(right))
    }

    /// Create an equality comparison
    pub fn equal(left: Expr, right: Expr) -> Self {
        Self::Equal(Box::new(left), Box::new(right))
    }

    /// Create a not equal comparison
    pub fn not_equal(left: Expr, right: Expr) -> Self {
        Self::NotEqual(Box::new(left), Box::new(right))
    }

    /// Create an AND logical operation
    pub fn and(left: Expr, right: Expr) -> Self {
        Self::And(Box::new(left), Box::new(right))
    }

    /// Create an OR logical operation
    pub fn or(left: Expr, right: Expr) -> Self {
        Self::Or(Box::new(left), Box::new(right))
    }

    /// Create a NOT logical operation
    pub fn new_not(expr: Expr) -> Self {
        Self::Not(Box::new(expr))
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::NumberWithUnit { value, unit } => write!(f, "{}{}", value, unit),
            Expr::String(s) => write!(f, "\"{}\"", s),
            Expr::CellRef { col, row } => write!(f, "{}{}", col, row),
            Expr::NamedRef { name } => write!(f, "{}", name),
            Expr::Range { start, end } => write!(f, "{}:{}", start, end),
            Expr::Add(l, r) => write!(f, "({} + {})", l, r),
            Expr::Subtract(l, r) => write!(f, "({} - {})", l, r),
            Expr::Multiply(l, r) => write!(f, "({} * {})", l, r),
            Expr::Divide(l, r) => write!(f, "({} / {})", l, r),
            Expr::Negate(e) => write!(f, "(-{})", e),
            Expr::Function { name, args } => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Expr::Boolean(b) => write!(f, "{}", if *b { "TRUE" } else { "FALSE" }),
            Expr::GreaterThan(l, r) => write!(f, "({} > {})", l, r),
            Expr::LessThan(l, r) => write!(f, "({} < {})", l, r),
            Expr::GreaterOrEqual(l, r) => write!(f, "({} >= {})", l, r),
            Expr::LessOrEqual(l, r) => write!(f, "({} <= {})", l, r),
            Expr::Equal(l, r) => write!(f, "({} == {})", l, r),
            Expr::NotEqual(l, r) => write!(f, "({} != {})", l, r),
            Expr::And(l, r) => write!(f, "({} AND {})", l, r),
            Expr::Or(l, r) => write!(f, "({} OR {})", l, r),
            Expr::Not(e) => write!(f, "(NOT {})", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_display() {
        let expr = Expr::new_add(Expr::number(1.0), Expr::number(2.0));
        assert_eq!(format!("{}", expr), "(1 + 2)");

        let expr = Expr::new_multiply(
            Expr::number_with_unit(100.0, "m"),
            Expr::number_with_unit(2.0, "m"),
        );
        assert_eq!(format!("{}", expr), "(100m * 2m)");

        let expr = Expr::cell_ref("A", 1);
        assert_eq!(format!("{}", expr), "A1");
    }

    #[test]
    fn test_expr_constructors() {
        let num = Expr::number(42.0);
        assert!(matches!(num, Expr::Number(42.0)));

        let with_unit = Expr::number_with_unit(100.0, "m");
        assert!(matches!(
            with_unit,
            Expr::NumberWithUnit { value: 100.0, .. }
        ));

        let cell = Expr::cell_ref("B", 5);
        assert!(matches!(cell, Expr::CellRef { col, row: 5 } if col == "B"));
    }
}
