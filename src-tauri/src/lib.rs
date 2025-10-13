// Core library for Unicel unit-aware spreadsheet

pub mod core;
pub mod formats;
pub mod mcp;

// Re-export main types
pub use core::{
    cell::Cell,
    units::Unit,
    workbook::Workbook,
};
