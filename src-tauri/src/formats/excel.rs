// Excel import/export functionality

use crate::core::workbook::Workbook;
use crate::core::table::Sheet;
use crate::core::units::UnitLibrary;
use rust_xlsxwriter::{Format, Workbook as XlsxWorkbook, Worksheet};
use std::collections::BTreeMap;
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

/// Represents a unit conversion factor used in the workbook
#[derive(Debug, Clone)]
struct ConversionEntry {
    name: String,
    multiplier: f64,
    offset: f64,
    from_unit: String,
    to_unit: String,
    description: String,
}

/// Expand CONVERT functions in formulas to Excel-compatible math with conversion factors
/// Returns the expanded formula and optionally a conversion entry to add to the Conversions sheet
/// Example: =CONVERT(A1, 1ft) becomes =A1*Conversions!B2 where B2 contains the conversion factor
fn expand_convert_formula(
    formula: &str,
    sheet: &Sheet,
    conversions: &mut BTreeMap<String, ConversionEntry>,
) -> Option<(String, Option<ConversionEntry>)> {
    // Simple regex-based approach to detect CONVERT function
    // Format: CONVERT(arg1, arg2) where arg2 can be "1 unit" or a cell reference

    if !formula.to_uppercase().contains("CONVERT") {
        tracing::trace!("Formula does not contain CONVERT: {}", formula);
        return None;
    }

    tracing::debug!("Processing CONVERT formula: {}", formula);

    let library = UnitLibrary::new();

    // Try to parse and expand the formula
    // This is a simplified implementation that handles common cases
    let formula_upper = formula.to_uppercase();

    // Find CONVERT( position
    let convert_pos = formula_upper.find("CONVERT(")?;
    let start = convert_pos + 8; // After "CONVERT("

    // Find matching closing parenthesis
    let mut paren_depth = 1;
    let mut end = start;
    for (i, ch) in formula[start..].chars().enumerate() {
        match ch {
            '(' => paren_depth += 1,
            ')' => {
                paren_depth -= 1;
                if paren_depth == 0 {
                    end = start + i;
                    break;
                }
            }
            _ => {}
        }
    }

    if paren_depth != 0 {
        tracing::warn!("Unmatched parentheses in CONVERT formula: {}", formula);
        return None; // Unmatched parentheses
    }

    // Extract arguments
    let args_str = &formula[start..end];
    let args: Vec<&str> = args_str.split(',').map(|s| s.trim()).collect();

    if args.len() != 2 {
        tracing::warn!("CONVERT function requires exactly 2 arguments, got {}: {}", args.len(), formula);
        return None; // CONVERT requires exactly 2 arguments
    }

    let source_expr = args[0];
    let target_unit_expr = args[1];

    tracing::debug!("CONVERT args: source='{}', target='{}'", source_expr, target_unit_expr);

    // Determine source unit
    // If source is a cell reference, get its unit
    let source_unit = if let Some(source_cell) = parse_cell_ref(source_expr) {
        tracing::debug!("Source is cell reference: {:?}", source_cell);
        match sheet.get(&source_cell) {
            Some(cell) => {
                let unit = cell.storage_unit().canonical().to_string();
                tracing::debug!("Source cell unit: {}", unit);
                unit
            }
            None => {
                tracing::warn!("Source cell not found: {:?}", source_cell);
                return None;
            }
        }
    } else {
        tracing::debug!("Source is value expression");
        match extract_unit_from_value(source_expr) {
            Some(unit) => {
                tracing::debug!("Extracted source unit: {}", unit);
                unit
            }
            None => {
                tracing::warn!("Could not extract unit from source: {}", source_expr);
                return None;
            }
        }
    };

    // Determine target unit
    let target_unit = if let Some(target_cell) = parse_cell_ref(target_unit_expr) {
        tracing::debug!("Target is cell reference: {:?}", target_cell);
        // Cell reference - get unit from cell
        match sheet.get(&target_cell) {
            Some(cell) => {
                // Check if it's text (unit name) or has a storage unit
                if let Some(text) = cell.as_text() {
                    tracing::debug!("Target cell contains text: {}", text);
                    text.trim().to_string()
                } else {
                    let unit = cell.storage_unit().canonical().to_string();
                    tracing::debug!("Target cell unit: {}", unit);
                    unit
                }
            }
            None => {
                tracing::warn!("Target cell not found: {:?}", target_cell);
                return None;
            }
        }
    } else {
        tracing::debug!("Target is unit expression: {}", target_unit_expr);
        match extract_unit_from_target(target_unit_expr) {
            Some(unit) => {
                tracing::debug!("Extracted target unit: {}", unit);
                unit
            }
            None => {
                tracing::warn!("Could not extract unit from target: {}", target_unit_expr);
                return None;
            }
        }
    };

    // Get conversion factor (with multiplier and offset)
    tracing::debug!("Looking up conversion: {} -> {}", source_unit, target_unit);

    // Try to get direct conversion first
    let conversion_factor = if let Some(factor) = library.get_conversion(&source_unit, &target_unit) {
        tracing::debug!("Found direct conversion: multiplier={}, offset={}", factor.multiplier, factor.offset);
        (factor.multiplier, factor.offset)
    } else {
        // Fall back to converting 1.0 to get the factor (for multi-hop conversions)
        // This works for linear conversions but won't capture offset correctly
        // For now, we'll use this as fallback but warn if it's a temperature conversion
        match library.convert(1.0, &source_unit, &target_unit) {
            Some(result) => {
                tracing::debug!("Using computed conversion factor: {}", result);
                // Check if this might have an offset (temperature units)
                if is_temperature_unit(&source_unit) || is_temperature_unit(&target_unit) {
                    tracing::warn!("Multi-hop temperature conversion detected - offset may be incorrect");
                }
                (result, 0.0) // Assume no offset for multi-hop
            }
            None => {
                tracing::warn!("No conversion found for {} -> {}", source_unit, target_unit);
                return None;
            }
        }
    };

    let (multiplier, offset) = conversion_factor;

    // Create conversion name (e.g., "m_to_ft")
    let conversion_name = format!("{}_{}",
        source_unit.replace('/', "_per_").replace(' ', "_"),
        target_unit.replace('/', "_per_").replace(' ', "_")
    );

    // Create conversion entry
    let conversion_entry = ConversionEntry {
        name: conversion_name.clone(),
        multiplier,
        offset,
        from_unit: source_unit.clone(),
        to_unit: target_unit.clone(),
        description: if offset != 0.0 {
            format!("Convert {} to {}: value * {} + {}", source_unit, target_unit, multiplier, offset)
        } else {
            format!("Convert {} to {}: value * {}", source_unit, target_unit, multiplier)
        },
    };

    // Build expanded formula
    let expanded = if multiplier == 1.0 && offset == 0.0 {
        // No conversion needed
        source_expr.to_string()
    } else {
        // Add to conversions map if not already present
        if !conversions.contains_key(&conversion_name) {
            conversions.insert(conversion_name.clone(), conversion_entry.clone());
        }

        // Generate Excel formula based on whether there's an offset
        if offset != 0.0 {
            // For temperature conversions: value * multiplier + offset
            // Create named ranges for both multiplier and offset
            format!("{}*{}_m+{}_o", source_expr, conversion_name, conversion_name)
        } else {
            // For simple conversions: value * multiplier
            format!("{}*{}_m", source_expr, conversion_name)
        }
    };

    // Replace CONVERT(...) with expanded version
    let before = &formula[..convert_pos];
    let after = &formula[end + 1..]; // After the closing )

    Some((format!("{}{}{}", before, expanded, after), Some(conversion_entry)))
}

