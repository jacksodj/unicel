// Basic unit parser

use super::{Unit, UnitLibrary};
use std::result::Result;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnknownUnit(String),
    InvalidFormat(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnknownUnit(unit) => write!(f, "Unknown unit: {}", unit),
            ParseError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
        }
    }
}

impl std::error::Error for ParseError {}

/// Parse a unit symbol into a Unit
pub fn parse_unit(symbol: &str, library: &UnitLibrary) -> Result<Unit, ParseError> {
    let symbol = symbol.trim();

    // Check if it's in the library
    if let Some(unit) = library.get(symbol) {
        return Ok(unit.clone());
    }

    // If empty, return dimensionless
    if symbol.is_empty() {
        return Ok(Unit::dimensionless());
    }

    Err(ParseError::UnknownUnit(symbol.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_known_units() {
        let library = UnitLibrary::new();

        // Parse length units
        let meters = parse_unit("m", &library).unwrap();
        assert_eq!(meters.original(), "m");

        let feet = parse_unit("ft", &library).unwrap();
        assert_eq!(feet.original(), "ft");

        // Parse mass units
        let kg = parse_unit("kg", &library).unwrap();
        assert_eq!(kg.original(), "kg");

        // Parse time units
        let seconds = parse_unit("s", &library).unwrap();
        assert_eq!(seconds.original(), "s");
    }

    #[test]
    fn test_parse_unknown_unit() {
        let library = UnitLibrary::new();

        let result = parse_unit("xyz", &library);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ParseError::UnknownUnit("xyz".to_string())
        );
    }

    #[test]
    fn test_parse_empty_unit() {
        let library = UnitLibrary::new();

        let result = parse_unit("", &library).unwrap();
        assert!(result.is_dimensionless());
    }

    #[test]
    fn test_parse_with_whitespace() {
        let library = UnitLibrary::new();

        let result = parse_unit("  m  ", &library).unwrap();
        assert_eq!(result.original(), "m");
    }
}
