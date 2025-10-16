// Unit preference settings for the application

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Metric system variant
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum MetricSystem {
    /// CGS: centimeter-gram-second
    CGS,
    /// MKS: meter-kilogram-second
    #[default]
    MKS,
}

/// Unit preferences for different categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitPreferences {
    /// Metric system choice (CGS or MKS)
    pub metric_system: MetricSystem,

    /// Preferred unit for each base dimension when in Metric mode
    pub metric_length: String, // e.g., "m" or "cm"
    pub metric_mass: String, // e.g., "kg" or "g"
    pub metric_time: String, // e.g., "s"

    /// Preferred unit for each base dimension when in Imperial mode
    pub imperial_length: String, // e.g., "ft" or "in"
    pub imperial_mass: String, // e.g., "lb" or "oz"
    pub imperial_time: String, // e.g., "s" or "hr"

    /// Preferred digital storage unit
    pub digital_storage_unit: String, // e.g., "GB", "MB", "TB"

    /// Preferred time unit for rates (e.g., per hour vs per month)
    pub time_rate_unit: String, // e.g., "hr", "month", "day"

    /// Preferred currency
    pub currency: String, // e.g., "USD", "EUR", "GBP"

    /// Currency conversion rates (relative to USD)
    /// e.g., {"EUR": 0.92, "GBP": 0.79}
    pub currency_rates: HashMap<String, f64>,

    /// Temperature preference for metric
    pub metric_temperature: String, // e.g., "C"

    /// Temperature preference for imperial
    pub imperial_temperature: String, // e.g., "F"
}

impl Default for UnitPreferences {
    fn default() -> Self {
        let mut rates = HashMap::new();
        rates.insert("USD".to_string(), 1.0);
        rates.insert("EUR".to_string(), 0.92);
        rates.insert("GBP".to_string(), 0.79);
        rates.insert("JPY".to_string(), 149.50);
        rates.insert("CAD".to_string(), 1.36);
        rates.insert("AUD".to_string(), 1.53);

        Self {
            metric_system: MetricSystem::MKS,
            metric_length: "m".to_string(),
            metric_mass: "kg".to_string(),
            metric_time: "s".to_string(),
            imperial_length: "ft".to_string(),
            imperial_mass: "lb".to_string(),
            imperial_time: "s".to_string(),
            digital_storage_unit: "GB".to_string(),
            time_rate_unit: "hr".to_string(),
            currency: "USD".to_string(),
            currency_rates: rates,
            metric_temperature: "C".to_string(),
            imperial_temperature: "F".to_string(),
        }
    }
}

impl UnitPreferences {
    /// Get the preferred display unit for a given base dimension and display mode
    pub fn get_preferred_unit(&self, base_dimension: &str, is_metric: bool) -> String {
        match (base_dimension, is_metric) {
            ("Length", true) => match self.metric_system {
                MetricSystem::CGS => "cm".to_string(),
                MetricSystem::MKS => self.metric_length.clone(),
            },
            ("Length", false) => self.imperial_length.clone(),
            ("Mass", true) => match self.metric_system {
                MetricSystem::CGS => "g".to_string(),
                MetricSystem::MKS => self.metric_mass.clone(),
            },
            ("Mass", false) => self.imperial_mass.clone(),
            ("Time", true) => self.metric_time.clone(),
            ("Time", false) => self.imperial_time.clone(),
            ("Temperature", true) => self.metric_temperature.clone(),
            ("Temperature", false) => self.imperial_temperature.clone(),
            ("Currency", _) => self.currency.clone(),
            ("DigitalStorage", _) => self.digital_storage_unit.clone(),
            _ => base_dimension.to_string(),
        }
    }

    /// Convert currency value from one currency to another
    pub fn convert_currency(&self, value: f64, from: &str, to: &str) -> Option<f64> {
        if from == to {
            return Some(value);
        }

        let from_rate = self.currency_rates.get(from)?;
        let to_rate = self.currency_rates.get(to)?;

        // Convert to USD first, then to target currency
        let usd_value = value / from_rate;
        Some(usd_value * to_rate)
    }

    /// Update a currency conversion rate
    pub fn set_currency_rate(&mut self, currency: String, rate: f64) {
        self.currency_rates.insert(currency, rate);
    }

    /// Get all available currencies
    pub fn get_currencies(&self) -> Vec<String> {
        let mut currencies: Vec<String> = self.currency_rates.keys().cloned().collect();
        currencies.sort();
        currencies
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_preferences() {
        let prefs = UnitPreferences::default();
        assert_eq!(prefs.metric_system, MetricSystem::MKS);
        assert_eq!(prefs.metric_length, "m");
        assert_eq!(prefs.currency, "USD");
    }

    #[test]
    fn test_get_preferred_unit_mks() {
        let prefs = UnitPreferences::default();
        assert_eq!(prefs.get_preferred_unit("Length", true), "m");
        assert_eq!(prefs.get_preferred_unit("Mass", true), "kg");
        assert_eq!(prefs.get_preferred_unit("Length", false), "ft");
    }

    #[test]
    fn test_get_preferred_unit_cgs() {
        let mut prefs = UnitPreferences::default();
        prefs.metric_system = MetricSystem::CGS;
        assert_eq!(prefs.get_preferred_unit("Length", true), "cm");
        assert_eq!(prefs.get_preferred_unit("Mass", true), "g");
    }

    #[test]
    fn test_currency_conversion() {
        let prefs = UnitPreferences::default();

        // USD to USD
        assert_eq!(prefs.convert_currency(100.0, "USD", "USD"), Some(100.0));

        // USD to EUR (approximately, depends on rate)
        let eur = prefs.convert_currency(100.0, "USD", "EUR").unwrap();
        assert!((eur - 92.0).abs() < 1.0);

        // EUR to GBP
        let gbp = prefs.convert_currency(100.0, "EUR", "GBP");
        assert!(gbp.is_some());
    }

    #[test]
    fn test_set_currency_rate() {
        let mut prefs = UnitPreferences::default();
        prefs.set_currency_rate("CAD".to_string(), 1.35);
        assert_eq!(prefs.currency_rates.get("CAD"), Some(&1.35));
    }
}
