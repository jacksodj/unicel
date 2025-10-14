// Unit Conversion Tutorial Workbook
// Demonstrates all unit conversion features of Unicel

use unicel_lib::core::{
    cell::Cell,
    table::{CellAddr, Sheet},
    units::{BaseDimension, Unit, UnitLibrary},
    workbook::Workbook,
};
use unicel_lib::formats::json::WorkbookFile;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut workbook = Workbook::new("Unit Conversion Tutorial");
    let unit_library = UnitLibrary::new();

    // Sheet 1: Introduction to Units
    create_intro_sheet(&mut workbook)?;

    // Sheet 2: Length Conversions
    workbook.add_sheet_with_name("Length Conversions");
    workbook.set_active_sheet(1);
    create_length_sheet(&mut workbook, &unit_library)?;

    // Sheet 3: Mass/Weight Conversions
    workbook.add_sheet_with_name("Mass & Weight");
    workbook.set_active_sheet(2);
    create_mass_sheet(&mut workbook, &unit_library)?;

    // Sheet 4: Temperature Conversions
    workbook.add_sheet_with_name("Temperature");
    workbook.set_active_sheet(3);
    create_temperature_sheet(&mut workbook, &unit_library)?;

    // Sheet 5: Time Conversions
    workbook.add_sheet_with_name("Time");
    workbook.set_active_sheet(4);
    create_time_sheet(&mut workbook, &unit_library)?;

    // Sheet 6: Digital Storage
    workbook.add_sheet_with_name("Digital Storage");
    workbook.set_active_sheet(5);
    create_storage_sheet(&mut workbook, &unit_library)?;

    // Sheet 7: Currency Conversions
    workbook.add_sheet_with_name("Currency");
    workbook.set_active_sheet(6);
    create_currency_sheet(&mut workbook, &unit_library)?;

    // Sheet 8: Compound Units
    workbook.add_sheet_with_name("Compound Units");
    workbook.set_active_sheet(7);
    create_compound_sheet(&mut workbook, &unit_library)?;

    // Sheet 9: Unit Cancellation
    workbook.add_sheet_with_name("Unit Cancellation");
    workbook.set_active_sheet(8);
    create_cancellation_sheet(&mut workbook, &unit_library)?;

    // Reset to first sheet
    workbook.set_active_sheet(0);

    // Save workbook
    let file = WorkbookFile::from_workbook(&workbook);
    let path = PathBuf::from("examples/unit_conversion_tutorial.usheet");
    file.save_to_file(&path)?;

    println!("✓ Created tutorial workbook: {}", path.display());
    println!("  - 9 sheets covering all unit conversion features");
    println!("  - Interactive examples with formulas");
    println!("  - Comprehensive coverage of supported units");

    Ok(())
}

