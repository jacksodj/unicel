// Excel import/export functionality
// TODO: Implement Excel file format support in Phase 8

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExcelError {
    #[error("Excel support not yet implemented")]
    NotImplemented,
}
