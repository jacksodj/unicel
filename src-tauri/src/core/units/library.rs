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
        library.add_digital_storage_units();

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
    /// Uses BFS to find multi-hop conversion paths if no direct conversion exists
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

        // Try direct conversion first
        if let Some(factor) = self.get_conversion(from, to) {
            return Some(factor.convert(value));
        }

        // If no direct conversion, use BFS to find a path
        self.convert_via_path(value, from, to)
    }

    /// Find and apply a multi-hop conversion path using BFS
    fn convert_via_path(&self, value: f64, from: &str, to: &str) -> Option<f64> {
        use std::collections::{VecDeque, HashSet};

        // BFS to find shortest conversion path
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent: HashMap<String, String> = HashMap::new();

        queue.push_back(from.to_string());
        visited.insert(from.to_string());

        // BFS to find path
        while let Some(current) = queue.pop_front() {
            if current == to {
                // Found path! Reconstruct it
                let mut path: Vec<(String, String)> = Vec::new();
                let mut node = to.to_string();

                while let Some(prev) = parent.get(&node) {
                    path.push((prev.clone(), node.clone()));
                    node = prev.clone();
                }

                path.reverse();

                // Apply conversions along the path
                let mut result = value;
                for (from_unit, to_unit) in path {
                    if let Some(factor) = self.get_conversion(&from_unit, &to_unit) {
                        result = factor.convert(result);
                    } else {
                        return None; // Path exists but conversion missing (shouldn't happen)
                    }
                }

                return Some(result);
            }

            // Explore neighbors (all units that current can convert to)
            for ((conv_from, conv_to), _) in &self.conversions {
                if conv_from == &current && !visited.contains(conv_to) {
                    visited.insert(conv_to.clone());
                    parent.insert(conv_to.clone(), current.clone());
                    queue.push_back(conv_to.clone());
                }
            }
        }

        None // No path found
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
        self.add_unit("hour", Unit::simple("hour", BaseDimension::Time)); // alias for hr
        self.add_unit("day", Unit::simple("day", BaseDimension::Time));
        self.add_unit("month", Unit::simple("month", BaseDimension::Time));
        self.add_unit("year", Unit::simple("year", BaseDimension::Time));

        // Conversions (all to seconds as base)
        self.add_conversion("min", "s", ConversionFactor::new(60.0));
        self.add_conversion("s", "min", ConversionFactor::new(1.0 / 60.0));
        self.add_conversion("hr", "s", ConversionFactor::new(3600.0));
        self.add_conversion("s", "hr", ConversionFactor::new(1.0 / 3600.0));
        self.add_conversion("h", "s", ConversionFactor::new(3600.0));
        self.add_conversion("s", "h", ConversionFactor::new(1.0 / 3600.0));
        self.add_conversion("hour", "s", ConversionFactor::new(3600.0));
        self.add_conversion("s", "hour", ConversionFactor::new(1.0 / 3600.0));
        self.add_conversion("day", "s", ConversionFactor::new(86400.0));
        self.add_conversion("s", "day", ConversionFactor::new(1.0 / 86400.0));
        self.add_conversion("month", "s", ConversionFactor::new(2_628_000.0)); // 30.42 days average
        self.add_conversion("s", "month", ConversionFactor::new(1.0 / 2_628_000.0));
        self.add_conversion("year", "s", ConversionFactor::new(31_536_000.0)); // 365 days
        self.add_conversion("s", "year", ConversionFactor::new(1.0 / 31_536_000.0));

        // Time to Time
        self.add_conversion("hr", "min", ConversionFactor::new(60.0));
        self.add_conversion("min", "hr", ConversionFactor::new(1.0 / 60.0));
        self.add_conversion("hour", "min", ConversionFactor::new(60.0));
        self.add_conversion("min", "hour", ConversionFactor::new(1.0 / 60.0));
        self.add_conversion("day", "hr", ConversionFactor::new(24.0));
        self.add_conversion("hr", "day", ConversionFactor::new(1.0 / 24.0));
        self.add_conversion("day", "hour", ConversionFactor::new(24.0));
        self.add_conversion("hour", "day", ConversionFactor::new(1.0 / 24.0));
        self.add_conversion("month", "day", ConversionFactor::new(30.42));
        self.add_conversion("day", "month", ConversionFactor::new(1.0 / 30.42));
        self.add_conversion("month", "hr", ConversionFactor::new(730.0)); // 30.42 * 24
        self.add_conversion("hr", "month", ConversionFactor::new(1.0 / 730.0));
        self.add_conversion("month", "hour", ConversionFactor::new(730.0));
        self.add_conversion("hour", "month", ConversionFactor::new(1.0 / 730.0));
        self.add_conversion("year", "day", ConversionFactor::new(365.0));
        self.add_conversion("day", "year", ConversionFactor::new(1.0 / 365.0));
        self.add_conversion("year", "month", ConversionFactor::new(12.0));
        self.add_conversion("month", "year", ConversionFactor::new(1.0 / 12.0));
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

    // === Digital Storage Units ===
    fn add_digital_storage_units(&mut self) {
        // Bytes (base 1024)
        self.add_unit("B", Unit::simple("B", BaseDimension::DigitalStorage));
        self.add_unit("KB", Unit::simple("KB", BaseDimension::DigitalStorage));
        self.add_unit("MB", Unit::simple("MB", BaseDimension::DigitalStorage));
        self.add_unit("GB", Unit::simple("GB", BaseDimension::DigitalStorage));
        self.add_unit("TB", Unit::simple("TB", BaseDimension::DigitalStorage));
        self.add_unit("PB", Unit::simple("PB", BaseDimension::DigitalStorage));

        // Bits (lowercase b, base 1000)
        self.add_unit("b", Unit::simple("b", BaseDimension::DigitalStorage));
        self.add_unit("Kb", Unit::simple("Kb", BaseDimension::DigitalStorage));
        self.add_unit("Mb", Unit::simple("Mb", BaseDimension::DigitalStorage));
        self.add_unit("Gb", Unit::simple("Gb", BaseDimension::DigitalStorage));
        self.add_unit("Tb", Unit::simple("Tb", BaseDimension::DigitalStorage));
        self.add_unit("Pb", Unit::simple("Pb", BaseDimension::DigitalStorage));

        // Tokens (for LLMs)
        self.add_unit("Tok", Unit::simple("Tok", BaseDimension::DigitalStorage));
        self.add_unit("MTok", Unit::simple("MTok", BaseDimension::DigitalStorage));

        // Byte conversions (using powers of 1024)
        self.add_conversion("KB", "B", ConversionFactor::new(1024.0));
        self.add_conversion("B", "KB", ConversionFactor::new(1.0 / 1024.0));
        self.add_conversion("MB", "KB", ConversionFactor::new(1024.0));
        self.add_conversion("KB", "MB", ConversionFactor::new(1.0 / 1024.0));
        self.add_conversion("MB", "B", ConversionFactor::new(1024.0 * 1024.0));
        self.add_conversion("B", "MB", ConversionFactor::new(1.0 / (1024.0 * 1024.0)));
        self.add_conversion("GB", "MB", ConversionFactor::new(1024.0));
        self.add_conversion("MB", "GB", ConversionFactor::new(1.0 / 1024.0));
        self.add_conversion("GB", "KB", ConversionFactor::new(1024.0 * 1024.0));
        self.add_conversion("KB", "GB", ConversionFactor::new(1.0 / (1024.0 * 1024.0)));
        self.add_conversion("GB", "B", ConversionFactor::new(1024.0 * 1024.0 * 1024.0));
        self.add_conversion("B", "GB", ConversionFactor::new(1.0 / (1024.0 * 1024.0 * 1024.0)));
        self.add_conversion("TB", "GB", ConversionFactor::new(1024.0));
        self.add_conversion("GB", "TB", ConversionFactor::new(1.0 / 1024.0));
        self.add_conversion("TB", "MB", ConversionFactor::new(1024.0 * 1024.0));
        self.add_conversion("MB", "TB", ConversionFactor::new(1.0 / (1024.0 * 1024.0)));
        self.add_conversion("TB", "B", ConversionFactor::new(1024.0 * 1024.0 * 1024.0 * 1024.0));
        self.add_conversion("B", "TB", ConversionFactor::new(1.0 / (1024.0 * 1024.0 * 1024.0 * 1024.0)));
        self.add_conversion("PB", "TB", ConversionFactor::new(1024.0));
        self.add_conversion("TB", "PB", ConversionFactor::new(1.0 / 1024.0));

        // Bit conversions (using powers of 1000)
        self.add_conversion("Kb", "b", ConversionFactor::new(1000.0));
        self.add_conversion("b", "Kb", ConversionFactor::new(1.0 / 1000.0));
        self.add_conversion("Mb", "Kb", ConversionFactor::new(1000.0));
        self.add_conversion("Kb", "Mb", ConversionFactor::new(1.0 / 1000.0));
        self.add_conversion("Mb", "b", ConversionFactor::new(1_000_000.0));
        self.add_conversion("b", "Mb", ConversionFactor::new(1.0 / 1_000_000.0));
        self.add_conversion("Gb", "Mb", ConversionFactor::new(1000.0));
        self.add_conversion("Mb", "Gb", ConversionFactor::new(1.0 / 1000.0));
        self.add_conversion("Gb", "Kb", ConversionFactor::new(1_000_000.0));
        self.add_conversion("Kb", "Gb", ConversionFactor::new(1.0 / 1_000_000.0));
        self.add_conversion("Gb", "b", ConversionFactor::new(1_000_000_000.0));
        self.add_conversion("b", "Gb", ConversionFactor::new(1.0 / 1_000_000_000.0));
        self.add_conversion("Tb", "Gb", ConversionFactor::new(1000.0));
        self.add_conversion("Gb", "Tb", ConversionFactor::new(1.0 / 1000.0));
        self.add_conversion("Tb", "Mb", ConversionFactor::new(1_000_000.0));
        self.add_conversion("Mb", "Tb", ConversionFactor::new(1.0 / 1_000_000.0));
        self.add_conversion("Tb", "b", ConversionFactor::new(1_000_000_000_000.0));
        self.add_conversion("b", "Tb", ConversionFactor::new(1.0 / 1_000_000_000_000.0));
        self.add_conversion("Pb", "Tb", ConversionFactor::new(1000.0));
        self.add_conversion("Tb", "Pb", ConversionFactor::new(1.0 / 1000.0));

        // Bits to Bytes conversions (1 byte = 8 bits)
        self.add_conversion("B", "b", ConversionFactor::new(8.0));
        self.add_conversion("b", "B", ConversionFactor::new(1.0 / 8.0));
        self.add_conversion("KB", "Kb", ConversionFactor::new(8.192)); // 1024 bytes * 8 / 1000
        self.add_conversion("Kb", "KB", ConversionFactor::new(1.0 / 8.192));
        self.add_conversion("MB", "Mb", ConversionFactor::new(8.388608)); // 1024^2 * 8 / 1000^2
        self.add_conversion("Mb", "MB", ConversionFactor::new(1.0 / 8.388608));
        self.add_conversion("GB", "Gb", ConversionFactor::new(8.589934592)); // 1024^3 * 8 / 1000^3
        self.add_conversion("Gb", "GB", ConversionFactor::new(1.0 / 8.589934592));
        self.add_conversion("TB", "Tb", ConversionFactor::new(8.796093022208)); // 1024^4 * 8 / 1000^4
        self.add_conversion("Tb", "TB", ConversionFactor::new(1.0 / 8.796093022208));

        // Additional cross-magnitude conversions (bits to bytes)
        // Gb to TB: Gb → GB → TB
        self.add_conversion("Gb", "TB", ConversionFactor::new(1.0 / (8.589934592 * 1024.0))); // Gb → GB → TB
        self.add_conversion("TB", "Gb", ConversionFactor::new(8.589934592 * 1024.0));
        // Mb to GB
        self.add_conversion("Mb", "GB", ConversionFactor::new(1.0 / (8.388608 * 1024.0)));
        self.add_conversion("GB", "Mb", ConversionFactor::new(8.388608 * 1024.0));
        // Kb to MB
        self.add_conversion("Kb", "MB", ConversionFactor::new(1.0 / (8.192 * 1024.0)));
        self.add_conversion("MB", "Kb", ConversionFactor::new(8.192 * 1024.0));

        // Token conversions
        self.add_conversion("MTok", "Tok", ConversionFactor::new(1_000_000.0));
        self.add_conversion("Tok", "MTok", ConversionFactor::new(1.0 / 1_000_000.0));
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

    // Property-based tests for conversion commutativity
    #[cfg(test)]
    mod proptests {
        use super::*;
        use proptest::prelude::*;

        // Define unit pairs for each dimension to test
        const LENGTH_UNITS: &[&str] = &["m", "cm", "mm", "km", "in", "ft", "yd", "mi"];
        const MASS_UNITS: &[&str] = &["g", "kg", "mg", "oz", "lb"];
        const TIME_UNITS: &[&str] = &["s", "min", "hr", "h", "hour", "day", "month", "year"];
        const TEMPERATURE_UNITS: &[&str] = &["C", "F", "K"];
        const CURRENCY_UNITS: &[&str] = &["USD", "EUR", "GBP"];
        const STORAGE_UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "b", "Kb", "Mb", "Gb", "Tb"];

        /// Test that converting A → B → A returns the original value (within tolerance)
        fn test_round_trip_conversion(library: &UnitLibrary, from: &str, to: &str, value: f64, tolerance: f64) {
            if let Some(forward) = library.convert(value, from, to) {
                if let Some(back) = library.convert(forward, to, from) {
                    let diff = (value - back).abs();
                    let relative_error = if value != 0.0 { diff / value.abs() } else { diff };
                    assert!(
                        relative_error < tolerance,
                        "Round-trip conversion failed: {} {} → {} → {} (got {}, expected {}, error: {})",
                        value, from, to, from, back, value, relative_error
                    );
                }
            }
        }

        proptest! {
            #[test]
            fn prop_length_round_trip(
                value in -1e6..1e6f64,
                from_idx in 0..LENGTH_UNITS.len(),
                to_idx in 0..LENGTH_UNITS.len()
            ) {
                let library = UnitLibrary::new();
                let from = LENGTH_UNITS[from_idx];
                let to = LENGTH_UNITS[to_idx];
                // Use 0.01% tolerance for floating point errors
                test_round_trip_conversion(&library, from, to, value, 1e-4);
            }

            #[test]
            fn prop_mass_round_trip(
                value in -1e6..1e6f64,
                from_idx in 0..MASS_UNITS.len(),
                to_idx in 0..MASS_UNITS.len()
            ) {
                let library = UnitLibrary::new();
                let from = MASS_UNITS[from_idx];
                let to = MASS_UNITS[to_idx];
                test_round_trip_conversion(&library, from, to, value, 1e-4);
            }

            #[test]
            fn prop_time_round_trip(
                value in -1e6..1e6f64,
                from_idx in 0..TIME_UNITS.len(),
                to_idx in 0..TIME_UNITS.len()
            ) {
                let library = UnitLibrary::new();
                let from = TIME_UNITS[from_idx];
                let to = TIME_UNITS[to_idx];
                test_round_trip_conversion(&library, from, to, value, 1e-4);
            }

            #[test]
            fn prop_temperature_round_trip(
                value in -273.15..1e4f64, // Absolute zero minimum for Celsius
                from_idx in 0..TEMPERATURE_UNITS.len(),
                to_idx in 0..TEMPERATURE_UNITS.len()
            ) {
                let library = UnitLibrary::new();
                let from = TEMPERATURE_UNITS[from_idx];
                let to = TEMPERATURE_UNITS[to_idx];
                // Temperature conversions with offsets need slightly higher tolerance
                test_round_trip_conversion(&library, from, to, value, 1e-3);
            }

            #[test]
            fn prop_currency_round_trip(
                value in -1e9..1e9f64,
                from_idx in 0..CURRENCY_UNITS.len(),
                to_idx in 0..CURRENCY_UNITS.len()
            ) {
                let library = UnitLibrary::new();
                let from = CURRENCY_UNITS[from_idx];
                let to = CURRENCY_UNITS[to_idx];
                test_round_trip_conversion(&library, from, to, value, 1e-4);
            }

            #[test]
            fn prop_storage_round_trip(
                value in 0.0..1e15f64, // Storage must be non-negative
                from_idx in 0..STORAGE_UNITS.len(),
                to_idx in 0..STORAGE_UNITS.len()
            ) {
                let library = UnitLibrary::new();
                let from = STORAGE_UNITS[from_idx];
                let to = STORAGE_UNITS[to_idx];
                test_round_trip_conversion(&library, from, to, value, 1e-4);
            }

            /// Test that conversion is commutative: if A→B exists, then B→A should exist
            #[test]
            fn prop_conversion_symmetry(
                from_idx in 0..LENGTH_UNITS.len(),
                to_idx in 0..LENGTH_UNITS.len()
            ) {
                let library = UnitLibrary::new();
                let from = LENGTH_UNITS[from_idx];
                let to = LENGTH_UNITS[to_idx];

                let forward_exists = library.can_convert(from, to);
                let backward_exists = library.can_convert(to, from);

                // If forward conversion exists, backward should also exist (except for same unit)
                if forward_exists && from != to {
                    assert!(backward_exists, "Conversion {} → {} exists but {} → {} doesn't", from, to, to, from);
                }
            }

            /// Test edge cases: zero, very small, very large numbers
            #[test]
            fn prop_zero_conversion(
                from_idx in 0..LENGTH_UNITS.len(),
                to_idx in 0..LENGTH_UNITS.len()
            ) {
                let library = UnitLibrary::new();
                let from = LENGTH_UNITS[from_idx];
                let to = LENGTH_UNITS[to_idx];

                if let Some(result) = library.convert(0.0, from, to) {
                    // Zero should always convert to zero (except temperature with offsets)
                    if !TEMPERATURE_UNITS.contains(&from) {
                        assert_eq!(result, 0.0, "Converting 0 {} to {} should yield 0, got {}", from, to, result);
                    }
                }
            }

            /// Test negative values work correctly
            #[test]
            fn prop_negative_values(
                value in -1e6..-1.0f64,
                from_idx in 0..LENGTH_UNITS.len(),
                to_idx in 0..LENGTH_UNITS.len()
            ) {
                let library = UnitLibrary::new();
                let from = LENGTH_UNITS[from_idx];
                let to = LENGTH_UNITS[to_idx];

                if let Some(converted) = library.convert(value, from, to) {
                    // Negative input should generally give negative output (except temperature)
                    if !TEMPERATURE_UNITS.contains(&from) && value < 0.0 {
                        assert!(converted < 0.0, "Converting negative {} {} to {} should remain negative", value, from, to);
                    }
                }
            }
        }
    }
}
