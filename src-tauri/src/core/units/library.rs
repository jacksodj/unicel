// Built-in unit library with Tier 1 units

use super::{BaseDimension, Unit};
use std::collections::HashMap;

/// Conversion factor from one unit to another
#[derive(Debug, Clone)]
pub struct ConversionFactor {
    /// Multiplier to convert from source to target
    pub multiplier: f64,
    /// Offset to apply after multiplying (for temperature)
    pub offset: f64,
}

impl ConversionFactor {
    pub fn new(multiplier: f64) -> Self {
        Self {
            multiplier,
            offset: 0.0,
        }
    }

    pub fn with_offset(multiplier: f64, offset: f64) -> Self {
        Self { multiplier, offset }
    }

    /// Convert a value using this conversion factor
    pub fn convert(&self, value: f64) -> f64 {
        value * self.multiplier + self.offset
    }
}

/// Unit library containing all predefined units
#[derive(Debug)]
pub struct UnitLibrary {
    units: HashMap<String, Unit>,
    conversions: HashMap<(String, String), ConversionFactor>,
}

impl UnitLibrary {
    /// Create a new unit library with Tier 1 units
    pub fn new() -> Self {
        let mut library = Self {
            units: HashMap::new(),
            conversions: HashMap::new(),
        };

        library.add_length_units();
        library.add_mass_units();
        library.add_time_units();
        library.add_temperature_units();
        library.add_currency_units();

        library
    }

    /// Add a unit to the library
    fn add_unit(&mut self, symbol: &str, unit: Unit) {
        self.units.insert(symbol.to_string(), unit);
    }

    /// Add a conversion factor between two units
    fn add_conversion(&mut self, from: &str, to: &str, factor: ConversionFactor) {
        self.conversions
            .insert((from.to_string(), to.to_string()), factor);
    }

    /// Get a unit by its symbol
    pub fn get(&self, symbol: &str) -> Option<&Unit> {
        self.units.get(symbol)
    }

    /// Get a conversion factor between two units
    pub fn get_conversion(&self, from: &str, to: &str) -> Option<&ConversionFactor> {
        self.conversions.get(&(from.to_string(), to.to_string()))
    }

    /// Check if a unit symbol exists
    pub fn contains(&self, symbol: &str) -> bool {
        self.units.contains_key(symbol)
    }

    /// Convert a value from one unit to another
    /// Returns None if units are not compatible or conversion doesn't exist
    pub fn convert(&self, value: f64, from: &str, to: &str) -> Option<f64> {
        // If same unit, no conversion needed
        if from == to {
            return Some(value);
        }

        // Check if both units exist
        let from_unit = self.get(from)?;
        let to_unit = self.get(to)?;

        // Check if units are compatible
        if !from_unit.is_compatible(to_unit) {
            return None;
        }

        // Get conversion factor
        let factor = self.get_conversion(from, to)?;

        // Apply conversion
        Some(factor.convert(value))
    }

    /// Check if two units are compatible (can be converted)
    pub fn can_convert(&self, from: &str, to: &str) -> bool {
        if from == to {
            return true;
        }

        if let (Some(from_unit), Some(to_unit)) = (self.get(from), self.get(to)) {
            from_unit.is_compatible(to_unit)
        } else {
            false
        }
    }