fn create_intro_sheet(workbook: &mut Workbook) -> Result<(), Box<dyn std::error::Error>> {
    let sheet = workbook.active_sheet_mut();
    sheet.set_name("Introduction");

    // Title
    sheet.set(CellAddr::new("A", 1), Cell::with_text("Welcome to Unicel: Unit-Aware Spreadsheets"))?;
    sheet.set(CellAddr::new("A", 2), Cell::with_text("=" .repeat(60)))?;

    // Introduction
    sheet.set(CellAddr::new("A", 4), Cell::with_text("What makes Unicel different?"))?;
    sheet.set(CellAddr::new("A", 5), Cell::with_text("• Units are first-class data types, not just formatting"))?;
    sheet.set(CellAddr::new("A", 6), Cell::with_text("• Automatic dimensional analysis prevents unit errors"))?;
    sheet.set(CellAddr::new("A", 7), Cell::with_text("• Natural unit cancellation (e.g., m/m = dimensionless)"))?;
    sheet.set(CellAddr::new("A", 8), Cell::with_text("• Compound units work naturally (USD/hr × hr = USD)"))?;

    // Example section
    sheet.set(CellAddr::new("A", 10), Cell::with_text("Quick Example:"))?;
    sheet.set(CellAddr::new("A", 11), Cell::with_text("Distance"))?;
    sheet.set(CellAddr::new("B", 11), Cell::new(100.0, Unit::simple("mi", BaseDimension::Length)))?;

    sheet.set(CellAddr::new("A", 12), Cell::with_text("Time"))?;
    sheet.set(CellAddr::new("B", 12), Cell::new(2.0, Unit::simple("hr", BaseDimension::Time)))?;

    sheet.set(CellAddr::new("A", 13), Cell::with_text("Speed"))?;
    sheet.set(CellAddr::new("B", 13), Cell::with_formula("=B11 / B12"))?;
    sheet.set(CellAddr::new("C", 13), Cell::with_text("← Automatically creates mi/hr"))?;

    // Supported units
    sheet.set(CellAddr::new("A", 15), Cell::with_text("Supported Unit Categories:"))?;
    sheet.set(CellAddr::new("A", 16), Cell::with_text("• Length: m, cm, mm, km, in, ft, yd, mi"))?;
    sheet.set(CellAddr::new("A", 17), Cell::with_text("• Mass: g, kg, mg, oz, lb"))?;
    sheet.set(CellAddr::new("A", 18), Cell::with_text("• Time: s, min, hr, day, month, year"))?;
    sheet.set(CellAddr::new("A", 19), Cell::with_text("• Temperature: C, F, K (with offset conversion)"))?;
    sheet.set(CellAddr::new("A", 20), Cell::with_text("• Currency: USD, EUR, GBP"))?;
    sheet.set(CellAddr::new("A", 21), Cell::with_text("• Digital Storage: B, KB, MB, GB, TB, PB"))?;

    // Navigation
    sheet.set(CellAddr::new("A", 23), Cell::with_text("Tutorial Sheets:"))?;
    sheet.set(CellAddr::new("A", 24), Cell::with_text("1. Introduction (this sheet)"))?;
    sheet.set(CellAddr::new("A", 25), Cell::with_text("2. Length Conversions - Metric ↔ Imperial"))?;
    sheet.set(CellAddr::new("A", 26), Cell::with_text("3. Mass & Weight - grams, pounds, ounces"))?;
    sheet.set(CellAddr::new("A", 27), Cell::with_text("4. Temperature - Celsius, Fahrenheit, Kelvin"))?;
    sheet.set(CellAddr::new("A", 28), Cell::with_text("5. Time - seconds to years"))?;
    sheet.set(CellAddr::new("A", 29), Cell::with_text("6. Digital Storage - bytes to petabytes"))?;
    sheet.set(CellAddr::new("A", 30), Cell::with_text("7. Currency - multi-currency calculations"))?;
    sheet.set(CellAddr::new("A", 31), Cell::with_text("8. Compound Units - speed, density, rates"))?;
    sheet.set(CellAddr::new("A", 32), Cell::with_text("9. Unit Cancellation - dimensional analysis"))?;

    Ok(())
}

fn create_length_sheet(workbook: &mut Workbook, _lib: &UnitLibrary) -> Result<(), Box<dyn std::error::Error>> {
    let sheet = workbook.active_sheet_mut();

    // Header
    sheet.set(CellAddr::new("A", 1), Cell::with_text("Length Conversions"))?;
    sheet.set(CellAddr::new("A", 2), Cell::with_text("=" .repeat(40)))?;

    // Metric system
    sheet.set(CellAddr::new("A", 4), Cell::with_text("Metric System"))?;
    sheet.set(CellAddr::new("A", 5), Cell::with_text("1 kilometer"))?;
    sheet.set(CellAddr::new("B", 5), Cell::new(1.0, Unit::simple("km", BaseDimension::Length)))?;

    sheet.set(CellAddr::new("A", 6), Cell::with_text("= meters"))?;
    sheet.set(CellAddr::new("B", 6), Cell::with_formula("=B5"))?; // Display as m
    sheet.set(CellAddr::new("C", 6), Cell::with_text("(1 km = 1000 m)"))?;

    sheet.set(CellAddr::new("A", 7), Cell::with_text("= centimeters"))?;
    sheet.set(CellAddr::new("B", 7), Cell::with_formula("=B5"))?; // Display as cm
    sheet.set(CellAddr::new("C", 7), Cell::with_text("(1 km = 100,000 cm)"))?;

    // Imperial system
    sheet.set(CellAddr::new("A", 9), Cell::with_text("Imperial System"))?;
    sheet.set(CellAddr::new("A", 10), Cell::with_text("1 mile"))?;
    sheet.set(CellAddr::new("B", 10), Cell::new(1.0, Unit::simple("mi", BaseDimension::Length)))?;

    sheet.set(CellAddr::new("A", 11), Cell::with_text("= yards"))?;
    sheet.set(CellAddr::new("B", 11), Cell::with_formula("=B10"))?; // Display as yd
    sheet.set(CellAddr::new("C", 11), Cell::with_text("(1 mi = 1760 yd)"))?;

    sheet.set(CellAddr::new("A", 12), Cell::with_text("= feet"))?;
    sheet.set(CellAddr::new("B", 12), Cell::with_formula("=B10"))?; // Display as ft
    sheet.set(CellAddr::new("C", 12), Cell::with_text("(1 mi = 5280 ft)"))?;

    // Cross-conversion
    sheet.set(CellAddr::new("A", 14), Cell::with_text("Metric ↔ Imperial"))?;
    sheet.set(CellAddr::new("A", 15), Cell::with_text("100 meters"))?;
    sheet.set(CellAddr::new("B", 15), Cell::new(100.0, Unit::simple("m", BaseDimension::Length)))?;

    sheet.set(CellAddr::new("A", 16), Cell::with_text("= feet"))?;
    sheet.set(CellAddr::new("B", 16), Cell::with_formula("=B15"))?; // Display as ft
    sheet.set(CellAddr::new("C", 16), Cell::with_text("(≈ 328.08 ft)"))?;

    sheet.set(CellAddr::new("A", 17), Cell::with_text("= inches"))?;
    sheet.set(CellAddr::new("B", 17), Cell::with_formula("=B15"))?; // Display as in
    sheet.set(CellAddr::new("C", 17), Cell::with_text("(≈ 3937 in)"))?;

    // Calculations
    sheet.set(CellAddr::new("A", 19), Cell::with_text("Calculations with Units"))?;
    sheet.set(CellAddr::new("A", 20), Cell::with_text("Room length"))?;
    sheet.set(CellAddr::new("B", 20), Cell::new(15.0, Unit::simple("ft", BaseDimension::Length)))?;

    sheet.set(CellAddr::new("A", 21), Cell::with_text("Room width"))?;
    sheet.set(CellAddr::new("B", 21), Cell::new(12.0, Unit::simple("ft", BaseDimension::Length)))?;

    sheet.set(CellAddr::new("A", 22), Cell::with_text("Perimeter"))?;
    sheet.set(CellAddr::new("B", 22), Cell::with_formula("=2 * (B20 + B21)"))?;
    sheet.set(CellAddr::new("C", 22), Cell::with_text("← Maintains feet unit"))?;

    Ok(())
}

