use unicel_lib::{
    core::{
        cell::Cell,
        table::CellAddr,
        units::{BaseDimension, Unit, UnitLibrary},
    },
    formats::json::WorkbookFile,
};

fn main() {
    println!("Testing multi-hop unit conversions...\n");

    // Load the tutorial workbook
    let file = WorkbookFile::load_from_file(
        std::path::Path::new("examples/unit_conversion_tutorial.usheet")
    ).expect("Failed to load tutorial workbook");

    let workbook = file.to_workbook().expect("Failed to convert to workbook");

    // Test Length Conversions sheet (sheet 1)
    println!("=== Length Conversions ===");
    let sheet = workbook.get_sheet(1).expect("Sheet 1 not found");

    // B5 should be 5 km
    if let Some(b5) = sheet.get(&CellAddr::new("B", 5)) {
        println!("B5: {} {}", b5.as_number().unwrap_or(0.0), b5.storage_unit().canonical());
    }

    // B7 should be km → cm conversion (multi-hop: km → m → cm)
    if let Some(b7) = sheet.get(&CellAddr::new("B", 7)) {
        println!("B7 (km→cm via CONVERT):");
        println!("  Formula: {:?}", b7.formula());
        println!("  Value: {:?}", b7.value());
        println!("  Unit: {}", b7.storage_unit().canonical());
        if b7.warning().is_some() {
            println!("  ⚠️ Warning: {}", b7.warning().unwrap());
        } else {
            println!("  ✓ Success!");
        }
    }

    // B11 should be mi → yd conversion (multi-hop: mi → ft → yd)
    if let Some(b11) = sheet.get(&CellAddr::new("B", 11)) {
        println!("\nB11 (mi→yd via CONVERT):");
        println!("  Formula: {:?}", b11.formula());
        println!("  Value: {:?}", b11.value());
        println!("  Unit: {}", b11.storage_unit().canonical());
        if b11.warning().is_some() {
            println!("  ⚠️ Warning: {}", b11.warning().unwrap());
        } else {
            println!("  ✓ Success!");
        }
    }

    // Direct test with UnitLibrary
    println!("\n=== Direct UnitLibrary Tests ===");
    let lib = UnitLibrary::default();

    // Test km → cm (should use path: km → m → cm)
    if let Some(result) = lib.convert(5.0, "km", "cm") {
        println!("5 km → cm: {} cm ✓", result);
    } else {
        println!("5 km → cm: FAILED ✗");
    }

    // Test mi → yd (should use path: mi → ft → yd)
    if let Some(result) = lib.convert(1.0, "mi", "yd") {
        println!("1 mi → yd: {} yd ✓", result);
    } else {
        println!("1 mi → yd: FAILED ✗");
    }

    println!("\n✅ Multi-hop conversion testing complete!");
}
