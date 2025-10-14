use unicel_lib::{
    core::table::CellAddr,
    formats::json::WorkbookFile,
};

fn main() {
    println!("Testing unit cancellation preservation...\n");

    // Load the tutorial workbook
    let file = WorkbookFile::load_from_file(
        std::path::Path::new("examples/unit_conversion_tutorial.usheet")
    ).expect("Failed to load tutorial workbook");

    let workbook = file.to_workbook().expect("Failed to convert to workbook");

    // Get the Unit Cancellation sheet (sheet 8)
    let sheet_idx = workbook.sheet_count() - 1; // Last sheet
    println!("Checking sheet: {}", workbook.get_sheet(sheet_idx).unwrap().name());

    let sheet = workbook.get_sheet(sheet_idx).expect("Sheet not found");

    // Check B22 (speed)
    if let Some(b22) = sheet.get(&CellAddr::new("B", 22)) {
        println!("B22: {} {}",
            b22.as_number().unwrap_or(0.0),
            b22.storage_unit().canonical()
        );
    }

    // Check B23 (time)
    if let Some(b23) = sheet.get(&CellAddr::new("B", 23)) {
        println!("B23: {} {}",
            b23.as_number().unwrap_or(0.0),
            b23.storage_unit().canonical()
        );
    }

    // Check B24 (should be mi after cancellation)
    println!("\nB24 (=B22 * B23):");

    // Evaluate the formula directly
    match sheet.evaluate_formula("=B22 * B23") {
        Ok((value, unit)) => {
            println!("  Evaluated Value: {}", value);
            println!("  Evaluated Unit: {}", unit.canonical());

            if unit.canonical() == "mi" {
                println!("  ✅ SUCCESS! Unit is correctly preserved as 'mi'");
            } else {
                println!("  ❌ FAILED! Expected 'mi' but got '{}'", unit.canonical());
            }
        }
        Err(e) => {
            println!("  ❌ Evaluation failed: {:?}", e);
        }
    }

    // Also check the stored cell
    if let Some(b24) = sheet.get(&CellAddr::new("B", 24)) {
        println!("\n  Stored cell info:");
        println!("    Formula: {:?}", b24.formula());
        println!("    Stored Value: {:?}", b24.value());
        println!("    Storage Unit: {}", b24.storage_unit().canonical());

        if let Some(warning) = b24.warning() {
            println!("    ⚠️ Warning: {}", warning);
        }
    }
}