fn create_mass_sheet(workbook: &mut Workbook, _lib: &UnitLibrary) -> Result<(), Box<dyn std::error::Error>> {
    let sheet = workbook.active_sheet_mut();

    // Header
    sheet.set(CellAddr::new("A", 1), Cell::with_text("Mass & Weight Conversions"))?;
    sheet.set(CellAddr::new("A", 2), Cell::with_text("=" .repeat(40)))?;

    // Metric
    sheet.set(CellAddr::new("A", 4), Cell::with_text("Metric (Mass)"))?;
    sheet.set(CellAddr::new("A", 5), Cell::with_text("1 kilogram"))?;
    sheet.set(CellAddr::new("B", 5), Cell::new(1.0, Unit::simple("kg", BaseDimension::Mass)))?;

    sheet.set(CellAddr::new("A", 6), Cell::with_text("= grams"))?;
    sheet.set(CellAddr::new("B", 6), Cell::with_formula("=B5"))?;
    sheet.set(CellAddr::new("C", 6), Cell::with_text("(1 kg = 1000 g)"))?;

    sheet.set(CellAddr::new("A", 7), Cell::with_text("= milligrams"))?;
    sheet.set(CellAddr::new("B", 7), Cell::with_formula("=B5"))?;
    sheet.set(CellAddr::new("C", 7), Cell::with_text("(1 kg = 1,000,000 mg)"))?;

    // Imperial
    sheet.set(CellAddr::new("A", 9), Cell::with_text("Imperial (Weight)"))?;
    sheet.set(CellAddr::new("A", 10), Cell::with_text("1 pound"))?;
    sheet.set(CellAddr::new("B", 10), Cell::new(1.0, Unit::simple("lb", BaseDimension::Mass)))?;

    sheet.set(CellAddr::new("A", 11), Cell::with_text("= ounces"))?;
    sheet.set(CellAddr::new("B", 11), Cell::with_formula("=B10"))?;
    sheet.set(CellAddr::new("C", 11), Cell::with_text("(1 lb = 16 oz)"))?;

    // Cross-conversion
    sheet.set(CellAddr::new("A", 13), Cell::with_text("Metric ↔ Imperial"))?;
    sheet.set(CellAddr::new("A", 14), Cell::with_text("1 kilogram"))?;
    sheet.set(CellAddr::new("B", 14), Cell::new(1.0, Unit::simple("kg", BaseDimension::Mass)))?;

    sheet.set(CellAddr::new("A", 15), Cell::with_text("= pounds"))?;
    sheet.set(CellAddr::new("B", 15), Cell::with_formula("=B14"))?;
    sheet.set(CellAddr::new("C", 15), Cell::with_text("(1 kg ≈ 2.205 lb)"))?;

    // Practical example
    sheet.set(CellAddr::new("A", 17), Cell::with_text("Practical Example: Package Shipping"))?;
    sheet.set(CellAddr::new("A", 18), Cell::with_text("Item weight"))?;
    sheet.set(CellAddr::new("B", 18), Cell::new(2.5, Unit::simple("kg", BaseDimension::Mass)))?;

    sheet.set(CellAddr::new("A", 19), Cell::with_text("Shipping limit"))?;
    sheet.set(CellAddr::new("B", 19), Cell::new(5.0, Unit::simple("lb", BaseDimension::Mass)))?;

    sheet.set(CellAddr::new("A", 20), Cell::with_text("Over limit?"))?;
    sheet.set(CellAddr::new("B", 20), Cell::with_text("Compare converted values"))?;

    Ok(())
}