/// Parse a cell reference like "A1", "B2", etc.
/// Valid format: LETTERS followed by NUMBERS (e.g., "A1", "AB123")
/// Invalid: "1m" (numbers before letters)
fn parse_cell_ref(s: &str) -> Option<crate::core::table::CellAddr> {
    use crate::core::table::CellAddr;

    let trimmed = s.trim();
    let mut col = String::new();
    let mut row = String::new();
    let mut seen_digit = false;

    for ch in trimmed.chars() {
        if ch.is_ascii_alphabetic() {
            if seen_digit {
                // Letters after digits -> not a valid cell reference (e.g., "1m")
                return None;
            }
            col.push(ch.to_ascii_uppercase());
        } else if ch.is_ascii_digit() {
            seen_digit = true;
            row.push(ch);
        } else {
            return None; // Invalid character
        }
    }

    if col.is_empty() || row.is_empty() {
        return None;
    }

    let row_num: usize = row.parse().ok()?;
    Some(CellAddr::new(col, row_num))
}

/// Transform cell references in a formula to account for doubled columns in Excel export
/// Example: "=A1+B2" becomes "=A1+C2" because B maps to Excel column C
/// Handles both uppercase and lowercase cell references
fn transform_formula_for_excel(formula: &str) -> String {
    let mut result = String::new();
    let mut chars = formula.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch.is_ascii_alphabetic() {
            // Potential cell reference - collect the column letters
            let mut col = String::new();
            col.push(ch.to_ascii_uppercase());

            // Collect additional column letters
            while let Some(&next_ch) = chars.peek() {
                if next_ch.is_ascii_alphabetic() {
                    col.push(chars.next().unwrap().to_ascii_uppercase());
                } else {
                    break;
                }
            }

            // Check if followed by digits (making it a cell reference)
            let mut row = String::new();
            while let Some(&next_ch) = chars.peek() {
                if next_ch.is_ascii_digit() {
                    row.push(chars.next().unwrap());
                } else {
                    break;
                }
            }

            if !row.is_empty() {
                // This is a cell reference - transform it
                let col_num = column_letter_to_number(&col);
                let excel_col_num = col_num * 2; // Double for value/unit layout
                let excel_col = number_to_column_letter(excel_col_num);
                result.push_str(&excel_col);
                result.push_str(&row);
            } else {
                // Not a cell reference, just letters (like a function name)
                // Preserve original case for function names
                result.push_str(&col);
            }
        } else {
            result.push(ch);
        }
    }

    result
}

