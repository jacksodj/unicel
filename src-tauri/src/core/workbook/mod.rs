// Workbook management and in-memory SQLite

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workbook {
    /// Workbook-level settings
    pub settings: WorkbookSettings,

    /// Current dirty state
    pub dirty: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbookSettings {
    /// Default unit system (Metric or Imperial)
    pub unit_preference: UnitSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnitSystem {
    Metric,
    Imperial,
}

impl Default for Workbook {
    fn default() -> Self {
        Self {
            settings: WorkbookSettings {
                unit_preference: UnitSystem::Metric,
            },
            dirty: false,
        }
    }
}
