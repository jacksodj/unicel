// Core library for Unicel unit-aware spreadsheet

pub mod core;
pub mod formats;
pub mod mcp;
pub mod commands;

// Re-export main types
pub use core::{
    cell::{Cell, CellValue},
    units::Unit,
    workbook::Workbook,
};
