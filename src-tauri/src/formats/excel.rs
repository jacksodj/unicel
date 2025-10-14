// Excel import/export functionality

use crate::core::workbook::Workbook;
use rust_xlsxwriter::{Format, Workbook as XlsxWorkbook, Worksheet};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExcelError {
    #[error("Excel export failed: {0}")]
    ExportFailed(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Excel library error: {0}")]
    XlsxError(#[from] rust_xlsxwriter::XlsxError),
}

/// Export a workbook to Excel format
pub fn export_to_excel(
    workbook: &Workbook,
    path: impl AsRef<Path>,
) -> Result<(), ExcelError> {
    let mut xlsx_workbook = XlsxWorkbook::new();

    // Add warning sheet first
    let mut warning_sheet = create_warning_sheet()?;
    warning_sheet.set_name("⚠️ READ THIS FIRST")?;
    xlsx_workbook.push_worksheet(warning_sheet);

    // Track metadata for all cells with units
    let mut metadata_rows: Vec<(String, String, String, String)> = Vec::new();

    // Export each sheet
    for sheet_index in 0..workbook.sheet_count() {
        let sheet = workbook
            .get_sheet(sheet_index)
            .ok_or_else(|| ExcelError::ExportFailed(format!("Sheet {} not found", sheet_index)))?;

        let mut worksheet = Worksheet::new();
        worksheet.set_name(sheet.name())?;

        // Get all cell addresses and sort them
        let mut addresses = sheet.cell_addresses();
        addresses.sort_by(|a, b| {
            // Sort by row first, then by column
            match a.row.cmp(&b.row) {
                std::cmp::Ordering::Equal => a.col.cmp(&b.col),
                other => other,
            }
        });

        // Create formats
        let formula_format = Format::new().set_font_color(rust_xlsxwriter::Color::Blue).set_italic();
        let _unit_format = Format::new().set_italic();

        // Export each cell - using doubled columns (value, unit, value, unit, ...)
        for addr in addresses {
            if let Some(cell) = sheet.get(&addr) {
                // Convert column letter to number (A=0, B=1, etc.)
                // Double it to leave room for unit columns
                let col_num = column_letter_to_number(&addr.col) * 2;
                let row_num = (addr.row - 1) as u32; // Excel rows are 0-indexed internally

                let cell_ref = format!("{}{}", addr.col, addr.row);

                // Export the cell based on its content
                if cell.formula().is_some() {
                    // For formulas, export the CALCULATED VALUE (not the formula)
                    // because Excel can't recalculate with unit logic
                    if let Some(value) = cell.as_number() {
                        // Value in column N*2
                        worksheet.write_number_with_format(row_num, col_num as u16, value, &formula_format)?;

                        // Unit in column N*2+1
                        let unit_str = cell.storage_unit().canonical();
                        if !unit_str.is_empty() && unit_str != "1" {
                            worksheet.write_string(row_num, (col_num + 1) as u16, unit_str)?;

                            // Track metadata
                            metadata_rows.push((
                                sheet.name().to_string(),
                                cell_ref,
                                format!("Formula: {}", cell.formula().unwrap()),
                                unit_str.to_string(),
                            ));
                        }
                    }
                } else if let Some(value) = cell.as_number() {
                    // Export raw number (no units) so Excel can use it in calculations
                    // Value in column N*2
                    worksheet.write_number(row_num, col_num as u16, value)?;

                    // Unit in column N*2+1
                    let unit_str = cell.storage_unit().canonical();
                    if !unit_str.is_empty() && unit_str != "1" {
                        worksheet.write_string(row_num, (col_num + 1) as u16, unit_str)?;

                        // Track metadata
                        metadata_rows.push((
                            sheet.name().to_string(),
                            cell_ref,
                            "Value".to_string(),
                            unit_str.to_string(),
                        ));
                    }
                } else if let Some(text) = cell.as_text() {
                    // Export text in column N*2
                    worksheet.write_string(row_num, col_num as u16, text)?;
                }
            }
        }

        xlsx_workbook.push_worksheet(worksheet);
    }

    // Add metadata sheet at the end
    if !metadata_rows.is_empty() {
        let metadata_sheet = create_metadata_sheet(metadata_rows)?;
        xlsx_workbook.push_worksheet(metadata_sheet);
    }

    // Save the workbook
    xlsx_workbook.save(path)?;

    Ok(())
}

/// Create a warning sheet explaining the export limitations
fn create_warning_sheet() -> Result<Worksheet, ExcelError> {
    let mut sheet = Worksheet::new();

    let header_format = Format::new()
        .set_bold()
        .set_font_size(16)
        .set_background_color(rust_xlsxwriter::Color::RGB(0xFF6B6B));

    let bold_format = Format::new().set_bold();

    sheet.write_string_with_format(0, 0, "⚠️ IMPORTANT: Unit Information Lost in Excel Export", &header_format)?;

    sheet.write_string(2, 0, "This Excel file was exported from Unicel, a unit-aware spreadsheet.")?;
    sheet.write_string(3, 0, "Excel does NOT support units, so:")?;

    sheet.write_string_with_format(5, 0, "1. All unit information has been removed from cells", &bold_format)?;
    sheet.write_string(6, 0, "   - Numbers are exported as plain values (e.g., 100 instead of \"100 m\")")?;
    sheet.write_string(7, 0, "   - This allows Excel formulas to work, but units are lost")?;

    sheet.write_string_with_format(9, 0, "2. Formulas show calculated results, NOT the original formulas", &bold_format)?;
    sheet.write_string(10, 0, "   - Formula cells (in blue italic) show the final value")?;
    sheet.write_string(11, 0, "   - Excel cannot recalculate them because it doesn't understand unit operations")?;

    sheet.write_string_with_format(13, 0, "3. See the \"Unit Metadata\" sheet for original unit information", &bold_format)?;
    sheet.write_string(14, 0, "   - Lists which cells had units and what those units were")?;
    sheet.write_string(15, 0, "   - Shows original formulas for formula cells")?;

    sheet.write_string(17, 0, "⚠️ WARNING: Changes made in Excel cannot be imported back to Unicel")?;
    sheet.write_string(18, 0, "This is a ONE-WAY export for sharing data only.")?;

    sheet.write_string_with_format(20, 0, "To preserve full unit information, use Unicel's native .usheet format.", &bold_format)?;

    // Set column width
    sheet.set_column_width(0, 80)?;

    Ok(sheet)
}

/// Create a metadata sheet documenting which cells have units
fn create_metadata_sheet(rows: Vec<(String, String, String, String)>) -> Result<Worksheet, ExcelError> {
    let mut sheet = Worksheet::new();
    sheet.set_name("Unit Metadata")?;

    let header_format = Format::new().set_bold().set_background_color(rust_xlsxwriter::Color::RGB(0xD3D3D3));

    // Headers
    sheet.write_string_with_format(0, 0, "Sheet", &header_format)?;
    sheet.write_string_with_format(0, 1, "Cell", &header_format)?;
    sheet.write_string_with_format(0, 2, "Type", &header_format)?;
    sheet.write_string_with_format(0, 3, "Unit", &header_format)?;

    // Data rows
    for (idx, (sheet_name, cell_ref, cell_type, unit)) in rows.iter().enumerate() {
        let row = (idx + 1) as u32;
        sheet.write_string(row, 0, sheet_name)?;
        sheet.write_string(row, 1, cell_ref)?;
        sheet.write_string(row, 2, cell_type)?;
        sheet.write_string(row, 3, unit)?;
    }

    // Set column widths
    sheet.set_column_width(0, 20)?;
    sheet.set_column_width(1, 10)?;
    sheet.set_column_width(2, 30)?;
    sheet.set_column_width(3, 15)?;

    Ok(sheet)
}

/// Convert column letter to number (A=0, B=1, ..., Z=25, AA=26, etc.)
fn column_letter_to_number(col: &str) -> u32 {
    let mut num = 0u32;
    for ch in col.chars() {
        num = num * 26 + (ch.to_ascii_uppercase() as u32 - 'A' as u32 + 1);
    }
    num - 1 // Convert to 0-indexed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_letter_to_number() {
        assert_eq!(column_letter_to_number("A"), 0);
        assert_eq!(column_letter_to_number("B"), 1);
        assert_eq!(column_letter_to_number("Z"), 25);
        assert_eq!(column_letter_to_number("AA"), 26);
        assert_eq!(column_letter_to_number("AB"), 27);
        assert_eq!(column_letter_to_number("AZ"), 51);
        assert_eq!(column_letter_to_number("BA"), 52);
    }
}
