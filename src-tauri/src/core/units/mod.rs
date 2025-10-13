// Unit system with dimensional analysis

mod library;
mod parser;

pub use library::{ConversionFactor, UnitLibrary};
pub use parser::{parse_unit, ParseError};

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Unit {
    /// Canonical form (normalized for comparison)
    canonical: String,

    /// Original as entered (for exact round-trip)
    original: String,

    /// Dimension for compatibility checking
    dimension: Dimension,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Dimension {
    Dimensionless,
    Simple(BaseDimension),
    Compound {
        numerator: Vec<(BaseDimension, i32)>,
        denominator: Vec<(BaseDimension, i32)>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BaseDimension {
    Length,
    Mass,
    Time,
    Currency,
    Temperature,
    DigitalStorage,
    Custom(String),
}

impl Unit {
    /// Create a dimensionless unit (no units)
    pub fn dimensionless() -> Self {
        Self {
            canonical: String::new(),
            original: String::new(),
            dimension: Dimension::Dimensionless,
        }
    }

    /// Create a simple unit with a single dimension
    pub fn simple(symbol: impl Into<String>, dimension: BaseDimension) -> Self {
        let symbol = symbol.into();
        Self {
            canonical: symbol.clone(),
            original: symbol,
            dimension: Dimension::Simple(dimension),
        }
    }

    /// Create a compound unit (e.g., m/s, kg*m/s²)
    pub fn compound(
        symbol: impl Into<String>,
        numerator: Vec<(BaseDimension, i32)>,
        denominator: Vec<(BaseDimension, i32)>,
    ) -> Self {
        let symbol = symbol.into();
        Self {
            canonical: symbol.clone(),
            original: symbol,
            dimension: Dimension::Compound {
                numerator,
                denominator,
            },
        }
    }

    /// Get the canonical form of the unit
    pub fn canonical(&self) -> &str {
        &self.canonical
    }

    /// Get the original form as entered
    pub fn original(&self) -> &str {
        &self.original
    }

    /// Get the dimension
    pub fn dimension(&self) -> &Dimension {
        &self.dimension
    }

    /// Check if this unit is dimensionless
    pub fn is_dimensionless(&self) -> bool {
        matches!(self.dimension, Dimension::Dimensionless)
    }

    /// Check if this unit is compatible with another (same dimension)
    pub fn is_compatible(&self, other: &Unit) -> bool {
        self.dimension == other.dimension
    }

    /// Check if two units are exactly equal (same canonical form)
    pub fn is_equal(&self, other: &Unit) -> bool {
        self.canonical == other.canonical
    }
}

impl Dimension {
    /// Check if dimension is dimensionless
    pub fn is_dimensionless(&self) -> bool {
        matches!(self, Dimension::Dimensionless)
    }

    /// Get the base dimension if this is a simple dimension
    pub fn as_simple(&self) -> Option<&BaseDimension> {
        match self {
            Dimension::Simple(base) => Some(base),
            _ => None,
        }
    }
}

impl BaseDimension {
    /// Get the standard symbol for this base dimension
    pub fn symbol(&self) -> &str {
        match self {
            BaseDimension::Length => "L",
            BaseDimension::Mass => "M",
            BaseDimension::Time => "T",
            BaseDimension::Currency => "$",
            BaseDimension::Temperature => "Θ",
            BaseDimension::DigitalStorage => "B",
            BaseDimension::Custom(name) => name,
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.original)
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dimension::Dimensionless => write!(f, "dimensionless"),
            Dimension::Simple(base) => write!(f, "{}", base.symbol()),
            Dimension::Compound {
                numerator,
                denominator,
            } => {
                // Format numerator
                let mut first = true;
                for (base, power) in numerator {
                    if !first {
                        write!(f, "·")?;
                    }
                    write!(f, "{}", base.symbol())?;
                    if *power != 1 {
                        write!(f, "^{}", power)?;
                    }
                    first = false;
                }

                // Format denominator
                if !denominator.is_empty() {
                    write!(f, "/")?;
                    let mut first = true;
                    for (base, power) in denominator {
                        if !first {
                            write!(f, "·")?;
                        }
                        write!(f, "{}", base.symbol())?;
                        if *power != 1 {
                            write!(f, "^{}", power)?;
                        }
                        first = false;
                    }
                }

                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimensionless_unit() {
        let unit = Unit::dimensionless();
        assert!(unit.is_dimensionless());
        assert_eq!(unit.canonical(), "");
        assert_eq!(unit.original(), "");
    }

    #[test]
    fn test_simple_unit() {
        let meters = Unit::simple("m", BaseDimension::Length);
        assert!(!meters.is_dimensionless());
        assert_eq!(meters.canonical(), "m");
        assert_eq!(meters.original(), "m");
        assert_eq!(
            meters.dimension(),
            &Dimension::Simple(BaseDimension::Length)
        );
    }

    #[test]
    fn test_unit_compatibility() {
        let meters = Unit::simple("m", BaseDimension::Length);
        let feet = Unit::simple("ft", BaseDimension::Length);
        let seconds = Unit::simple("s", BaseDimension::Time);

        assert!(meters.is_compatible(&feet));
        assert!(!meters.is_compatible(&seconds));
    }

    #[test]
    fn test_unit_equality() {
        let m1 = Unit::simple("m", BaseDimension::Length);
        let m2 = Unit::simple("m", BaseDimension::Length);
        let ft = Unit::simple("ft", BaseDimension::Length);

        assert!(m1.is_equal(&m2));
        assert!(!m1.is_equal(&ft));
    }

    #[test]
    fn test_compound_unit() {
        let velocity = Unit::compound(
            "m/s",
            vec![(BaseDimension::Length, 1)],
            vec![(BaseDimension::Time, 1)],
        );

        assert!(!velocity.is_dimensionless());
        assert_eq!(velocity.canonical(), "m/s");
    }

    #[test]
    fn test_dimension_display() {
        let dimensionless = Dimension::Dimensionless;
        assert_eq!(format!("{}", dimensionless), "dimensionless");

        let length = Dimension::Simple(BaseDimension::Length);
        assert_eq!(format!("{}", length), "L");

        let velocity = Dimension::Compound {
            numerator: vec![(BaseDimension::Length, 1)],
            denominator: vec![(BaseDimension::Time, 1)],
        };
        assert_eq!(format!("{}", velocity), "L/T");
    }
}