fn create_temperature_sheet(workbook: &mut Workbook, _lib: &UnitLibrary) -> Result<(), Box<dyn std::error::Error>> {
    let sheet = workbook.active_sheet_mut();

    // Header
    sheet.set(CellAddr::new("A", 1), Cell::with_text("Temperature Conversions"))?;
    sheet.set(CellAddr::new("A", 2), Cell::with_text("=" .repeat(40)))?;
    sheet.set(CellAddr::new("A", 3), Cell::with_text("Note: Temperature uses offset conversion (not just scaling)"))?;

    // Celsius examples
    sheet.set(CellAddr::new("A", 5), Cell::with_text("Water Freezing Point"))?;
    sheet.set(CellAddr::new("B", 5), Cell::new(0.0, Unit::simple("C", BaseDimension::Temperature)))?;
    sheet.set(CellAddr::new("C", 5), Cell::with_text("(Celsius)"))?;

    sheet.set(CellAddr::new("A", 6), Cell::with_text("= Fahrenheit"))?;
    sheet.set(CellAddr::new("B", 6), Cell::with_formula("=B5"))?;
    sheet.set(CellAddr::new("C", 6), Cell::with_text("(= 32°F)"))?;

    sheet.set(CellAddr::new("A", 7), Cell::with_text("= Kelvin"))?;
    sheet.set(CellAddr::new("B", 7), Cell::with_formula("=B5"))?;
    sheet.set(CellAddr::new("C", 7), Cell::with_text("(= 273.15 K)"))?;

    // Boiling point
    sheet.set(CellAddr::new("A", 9), Cell::with_text("Water Boiling Point"))?;
    sheet.set(CellAddr::new("B", 9), Cell::new(100.0, Unit::simple("C", BaseDimension::Temperature)))?;

    sheet.set(CellAddr::new("A", 10), Cell::with_text("= Fahrenheit"))?;
    sheet.set(CellAddr::new("B", 10), Cell::with_formula("=B9"))?;
    sheet.set(CellAddr::new("C", 10), Cell::with_text("(= 212°F)"))?;

    // Room temperature
    sheet.set(CellAddr::new("A", 12), Cell::with_text("Room Temperature"))?;
    sheet.set(CellAddr::new("B", 12), Cell::new(68.0, Unit::simple("F", BaseDimension::Temperature)))?;

    sheet.set(CellAddr::new("A", 13), Cell::with_text("= Celsius"))?;
    sheet.set(CellAddr::new("B", 13), Cell::with_formula("=B12"))?;
    sheet.set(CellAddr::new("C", 13), Cell::with_text("(= 20°C)"))?;

    // Absolute zero
    sheet.set(CellAddr::new("A", 15), Cell::with_text("Absolute Zero"))?;
    sheet.set(CellAddr::new("B", 15), Cell::new(0.0, Unit::simple("K", BaseDimension::Temperature)))?;

    sheet.set(CellAddr::new("A", 16), Cell::with_text("= Celsius"))?;
    sheet.set(CellAddr::new("B", 16), Cell::with_formula("=B15"))?;
    sheet.set(CellAddr::new("C", 16), Cell::with_text("(= -273.15°C)"))?;

    sheet.set(CellAddr::new("A", 18), Cell::with_text("Conversion Formulas:"))?;
    sheet.set(CellAddr::new("A", 19), Cell::with_text("°F = (°C × 9/5) + 32"))?;
    sheet.set(CellAddr::new("A", 20), Cell::with_text("K = °C + 273.15"))?;
    sheet.set(CellAddr::new("A", 21), Cell::with_text("°C = (°F - 32) × 5/9"))?;

    Ok(())
}

