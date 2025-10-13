// Spreadsheet sheet with cell management

use crate::core::cell::{Cell, CellValue};
use crate::core::formula::{parse_formula, EvalError, EvalResult, Expr};
use crate::core::units::UnitLibrary;
use std::collections::{HashMap, HashSet};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SheetError {
    #[error("Invalid cell reference: {0}")]
    InvalidCellRef(String),

    #[error("Circular reference detected involving cell {0}")]
    CircularReference(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Evaluation error: {0}")]
    EvalError(#[from] EvalError),
}

/// A cell address (column, row)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CellAddr {
    pub col: String,
    pub row: usize,
}

impl CellAddr {
    pub fn new(col: impl Into<String>, row: usize) -> Self {
        Self {
            col: col.into(),
            row,
        }
    }

    pub fn from_string(s: &str) -> Result<Self, SheetError> {
        let mut col = String::new();
        let mut row = String::new();

        for ch in s.chars() {
            if ch.is_ascii_alphabetic() {
                col.push(ch.to_ascii_uppercase());
            } else if ch.is_ascii_digit() {
                row.push(ch);
            }
        }

        if col.is_empty() || row.is_empty() {
            return Err(SheetError::InvalidCellRef(s.to_string()));
        }

        let row_num = row
            .parse::<usize>()
            .map_err(|_| SheetError::InvalidCellRef(s.to_string()))?;

        Ok(Self { col, row: row_num })
    }

    pub fn to_string(&self) -> String {
        format!("{}{}", self.col, self.row)
    }
}

impl std::fmt::Display for CellAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.col, self.row)
    }
}

/// Dependency graph for tracking cell dependencies
#[derive(Debug, Default)]
pub struct DependencyGraph {
    /// Maps a cell to the cells it depends on
    dependencies: HashMap<CellAddr, HashSet<CellAddr>>,

    /// Maps a cell to the cells that depend on it
    dependents: HashMap<CellAddr, HashSet<CellAddr>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a dependency: `cell` depends on `depends_on`
    pub fn add_dependency(&mut self, cell: CellAddr, depends_on: CellAddr) {
        self.dependencies
            .entry(cell.clone())
            .or_default()
            .insert(depends_on.clone());

        self.dependents
            .entry(depends_on)
            .or_default()
            .insert(cell);
    }

    /// Remove all dependencies for a cell
    pub fn remove_dependencies(&mut self, cell: &CellAddr) {
        // Remove from dependencies map
        if let Some(deps) = self.dependencies.remove(cell) {
            // Remove from dependents of each dependency
            for dep in deps {
                if let Some(dependents) = self.dependents.get_mut(&dep) {
                    dependents.remove(cell);
                }
            }
        }
    }

    /// Get cells that this cell depends on
    pub fn get_dependencies(&self, cell: &CellAddr) -> HashSet<CellAddr> {
        self.dependencies
            .get(cell)
            .cloned()
            .unwrap_or_default()
    }

    /// Get cells that depend on this cell
    pub fn get_dependents(&self, cell: &CellAddr) -> HashSet<CellAddr> {
        self.dependents
            .get(cell)
            .cloned()
            .unwrap_or_default()
    }

    /// Check for circular references starting from a cell
    pub fn has_circular_reference(&self, start: &CellAddr) -> bool {
        let mut visited = HashSet::new();
        let mut stack = vec![start.clone()];

        while let Some(cell) = stack.pop() {
            if !visited.insert(cell.clone()) {
                // We've seen this cell before in this path
                if cell == *start {
                    return true;
                }
                continue;
            }

            // Add dependencies to explore
            if let Some(deps) = self.dependencies.get(&cell) {
                for dep in deps {
                    stack.push(dep.clone());
                }
            }
        }

        false
    }

    /// Get the calculation order (topological sort)
    /// Returns cells in dependency order (cells with no deps first)
    pub fn calculation_order(&self, changed_cells: &[CellAddr]) -> Vec<CellAddr> {
        let mut order = Vec::new();
        let mut visited = HashSet::new();

        // Start from changed cells and traverse dependents
        for cell in changed_cells {
            self.visit_dependents(cell, &mut visited, &mut order);
        }

        order
    }

