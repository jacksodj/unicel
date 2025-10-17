use unicel_lib::commands::workbook::{parse_cell_input, set_cell_impl, AppState, CellValueData};
use unicel_lib::core::table::CellAddr;

#[test]
fn test_percentage_multiplication_bug() {
    // This test reproduces the percentage bug:
    // B2 = 250000 $/yr
    // C7 = 10% (stored as 0.1 with unit "%")
    // Formula: =B2*C7
    // Expected: 25000 $/yr (percentage removed)
    // Bug: 25000 $·%/yr (percentage NOT removed)

    let state = AppState::default();

    // Create a workbook
    unicel_lib::commands::workbook::create_workbook_impl(&state, "Test".to_string()).unwrap();

    // Set B2 = 250000 $/yr
    let b2_cell = parse_cell_input("250000 $/yr").unwrap();
    println!(
        "B2 cell: value={:?}, unit canonical={:?}",
        b2_cell.value(),
        b2_cell.storage_unit().canonical()
    );
    {
        let mut workbook_guard = state.workbook.lock().unwrap();
        let workbook = workbook_guard.as_mut().unwrap();
        let b2_addr = CellAddr::from_string("B2").unwrap();
        workbook.active_sheet_mut().set(b2_addr, b2_cell).unwrap();
    }

    // Set C7 = 10%
    let c7_cell = parse_cell_input("10%").unwrap();
    println!(
        "C7 cell: value={:?}, unit canonical={:?}, unit original={:?}, is_dimensionless={:?}",
        c7_cell.value(),
        c7_cell.storage_unit().canonical(),
        c7_cell.storage_unit().original(),
        c7_cell.storage_unit().is_dimensionless()
    );

    // Test is_percentage_unit manually
    fn is_percentage_unit(unit: &unicel_lib::core::units::Unit) -> bool {
        unit.canonical() == "%"
    }
    println!(
        "Manual is_percentage_unit check: {}",
        is_percentage_unit(c7_cell.storage_unit())
    );

    {
        let mut workbook_guard = state.workbook.lock().unwrap();
        let workbook = workbook_guard.as_mut().unwrap();
        let c7_addr = CellAddr::from_string("C7").unwrap();
        workbook.active_sheet_mut().set(c7_addr, c7_cell).unwrap();
    }

    // Set B8 = =B2*C7
    println!("\nSetting B8 formula =B2*C7...");
    let b8_result = set_cell_impl(&state, "B8".to_string(), "=B2*C7".to_string()).unwrap();

    // Check the result
    println!(
        "B8 result: storage_unit={:?}, value={:?}",
        b8_result.storage_unit, b8_result.value
    );
    println!("B8 formula: {:?}", b8_result.formula);

    // The storage_unit should be "$/yr" (percentage removed), NOT "$·%/yr"
    assert_eq!(
        b8_result.storage_unit, "$/yr",
        "Expected unit to be '$/yr' but got '{}'",
        b8_result.storage_unit
    );

    // The value should be 25000
    if let CellValueData::Number { value } = b8_result.value {
        assert_eq!(
            value, 25000.0,
            "Expected value to be 25000 but got {}",
            value
        );
    } else {
        panic!("Expected number value, got {:?}", b8_result.value);
    }
}