fn create_time_sheet(workbook: &mut Workbook, _lib: &UnitLibrary) -> Result<(), Box<dyn std::error::Error>> {
    let sheet = workbook.active_sheet_mut();

    // Header
    sheet.set(CellAddr::new("A", 1), Cell::with_text("Time Conversions"))?;
    sheet.set(CellAddr::new("A", 2), Cell::with_text("=" .repeat(40)))?;

    // Basic conversions
    sheet.set(CellAddr::new("A", 4), Cell::with_text("1 hour"))?;
    sheet.set(CellAddr::new("B", 4), Cell::new(1.0, Unit::simple("hr", BaseDimension::Time)))?;

    sheet.set(CellAddr::new("A", 5), Cell::with_text("= minutes"))?;
    sheet.set(CellAddr::new("B", 5), Cell::with_formula("=B4"))?;
    sheet.set(CellAddr::new("C", 5), Cell::with_text("(= 60 min)"))?;

    sheet.set(CellAddr::new("A", 6), Cell::with_text("= seconds"))?;
    sheet.set(CellAddr::new("B", 6), Cell::with_formula("=B4"))?;
    sheet.set(CellAddr::new("C", 6), Cell::with_text("(= 3600 s)"))?;

    // Days
    sheet.set(CellAddr::new("A", 8), Cell::with_text("1 day"))?;
    sheet.set(CellAddr::new("B", 8), Cell::new(1.0, Unit::simple("day", BaseDimension::Time)))?;

    sheet.set(CellAddr::new("A", 9), Cell::with_text("= hours"))?;
    sheet.set(CellAddr::new("B", 9), Cell::with_formula("=B8"))?;
    sheet.set(CellAddr::new("C", 9), Cell::with_text("(= 24 hr)"))?;

    // Larger units
    sheet.set(CellAddr::new("A", 11), Cell::with_text("1 year (approximate)"))?;
    sheet.set(CellAddr::new("B", 11), Cell::new(1.0, Unit::simple("year", BaseDimension::Time)))?;

    sheet.set(CellAddr::new("A", 12), Cell::with_text("= days"))?;
    sheet.set(CellAddr::new("B", 12), Cell::with_formula("=B11"))?;
    sheet.set(CellAddr::new("C", 12), Cell::with_text("(≈ 365.25 days)"))?;

    sheet.set(CellAddr::new("A", 13), Cell::with_text("= hours"))?;
    sheet.set(CellAddr::new("B", 13), Cell::with_formula("=B11"))?;
    sheet.set(CellAddr::new("C", 13), Cell::with_text("(≈ 8766 hr)"))?;

    // Practical calculation
    sheet.set(CellAddr::new("A", 15), Cell::with_text("Project Time Calculation"))?;
    sheet.set(CellAddr::new("A", 16), Cell::with_text("Work hours per day"))?;
    sheet.set(CellAddr::new("B", 16), Cell::new(8.0, Unit::simple("hr", BaseDimension::Time)))?;

    sheet.set(CellAddr::new("A", 17), Cell::with_text("Work days"))?;
    sheet.set(CellAddr::new("B", 17), Cell::new(5.0, Unit::dimensionless()))?;

    sheet.set(CellAddr::new("A", 18), Cell::with_text("Total hours"))?;
    sheet.set(CellAddr::new("B", 18), Cell::with_formula("=B16 * B17"))?;
    sheet.set(CellAddr::new("C", 18), Cell::with_text("← Still in hours"))?;

    Ok(())
}

fn create_storage_sheet(workbook: &mut Workbook, _lib: &UnitLibrary) -> Result<(), Box<dyn std::error::Error>> {
    let sheet = workbook.active_sheet_mut();

    // Header
    sheet.set(CellAddr::new("A", 1), Cell::with_text("Digital Storage Conversions"))?;
    sheet.set(CellAddr::new("A", 2), Cell::with_text("=" .repeat(40)))?;

    // Bytes
    sheet.set(CellAddr::new("A", 4), Cell::with_text("1 Gigabyte"))?;
    sheet.set(CellAddr::new("B", 4), Cell::new(1.0, Unit::simple("GB", BaseDimension::Custom("Data".to_string()))))?;

    sheet.set(CellAddr::new("A", 5), Cell::with_text("= Megabytes"))?;
    sheet.set(CellAddr::new("B", 5), Cell::with_formula("=B4"))?;
    sheet.set(CellAddr::new("C", 5), Cell::with_text("(= 1024 MB)"))?;

    sheet.set(CellAddr::new("A", 6), Cell::with_text("= Kilobytes"))?;
    sheet.set(CellAddr::new("B", 6), Cell::with_formula("=B4"))?;
    sheet.set(CellAddr::new("C", 6), Cell::with_text("(= 1,048,576 KB)"))?;

    sheet.set(CellAddr::new("A", 7), Cell::with_text("= Bytes"))?;
    sheet.set(CellAddr::new("B", 7), Cell::with_formula("=B4"))?;
    sheet.set(CellAddr::new("C", 7), Cell::with_text("(= 1,073,741,824 B)"))?;

    // Large storage
    sheet.set(CellAddr::new("A", 9), Cell::with_text("1 Terabyte"))?;
    sheet.set(CellAddr::new("B", 9), Cell::new(1.0, Unit::simple("TB", BaseDimension::Custom("Data".to_string()))))?;

    sheet.set(CellAddr::new("A", 10), Cell::with_text("= Gigabytes"))?;
    sheet.set(CellAddr::new("B", 10), Cell::with_formula("=B9"))?;
    sheet.set(CellAddr::new("C", 10), Cell::with_text("(= 1024 GB)"))?;

    // Practical example
    sheet.set(CellAddr::new("A", 12), Cell::with_text("Cloud Storage Plan"))?;
    sheet.set(CellAddr::new("A", 13), Cell::with_text("Available storage"))?;
    sheet.set(CellAddr::new("B", 13), Cell::new(2.0, Unit::simple("TB", BaseDimension::Custom("Data".to_string()))))?;

    sheet.set(CellAddr::new("A", 14), Cell::with_text("File size"))?;
    sheet.set(CellAddr::new("B", 14), Cell::new(500.0, Unit::simple("MB", BaseDimension::Custom("Data".to_string()))))?;

    sheet.set(CellAddr::new("A", 15), Cell::with_text("Files that fit"))?;
    sheet.set(CellAddr::new("B", 15), Cell::with_formula("=B13 / B14"))?;
    sheet.set(CellAddr::new("C", 15), Cell::with_text("← Dimensionless (count)"))?;

    // Bits vs Bytes
    sheet.set(CellAddr::new("A", 17), Cell::with_text("Note: Bits vs Bytes"))?;
    sheet.set(CellAddr::new("A", 18), Cell::with_text("1 Byte = 8 bits"))?;
    sheet.set(CellAddr::new("A", 19), Cell::with_text("Network speeds often use bits: Mb, Gb"))?;
    sheet.set(CellAddr::new("A", 20), Cell::with_text("Storage sizes use Bytes: MB, GB"))?;

    Ok(())
}