/// Convert column number to letter (inverse of column_letter_to_number)
/// 0 -> "A", 1 -> "B", 25 -> "Z", 26 -> "AA", etc.
fn number_to_column_letter(mut num: u32) -> String {
    let mut result = String::new();
    num += 1; // Convert from 0-indexed to 1-indexed

    while num > 0 {
        num -= 1; // Adjust for 0-based modulo
        let remainder = (num % 26) as u8;
        result.push((b'A' + remainder) as char);
        num /= 26;
    }

    result.chars().rev().collect()
}

/// Extract unit from a value expression like "100 m" or "1 km"
fn extract_unit_from_value(s: &str) -> Option<String> {
    let parts: Vec<&str> = s.trim().split_whitespace().collect();
    if parts.len() >= 2 {
        Some(parts[1..].join(" "))
    } else {
        None
    }
}

/// Extract unit from target expression like "1 ft" or "1km"
fn extract_unit_from_target(s: &str) -> Option<String> {
    let trimmed = s.trim();

    // Try "1 unit" format
    if let Some(space_pos) = trimmed.find(' ') {
        return Some(trimmed[space_pos + 1..].trim().to_string());
    }

    // Try "1unit" format (no space)
    if trimmed.starts_with('1') {
        return Some(trimmed[1..].trim().to_string());
    }

    None
}

