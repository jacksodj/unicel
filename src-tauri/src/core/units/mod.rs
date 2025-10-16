// Unit system with dimensional analysis
//
// Design Philosophy:
// - Storage vs Display: Values are always stored in their original units (non-destructive).
//   Display conversion is applied on-the-fly based on user preferences (Metric/Imperial toggle).
// - Dimensional Analysis: Operations check unit compatibility automatically.
// - Unit Cancellation: Compound units simplify automatically (e.g., mi/hr ÷ hr → mi).
//
// For MLP, we use a simplified direct-lookup conversion system rather than graph pathfinding.
// This is sufficient for Tier 1 units and keeps the implementation straightforward.

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

    /// Extract base unit symbols (simplified, no exponents)
    /// Examples:
    /// - "m" → ["m"]
    /// - "ft^2" → ["ft"]
    /// - "m/s" → ["m", "s"]
    /// - "$/hr" → ["$", "hr"]
    /// - "ft*ft" → ["ft"]
    /// - "1/m^3" → ["m"]
    pub fn base_units(&self) -> Vec<String> {
        use std::collections::HashSet;

        // Handle dimensionless or empty canonical
        if self.canonical.is_empty() {
            return Vec::new();
        }

        // Split on operators: ^, *, /
        let parts: Vec<&str> = self.canonical.split(['^', '*', '/']).collect();

        // Use HashSet to eliminate duplicates
        let mut unique_units = HashSet::new();

        for part in parts {
            let trimmed = part.trim();

            // Skip empty strings, "1", and numbers (exponents)
            if trimmed.is_empty() || trimmed == "1" || trimmed.parse::<i32>().is_ok() {
                continue;
            }

            unique_units.insert(trimmed.to_string());
        }

        // Convert to sorted Vec
        let mut result: Vec<String> = unique_units.into_iter().collect();
        result.sort();
        result
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

    #[test]
    fn test_base_units_simple() {
        let m = Unit::simple("m", BaseDimension::Length);
        assert_eq!(m.base_units(), vec!["m"]);
    }

    #[test]
    fn test_base_units_with_exponent() {
        let ft2 = Unit::compound("ft^2", vec![(BaseDimension::Length, 2)], vec![]);
        assert_eq!(ft2.base_units(), vec!["ft"]);
    }

    #[test]
    fn test_base_units_compound() {
        let velocity = Unit::compound(
            "m/s",
            vec![(BaseDimension::Length, 1)],
            vec![(BaseDimension::Time, 1)],
        );
        let mut units = velocity.base_units();
        units.sort();
        assert_eq!(units, vec!["m", "s"]);
    }

    #[test]
    fn test_base_units_duplicate() {
        let area = Unit::compound("ft*ft", vec![(BaseDimension::Length, 2)], vec![]);
        assert_eq!(area.base_units(), vec!["ft"]);
    }

    #[test]
    fn test_base_units_dimensionless() {
        let dimensionless = Unit::dimensionless();
        assert_eq!(dimensionless.base_units(), Vec::<String>::new());
    }

    #[test]
    fn test_base_units_currency_per_hour() {
        let rate = Unit::compound(
            "$/hr",
            vec![(BaseDimension::Currency, 1)],
            vec![(BaseDimension::Time, 1)],
        );
        let units = rate.base_units();
        assert_eq!(units.len(), 2);
        assert!(units.contains(&"$".to_string()));
        assert!(units.contains(&"hr".to_string()));
    }

    #[test]
    fn test_base_units_with_leading_one() {
        let inv_volume = Unit::compound("1/m^3", vec![], vec![(BaseDimension::Length, 3)]);
        assert_eq!(inv_volume.base_units(), vec!["m"]);
    }
}
