// Unit system with dimensional analysis

use serde::{Deserialize, Serialize};

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
    pub fn dimensionless() -> Self {
        Self {
            canonical: String::new(),
            original: String::new(),
            dimension: Dimension::Dimensionless,
        }
    }
}