/// Check if a unit is a temperature unit
fn is_temperature_unit(unit: &str) -> bool {
    matches!(unit, "C" | "F" | "K")
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

    // Track all conversions used in formulas (BTreeMap for consistent ordering)
    let mut conversions: BTreeMap<String, ConversionEntry> = BTreeMap::new();

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
                if let Some(formula) = cell.formula() {
                    // Check if formula contains CONVERT function
                    if formula.to_uppercase().contains("CONVERT") {
                        // Try to expand CONVERT functions to Excel-compatible formulas
                        if let Some((expanded_formula, _conversion_entry)) = expand_convert_formula(formula, sheet, &mut conversions) {
                            // Transform cell references for doubled-column layout
                            let excel_formula = transform_formula_for_excel(&expanded_formula);
                            tracing::debug!("Exporting CONVERT formula: {} → {} → {}", formula, expanded_formula, excel_formula);
                            // Export as Excel formula with expanded CONVERT
                            worksheet.write_formula_with_format(row_num, col_num as u16, excel_formula.as_str(), &formula_format)?;

                            // Unit in column N*2+1
                            let unit_str = cell.storage_unit().canonical();
                            if !unit_str.is_empty() && unit_str != "1" {
                                worksheet.write_string(row_num, (col_num + 1) as u16, unit_str)?;

                                // Track metadata
                                metadata_rows.push((
                                    sheet.name().to_string(),
                                    cell_ref.clone(),
                                    format!("Original: {} → Excel: {}", formula, excel_formula),
                                    unit_str.to_string(),
                                ));
                            }
                        } else {
                            tracing::warn!("Failed to expand CONVERT formula, exporting as VALUE: {}", formula);
                            // CONVERT expansion failed, export the CALCULATED VALUE
                            if let Some(value) = cell.as_number() {
                                worksheet.write_number_with_format(row_num, col_num as u16, value, &formula_format)?;

                                let unit_str = cell.storage_unit().canonical();
                                if !unit_str.is_empty() && unit_str != "1" {
                                    worksheet.write_string(row_num, (col_num + 1) as u16, unit_str)?;
                                    metadata_rows.push((
                                        sheet.name().to_string(),
                                        cell_ref.clone(),
                                        format!("Failed CONVERT: {}", formula),
                                        unit_str.to_string(),
                                    ));
                                }
                            }
                        }
                    } else {
                        // Regular formula (no CONVERT) - transform and export
                        let excel_formula = transform_formula_for_excel(formula);
                        tracing::debug!("Exporting regular formula: {} → {}", formula, excel_formula);
                        worksheet.write_formula_with_format(row_num, col_num as u16, excel_formula.as_str(), &formula_format)?;

                        // Unit in column N*2+1
                        let unit_str = cell.storage_unit().canonical();
                        if !unit_str.is_empty() && unit_str != "1" {
                            worksheet.write_string(row_num, (col_num + 1) as u16, unit_str)?;

                            // Track metadata
                            metadata_rows.push((
                                sheet.name().to_string(),
                                cell_ref.clone(),
                                format!("Formula: {}", formula),
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

    // Add conversions sheet if any conversions were used
    if !conversions.is_empty() {
        let conversions_sheet = create_conversions_sheet(&conversions)?;
        xlsx_workbook.push_worksheet(conversions_sheet);

        // Define named ranges for each conversion factor
        // For each conversion, create two named ranges: name_m (multiplier) and name_o (offset)
        for (idx, (name, entry)) in conversions.iter().enumerate() {
            let row = idx + 1; // +1 because row 0 is the header
            let excel_row = row + 1; // +1 for Excel 1-based indexing

            // Define multiplier named range (column B)
            let mult_cell_ref = format!("=Conversions!$B${}", excel_row);
            let mult_name = format!("{}_m", name);
            xlsx_workbook.define_name(&mult_name, &mult_cell_ref)?;
            tracing::debug!("Defined multiplier named range: {} -> {}", mult_name, mult_cell_ref);

            // Define offset named range (column C) - only if offset is non-zero
            if entry.offset != 0.0 {
                let offset_cell_ref = format!("=Conversions!$C${}", excel_row);
                let offset_name = format!("{}_o", name);
                xlsx_workbook.define_name(&offset_name, &offset_cell_ref)?;
                tracing::debug!("Defined offset named range: {} -> {}", offset_name, offset_cell_ref);
            }
        }
    }

    // Export user-defined named ranges from the workbook
    let named_ranges = workbook.list_named_ranges();
    for (name, sheet_index, addr) in &named_ranges {
        // Get the sheet name for the named range
        if let Some(sheet) = workbook.get_sheet(*sheet_index) {
            // Convert column letter to number and double it (for value/unit layout)
            let col_num = column_letter_to_number(&addr.col) * 2;
            let excel_col = number_to_column_letter(col_num);

            // Excel uses 1-based row indexing
            let excel_row = addr.row;

            // Create the reference in Excel format
            // If sheet name contains spaces or special chars, wrap in single quotes
            let cell_ref = if sheet.name().contains(' ') || sheet.name().contains('!') {
                format!("='{}'!${}${}", sheet.name(), excel_col, excel_row)
            } else {
                format!("={}!${}${}", sheet.name(), excel_col, excel_row)
            };

            // Define the named range in Excel
            xlsx_workbook.define_name(name, &cell_ref)?;
            tracing::debug!("Defined user named range: {} -> {}", name, cell_ref);
        }
    }

    // Add metadata sheet at the end (include named ranges info)
    if !metadata_rows.is_empty() || !named_ranges.is_empty() {
        let metadata_sheet = create_metadata_sheet(metadata_rows, named_ranges)?;
        xlsx_workbook.push_worksheet(metadata_sheet);
    }

    // Save the workbook
    xlsx_workbook.save(path)?;

    Ok(())
}

/// Create a conversions sheet with all unit conversion factors used in formulas
fn create_conversions_sheet(conversions: &BTreeMap<String, ConversionEntry>) -> Result<Worksheet, ExcelError> {
    let mut sheet = Worksheet::new();
    sheet.set_name("Conversions")?;

    let header_format = Format::new()
        .set_bold()
        .set_background_color(rust_xlsxwriter::Color::RGB(0x4CAF50))
        .set_font_color(rust_xlsxwriter::Color::White);

    let number_format = Format::new().set_num_format("0.00000000");

    // Headers
    sheet.write_string_with_format(0, 0, "Name", &header_format)?;
    sheet.write_string_with_format(0, 1, "Multiplier", &header_format)?;
    sheet.write_string_with_format(0, 2, "Offset", &header_format)?;
    sheet.write_string_with_format(0, 3, "From Unit", &header_format)?;
    sheet.write_string_with_format(0, 4, "To Unit", &header_format)?;
    sheet.write_string_with_format(0, 5, "Description", &header_format)?;

    // Data rows (BTreeMap is already sorted by key)
    for (idx, entry) in conversions.values().enumerate() {
        let row = (idx + 1) as u32;
        sheet.write_string(row, 0, &entry.name)?;
        sheet.write_number_with_format(row, 1, entry.multiplier, &number_format)?;
        sheet.write_number_with_format(row, 2, entry.offset, &number_format)?;
        sheet.write_string(row, 3, &entry.from_unit)?;
        sheet.write_string(row, 4, &entry.to_unit)?;
        sheet.write_string(row, 5, &entry.description)?;
    }

    // Set column widths
    sheet.set_column_width(0, 20)?; // Name
    sheet.set_column_width(1, 15)?; // Multiplier
    sheet.set_column_width(2, 15)?; // Offset
    sheet.set_column_width(3, 12)?; // From Unit
    sheet.set_column_width(4, 12)?; // To Unit
    sheet.set_column_width(5, 50)?; // Description

    Ok(sheet)
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

    sheet.write_string_with_format(13, 0, "3. See the \"Conversions\" sheet for unit conversion factors", &bold_format)?;
    sheet.write_string(14, 0, "   - Lists all conversion factors used in formulas (multiplier and offset)")?;
    sheet.write_string(15, 0, "   - Simple conversions: value × multiplier (e.g., meters to feet)")?;
    sheet.write_string(16, 0, "   - Temperature conversions: value × multiplier + offset (e.g., Celsius to Fahrenheit)")?;
    sheet.write_string(17, 0, "   - Formulas reference these cells using named ranges (e.g., =A1*C_F_m+C_F_o)")?;
    sheet.write_string(18, 0, "   - You can edit conversion factors and formulas will update")?;

    sheet.write_string_with_format(20, 0, "4. Named ranges are exported to Excel", &bold_format)?;
    sheet.write_string(21, 0, "   - User-defined named ranges (like 'revenue', 'tax_rate') are preserved")?;
    sheet.write_string(22, 0, "   - You can use these names in Excel formulas (e.g., =revenue * tax_rate)")?;
    sheet.write_string(23, 0, "   - See \"Unit Metadata\" sheet for the complete list")?;

    sheet.write_string_with_format(25, 0, "5. See the \"Unit Metadata\" sheet for original unit and name information", &bold_format)?;
    sheet.write_string(26, 0, "   - Lists which cells had units and what those units were")?;
    sheet.write_string(27, 0, "   - Shows original formulas for formula cells")?;
    sheet.write_string(28, 0, "   - Documents all named ranges and their cell references")?;

    sheet.write_string(30, 0, "⚠️ WARNING: Changes made in Excel cannot be imported back to Unicel")?;
    sheet.write_string(31, 0, "This is a ONE-WAY export for sharing data only.")?;

    sheet.write_string_with_format(33, 0, "To preserve full unit information, use Unicel's native .usheet format.", &bold_format)?;

    // Set column width
    sheet.set_column_width(0, 80)?;

    Ok(sheet)
}

/// Create a metadata sheet documenting which cells have units and named ranges
fn create_metadata_sheet(
    rows: Vec<(String, String, String, String)>,
    named_ranges: Vec<(String, usize, crate::core::table::CellAddr)>,
) -> Result<Worksheet, ExcelError> {
    let mut sheet = Worksheet::new();
    sheet.set_name("Unit Metadata")?;

    let header_format = Format::new().set_bold().set_background_color(rust_xlsxwriter::Color::RGB(0xD3D3D3));

    let section_header_format = Format::new()
        .set_bold()
        .set_font_size(14)
        .set_background_color(rust_xlsxwriter::Color::RGB(0x4CAF50))
        .set_font_color(rust_xlsxwriter::Color::White);

    let mut current_row = 0u32;

    // Section 1: Unit Information
    if !rows.is_empty() {
        sheet.write_string_with_format(current_row, 0, "Unit Information", &section_header_format)?;
        current_row += 1;

        // Headers
        sheet.write_string_with_format(current_row, 0, "Sheet", &header_format)?;
        sheet.write_string_with_format(current_row, 1, "Cell", &header_format)?;
        sheet.write_string_with_format(current_row, 2, "Type", &header_format)?;
        sheet.write_string_with_format(current_row, 3, "Unit", &header_format)?;
        current_row += 1;

        // Data rows
        for (sheet_name, cell_ref, cell_type, unit) in rows.iter() {
            sheet.write_string(current_row, 0, sheet_name)?;
            sheet.write_string(current_row, 1, cell_ref)?;
            sheet.write_string(current_row, 2, cell_type)?;
            sheet.write_string(current_row, 3, unit)?;
            current_row += 1;
        }

        current_row += 1; // Blank row
    }

    // Section 2: Named Ranges
    if !named_ranges.is_empty() {
        sheet.write_string_with_format(current_row, 0, "Named Ranges", &section_header_format)?;
        current_row += 1;

        // Headers
        sheet.write_string_with_format(current_row, 0, "Name", &header_format)?;
        sheet.write_string_with_format(current_row, 1, "Refers To", &header_format)?;
        sheet.write_string_with_format(current_row, 2, "Excel Formula", &header_format)?;
        current_row += 1;

        // Data rows
        for (name, _sheet_index, addr) in named_ranges.iter() {
            let unicel_ref = format!("{}", addr);
            let col_num = column_letter_to_number(&addr.col) * 2;
            let excel_col = number_to_column_letter(col_num);
            let excel_ref = format!("{}{}", excel_col, addr.row);

            sheet.write_string(current_row, 0, name)?;
            sheet.write_string(current_row, 1, &unicel_ref)?;
            sheet.write_string(current_row, 2, &excel_ref)?;
            current_row += 1;
        }
    }

    // Set column widths
    sheet.set_column_width(0, 20)?;
    sheet.set_column_width(1, 15)?;
    sheet.set_column_width(2, 30)?;
    sheet.set_column_width(3, 20)?;

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

    #[test]
    fn test_number_to_column_letter() {
        assert_eq!(number_to_column_letter(0), "A");
        assert_eq!(number_to_column_letter(1), "B");
        assert_eq!(number_to_column_letter(25), "Z");
        assert_eq!(number_to_column_letter(26), "AA");
        assert_eq!(number_to_column_letter(27), "AB");
        assert_eq!(number_to_column_letter(51), "AZ");
        assert_eq!(number_to_column_letter(52), "BA");
    }

    #[test]
    fn test_transform_formula_for_excel() {
        // Simple cell references
        assert_eq!(transform_formula_for_excel("=A1"), "=A1"); // A*2 = 0, stays A
        assert_eq!(transform_formula_for_excel("=B1"), "=C1"); // B*2 = 2, becomes C
        assert_eq!(transform_formula_for_excel("=C1"), "=E1"); // C*2 = 4, becomes E

        // Formulas with operations
        assert_eq!(transform_formula_for_excel("=B11/B12"), "=C11/C12");
        assert_eq!(transform_formula_for_excel("=A1+B2"), "=A1+C2");
        assert_eq!(transform_formula_for_excel("=2*(B20+B21)"), "=2*(C20+C21)");

        // Mixed case
        assert_eq!(transform_formula_for_excel("=b1"), "=C1");
        assert_eq!(transform_formula_for_excel("=B1+b2"), "=C1+C2");

        // Functions (should not be transformed)
        assert_eq!(transform_formula_for_excel("=SUM(B1:B10)"), "=SUM(C1:C10)");
        assert_eq!(transform_formula_for_excel("=AVERAGE(A1:A5)"), "=AVERAGE(A1:A5)");
    }

    #[test]
    fn test_parse_cell_ref_rejects_invalid() {
        // Should reject "1m" (number before letter)
        assert!(parse_cell_ref("1m").is_none());
        assert!(parse_cell_ref("1kg").is_none());
        assert!(parse_cell_ref("5ft").is_none());

        // Should accept valid cell references
        assert!(parse_cell_ref("A1").is_some());
        assert!(parse_cell_ref("B12").is_some());
        assert!(parse_cell_ref("AA100").is_some());
    }
}