fn create_currency_sheet(workbook: &mut Workbook, _lib: &UnitLibrary) -> Result<(), Box<dyn std::error::Error>> {
    let sheet = workbook.active_sheet_mut();

    // Header
    sheet.set(CellAddr::new("A", 1), Cell::with_text("Currency Conversions"))?;
    sheet.set(CellAddr::new("A", 2), Cell::with_text("=" .repeat(40)))?;
    sheet.set(CellAddr::new("A", 3), Cell::with_text("Note: Exchange rates are hardcoded for demonstration"))?;

    // Basic conversions
    sheet.set(CellAddr::new("A", 5), Cell::with_text("100 USD"))?;
    sheet.set(CellAddr::new("B", 5), Cell::new(100.0, Unit::simple("USD", BaseDimension::Currency)))?;

    sheet.set(CellAddr::new("A", 6), Cell::with_text("= EUR"))?;
    sheet.set(CellAddr::new("B", 6), Cell::with_formula("=B5"))?;
    sheet.set(CellAddr::new("C", 6), Cell::with_text("(approximate)"))?;

    sheet.set(CellAddr::new("A", 7), Cell::with_text("= GBP"))?;
    sheet.set(CellAddr::new("B", 7), Cell::with_formula("=B5"))?;
    sheet.set(CellAddr::new("C", 7), Cell::with_text("(approximate)"))?;

    // Multi-currency budget
    sheet.set(CellAddr::new("A", 9), Cell::with_text("Multi-Currency Budget"))?;
    sheet.set(CellAddr::new("A", 10), Cell::with_text("US Office"))?;
    sheet.set(CellAddr::new("B", 10), Cell::new(50000.0, Unit::simple("USD", BaseDimension::Currency)))?;

    sheet.set(CellAddr::new("A", 11), Cell::with_text("EU Office"))?;
    sheet.set(CellAddr::new("B", 11), Cell::new(40000.0, Unit::simple("EUR", BaseDimension::Currency)))?;

    sheet.set(CellAddr::new("A", 12), Cell::with_text("UK Office"))?;
    sheet.set(CellAddr::new("B", 12), Cell::new(35000.0, Unit::simple("GBP", BaseDimension::Currency)))?;

    sheet.set(CellAddr::new("A", 13), Cell::with_text("Total (USD)"))?;
    sheet.set(CellAddr::new("B", 13), Cell::with_formula("=B10 + B11 + B12"))?;
    sheet.set(CellAddr::new("C", 13), Cell::with_text("← Auto-converted to USD"))?;

    // Price comparison
    sheet.set(CellAddr::new("A", 15), Cell::with_text("Price Comparison"))?;
    sheet.set(CellAddr::new("A", 16), Cell::with_text("US Price"))?;
    sheet.set(CellAddr::new("B", 16), Cell::new(999.0, Unit::simple("USD", BaseDimension::Currency)))?;

    sheet.set(CellAddr::new("A", 17), Cell::with_text("EU Price"))?;
    sheet.set(CellAddr::new("B", 17), Cell::new(899.0, Unit::simple("EUR", BaseDimension::Currency)))?;

    sheet.set(CellAddr::new("A", 18), Cell::with_text("Difference (USD)"))?;
    sheet.set(CellAddr::new("B", 18), Cell::with_formula("=B16 - B17"))?;

    Ok(())
}