    // === Length Units ===
    fn add_length_units(&mut self) {
        // Metric
        self.add_unit("m", Unit::simple("m", BaseDimension::Length));
        self.add_unit("cm", Unit::simple("cm", BaseDimension::Length));
        self.add_unit("mm", Unit::simple("mm", BaseDimension::Length));
        self.add_unit("km", Unit::simple("km", BaseDimension::Length));

        // Imperial
        self.add_unit("in", Unit::simple("in", BaseDimension::Length));
        self.add_unit("ft", Unit::simple("ft", BaseDimension::Length));
        self.add_unit("yd", Unit::simple("yd", BaseDimension::Length));
        self.add_unit("mi", Unit::simple("mi", BaseDimension::Length));

        // Conversions (all to meters as base)
        // Metric
        self.add_conversion("cm", "m", ConversionFactor::new(0.01));
        self.add_conversion("m", "cm", ConversionFactor::new(100.0));
        self.add_conversion("mm", "m", ConversionFactor::new(0.001));
        self.add_conversion("m", "mm", ConversionFactor::new(1000.0));
        self.add_conversion("km", "m", ConversionFactor::new(1000.0));
        self.add_conversion("m", "km", ConversionFactor::new(0.001));

        // Imperial
        self.add_conversion("in", "m", ConversionFactor::new(0.0254));
        self.add_conversion("m", "in", ConversionFactor::new(39.3701));
        self.add_conversion("ft", "m", ConversionFactor::new(0.3048));
        self.add_conversion("m", "ft", ConversionFactor::new(3.28084));
        self.add_conversion("yd", "m", ConversionFactor::new(0.9144));
        self.add_conversion("m", "yd", ConversionFactor::new(1.09361));
        self.add_conversion("mi", "m", ConversionFactor::new(1609.34));
        self.add_conversion("m", "mi", ConversionFactor::new(0.000621371));

        // Imperial to Imperial
        self.add_conversion("ft", "in", ConversionFactor::new(12.0));
        self.add_conversion("in", "ft", ConversionFactor::new(1.0 / 12.0));
        self.add_conversion("yd", "ft", ConversionFactor::new(3.0));
        self.add_conversion("ft", "yd", ConversionFactor::new(1.0 / 3.0));
        self.add_conversion("mi", "ft", ConversionFactor::new(5280.0));
        self.add_conversion("ft", "mi", ConversionFactor::new(1.0 / 5280.0));
    }

    // === Mass Units ===
    fn add_mass_units(&mut self) {
        // Metric
        self.add_unit("g", Unit::simple("g", BaseDimension::Mass));
        self.add_unit("kg", Unit::simple("kg", BaseDimension::Mass));
        self.add_unit("mg", Unit::simple("mg", BaseDimension::Mass));

        // Imperial
        self.add_unit("oz", Unit::simple("oz", BaseDimension::Mass));
        self.add_unit("lb", Unit::simple("lb", BaseDimension::Mass));

        // Conversions (all to kg as base)
        // Metric
        self.add_conversion("g", "kg", ConversionFactor::new(0.001));
        self.add_conversion("kg", "g", ConversionFactor::new(1000.0));
        self.add_conversion("mg", "kg", ConversionFactor::new(0.000001));
        self.add_conversion("kg", "mg", ConversionFactor::new(1_000_000.0));

        // Imperial
        self.add_conversion("oz", "kg", ConversionFactor::new(0.0283495));
        self.add_conversion("kg", "oz", ConversionFactor::new(35.274));
        self.add_conversion("lb", "kg", ConversionFactor::new(0.453592));
        self.add_conversion("kg", "lb", ConversionFactor::new(2.20462));

        // Imperial to Imperial
        self.add_conversion("lb", "oz", ConversionFactor::new(16.0));
        self.add_conversion("oz", "lb", ConversionFactor::new(1.0 / 16.0));
    }

    // === Time Units ===
    fn add_time_units(&mut self) {
        self.add_unit("s", Unit::simple("s", BaseDimension::Time));
        self.add_unit("min", Unit::simple("min", BaseDimension::Time));
        self.add_unit("hr", Unit::simple("hr", BaseDimension::Time));
        self.add_unit("h", Unit::simple("h", BaseDimension::Time)); // alias for hr
        self.add_unit("day", Unit::simple("day", BaseDimension::Time));

        // Conversions (all to seconds as base)
        self.add_conversion("min", "s", ConversionFactor::new(60.0));
        self.add_conversion("s", "min", ConversionFactor::new(1.0 / 60.0));
        self.add_conversion("hr", "s", ConversionFactor::new(3600.0));
        self.add_conversion("s", "hr", ConversionFactor::new(1.0 / 3600.0));
        self.add_conversion("h", "s", ConversionFactor::new(3600.0));
        self.add_conversion("s", "h", ConversionFactor::new(1.0 / 3600.0));
        self.add_conversion("day", "s", ConversionFactor::new(86400.0));
        self.add_conversion("s", "day", ConversionFactor::new(1.0 / 86400.0));

        // Time to Time
        self.add_conversion("hr", "min", ConversionFactor::new(60.0));
        self.add_conversion("min", "hr", ConversionFactor::new(1.0 / 60.0));
        self.add_conversion("day", "hr", ConversionFactor::new(24.0));
        self.add_conversion("hr", "day", ConversionFactor::new(1.0 / 24.0));
    }

