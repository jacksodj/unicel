// Cell data structure and operations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::units::Unit;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    /// Numeric value
    pub value: f64,

    /// Unit as stored (never changes unless user edits or CONVERT() used)
    pub stored_unit: Unit,

    /// Optional formula expression
    pub formula: Option<String>,

    /// Optional display unit override (from column setting)
    pub display_override: Option<Unit>,

    /// Last modified timestamp
    pub modified_at: DateTime<Utc>,
}

impl Cell {
    pub fn new(value: f64, unit: Unit) -> Self {
        Self {
            value,
            stored_unit: unit,
            formula: None,
            display_override: None,
            modified_at: Utc::now(),
        }
    }
}