fn create_compound_sheet(workbook: &mut Workbook, _lib: &UnitLibrary) -> Result<(), Box<dyn std::error::Error>> {
    let sheet = workbook.active_sheet_mut();

    // Header
    sheet.set(CellAddr::new("A", 1), Cell::with_text("Compound Units"))?;
    sheet.set(CellAddr::new("A", 2), Cell::with_text("=" .repeat(40)))?;
    sheet.set(CellAddr::new("A", 3), Cell::with_text("Compound units are created automatically through operations"))?;

    // Speed (length/time)
    sheet.set(CellAddr::new("A", 5), Cell::with_text("Speed = Distance / Time"))?;
    sheet.set(CellAddr::new("A", 6), Cell::with_text("Distance"))?;
    sheet.set(CellAddr::new("B", 6), Cell::new(100.0, Unit::simple("mi", BaseDimension::Length)))?;

    sheet.set(CellAddr::new("A", 7), Cell::with_text("Time"))?;
    sheet.set(CellAddr::new("B", 7), Cell::new(2.0, Unit::simple("hr", BaseDimension::Time)))?;

    sheet.set(CellAddr::new("A", 8), Cell::with_text("Speed"))?;
    sheet.set(CellAddr::new("B", 8), Cell::with_formula("=B6 / B7"))?;
    sheet.set(CellAddr::new("C", 8), Cell::with_text("← Creates mi/hr"))?;

    // Hourly rate
    sheet.set(CellAddr::new("A", 10), Cell::with_text("Hourly Rate = Money / Time"))?;
    sheet.set(CellAddr::new("A", 11), Cell::with_text("Rate"))?;
    sheet.set(CellAddr::new("B", 11), Cell::new(50.0, Unit::simple("USD", BaseDimension::Currency)))?;
    sheet.set(CellAddr::new("C", 11), Cell::with_text("per hour"))?;

    sheet.set(CellAddr::new("A", 12), Cell::with_text("Hours worked"))?;
    sheet.set(CellAddr::new("B", 12), Cell::new(40.0, Unit::simple("hr", BaseDimension::Time)))?;

    sheet.set(CellAddr::new("A", 13), Cell::with_text("Total earnings"))?;
    sheet.set(CellAddr::new("B", 13), Cell::with_formula("=B11 * B12"))?;
    sheet.set(CellAddr::new("C", 13), Cell::with_text("← USD/hr × hr = USD"))?;

    // Data transfer rate
    sheet.set(CellAddr::new("A", 15), Cell::with_text("Data Transfer Rate"))?;
    sheet.set(CellAddr::new("A", 16), Cell::with_text("File size"))?;
    sheet.set(CellAddr::new("B", 16), Cell::new(500.0, Unit::simple("MB", BaseDimension::Custom("Data".to_string()))))?;

    sheet.set(CellAddr::new("A", 17), Cell::with_text("Download time"))?;
    sheet.set(CellAddr::new("B", 17), Cell::new(50.0, Unit::simple("s", BaseDimension::Time)))?;

    sheet.set(CellAddr::new("A", 18), Cell::with_text("Transfer rate"))?;
    sheet.set(CellAddr::new("B", 18), Cell::with_formula("=B16 / B17"))?;
    sheet.set(CellAddr::new("C", 18), Cell::with_text("← Creates MB/s"))?;

    // Density (mass/volume would need volume units - show concept)
    sheet.set(CellAddr::new("A", 20), Cell::with_text("Other Compound Units:"))?;
    sheet.set(CellAddr::new("A", 21), Cell::with_text("• Density: kg/m³ (mass per volume)"))?;
    sheet.set(CellAddr::new("A", 22), Cell::with_text("• Acceleration: m/s² (velocity change per time)"))?;
    sheet.set(CellAddr::new("A", 23), Cell::with_text("• Pressure: N/m² or Pa (force per area)"))?;
    sheet.set(CellAddr::new("A", 24), Cell::with_text("• Energy: J or Wh (work or power × time)"))?;

    Ok(())
}

