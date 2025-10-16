// Core library for Unicel unit-aware spreadsheet

pub mod commands;
pub mod core;
pub mod formats;
pub mod mcp;

// Re-export main types
pub use core::{
    cell::{Cell, CellValue},
    units::Unit,
    workbook::Workbook,
};