    fn visit_dependents(
        &self,
        cell: &CellAddr,
        visited: &mut HashSet<CellAddr>,
        order: &mut Vec<CellAddr>,
    ) {
        if visited.contains(cell) {
            return;
        }

        visited.insert(cell.clone());

        // Visit dependents recursively
        if let Some(deps) = self.dependents.get(cell) {
            for dep in deps {
                self.visit_dependents(dep, visited, order);
            }
        }

        order.push(cell.clone());
    }
}

/// A spreadsheet sheet
#[derive(Debug)]
pub struct Sheet {
    /// Sheet name
    name: String,

    /// Cell storage
    cells: HashMap<CellAddr, Cell>,

    /// Dependency graph
    dependencies: DependencyGraph,

    /// Unit library for conversions
    library: UnitLibrary,
}

impl Sheet {
    pub fn new() -> Self {
        Self::with_name("Sheet1")
    }

    pub fn with_name(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            cells: HashMap::new(),
            dependencies: DependencyGraph::new(),
            library: UnitLibrary::new(),
        }
    }

    /// Get the sheet name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the sheet name
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    /// Get a cell
    pub fn get(&self, addr: &CellAddr) -> Option<&Cell> {
        self.cells.get(addr)
    }

    /// Get a mutable reference to a cell
    pub fn get_mut(&mut self, addr: &CellAddr) -> Option<&mut Cell> {
        self.cells.get_mut(addr)
    }

    /// Get cells in a range (single column only for MLP)
    pub fn get_range(&self, start: &CellAddr, end: &CellAddr) -> Vec<(CellAddr, &Cell)> {
        let mut result = Vec::new();

        // For MLP, only support single-column ranges
        if start.col != end.col {
            return result;
        }

        for row in start.row..=end.row {
            let addr = CellAddr::new(&start.col, row);
            if let Some(cell) = self.get(&addr) {
                result.push((addr, cell));
            }
        }

        result
    }

    /// Remove a cell
    pub fn remove(&mut self, addr: &CellAddr) -> Option<Cell> {
        self.dependencies.remove_dependencies(addr);
        self.cells.remove(addr)
    }

    /// Clear all cells
    pub fn clear(&mut self) {
        self.cells.clear();
        self.dependencies = DependencyGraph::new();
    }

    /// Get all non-empty cell addresses
    pub fn cell_addresses(&self) -> Vec<CellAddr> {
        self.cells.keys().cloned().collect()
    }

    /// Get the count of non-empty cells
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    /// Set a cell with a direct value
    pub fn set(&mut self, addr: CellAddr, cell: Cell) -> Result<(), SheetError> {
        // If the cell has a formula, extract dependencies
        if let Some(formula) = cell.formula() {
            self.update_dependencies(&addr, formula)?;
        } else {
            // Clear dependencies if no formula
            self.dependencies.remove_dependencies(&addr);
        }

        self.cells.insert(addr, cell);
        Ok(())
    }

    /// Update dependencies for a formula
    fn update_dependencies(&mut self, addr: &CellAddr, formula: &str) -> Result<(), SheetError> {
        // Clear existing dependencies
        self.dependencies.remove_dependencies(addr);

        // Parse formula and extract cell references
        let expr = parse_formula(formula)
            .map_err(|e| SheetError::ParseError(e.to_string()))?;

        let deps = extract_cell_refs(&expr);

        // Add new dependencies
        for dep in deps {
            self.dependencies.add_dependency(addr.clone(), dep);
        }

        // Check for circular references
        if self.dependencies.has_circular_reference(addr) {
            return Err(SheetError::CircularReference(addr.to_string()));
        }

        Ok(())
    }

    /// Evaluate a formula in the context of this sheet
    pub fn evaluate_formula(&self, formula: &str) -> Result<(f64, crate::core::units::Unit), SheetError> {
        let expr = parse_formula(formula)
            .map_err(|e| SheetError::ParseError(e.to_string()))?;

        let evaluator = SheetEvaluator {
            sheet: self,
            library: &self.library,
        };

        let result = evaluator.eval(&expr)?;
        Ok((result.value, result.unit))
    }

    /// Recalculate cells that depend on changed cells
    pub fn recalculate(&mut self, changed: &[CellAddr]) -> Result<(), SheetError> {
        let order = self.dependencies.calculation_order(changed);

        for addr in order {
            if let Some(cell) = self.cells.get(&addr).cloned() {
                if let Some(formula) = cell.formula() {
                    match self.evaluate_formula(formula) {
                        Ok((value, unit)) => {
                            let mut updated_cell = cell;
                            updated_cell.set_value(CellValue::Number(value));
                            updated_cell.set_storage_unit(unit);
                            self.cells.insert(addr, updated_cell);
                        }
                        Err(e) => {
                            let mut updated_cell = cell;
                            updated_cell.set_value(CellValue::Error(e.to_string()));
                            self.cells.insert(addr, updated_cell);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

impl Default for Sheet {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract cell references from an expression
fn extract_cell_refs(expr: &Expr) -> HashSet<CellAddr> {
    let mut refs = HashSet::new();
    extract_cell_refs_recursive(expr, &mut refs);
    refs
}

fn extract_cell_refs_recursive(expr: &Expr, refs: &mut HashSet<CellAddr>) {
    match expr {
        Expr::CellRef { col, row } => {
            refs.insert(CellAddr::new(col.clone(), *row));
        }
        Expr::Range { start, end } => {
            extract_cell_refs_recursive(start, refs);
            extract_cell_refs_recursive(end, refs);
        }
        Expr::Add(l, r) | Expr::Subtract(l, r) | Expr::Multiply(l, r) | Expr::Divide(l, r) => {
            extract_cell_refs_recursive(l, refs);
            extract_cell_refs_recursive(r, refs);
        }
        Expr::Negate(e) => {
            extract_cell_refs_recursive(e, refs);
        }
        Expr::Function { args, .. } => {
            for arg in args {
                extract_cell_refs_recursive(arg, refs);
            }
        }
        _ => {}
    }
}

/// Evaluator that can resolve cell references
struct SheetEvaluator<'a> {
    sheet: &'a Sheet,
    library: &'a UnitLibrary,
}

impl<'a> SheetEvaluator<'a> {
    fn eval(&self, expr: &Expr) -> Result<EvalResult, EvalError> {

        match expr {
            Expr::Number(n) => Ok(EvalResult::new(*n, crate::core::units::Unit::dimensionless())),

            Expr::NumberWithUnit { value, unit } => {
                if !self.library.contains(unit) {
                    return Err(EvalError::UnknownUnit(unit.clone()));
                }
                let unit_obj = self.library.get(unit).unwrap().clone();
                Ok(EvalResult::new(*value, unit_obj))
            }

            Expr::CellRef { col, row } => {
                let addr = CellAddr::new(col.clone(), *row);
                let cell = self.sheet.get(&addr).ok_or_else(|| {
                    EvalError::CellNotFound(addr.to_string())
                })?;

                let value = cell.as_number().ok_or_else(|| {
                    EvalError::InvalidOperation(format!("Cell {} does not contain a number", addr))
                })?;

                Ok(EvalResult::new(
                    value,
                    cell.storage_unit().clone(),
                ))
            }

            Expr::Add(left, right) => {
                let left_result = self.eval(left)?;
                let right_result = self.eval(right)?;
                self.eval_binary_op(left_result, right_result, |a, b| a + b, "add")
            }

            Expr::Subtract(left, right) => {
                let left_result = self.eval(left)?;
                let right_result = self.eval(right)?;
                self.eval_binary_op(left_result, right_result, |a, b| a - b, "subtract")
            }

            Expr::Multiply(left, right) => {
                let left_result = self.eval(left)?;
                let right_result = self.eval(right)?;

                let value = left_result.value * right_result.value;

                // Unit multiplication logic (simplified for now)
                if left_result.unit.is_dimensionless() && right_result.unit.is_dimensionless() {
                    return Ok(EvalResult::new(value, crate::core::units::Unit::dimensionless()));
                }
                if left_result.unit.is_dimensionless() {
                    return Ok(EvalResult::new(value, right_result.unit.clone()));
                }
                if right_result.unit.is_dimensionless() {
                    return Ok(EvalResult::new(value, left_result.unit.clone()));
                }

                // Create compound unit
                let compound_symbol = format!("{}*{}", left_result.unit.canonical(), right_result.unit.canonical());
                let compound_unit = crate::core::units::Unit::simple(
                    compound_symbol.clone(),
                    crate::core::units::BaseDimension::Custom(compound_symbol)
                );

                Ok(EvalResult::new(value, compound_unit))
            }

            Expr::Divide(left, right) => {
                let left_result = self.eval(left)?;
                let right_result = self.eval(right)?;

                if right_result.value == 0.0 {
                    return Err(EvalError::DivisionByZero);
                }

                let value = left_result.value / right_result.value;

                // Unit division logic
                if left_result.unit.is_dimensionless() && right_result.unit.is_dimensionless() {
                    return Ok(EvalResult::new(value, crate::core::units::Unit::dimensionless()));
                }
                if right_result.unit.is_dimensionless() {
                    return Ok(EvalResult::new(value, left_result.unit.clone()));
                }
                if left_result.unit.is_equal(&right_result.unit) {
                    return Ok(EvalResult::new(value, crate::core::units::Unit::dimensionless()));
                }

                // Create compound unit
                let compound_symbol = format!("{}/{}", left_result.unit.canonical(), right_result.unit.canonical());
                let compound_unit = crate::core::units::Unit::simple(
                    compound_symbol.clone(),
                    crate::core::units::BaseDimension::Custom(compound_symbol)
                );

                Ok(EvalResult::new(value, compound_unit))
            }

            Expr::Negate(expr) => {
                let result = self.eval(expr)?;
                Ok(EvalResult::new(-result.value, result.unit))
            }

            Expr::Range { .. } => {
                Err(EvalError::InvalidOperation("Ranges can only be used in functions".to_string()))
            }

            Expr::Function { name, args } => {
                match name.to_uppercase().as_str() {
                    "SUM" => self.eval_sum(args),
                    "AVERAGE" => self.eval_average(args),
                    _ => Err(EvalError::FunctionNotImplemented(name.clone())),
                }
            }
        }
    }

    fn eval_binary_op<F>(
        &self,
        left: EvalResult,
        right: EvalResult,
        op: F,
        op_name: &str,
    ) -> Result<EvalResult, EvalError>
    where
        F: Fn(f64, f64) -> f64,
    {
        // Both dimensionless - simple operation
        if left.unit.is_dimensionless() && right.unit.is_dimensionless() {
            return Ok(EvalResult::new(
                op(left.value, right.value),
                crate::core::units::Unit::dimensionless(),
            ));
        }

        // Check if units are compatible
        if !left.unit.is_compatible(&right.unit) {
            return Err(EvalError::IncompatibleUnits {
                operation: op_name.to_string(),
                left: left.unit.to_string(),
                right: right.unit.to_string(),
            });
        }

        // If units are exactly the same, just operate
        if left.unit.is_equal(&right.unit) {
            return Ok(EvalResult::new(
                op(left.value, right.value),
                left.unit.clone(),
            ));
        }

        // Units are compatible but different - convert right to left's unit
        let right_value_converted = self.library.convert(
            right.value,
            right.unit.canonical(),
            left.unit.canonical(),
        ).ok_or_else(|| EvalError::IncompatibleUnits {
            operation: op_name.to_string(),
            left: left.unit.to_string(),
            right: right.unit.to_string(),
        })?;

        Ok(EvalResult::new(
            op(left.value, right_value_converted),
            left.unit.clone(),
        ))
    }

    /// Evaluate SUM function
    fn eval_sum(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        if args.is_empty() {
            return Ok(EvalResult::new(0.0, crate::core::units::Unit::dimensionless()));
        }

        // Collect all values from arguments (including ranges)
        let values = self.collect_values(args)?;

        if values.is_empty() {
            return Ok(EvalResult::new(0.0, crate::core::units::Unit::dimensionless()));
        }

        // All values should have compatible units
        let first = &values[0];
        let mut sum = first.value;
        let result_unit = first.unit.clone();

        for val in &values[1..] {
            if !val.unit.is_compatible(&result_unit) {
                return Err(EvalError::IncompatibleUnits {
                    operation: "SUM".to_string(),
                    left: result_unit.to_string(),
                    right: val.unit.to_string(),
                });
            }

            // Convert to result unit if needed
            let converted_value = if val.unit.is_equal(&result_unit) {
                val.value
            } else {
                self.library.convert(val.value, val.unit.canonical(), result_unit.canonical())
                    .ok_or_else(|| EvalError::IncompatibleUnits {
                        operation: "SUM".to_string(),
                        left: result_unit.to_string(),
                        right: val.unit.to_string(),
                    })?
            };

            sum += converted_value;
        }

        Ok(EvalResult::new(sum, result_unit))
    }

    /// Evaluate AVERAGE function
    fn eval_average(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        if args.is_empty() {
            return Err(EvalError::InvalidOperation("AVERAGE requires at least one argument".to_string()));
        }

        // Collect all values from arguments (including ranges)
        let values = self.collect_values(args)?;

        if values.is_empty() {
            return Err(EvalError::InvalidOperation("AVERAGE requires at least one value".to_string()));
        }

        // Calculate sum
        let sum_result = self.eval_sum(args)?;

        // Divide by count
        let count = values.len() as f64;
        Ok(EvalResult::new(sum_result.value / count, sum_result.unit))
    }

    /// Collect values from arguments (expanding ranges)
    fn collect_values(&self, args: &[Expr]) -> Result<Vec<EvalResult>, EvalError> {
        let mut values = Vec::new();

        for arg in args {
            match arg {
                Expr::Range { start, end } => {
                    // Extract range bounds
                    let (start_col, start_row) = match start.as_ref() {
                        Expr::CellRef { col, row } => (col.clone(), *row),
                        _ => return Err(EvalError::InvalidOperation("Range must use cell references".to_string())),
                    };

                    let (end_col, end_row) = match end.as_ref() {
                        Expr::CellRef { col, row } => (col.clone(), *row),
                        _ => return Err(EvalError::InvalidOperation("Range must use cell references".to_string())),
                    };

                    // For simplicity, only support single-column ranges for now
                    if start_col != end_col {
                        return Err(EvalError::InvalidOperation("Only single-column ranges supported in MLP".to_string()));
                    }

                    // Iterate through rows
                    for row in start_row..=end_row {
                        let addr = CellAddr::new(&start_col, row);
                        if let Some(cell) = self.sheet.get(&addr) {
                            if let Some(value) = cell.as_number() {
                                values.push(EvalResult::new(value, cell.storage_unit().clone()));
                            }
                        }
                    }
                }
                _ => {
                    // Evaluate single expression
                    values.push(self.eval(arg)?);
                }
            }
        }

        Ok(values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::units::{BaseDimension, Unit};

    #[test]
    fn test_cell_addr() {
        let addr = CellAddr::new("A", 1);
        assert_eq!(addr.to_string(), "A1");

        let addr = CellAddr::from_string("B12").unwrap();
        assert_eq!(addr.col, "B");
        assert_eq!(addr.row, 12);
    }

    #[test]
    fn test_dependency_graph() {
        let mut graph = DependencyGraph::new();

        let a1 = CellAddr::new("A", 1);
        let a2 = CellAddr::new("A", 2);
        let a3 = CellAddr::new("A", 3);

        // A3 = A1 + A2
        graph.add_dependency(a3.clone(), a1.clone());
        graph.add_dependency(a3.clone(), a2.clone());

        let deps = graph.get_dependencies(&a3);
        assert_eq!(deps.len(), 2);
        assert!(deps.contains(&a1));
        assert!(deps.contains(&a2));

        let dependents = graph.get_dependents(&a1);
        assert_eq!(dependents.len(), 1);
        assert!(dependents.contains(&a3));
    }

    #[test]
    fn test_circular_reference_detection() {
        let mut graph = DependencyGraph::new();

        let a1 = CellAddr::new("A", 1);
        let a2 = CellAddr::new("A", 2);
        let a3 = CellAddr::new("A", 3);

        // A2 = A1 + 1
        graph.add_dependency(a2.clone(), a1.clone());
        // A3 = A2 + 1
        graph.add_dependency(a3.clone(), a2.clone());

        // No circular reference yet
        assert!(!graph.has_circular_reference(&a1));
        assert!(!graph.has_circular_reference(&a2));
        assert!(!graph.has_circular_reference(&a3));

        // Now create circular reference: A1 = A3 + 1
        graph.add_dependency(a1.clone(), a3.clone());

        // Should detect circular reference
        assert!(graph.has_circular_reference(&a1));
        assert!(graph.has_circular_reference(&a2));
        assert!(graph.has_circular_reference(&a3));
    }

    #[test]
    fn test_sheet_set_get() {
        let mut sheet = Sheet::new();
        let addr = CellAddr::new("A", 1);
        let cell = Cell::new(42.0, Unit::dimensionless());

        sheet.set(addr.clone(), cell).unwrap();

        let retrieved = sheet.get(&addr).unwrap();
        assert_eq!(retrieved.as_number(), Some(42.0));
    }

    #[test]
    fn test_formula_evaluation() {
        let mut sheet = Sheet::new();

        // Set A1 = 100m
        let a1 = CellAddr::new("A", 1);
        sheet.set(a1, Cell::new(100.0, Unit::simple("m", BaseDimension::Length))).unwrap();

        // Set A2 = 50m
        let a2 = CellAddr::new("A", 2);
        sheet.set(a2, Cell::new(50.0, Unit::simple("m", BaseDimension::Length))).unwrap();

        // Evaluate A1 + A2
        let (value, unit) = sheet.evaluate_formula("=A1 + A2").unwrap();
        assert_eq!(value, 150.0);
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_extract_cell_refs() {
        let expr = Expr::add(
            Expr::cell_ref("A", 1),
            Expr::cell_ref("B", 2),
        );

        let refs = extract_cell_refs(&expr);
        assert_eq!(refs.len(), 2);
        assert!(refs.contains(&CellAddr::new("A", 1)));
        assert!(refs.contains(&CellAddr::new("B", 2)));
    }

    #[test]
    fn test_sum_function() {
        let mut sheet = Sheet::new();

        // Set up cells A1-A3
        sheet.set(CellAddr::new("A", 1), Cell::new(10.0, Unit::simple("m", BaseDimension::Length))).unwrap();
        sheet.set(CellAddr::new("A", 2), Cell::new(20.0, Unit::simple("m", BaseDimension::Length))).unwrap();
        sheet.set(CellAddr::new("A", 3), Cell::new(30.0, Unit::simple("m", BaseDimension::Length))).unwrap();

        // Evaluate SUM(A1:A3)
        let (value, unit) = sheet.evaluate_formula("=SUM(A1:A3)").unwrap();
        assert_eq!(value, 60.0);
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_sum_with_individual_args() {
        let mut sheet = Sheet::new();

        // Set up cells
        sheet.set(CellAddr::new("A", 1), Cell::new(10.0, Unit::simple("m", BaseDimension::Length))).unwrap();
        sheet.set(CellAddr::new("A", 2), Cell::new(20.0, Unit::simple("m", BaseDimension::Length))).unwrap();

        // Evaluate SUM(A1, A2)
        let (value, unit) = sheet.evaluate_formula("=SUM(A1, A2)").unwrap();
        assert_eq!(value, 30.0);
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_average_function() {
        let mut sheet = Sheet::new();

        // Set up cells A1-A4
        sheet.set(CellAddr::new("A", 1), Cell::new(10.0, Unit::simple("m", BaseDimension::Length))).unwrap();
        sheet.set(CellAddr::new("A", 2), Cell::new(20.0, Unit::simple("m", BaseDimension::Length))).unwrap();
        sheet.set(CellAddr::new("A", 3), Cell::new(30.0, Unit::simple("m", BaseDimension::Length))).unwrap();
        sheet.set(CellAddr::new("A", 4), Cell::new(40.0, Unit::simple("m", BaseDimension::Length))).unwrap();

        // Evaluate AVERAGE(A1:A4)
        let (value, unit) = sheet.evaluate_formula("=AVERAGE(A1:A4)").unwrap();
        assert_eq!(value, 25.0);
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_sum_with_unit_conversion() {
        let mut sheet = Sheet::new();

        // Set up cells with different but compatible units
        sheet.set(CellAddr::new("A", 1), Cell::new(100.0, Unit::simple("m", BaseDimension::Length))).unwrap();
        sheet.set(CellAddr::new("A", 2), Cell::new(50.0, Unit::simple("cm", BaseDimension::Length))).unwrap();

        // Evaluate SUM(A1, A2) - should convert cm to m
        let (value, unit) = sheet.evaluate_formula("=SUM(A1, A2)").unwrap();
        assert_eq!(value, 100.5); // 100m + 0.5m
        assert_eq!(unit.canonical(), "m");
    }
}
