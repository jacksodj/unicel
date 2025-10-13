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

    /// A cell reference (e.g., A1, B12)
    CellRef {
        col: String,
        row: usize,
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

    /// Create a cell reference
    pub fn cell_ref(col: impl Into<String>, row: usize) -> Self {
        Self::CellRef {
            col: col.into(),
            row,
        }
    }

    /// Create a range
    pub fn range(start: Expr, end: Expr) -> Self {
        Self::Range {
            start: Box::new(start),
            end: Box::new(end),
        }
    }

    /// Create an addition
    pub fn add(left: Expr, right: Expr) -> Self {
        Self::Add(Box::new(left), Box::new(right))
    }

    /// Create a subtraction
    pub fn subtract(left: Expr, right: Expr) -> Self {
        Self::Subtract(Box::new(left), Box::new(right))
    }

    /// Create a multiplication
    pub fn multiply(left: Expr, right: Expr) -> Self {
        Self::Multiply(Box::new(left), Box::new(right))
    }

    /// Create a division
    pub fn divide(left: Expr, right: Expr) -> Self {
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
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::NumberWithUnit { value, unit } => write!(f, "{}{}", value, unit),
            Expr::CellRef { col, row } => write!(f, "{}{}", col, row),
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_display() {
        let expr = Expr::add(Expr::number(1.0), Expr::number(2.0));
        assert_eq!(format!("{}", expr), "(1 + 2)");

        let expr = Expr::multiply(
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