    // === Temperature Units ===
    fn add_temperature_units(&mut self) {
        self.add_unit("C", Unit::simple("C", BaseDimension::Temperature));
        self.add_unit("F", Unit::simple("F", BaseDimension::Temperature));
        self.add_unit("K", Unit::simple("K", BaseDimension::Temperature));

        // Temperature conversions (using formulas)
        // C to F: (C × 9/5) + 32
        // F to C: (F − 32) × 5/9
        // C to K: C + 273.15
        // K to C: K − 273.15

        self.add_conversion("C", "F", ConversionFactor::with_offset(1.8, 32.0));
        self.add_conversion("F", "C", ConversionFactor::with_offset(5.0 / 9.0, -160.0 / 9.0));
        self.add_conversion("C", "K", ConversionFactor::with_offset(1.0, 273.15));
        self.add_conversion("K", "C", ConversionFactor::with_offset(1.0, -273.15));
        self.add_conversion("F", "K", ConversionFactor::with_offset(5.0 / 9.0, 255.372));
        self.add_conversion("K", "F", ConversionFactor::with_offset(1.8, -459.67));
    }

    // === Currency Units ===
    fn add_currency_units(&mut self) {
        self.add_unit("USD", Unit::simple("USD", BaseDimension::Currency));
        self.add_unit("EUR", Unit::simple("EUR", BaseDimension::Currency));
        self.add_unit("GBP", Unit::simple("GBP", BaseDimension::Currency));

        // Note: Currency conversions are hardcoded for MLP
        // In the full version, these would come from MCP servers
        self.add_conversion("EUR", "USD", ConversionFactor::new(1.08)); // 1 EUR = 1.08 USD
        self.add_conversion("USD", "EUR", ConversionFactor::new(1.0 / 1.08));
        self.add_conversion("GBP", "USD", ConversionFactor::new(1.27)); // 1 GBP = 1.27 USD
        self.add_conversion("USD", "GBP", ConversionFactor::new(1.0 / 1.27));
        self.add_conversion("GBP", "EUR", ConversionFactor::new(1.18)); // 1 GBP = 1.18 EUR
        self.add_conversion("EUR", "GBP", ConversionFactor::new(1.0 / 1.18));
    }
}

impl Default for UnitLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_library_contains_tier1_units() {
        let library = UnitLibrary::new();

        // Length
        assert!(library.contains("m"));
        assert!(library.contains("ft"));
        assert!(library.contains("km"));
        assert!(library.contains("mi"));

        // Mass
        assert!(library.contains("kg"));
        assert!(library.contains("lb"));

        // Time
        assert!(library.contains("s"));
        assert!(library.contains("hr"));

        // Temperature
        assert!(library.contains("C"));
        assert!(library.contains("F"));