fn create_cancellation_sheet(workbook: &mut Workbook, _lib: &UnitLibrary) -> Result<(), Box<dyn std::error::Error>> {
    let sheet = workbook.active_sheet_mut();

    // Header
    sheet.set(CellAddr::new("A", 1), Cell::with_text("Unit Cancellation & Dimensional Analysis"))?;
    sheet.set(CellAddr::new("A", 2), Cell::with_text("=" .repeat(50)))?;
    sheet.set(CellAddr::new("A", 3), Cell::with_text("Units cancel naturally when dividing same dimensions"))?;

    // Basic cancellation
    sheet.set(CellAddr::new("A", 5), Cell::with_text("Basic Cancellation"))?;
    sheet.set(CellAddr::new("A", 6), Cell::with_text("Distance 1"))?;
    sheet.set(CellAddr::new("B", 6), Cell::new(100.0, Unit::simple("m", BaseDimension::Length)))?;

    sheet.set(CellAddr::new("A", 7), Cell::with_text("Distance 2"))?;
    sheet.set(CellAddr::new("B", 7), Cell::new(50.0, Unit::simple("m", BaseDimension::Length)))?;

    sheet.set(CellAddr::new("A", 8), Cell::with_text("Ratio"))?;
    sheet.set(CellAddr::new("B", 8), Cell::with_formula("=B6 / B7"))?;
    sheet.set(CellAddr::new("C", 8), Cell::with_text("← m/m cancels = dimensionless 2.0"))?;

    // Percentage calculation
    sheet.set(CellAddr::new("A", 10), Cell::with_text("Percentage Calculation"))?;
    sheet.set(CellAddr::new("A", 11), Cell::with_text("Sold"))?;
    sheet.set(CellAddr::new("B", 11), Cell::new(45.0, Unit::dimensionless()))?;
    sheet.set(CellAddr::new("C", 11), Cell::with_text("items"))?;

    sheet.set(CellAddr::new("A", 12), Cell::with_text("Total"))?;
    sheet.set(CellAddr::new("B", 12), Cell::new(100.0, Unit::dimensionless()))?;
    sheet.set(CellAddr::new("C", 12), Cell::with_text("items"))?;

    sheet.set(CellAddr::new("A", 13), Cell::with_text("Percentage"))?;
    sheet.set(CellAddr::new("B", 13), Cell::with_formula("=(B11 / B12) * 100"))?;
    sheet.set(CellAddr::new("C", 13), Cell::with_text("← 45%"))?;

    // Round trip cancellation
    sheet.set(CellAddr::new("A", 15), Cell::with_text("Round-Trip Cancellation"))?;
    sheet.set(CellAddr::new("A", 16), Cell::with_text("Price per item"))?;
    sheet.set(CellAddr::new("B", 16), Cell::new(25.0, Unit::simple("USD", BaseDimension::Currency)))?;

    sheet.set(CellAddr::new("A", 17), Cell::with_text("Quantity"))?;
    sheet.set(CellAddr::new("B", 17), Cell::new(10.0, Unit::dimensionless()))?;
    sheet.set(CellAddr::new("C", 17), Cell::with_text("items"))?;

    sheet.set(CellAddr::new("A", 18), Cell::with_text("Total cost"))?;
    sheet.set(CellAddr::new("B", 18), Cell::with_formula("=B16 * B17"))?;
    sheet.set(CellAddr::new("C", 18), Cell::with_text("← USD × dimensionless = USD"))?;

    sheet.set(CellAddr::new("A", 19), Cell::with_text("Cost per item"))?;
    sheet.set(CellAddr::new("B", 19), Cell::with_formula("=B18 / B17"))?;
    sheet.set(CellAddr::new("C", 19), Cell::with_text("← Back to USD"))?;

    // Compound cancellation
    sheet.set(CellAddr::new("A", 21), Cell::with_text("Compound Unit Cancellation"))?;
    sheet.set(CellAddr::new("A", 22), Cell::with_text("Speed"))?;
    sheet.set(CellAddr::new("B", 22), Cell::new(60.0, Unit::simple("mi", BaseDimension::Length)))?;
    sheet.set(CellAddr::new("C", 22), Cell::with_text("per hour (mi/hr)"))?;

    sheet.set(CellAddr::new("A", 23), Cell::with_text("Time"))?;
    sheet.set(CellAddr::new("B", 23), Cell::new(2.5, Unit::simple("hr", BaseDimension::Time)))?;

    sheet.set(CellAddr::new("A", 24), Cell::with_text("Distance"))?;
    sheet.set(CellAddr::new("B", 24), Cell::with_formula("=B22 * B23"))?;
    sheet.set(CellAddr::new("C", 24), Cell::with_text("← (mi/hr) × hr cancels to mi"))?;

    // Error prevention
    sheet.set(CellAddr::new("A", 26), Cell::with_text("Dimensional Analysis Prevents Errors"))?;
    sheet.set(CellAddr::new("A", 27), Cell::with_text("Adding incompatible units (5m + 10s) produces warnings"))?;
    sheet.set(CellAddr::new("A", 28), Cell::with_text("Ensures calculations are physically meaningful"))?;
    sheet.set(CellAddr::new("A", 29), Cell::with_text("Automatic unit tracking reduces human error"))?;

    Ok(())
}
