// Basic unit parser

use super::{BaseDimension, Unit, UnitLibrary};
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

    // Check if it's in the library first (for simple units)
    if let Some(unit) = library.get(symbol) {
        return Ok(unit.clone());
    }

    // If empty, return dimensionless
    if symbol.is_empty() {
        return Ok(Unit::dimensionless());
    }

    // Try to parse as compound unit with division (e.g., "USD/ft", "mi/hr", "$/ft")
    if let Some(pos) = symbol.find('/') {
        let numerator_str = &symbol[..pos];
        let denominator_str = &symbol[pos + 1..];

        let num_dim = get_base_dimension(numerator_str, library)?;
        let den_dim = get_base_dimension(denominator_str, library)?;

        return Ok(Unit::compound(
            symbol.to_string(),
            vec![(num_dim, 1)],
            vec![(den_dim, 1)],
        ));
    }

    // Try to parse as compound unit with multiplication (e.g., "ft*ft", "kg*m")
    if let Some(pos) = symbol.find('*') {
        let left_str = &symbol[..pos];
        let right_str = &symbol[pos + 1..];

        let left_dim = get_base_dimension(left_str, library)?;
        let right_dim = get_base_dimension(right_str, library)?;

        return Ok(Unit::compound(
            symbol.to_string(),
            vec![(left_dim.clone(), 1), (right_dim, 1)],
            vec![],
        ));
    }

    // Try to parse as simple unit using hardcoded mappings (e.g., "$", "ft")
    // This handles units that aren't in the library but are commonly used
    match get_base_dimension(symbol, library) {
        Ok(base_dim) => Ok(Unit::simple(symbol, base_dim)),
        Err(e) => Err(e),
    }
}

/// Get the base dimension for a unit symbol
fn get_base_dimension(unit_str: &str, library: &UnitLibrary) -> Result<BaseDimension, ParseError> {
    // First try to look up in library
    if let Some(unit) = library.get(unit_str) {
        if let Some(base) = unit.dimension().as_simple() {
            return Ok(base.clone());
        }
    }

    // Fallback to hardcoded mappings for common units
    match unit_str {
        "m" | "cm" | "mm" | "km" | "in" | "ft" | "yd" | "mi" => Ok(BaseDimension::Length),
        "g" | "kg" | "mg" | "oz" | "lb" => Ok(BaseDimension::Mass),
        "s" | "min" | "hr" | "h" | "day" | "month" | "quarter" | "year" | "yr" => {
            Ok(BaseDimension::Time)
        }
        "C" | "F" | "K" => Ok(BaseDimension::Temperature),
        "USD" | "EUR" | "GBP" | "$" => Ok(BaseDimension::Currency),
        // Digital storage units (bytes)
        "B" | "b" | "KB" | "Kb" | "MB" | "Mb" | "GB" | "Gb" | "TB" | "Tb" | "PB" | "Pb" => {
            Ok(BaseDimension::DigitalStorage)
        }
        // Bits
        "bits" | "Kbits" | "Mbits" | "Gbits" | "Tbits" => Ok(BaseDimension::DigitalStorage),
        // Token units
        "Tok" | "tok" | "KTok" | "Ktok" | "MTok" | "Mtok" => Ok(BaseDimension::DigitalStorage),
        _ => Err(ParseError::UnknownUnit(unit_str.to_string())),
    }
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