        // Currency
        assert!(library.contains("USD"));
        assert!(library.contains("EUR"));
    }

    #[test]
    fn test_length_conversions() {
        let library = UnitLibrary::new();

        // Meters to feet
        let m_to_ft = library.get_conversion("m", "ft").unwrap();
        assert!((m_to_ft.convert(1.0) - 3.28084).abs() < 0.0001);

        // Feet to inches
        let ft_to_in = library.get_conversion("ft", "in").unwrap();
        assert_eq!(ft_to_in.convert(1.0), 12.0);

        // Miles to kilometers
        let mi_to_m = library.get_conversion("mi", "m").unwrap();
        let m_to_km = library.get_conversion("m", "km").unwrap();
        let mi_in_km = m_to_km.convert(mi_to_m.convert(1.0));
        assert!((mi_in_km - 1.60934).abs() < 0.0001);
    }

    #[test]
    fn test_temperature_conversions() {
        let library = UnitLibrary::new();

        // 0°C = 32°F
        let c_to_f = library.get_conversion("C", "F").unwrap();
        assert_eq!(c_to_f.convert(0.0), 32.0);

        // 100°C = 212°F
        assert_eq!(c_to_f.convert(100.0), 212.0);

        // 0°C = 273.15K
        let c_to_k = library.get_conversion("C", "K").unwrap();
        assert_eq!(c_to_k.convert(0.0), 273.15);
    }

    #[test]
    fn test_currency_conversions() {
        let library = UnitLibrary::new();

        // EUR to USD
        let eur_to_usd = library.get_conversion("EUR", "USD").unwrap();
        assert_eq!(eur_to_usd.convert(100.0), 108.0);

        // GBP to USD
        let gbp_to_usd = library.get_conversion("GBP", "USD").unwrap();
        assert_eq!(gbp_to_usd.convert(100.0), 127.0);
    }

    #[test]
    fn test_time_conversions() {
        let library = UnitLibrary::new();

        // Hours to minutes
        let hr_to_min = library.get_conversion("hr", "min").unwrap();
        assert_eq!(hr_to_min.convert(1.0), 60.0);

        // Days to hours
        let day_to_hr = library.get_conversion("day", "hr").unwrap();
        assert_eq!(day_to_hr.convert(1.0), 24.0);
    }

    #[test]
    fn test_library_convert_method() {
        let library = UnitLibrary::new();

        // Test length conversions
        assert_eq!(library.convert(1.0, "m", "cm").unwrap(), 100.0);
        assert_eq!(library.convert(100.0, "cm", "m").unwrap(), 1.0);
        assert!((library.convert(1.0, "m", "ft").unwrap() - 3.28084).abs() < 0.0001);

        // Test mass conversions
        assert_eq!(library.convert(1.0, "kg", "g").unwrap(), 1000.0);
        assert_eq!(library.convert(1000.0, "g", "kg").unwrap(), 1.0);

        // Test time conversions
        assert_eq!(library.convert(1.0, "hr", "min").unwrap(), 60.0);
        assert_eq!(library.convert(60.0, "min", "hr").unwrap(), 1.0);

        // Test temperature conversions
        assert_eq!(library.convert(0.0, "C", "F").unwrap(), 32.0);
        assert_eq!(library.convert(100.0, "C", "F").unwrap(), 212.0);
        assert_eq!(library.convert(0.0, "C", "K").unwrap(), 273.15);

        // Test currency conversions
        assert_eq!(library.convert(100.0, "EUR", "USD").unwrap(), 108.0);
        assert_eq!(library.convert(100.0, "GBP", "USD").unwrap(), 127.0);
    }

    #[test]
    fn test_identity_conversion() {
        let library = UnitLibrary::new();

        // Converting a unit to itself should return the same value
        assert_eq!(library.convert(42.0, "m", "m").unwrap(), 42.0);
        assert_eq!(library.convert(100.0, "kg", "kg").unwrap(), 100.0);
        assert_eq!(library.convert(25.5, "USD", "USD").unwrap(), 25.5);
    }

    #[test]
    fn test_incompatible_conversion() {
        let library = UnitLibrary::new();

        // Cannot convert between different dimensions
        assert!(library.convert(1.0, "m", "kg").is_none());
        assert!(library.convert(1.0, "s", "m").is_none());
        assert!(library.convert(1.0, "USD", "kg").is_none());
    }

    #[test]
    fn test_unknown_unit_conversion() {
        let library = UnitLibrary::new();

        // Unknown units should return None
        assert!(library.convert(1.0, "xyz", "m").is_none());
        assert!(library.convert(1.0, "m", "xyz").is_none());
        assert!(library.convert(1.0, "abc", "xyz").is_none());
    }

    #[test]
    fn test_can_convert() {
        let library = UnitLibrary::new();

        // Compatible units
        assert!(library.can_convert("m", "ft"));
        assert!(library.can_convert("kg", "lb"));
        assert!(library.can_convert("C", "F"));

        // Same unit
        assert!(library.can_convert("m", "m"));

        // Incompatible units
        assert!(!library.can_convert("m", "kg"));
        assert!(!library.can_convert("s", "USD"));

        // Unknown units
        assert!(!library.can_convert("xyz", "m"));
    }

    #[test]
    fn test_conversion_accuracy() {
        let library = UnitLibrary::new();

        // Test round-trip conversions (should be close to original)
        let original = 100.0;
        let converted_ft = library.convert(original, "m", "ft").unwrap();
        let back_to_m = library.convert(converted_ft, "ft", "m").unwrap();
        assert!((original - back_to_m).abs() < 0.0001);

        // Test temperature round-trip
        let temp_c = 25.0;
        let temp_f = library.convert(temp_c, "C", "F").unwrap();
        let back_to_c = library.convert(temp_f, "F", "C").unwrap();
        assert!((temp_c - back_to_c).abs() < 0.0001);
    }
}
