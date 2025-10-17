// Spreadsheet sheet with cell management

use crate::core::cell::{Cell, CellValue};
use crate::core::formula::{parse_formula, EvalError, EvalResult, Expr};
use crate::core::units::UnitLibrary;
use statrs::statistics::{Data, Distribution, OrderStatistics};
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

        self.dependents.entry(depends_on).or_default().insert(cell);
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
        self.dependencies.get(cell).cloned().unwrap_or_default()
    }

    /// Get cells that depend on this cell
    pub fn get_dependents(&self, cell: &CellAddr) -> HashSet<CellAddr> {
        self.dependents.get(cell).cloned().unwrap_or_default()
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
        // First, collect all cells that need recalculation (changed cells + their dependents)
        let mut to_recalc = HashSet::new();
        for cell in changed_cells {
            self.collect_affected_cells(cell, &mut to_recalc);
        }

        // Now do topological sort on these cells
        let mut order = Vec::new();
        let mut visited = HashSet::new();
        let mut temp_mark = HashSet::new();

        for cell in &to_recalc {
            if !visited.contains(cell) {
                self.topological_sort(cell, &mut visited, &mut temp_mark, &mut order, &to_recalc);
            }
        }

        order
    }

    /// Collect all cells affected by a change (the cell and all its dependents recursively)
    fn collect_affected_cells(&self, cell: &CellAddr, affected: &mut HashSet<CellAddr>) {
        if affected.contains(cell) {
            return;
        }
        affected.insert(cell.clone());

        // Add all dependents recursively
        if let Some(deps) = self.dependents.get(cell) {
            for dep in deps {
                self.collect_affected_cells(dep, affected);
            }
        }
    }

    /// Topological sort using DFS
    /// Visits dependencies before the cell itself
    fn topological_sort(
        &self,
        cell: &CellAddr,
        visited: &mut HashSet<CellAddr>,
        temp_mark: &mut HashSet<CellAddr>,
        order: &mut Vec<CellAddr>,
        to_recalc: &HashSet<CellAddr>,
    ) {
        if visited.contains(cell) {
            return;
        }

        if temp_mark.contains(cell) {
            // Circular dependency detected, but we already check for this when setting formulas
            return;
        }

        temp_mark.insert(cell.clone());

        // Visit all dependencies first (cells this cell depends on)
        if let Some(deps) = self.dependencies.get(cell) {
            for dep in deps {
                // Only visit if this dependency is in our recalculation set
                if to_recalc.contains(dep) {
                    self.topological_sort(dep, visited, temp_mark, order, to_recalc);
                }
            }
        }

        temp_mark.remove(cell);
        visited.insert(cell.clone());
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

    /// Column widths (in pixels)
    column_widths: HashMap<String, f64>,

    /// Row heights (in pixels)
    row_heights: HashMap<usize, f64>,
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
            column_widths: HashMap::new(),
            row_heights: HashMap::new(),
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
        self.set_with_named_ranges(addr, cell, None)
    }

    /// Set a cell with named range context for dependency tracking
    pub fn set_with_named_ranges(
        &mut self,
        addr: CellAddr,
        cell: Cell,
        named_range_mapping: Option<&HashMap<String, CellAddr>>,
    ) -> Result<(), SheetError> {
        // If the cell has a formula, extract dependencies
        if let Some(formula) = cell.formula() {
            self.update_dependencies(&addr, formula, named_range_mapping)?;
        } else {
            // Clear dependencies if no formula
            self.dependencies.remove_dependencies(&addr);
        }

        self.cells.insert(addr, cell);
        Ok(())
    }

    /// Update dependencies for a formula
    fn update_dependencies(
        &mut self,
        addr: &CellAddr,
        formula: &str,
        named_range_mapping: Option<&HashMap<String, CellAddr>>,
    ) -> Result<(), SheetError> {
        // Clear existing dependencies
        self.dependencies.remove_dependencies(addr);

        // Parse formula and extract cell references
        let expr = parse_formula(formula).map_err(|e| SheetError::ParseError(e.to_string()))?;

        let deps = extract_cell_refs(&expr, named_range_mapping);

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
    pub fn evaluate_formula(
        &self,
        formula: &str,
    ) -> Result<(CellValue, crate::core::units::Unit), SheetError> {
        self.evaluate_formula_with_named_refs(formula, None)
    }

    /// Evaluate a formula with named range context
    pub fn evaluate_formula_with_named_refs(
        &self,
        formula: &str,
        named_refs: Option<&HashMap<String, (f64, crate::core::units::Unit)>>,
    ) -> Result<(CellValue, crate::core::units::Unit), SheetError> {
        let expr = parse_formula(formula).map_err(|e| SheetError::ParseError(e.to_string()))?;

        let evaluator = SheetEvaluator {
            sheet: self,
            library: &self.library,
            named_refs,
        };

        let result = evaluator.eval(&expr)?;
        // Convert EvalValue to CellValue
        let cell_value = match result.value {
            crate::core::formula::evaluator::EvalValue::Number(n) => CellValue::Number(n),
            crate::core::formula::evaluator::EvalValue::Text(s) => CellValue::Text(s),
        };
        Ok((cell_value, result.unit))
    }

    /// Recalculate cells that depend on changed cells
    pub fn recalculate(&mut self, changed: &[CellAddr]) -> Result<(), SheetError> {
        self.recalculate_with_named_refs(changed, None)
    }

    /// Recalculate cells with named range context
    pub fn recalculate_with_named_refs(
        &mut self,
        changed: &[CellAddr],
        named_refs: Option<&HashMap<String, (f64, crate::core::units::Unit)>>,
    ) -> Result<(), SheetError> {
        let order = self.dependencies.calculation_order(changed);

        for addr in order {
            if let Some(cell) = self.cells.get(&addr).cloned() {
                if let Some(formula) = cell.formula() {
                    match self.evaluate_formula_with_named_refs(formula, named_refs) {
                        Ok((value, unit)) => {
                            let mut updated_cell = cell;
                            updated_cell.set_value(value);
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

    // Column and row sizing methods

    /// Set the width of a column (in pixels)
    pub fn set_column_width(&mut self, col: String, width: f64) {
        self.column_widths.insert(col, width);
    }

    /// Get the width of a column (returns None if using default width)
    pub fn get_column_width(&self, col: &str) -> Option<f64> {
        self.column_widths.get(col).copied()
    }

    /// Get all custom column widths
    pub fn get_all_column_widths(&self) -> &HashMap<String, f64> {
        &self.column_widths
    }

    /// Set the height of a row (in pixels)
    pub fn set_row_height(&mut self, row: usize, height: f64) {
        self.row_heights.insert(row, height);
    }

    /// Get the height of a row (returns None if using default height)
    pub fn get_row_height(&self, row: usize) -> Option<f64> {
        self.row_heights.get(&row).copied()
    }

    /// Get all custom row heights
    pub fn get_all_row_heights(&self) -> &HashMap<usize, f64> {
        &self.row_heights
    }
}

impl Default for Sheet {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract cell references from an expression
fn extract_cell_refs(
    expr: &Expr,
    named_range_mapping: Option<&HashMap<String, CellAddr>>,
) -> HashSet<CellAddr> {
    let mut refs = HashSet::new();
    extract_cell_refs_recursive(expr, &mut refs, named_range_mapping);
    refs
}

fn extract_cell_refs_recursive(
    expr: &Expr,
    refs: &mut HashSet<CellAddr>,
    named_range_mapping: Option<&HashMap<String, CellAddr>>,
) {
    match expr {
        Expr::CellRef { col, row } => {
            refs.insert(CellAddr::new(col.clone(), *row));
        }
        Expr::NamedRef { name } => {
            // Resolve named reference to cell address
            if let Some(mapping) = named_range_mapping {
                if let Some(addr) = mapping.get(name) {
                    refs.insert(addr.clone());
                }
            }
        }
        Expr::Range { start, end } => {
            extract_cell_refs_recursive(start, refs, named_range_mapping);
            extract_cell_refs_recursive(end, refs, named_range_mapping);
        }
        Expr::Add(l, r)
        | Expr::Subtract(l, r)
        | Expr::Multiply(l, r)
        | Expr::Divide(l, r)
        | Expr::GreaterThan(l, r)
        | Expr::LessThan(l, r)
        | Expr::GreaterOrEqual(l, r)
        | Expr::LessOrEqual(l, r)
        | Expr::Equal(l, r)
        | Expr::NotEqual(l, r)
        | Expr::And(l, r)
        | Expr::Or(l, r) => {
            extract_cell_refs_recursive(l, refs, named_range_mapping);
            extract_cell_refs_recursive(r, refs, named_range_mapping);
        }
        Expr::Negate(e) | Expr::Not(e) => {
            extract_cell_refs_recursive(e, refs, named_range_mapping);
        }
        Expr::Function { args, .. } => {
            for arg in args {
                extract_cell_refs_recursive(arg, refs, named_range_mapping);
            }
        }
        _ => {}
    }
}

/// Evaluator that can resolve cell references
struct SheetEvaluator<'a> {
    sheet: &'a Sheet,
    library: &'a UnitLibrary,
    named_refs: Option<&'a HashMap<String, (f64, crate::core::units::Unit)>>,
}

impl<'a> SheetEvaluator<'a> {
    fn eval(&self, expr: &Expr) -> Result<EvalResult, EvalError> {
        match expr {
            Expr::Number(n) => Ok(EvalResult::new(
                *n,
                crate::core::units::Unit::dimensionless(),
            )),

            Expr::NumberWithUnit { value, unit } => {
                // Parse the unit (supports both simple and compound units)
                use crate::core::units::parse_unit;
                let unit_obj = parse_unit(unit, self.library)
                    .map_err(|_| EvalError::UnknownUnit(unit.clone()))?;
                Ok(EvalResult::new(*value, unit_obj))
            }

            Expr::String(s) => Ok(EvalResult::text(s.clone())),

            Expr::CellRef { col, row } => {
                let addr = CellAddr::new(col.clone(), *row);
                let cell = self
                    .sheet
                    .get(&addr)
                    .ok_or_else(|| EvalError::CellNotFound(addr.to_string()))?;

                // Handle both number and text cells
                match cell.value() {
                    CellValue::Number(n) => Ok(EvalResult::new(*n, cell.storage_unit().clone())),
                    CellValue::Text(t) => Ok(EvalResult::text(t.clone())),
                    CellValue::Empty => Err(EvalError::InvalidOperation(format!(
                        "Cell {} is empty",
                        addr
                    ))),
                    CellValue::Error(e) => Err(EvalError::InvalidOperation(format!(
                        "Cell {} has error: {}",
                        addr, e
                    ))),
                }
            }

            Expr::NamedRef { name } => {
                // Look up named reference in the provided HashMap
                if let Some(named_refs) = self.named_refs {
                    if let Some((value, unit)) = named_refs.get(name) {
                        Ok(EvalResult::new(*value, unit.clone()))
                    } else {
                        Err(EvalError::NamedRefNotFound(format!(
                            "Named reference '{}' not found",
                            name
                        )))
                    }
                } else {
                    Err(EvalError::NamedRefNotFound(format!(
                        "Named reference '{}' requires workbook context",
                        name
                    )))
                }
            }

            Expr::Add(left, right) => {
                let left_result = self.eval(left)?;
                let right_result = self.eval(right)?;

                // If either operand is text, perform string concatenation
                if left_result.is_text() || right_result.is_text() {
                    use crate::core::formula::evaluator::EvalValue;
                    let left_str = match &left_result.value {
                        EvalValue::Text(s) => s.clone(),
                        EvalValue::Number(n) => {
                            if left_result.unit.is_dimensionless() {
                                n.to_string()
                            } else {
                                format!("{} {}", n, left_result.unit)
                            }
                        }
                    };
                    let right_str = match &right_result.value {
                        EvalValue::Text(s) => s.clone(),
                        EvalValue::Number(n) => {
                            if right_result.unit.is_dimensionless() {
                                n.to_string()
                            } else {
                                format!("{} {}", n, right_result.unit)
                            }
                        }
                    };
                    return Ok(EvalResult::text(format!("{}{}", left_str, right_str)));
                }

                // Both are numbers - proceed with numeric addition
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

                // Multiplication requires both operands to be numbers
                let left_value = left_result.as_number().ok_or_else(|| {
                    EvalError::InvalidOperation("Cannot multiply with text values".to_string())
                })?;
                let right_value = right_result.as_number().ok_or_else(|| {
                    EvalError::InvalidOperation("Cannot multiply with text values".to_string())
                })?;

                let value = left_value * right_value;

                // Check if either operand is a percentage - treat as dimensionless multiplier
                let left_is_percent = left_result.unit.canonical() == "%";
                let right_is_percent = right_result.unit.canonical() == "%";

                // If one is percentage, result has the non-percentage unit (percentage gets removed)
                if left_is_percent && !right_is_percent {
                    return Ok(EvalResult::new(value, right_result.unit.clone()));
                }
                if right_is_percent && !left_is_percent {
                    return Ok(EvalResult::new(value, left_result.unit.clone()));
                }
                // If both are percentages, result is dimensionless
                if left_is_percent && right_is_percent {
                    return Ok(EvalResult::new(
                        value,
                        crate::core::units::Unit::dimensionless(),
                    ));
                }

                // If both dimensionless, result is dimensionless
                if left_result.unit.is_dimensionless() && right_result.unit.is_dimensionless() {
                    return Ok(EvalResult::new(
                        value,
                        crate::core::units::Unit::dimensionless(),
                    ));
                }

                // If one is dimensionless, result has the other's unit
                if left_result.unit.is_dimensionless() {
                    return Ok(EvalResult::new(value, right_result.unit.clone()));
                }
                if right_result.unit.is_dimensionless() {
                    return Ok(EvalResult::new(value, left_result.unit.clone()));
                }

                // Multiply units with cancellation
                use crate::core::formula::evaluator::multiply_units_with_cancellation;
                let result_unit =
                    multiply_units_with_cancellation(&left_result.unit, &right_result.unit);

                Ok(EvalResult::new(value, result_unit))
            }

            Expr::Divide(left, right) => {
                let left_result = self.eval(left)?;
                let right_result = self.eval(right)?;

                // Division requires both operands to be numbers
                let left_value = left_result.as_number().ok_or_else(|| {
                    EvalError::InvalidOperation("Cannot divide with text values".to_string())
                })?;
                let right_value = right_result.as_number().ok_or_else(|| {
                    EvalError::InvalidOperation("Cannot divide with text values".to_string())
                })?;

                if right_value == 0.0 {
                    return Err(EvalError::DivisionByZero);
                }

                let value = left_value / right_value;

                // Check if either operand is a percentage - treat as dimensionless multiplier
                let left_is_percent = left_result.unit.canonical() == "%";
                let right_is_percent = right_result.unit.canonical() == "%";

                // If right is percentage, result has left's unit (percentage gets removed)
                if right_is_percent && !left_is_percent {
                    return Ok(EvalResult::new(value, left_result.unit.clone()));
                }
                // If left is percentage and right is not, result is percentage/right_unit
                // This is unusual but should be handled - treated as dimensionless/right_unit
                if left_is_percent && !right_is_percent {
                    let compound_symbol = format!("1/{}", right_result.unit.canonical());
                    if let Some(right_dim) = right_result.unit.dimension().as_simple() {
                        let compound_unit = crate::core::units::Unit::compound(
                            compound_symbol.clone(),
                            vec![],
                            vec![(right_dim.clone(), 1)],
                        );
                        return Ok(EvalResult::new(value, compound_unit));
                    }
                }
                // If both are percentages, result is dimensionless
                if left_is_percent && right_is_percent {
                    return Ok(EvalResult::new(
                        value,
                        crate::core::units::Unit::dimensionless(),
                    ));
                }

                // If both dimensionless, result is dimensionless
                if left_result.unit.is_dimensionless() && right_result.unit.is_dimensionless() {
                    return Ok(EvalResult::new(
                        value,
                        crate::core::units::Unit::dimensionless(),
                    ));
                }

                // If right is dimensionless, result has left's unit
                if right_result.unit.is_dimensionless() {
                    return Ok(EvalResult::new(value, left_result.unit.clone()));
                }

                // If units are the same, they cancel out
                if left_result.unit.is_equal(&right_result.unit) {
                    return Ok(EvalResult::new(
                        value,
                        crate::core::units::Unit::dimensionless(),
                    ));
                }

                // Create compound unit using original symbols (don't cancel yet - that happens in multiply)
                let compound_symbol = format!(
                    "{}/{}",
                    left_result.unit.canonical(),
                    right_result.unit.canonical()
                );

                // For simple dimensions, create compound unit
                let compound_unit = if let (Some(left_dim), Some(right_dim)) = (
                    left_result.unit.dimension().as_simple(),
                    right_result.unit.dimension().as_simple(),
                ) {
                    crate::core::units::Unit::compound(
                        compound_symbol.clone(),
                        vec![(left_dim.clone(), 1)],
                        vec![(right_dim.clone(), 1)],
                    )
                } else {
                    // Fallback: create custom dimension
                    crate::core::units::Unit::simple(
                        compound_symbol.clone(),
                        crate::core::units::BaseDimension::Custom(compound_symbol),
                    )
                };

                Ok(EvalResult::new(value, compound_unit))
            }

            Expr::Negate(expr) => {
                let result = self.eval(expr)?;
                let value = result.as_number().ok_or_else(|| {
                    EvalError::InvalidOperation("Cannot negate a text value".to_string())
                })?;
                Ok(EvalResult::new(-value, result.unit))
            }

            Expr::Boolean(b) => Ok(EvalResult::new(
                if *b { 1.0 } else { 0.0 },
                crate::core::units::Unit::dimensionless(),
            )),

            Expr::GreaterThan(left, right) => self.eval_comparison(left, right, |a, b| a > b),
            Expr::LessThan(left, right) => self.eval_comparison(left, right, |a, b| a < b),
            Expr::GreaterOrEqual(left, right) => self.eval_comparison(left, right, |a, b| a >= b),
            Expr::LessOrEqual(left, right) => self.eval_comparison(left, right, |a, b| a <= b),
            Expr::Equal(left, right) => self.eval_comparison(left, right, |a, b| a == b),
            Expr::NotEqual(left, right) => self.eval_comparison(left, right, |a, b| a != b),

            Expr::And(left, right) => {
                let left_result = self.eval(left)?;
                let right_result = self.eval(right)?;
                let left_value = left_result.as_number().ok_or_else(|| {
                    EvalError::InvalidOperation("Cannot use AND with text values".to_string())
                })?;
                let right_value = right_result.as_number().ok_or_else(|| {
                    EvalError::InvalidOperation("Cannot use AND with text values".to_string())
                })?;
                let result = if left_value != 0.0 && right_value != 0.0 {
                    1.0
                } else {
                    0.0
                };
                Ok(EvalResult::new(
                    result,
                    crate::core::units::Unit::dimensionless(),
                ))
            }

            Expr::Or(left, right) => {
                let left_result = self.eval(left)?;
                let right_result = self.eval(right)?;
                let left_value = left_result.as_number().ok_or_else(|| {
                    EvalError::InvalidOperation("Cannot use OR with text values".to_string())
                })?;
                let right_value = right_result.as_number().ok_or_else(|| {
                    EvalError::InvalidOperation("Cannot use OR with text values".to_string())
                })?;
                let result = if left_value != 0.0 || right_value != 0.0 {
                    1.0
                } else {
                    0.0
                };
                Ok(EvalResult::new(
                    result,
                    crate::core::units::Unit::dimensionless(),
                ))
            }

            Expr::Not(expr) => {
                let result = self.eval(expr)?;
                let value = result.as_number().ok_or_else(|| {
                    EvalError::InvalidOperation("Cannot use NOT with text values".to_string())
                })?;
                let not_result = if value == 0.0 { 1.0 } else { 0.0 };
                Ok(EvalResult::new(
                    not_result,
                    crate::core::units::Unit::dimensionless(),
                ))
            }

            Expr::Range { .. } => Err(EvalError::InvalidOperation(
                "Ranges can only be used in functions".to_string(),
            )),

            Expr::Function { name, args } => match name.to_uppercase().as_str() {
                "SUM" => self.eval_sum(args),
                "AVERAGE" => self.eval_average(args),
                "CONVERT" => self.eval_convert(args),
                "PERCENT" => self.eval_percent(args),
                "COUNT" => self.eval_count(args),
                "MIN" => self.eval_min(args),
                "MAX" => self.eval_max(args),
                "ABS" => self.eval_abs(args),
                "ROUND" => self.eval_round(args),
                "FLOOR" => self.eval_floor(args),
                "CEIL" | "CEILING" => self.eval_ceil(args),
                "TRUNC" => self.eval_trunc(args),
                "MOD" => self.eval_mod(args),
                "SIGN" => self.eval_sign(args),
                "SQRT" => self.eval_sqrt(args),
                "POWER" => self.eval_power(args),
                "MEDIAN" => self.eval_median(args),
                "STDEV" => self.eval_stdev(args),
                "VAR" => self.eval_var(args),
                "IF" => self.eval_if(args),
                "AND" => self.eval_and(args),
                "OR" => self.eval_or(args),
                "NOT" => self.eval_not_fn(args),
                "GT" => self.eval_gt(args),
                "LT" => self.eval_lt(args),
                "GTE" => self.eval_gte(args),
                "LTE" => self.eval_lte(args),
                "EQ" => self.eval_eq(args),
                "NE" => self.eval_ne(args),
                _ => Err(EvalError::FunctionNotImplemented(name.clone())),
            },
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
        // Binary operations require both operands to be numbers
        let left_value = left.as_number().ok_or_else(|| {
            EvalError::InvalidOperation(format!("Cannot {} with text values", op_name))
        })?;
        let right_value = right.as_number().ok_or_else(|| {
            EvalError::InvalidOperation(format!("Cannot {} with text values", op_name))
        })?;

        // Both dimensionless - simple operation
        if left.unit.is_dimensionless() && right.unit.is_dimensionless() {
            return Ok(EvalResult::new(
                op(left_value, right_value),
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
                op(left_value, right_value),
                left.unit.clone(),
            ));
        }

        // Units are compatible but different - convert right to left's unit
        let right_value_converted = self
            .library
            .convert(right_value, right.unit.canonical(), left.unit.canonical())
            .ok_or_else(|| EvalError::IncompatibleUnits {
                operation: op_name.to_string(),
                left: left.unit.to_string(),
                right: right.unit.to_string(),
            })?;

        Ok(EvalResult::new(
            op(left_value, right_value_converted),
            left.unit.clone(),
        ))
    }

    /// Evaluate SUM function
    fn eval_sum(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        if args.is_empty() {
            return Ok(EvalResult::new(
                0.0,
                crate::core::units::Unit::dimensionless(),
            ));
        }

        // Collect all values from arguments (including ranges)
        let values = self.collect_values(args)?;

        if values.is_empty() {
            return Ok(EvalResult::new(
                0.0,
                crate::core::units::Unit::dimensionless(),
            ));
        }

        // All values should have compatible units
        let first = &values[0];
        let first_value = first.as_number().ok_or_else(|| {
            EvalError::InvalidOperation("SUM can only be used with numbers".to_string())
        })?;
        let mut sum = first_value;
        let result_unit = first.unit.clone();

        for val in &values[1..] {
            if !val.unit.is_compatible(&result_unit) {
                return Err(EvalError::IncompatibleUnits {
                    operation: "SUM".to_string(),
                    left: result_unit.to_string(),
                    right: val.unit.to_string(),
                });
            }

            // Get numeric value (SUM only works with numbers)
            let num_value = val.as_number().ok_or_else(|| {
                EvalError::InvalidOperation("SUM can only be used with numbers".to_string())
            })?;

            // Convert to result unit if needed
            let converted_value = if val.unit.is_equal(&result_unit) {
                num_value
            } else {
                self.library
                    .convert(num_value, val.unit.canonical(), result_unit.canonical())
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
            return Err(EvalError::InvalidOperation(
                "AVERAGE requires at least one argument".to_string(),
            ));
        }

        // Collect all values from arguments (including ranges)
        let values = self.collect_values(args)?;

        if values.is_empty() {
            return Err(EvalError::InvalidOperation(
                "AVERAGE requires at least one value".to_string(),
            ));
        }

        // Calculate sum
        let sum_result = self.eval_sum(args)?;

        // Divide by count
        let count = values.len() as f64;
        Ok(EvalResult::new(
            sum_result.numeric_value() / count,
            sum_result.unit,
        ))
    }

    /// Evaluate CONVERT function
    /// Syntax: CONVERT(value, target_unit)
    /// Example: CONVERT(A1, 1km) or CONVERT(100m, 1ft)
    fn eval_convert(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        if args.len() != 2 {
            return Err(EvalError::InvalidOperation(
                "CONVERT requires exactly 2 arguments: CONVERT(value, target_unit)".to_string(),
            ));
        }

        // Evaluate the value to convert
        let value_result = self.eval(&args[0])?;

        // Extract target unit from second argument
        // The target unit can be specified as "1 km" or just a reference with unit
        let target_unit = match &args[1] {
            Expr::NumberWithUnit { unit, .. } => {
                // Parse the unit string
                use crate::core::units::parse_unit;
                parse_unit(unit, self.library).map_err(|_| EvalError::UnknownUnit(unit.clone()))?
            }
            Expr::CellRef { col, row } => {
                // Get unit from referenced cell
                let addr = CellAddr::new(col.clone(), *row);
                let cell = self
                    .sheet
                    .get(&addr)
                    .ok_or_else(|| EvalError::CellNotFound(addr.to_string()))?;

                // Check if cell contains text - if so, parse it as a unit string
                match cell.value() {
                    CellValue::Text(text) => {
                        use crate::core::units::parse_unit;
                        parse_unit(text.as_str(), self.library)
                            .map_err(|_| EvalError::UnknownUnit(text.clone()))?
                    }
                    _ => {
                        // Otherwise, use the cell's storage unit
                        cell.storage_unit().clone()
                    }
                }
            }
            _ => {
                return Err(EvalError::InvalidOperation(
                    "CONVERT target unit must be specified as '1 unit' (e.g., 1km) or a cell reference".to_string()
                ));
            }
        };

        // Check if units are compatible
        if !value_result.unit.is_compatible(&target_unit) {
            return Err(EvalError::IncompatibleUnits {
                operation: "CONVERT".to_string(),
                left: value_result.unit.to_string(),
                right: target_unit.to_string(),
            });
        }

        // If units are already the same, no conversion needed
        if value_result.unit.is_equal(&target_unit) {
            return Ok(EvalResult::new(value_result.numeric_value(), target_unit));
        }

        // Perform conversion
        // Try compound unit conversion first (handles units like $/quarter -> $/year)
        let converted_value = if let Some(result) = convert_compound_unit_for_formula(
            value_result.numeric_value(),
            value_result.unit.canonical(),
            target_unit.canonical(),
            self.library,
        ) {
            result
        } else {
            // Fall back to simple unit conversion
            self.library
                .convert(
                    value_result.numeric_value(),
                    value_result.unit.canonical(),
                    target_unit.canonical(),
                )
                .ok_or_else(|| EvalError::IncompatibleUnits {
                    operation: "CONVERT".to_string(),
                    left: value_result.unit.to_string(),
                    right: target_unit.to_string(),
                })?
        };

        Ok(EvalResult::new(converted_value, target_unit))
    }

    /// Evaluate PERCENT function
    /// Syntax: PERCENT(value)
    /// Example: PERCENT(0.15) returns 0.15 with "%" unit → displays as "15%"
    fn eval_percent(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        if args.len() != 1 {
            return Err(EvalError::InvalidOperation(
                "PERCENT requires exactly 1 argument: PERCENT(value)".to_string(),
            ));
        }

        // Evaluate the value
        let value_result = self.eval(&args[0])?;

        // Create a percentage unit
        let percent_unit = crate::core::units::Unit::simple(
            "%",
            crate::core::units::BaseDimension::Custom("%".to_string()),
        );

        // Return the same value with "%" unit
        Ok(EvalResult::new(value_result.numeric_value(), percent_unit))
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
                        _ => {
                            return Err(EvalError::InvalidOperation(
                                "Range must use cell references".to_string(),
                            ))
                        }
                    };

                    let (end_col, end_row) = match end.as_ref() {
                        Expr::CellRef { col, row } => (col.clone(), *row),
                        _ => {
                            return Err(EvalError::InvalidOperation(
                                "Range must use cell references".to_string(),
                            ))
                        }
                    };

                    // For simplicity, only support single-column ranges for now
                    if start_col != end_col {
                        return Err(EvalError::InvalidOperation(
                            "Only single-column ranges supported in MLP".to_string(),
                        ));
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

    /// Evaluate COUNT function
    /// Counts non-empty cells in range or arguments
    fn eval_count(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        let values = self.collect_values(args)?;
        let count = values.len() as f64;
        Ok(EvalResult::new(
            count,
            crate::core::units::Unit::dimensionless(),
        ))
    }

    /// Evaluate MIN function
    /// Find minimum value in range with unit compatibility checking
    fn eval_min(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        let values = self.collect_values(args)?;

        if values.is_empty() {
            return Err(EvalError::InvalidOperation(
                "MIN requires at least one value".to_string(),
            ));
        }

        // All values must have compatible units
        let first_unit = &values[0].unit;
        let mut min_value = values[0].numeric_value();

        for val in &values[1..] {
            if !val.unit.is_compatible(first_unit) {
                return Err(EvalError::IncompatibleUnits {
                    operation: "MIN".to_string(),
                    left: first_unit.to_string(),
                    right: val.unit.to_string(),
                });
            }

            // Convert to first unit before comparing
            let converted_value = if val.unit.is_equal(first_unit) {
                val.numeric_value()
            } else {
                self.library
                    .convert(
                        val.numeric_value(),
                        val.unit.canonical(),
                        first_unit.canonical(),
                    )
                    .ok_or_else(|| EvalError::IncompatibleUnits {
                        operation: "MIN".to_string(),
                        left: first_unit.to_string(),
                        right: val.unit.to_string(),
                    })?
            };

            if converted_value < min_value {
                min_value = converted_value;
            }
        }

        Ok(EvalResult::new(min_value, first_unit.clone()))
    }

    /// Evaluate MAX function
    /// Find maximum value in range with unit compatibility checking
    fn eval_max(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        let values = self.collect_values(args)?;

        if values.is_empty() {
            return Err(EvalError::InvalidOperation(
                "MAX requires at least one value".to_string(),
            ));
        }

        // All values must have compatible units
        let first_unit = &values[0].unit;
        let mut max_value = values[0].numeric_value();

        for val in &values[1..] {
            if !val.unit.is_compatible(first_unit) {
                return Err(EvalError::IncompatibleUnits {
                    operation: "MAX".to_string(),
                    left: first_unit.to_string(),
                    right: val.unit.to_string(),
                });
            }

            // Convert to first unit before comparing
            let converted_value = if val.unit.is_equal(first_unit) {
                val.numeric_value()
            } else {
                self.library
                    .convert(
                        val.numeric_value(),
                        val.unit.canonical(),
                        first_unit.canonical(),
                    )
                    .ok_or_else(|| EvalError::IncompatibleUnits {
                        operation: "MAX".to_string(),
                        left: first_unit.to_string(),
                        right: val.unit.to_string(),
                    })?
            };

            if converted_value > max_value {
                max_value = converted_value;
            }
        }

        Ok(EvalResult::new(max_value, first_unit.clone()))
    }

    /// Evaluate ABS function
    /// Returns absolute value, preserving unit
    fn eval_abs(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        if args.len() != 1 {
            return Err(EvalError::InvalidOperation(
                "ABS requires exactly 1 argument".to_string(),
            ));
        }

        let result = self.eval(&args[0])?;
        Ok(EvalResult::new(result.numeric_value().abs(), result.unit))
    }

    /// Evaluate ROUND function
    /// Rounds to specified decimal places (default 0), preserving unit
    fn eval_round(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        if args.is_empty() || args.len() > 2 {
            return Err(EvalError::InvalidOperation(
                "ROUND requires 1 or 2 arguments: ROUND(value, [decimals])".to_string(),
            ));
        }

        let result = self.eval(&args[0])?;

        let decimals = if args.len() == 2 {
            let decimals_result = self.eval(&args[1])?;
            decimals_result.numeric_value().round() as i32
        } else {
            0
        };

        let multiplier = 10f64.powi(decimals);
        let rounded = (result.numeric_value() * multiplier).round() / multiplier;

        Ok(EvalResult::new(rounded, result.unit))
    }

    /// Evaluate FLOOR function
    /// Rounds down to integer, preserving unit
    fn eval_floor(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        if args.len() != 1 {
            return Err(EvalError::InvalidOperation(
                "FLOOR requires exactly 1 argument".to_string(),
            ));
        }

        let result = self.eval(&args[0])?;
        Ok(EvalResult::new(result.numeric_value().floor(), result.unit))
    }

    /// Evaluate CEIL function
    /// Rounds up to integer, preserving unit
    fn eval_ceil(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        if args.len() != 1 {
            return Err(EvalError::InvalidOperation(
                "CEIL requires exactly 1 argument".to_string(),
            ));
        }

        let result = self.eval(&args[0])?;
        Ok(EvalResult::new(result.numeric_value().ceil(), result.unit))
    }

    /// Evaluate TRUNC function
    /// Truncates to integer (removes decimals), preserving unit
    fn eval_trunc(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        if args.len() != 1 {
            return Err(EvalError::InvalidOperation(
                "TRUNC requires exactly 1 argument".to_string(),
            ));
        }

        let result = self.eval(&args[0])?;
        Ok(EvalResult::new(result.numeric_value().trunc(), result.unit))
    }

    /// Evaluate MOD function
    /// Modulo operation, both args must have compatible units
    fn eval_mod(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        if args.len() != 2 {
            return Err(EvalError::InvalidOperation(
                "MOD requires exactly 2 arguments: MOD(dividend, divisor)".to_string(),
            ));
        }

        let dividend = self.eval(&args[0])?;
        let divisor = self.eval(&args[1])?;

        // Check unit compatibility
        if !dividend.unit.is_compatible(&divisor.unit) {
            return Err(EvalError::IncompatibleUnits {
                operation: "MOD".to_string(),
                left: dividend.unit.to_string(),
                right: divisor.unit.to_string(),
            });
        }

        // Convert divisor to dividend's unit if needed
        let divisor_value = if divisor.unit.is_equal(&dividend.unit) {
            divisor.numeric_value()
        } else {
            self.library
                .convert(
                    divisor.numeric_value(),
                    divisor.unit.canonical(),
                    dividend.unit.canonical(),
                )
                .ok_or_else(|| EvalError::IncompatibleUnits {
                    operation: "MOD".to_string(),
                    left: dividend.unit.to_string(),
                    right: divisor.unit.to_string(),
                })?
        };

        let result_value = dividend.numeric_value() % divisor_value;
        Ok(EvalResult::new(result_value, dividend.unit))
    }

    /// Evaluate SIGN function
    /// Returns -1, 0, or 1 (dimensionless)
    fn eval_sign(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        if args.len() != 1 {
            return Err(EvalError::InvalidOperation(
                "SIGN requires exactly 1 argument".to_string(),
            ));
        }

        let result = self.eval(&args[0])?;
        let sign = if result.numeric_value() == 0.0 {
            0.0
        } else if result.numeric_value() > 0.0 {
            1.0
        } else {
            -1.0
        };
        Ok(EvalResult::new(
            sign,
            crate::core::units::Unit::dimensionless(),
        ))
    }

    /// Generic comparison evaluation with unit conversion
    fn eval_comparison<F>(
        &self,
        left: &Expr,
        right: &Expr,
        compare: F,
    ) -> Result<EvalResult, EvalError>
    where
        F: Fn(f64, f64) -> bool,
    {
        let left_val = self.eval(left)?;
        let right_val = self.eval(right)?;

        // Check if units are compatible
        if !left_val.unit.is_compatible(&right_val.unit) {
            return Err(EvalError::IncompatibleUnits {
                operation: "comparison".to_string(),
                left: left_val.unit.to_string(),
                right: right_val.unit.to_string(),
            });
        }

        // Convert right to left's unit if needed
        let right_converted = if left_val.unit.is_equal(&right_val.unit) {
            right_val.numeric_value()
        } else {
            self.library
                .convert(
                    right_val.numeric_value(),
                    right_val.unit.canonical(),
                    left_val.unit.canonical(),
                )
                .ok_or_else(|| EvalError::IncompatibleUnits {
                    operation: "comparison".to_string(),
                    left: left_val.unit.to_string(),
                    right: right_val.unit.to_string(),
                })?
        };

        // Compare and return boolean (as 1.0 or 0.0)
        let result = if compare(left_val.numeric_value(), right_converted) {
            1.0
        } else {
            0.0
        };
        Ok(EvalResult::new(
            result,
            crate::core::units::Unit::dimensionless(),
        ))
    }

    /// Evaluate IF function
    /// IF(condition, true_value, false_value)
    fn eval_if(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        if args.len() != 3 {
            return Err(EvalError::InvalidOperation(
                "IF requires exactly 3 arguments: IF(condition, true_value, false_value)"
                    .to_string(),
            ));
        }

        let condition = self.eval(&args[0])?;

        // Treat non-zero as true
        if condition.numeric_value() != 0.0 {
            self.eval(&args[1])
        } else {
            self.eval(&args[2])
        }
    }

    /// Evaluate AND function
    /// AND(arg1, arg2, ...)
    fn eval_and(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        if args.is_empty() {
            return Err(EvalError::InvalidOperation(
                "AND requires at least one argument".to_string(),
            ));
        }

        for arg in args {
            let val = self.eval(arg)?;
            if val.numeric_value() == 0.0 {
                return Ok(EvalResult::new(
                    0.0,
                    crate::core::units::Unit::dimensionless(),
                ));
            }
        }
        Ok(EvalResult::new(
            1.0,
            crate::core::units::Unit::dimensionless(),
        ))
    }

    /// Evaluate OR function
    /// OR(arg1, arg2, ...)
    fn eval_or(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        if args.is_empty() {
            return Err(EvalError::InvalidOperation(
                "OR requires at least one argument".to_string(),
            ));
        }

        for arg in args {
            let val = self.eval(arg)?;
            if val.numeric_value() != 0.0 {
                return Ok(EvalResult::new(
                    1.0,
                    crate::core::units::Unit::dimensionless(),
                ));
            }
        }
        Ok(EvalResult::new(
            0.0,
            crate::core::units::Unit::dimensionless(),
        ))
    }

    /// Evaluate NOT function
    /// NOT(value)
    fn eval_not_fn(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        if args.len() != 1 {
            return Err(EvalError::InvalidOperation(
                "NOT requires exactly 1 argument".to_string(),
            ));
        }

        let val = self.eval(&args[0])?;
        let result = if val.numeric_value() == 0.0 { 1.0 } else { 0.0 };
        Ok(EvalResult::new(
            result,
            crate::core::units::Unit::dimensionless(),
        ))
    }

    /// Evaluate GT (greater than) function
    /// GT(left, right)
    fn eval_gt(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        if args.len() != 2 {
            return Err(EvalError::InvalidOperation(
                "GT requires exactly 2 arguments: GT(left, right)".to_string(),
            ));
        }
        self.eval_comparison(&args[0], &args[1], |a, b| a > b)
    }

    /// Evaluate LT (less than) function
    /// LT(left, right)
    fn eval_lt(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        if args.len() != 2 {
            return Err(EvalError::InvalidOperation(
                "LT requires exactly 2 arguments: LT(left, right)".to_string(),
            ));
        }
        self.eval_comparison(&args[0], &args[1], |a, b| a < b)
    }

    /// Evaluate GTE (greater than or equal) function
    /// GTE(left, right)
    fn eval_gte(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        if args.len() != 2 {
            return Err(EvalError::InvalidOperation(
                "GTE requires exactly 2 arguments: GTE(left, right)".to_string(),
            ));
        }
        self.eval_comparison(&args[0], &args[1], |a, b| a >= b)
    }

    /// Evaluate LTE (less than or equal) function
    /// LTE(left, right)
    fn eval_lte(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        if args.len() != 2 {
            return Err(EvalError::InvalidOperation(
                "LTE requires exactly 2 arguments: LTE(left, right)".to_string(),
            ));
        }
        self.eval_comparison(&args[0], &args[1], |a, b| a <= b)
    }

    /// Evaluate EQ (equal) function
    /// EQ(left, right)
    fn eval_eq(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        if args.len() != 2 {
            return Err(EvalError::InvalidOperation(
                "EQ requires exactly 2 arguments: EQ(left, right)".to_string(),
            ));
        }
        self.eval_comparison(&args[0], &args[1], |a, b| a == b)
    }

    /// Evaluate NE (not equal) function
    /// NE(left, right)
    fn eval_ne(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        if args.len() != 2 {
            return Err(EvalError::InvalidOperation(
                "NE requires exactly 2 arguments: NE(left, right)".to_string(),
            ));
        }
        self.eval_comparison(&args[0], &args[1], |a, b| a != b)
    }

    /// Evaluate SQRT function
    /// Square root with dimension transformation: exponents divided by 2
    /// Example: SQRT(100 m²) = 10 m
    fn eval_sqrt(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        if args.len() != 1 {
            return Err(EvalError::InvalidOperation(
                "SQRT requires exactly 1 argument".to_string(),
            ));
        }

        let result = self.eval(&args[0])?;

        // Check for negative values
        if result.numeric_value() < 0.0 {
            return Err(EvalError::InvalidOperation(
                "SQRT of negative number is not supported (complex numbers not supported)"
                    .to_string(),
            ));
        }

        // Compute square root of value
        let sqrt_value = result.numeric_value().sqrt();

        // Transform unit by dividing exponents by 2
        use crate::core::formula::evaluator::transform_unit_exponents;
        let sqrt_unit = transform_unit_exponents(&result.unit, 0.5);

        Ok(EvalResult::new(sqrt_value, sqrt_unit))
    }

    /// Evaluate POWER function
    /// Raise to power with dimension transformation: exponents multiplied by power
    /// Example: POWER(5 m, 2) = 25 m²
    fn eval_power(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        if args.len() != 2 {
            return Err(EvalError::InvalidOperation(
                "POWER requires exactly 2 arguments: POWER(base, exponent)".to_string(),
            ));
        }

        // Evaluate base (with unit)
        let base_result = self.eval(&args[0])?;

        // Evaluate exponent (must be dimensionless)
        let exponent_result = self.eval(&args[1])?;

        // Check that exponent is dimensionless
        if !exponent_result.unit.is_dimensionless() {
            return Err(EvalError::InvalidOperation(format!(
                "Exponent must be dimensionless, got: {}",
                exponent_result.unit
            )));
        }

        let exponent = exponent_result.numeric_value();

        // Compute base^exponent
        let power_value = base_result.numeric_value().powf(exponent);

        // Special case: any unit^0 = dimensionless
        if exponent == 0.0 {
            return Ok(EvalResult::new(
                power_value,
                crate::core::units::Unit::dimensionless(),
            ));
        }

        // Transform unit by multiplying exponents by power
        use crate::core::formula::evaluator::transform_unit_exponents;
        let power_unit = transform_unit_exponents(&base_result.unit, exponent);

        Ok(EvalResult::new(power_value, power_unit))
    }

    /// Evaluate MEDIAN function
    fn eval_median(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        let values = self.collect_values(args)?;
        if values.is_empty() {
            return Err(EvalError::InvalidOperation(
                "MEDIAN requires at least one value".to_string(),
            ));
        }

        // Check unit compatibility
        let first_unit = &values[0].unit;
        let mut converted_values = Vec::new();

        for val in &values {
            if !val.unit.is_compatible(first_unit) {
                return Err(EvalError::IncompatibleUnits {
                    operation: "MEDIAN".to_string(),
                    left: first_unit.to_string(),
                    right: val.unit.to_string(),
                });
            }

            let converted = if val.unit != *first_unit {
                self.library
                    .convert(
                        val.numeric_value(),
                        val.unit.canonical(),
                        first_unit.canonical(),
                    )
                    .ok_or_else(|| EvalError::IncompatibleUnits {
                        operation: "MEDIAN".to_string(),
                        left: first_unit.to_string(),
                        right: val.unit.to_string(),
                    })?
            } else {
                val.numeric_value()
            };
            converted_values.push(converted);
        }

        // Use statrs to compute median
        let mut data = Data::new(converted_values);
        let median = data.median();

        Ok(EvalResult::new(median, first_unit.clone()))
    }

    /// Evaluate STDEV function (standard deviation)
    fn eval_stdev(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        let values = self.collect_values(args)?;
        if values.len() < 2 {
            return Err(EvalError::InvalidOperation(
                "STDEV requires at least 2 values".to_string(),
            ));
        }

        // Check compatibility and convert
        let first_unit = &values[0].unit;
        let mut converted_values = Vec::new();

        for val in &values {
            if !val.unit.is_compatible(first_unit) {
                return Err(EvalError::IncompatibleUnits {
                    operation: "STDEV".to_string(),
                    left: first_unit.to_string(),
                    right: val.unit.to_string(),
                });
            }

            let converted = if val.unit != *first_unit {
                self.library
                    .convert(
                        val.numeric_value(),
                        val.unit.canonical(),
                        first_unit.canonical(),
                    )
                    .ok_or_else(|| EvalError::IncompatibleUnits {
                        operation: "STDEV".to_string(),
                        left: first_unit.to_string(),
                        right: val.unit.to_string(),
                    })?
            } else {
                val.numeric_value()
            };
            converted_values.push(converted);
        }

        // Use statrs to compute standard deviation
        let data = Data::new(converted_values);
        let stdev = data.std_dev().ok_or_else(|| {
            EvalError::InvalidOperation("Failed to compute standard deviation".to_string())
        })?;

        Ok(EvalResult::new(stdev, first_unit.clone()))
    }

    /// Evaluate VAR function (variance)
    fn eval_var(&self, args: &[Expr]) -> Result<EvalResult, EvalError> {
        let values = self.collect_values(args)?;
        if values.len() < 2 {
            return Err(EvalError::InvalidOperation(
                "VAR requires at least 2 values".to_string(),
            ));
        }

        // Check compatibility and convert
        let first_unit = &values[0].unit;
        let mut converted_values = Vec::new();

        for val in &values {
            let converted = if !val.unit.is_equal(first_unit) {
                self.library
                    .convert(
                        val.numeric_value(),
                        val.unit.canonical(),
                        first_unit.canonical(),
                    )
                    .ok_or_else(|| EvalError::IncompatibleUnits {
                        operation: "VAR".to_string(),
                        left: first_unit.to_string(),
                        right: val.unit.to_string(),
                    })?
            } else {
                val.numeric_value()
            };
            converted_values.push(converted);
        }

        // Use statrs to compute variance
        let data = Data::new(converted_values);
        let variance = data
            .variance()
            .ok_or_else(|| EvalError::InvalidOperation("Failed to compute variance".to_string()))?;

        // Variance has squared units! Use transform_unit_exponents with factor 2
        use crate::core::formula::evaluator::transform_unit_exponents;
        let variance_unit = transform_unit_exponents(first_unit, 2.0);

        Ok(EvalResult::new(variance, variance_unit))
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
        sheet
            .set(
                a1,
                Cell::new(100.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // Set A2 = 50m
        let a2 = CellAddr::new("A", 2);
        sheet
            .set(
                a2,
                Cell::new(50.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // Evaluate A1 + A2
        let (value, unit) = sheet.evaluate_formula("=A1 + A2").unwrap();
        assert_eq!(value, CellValue::Number(150.0));
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_extract_cell_refs() {
        let expr = Expr::new_add(Expr::cell_ref("A", 1), Expr::cell_ref("B", 2));

        let refs = extract_cell_refs(&expr, None);
        assert_eq!(refs.len(), 2);
        assert!(refs.contains(&CellAddr::new("A", 1)));
        assert!(refs.contains(&CellAddr::new("B", 2)));
    }

    #[test]
    fn test_sum_function() {
        let mut sheet = Sheet::new();

        // Set up cells A1-A3
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(10.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(20.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 3),
                Cell::new(30.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // Evaluate SUM(A1:A3)
        let (value, unit) = sheet.evaluate_formula("=SUM(A1:A3)").unwrap();
        assert_eq!(value, CellValue::Number(60.0));
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_sum_with_individual_args() {
        let mut sheet = Sheet::new();

        // Set up cells
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(10.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(20.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // Evaluate SUM(A1, A2)
        let (value, unit) = sheet.evaluate_formula("=SUM(A1, A2)").unwrap();
        assert_eq!(value, CellValue::Number(30.0));
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_average_function() {
        let mut sheet = Sheet::new();

        // Set up cells A1-A4
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(10.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(20.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 3),
                Cell::new(30.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 4),
                Cell::new(40.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // Evaluate AVERAGE(A1:A4)
        let (value, unit) = sheet.evaluate_formula("=AVERAGE(A1:A4)").unwrap();
        assert_eq!(value, CellValue::Number(25.0));
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_sum_with_unit_conversion() {
        let mut sheet = Sheet::new();

        // Set up cells with different but compatible units
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(100.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(50.0, Unit::simple("cm", BaseDimension::Length)),
            )
            .unwrap();

        // Evaluate SUM(A1, A2) - should convert cm to m
        let (value, unit) = sheet.evaluate_formula("=SUM(A1, A2)").unwrap();
        assert_eq!(value, CellValue::Number(100.5)); // 100m + 0.5m
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_convert_function() {
        let mut sheet = Sheet::new();

        // Set up a cell with meters
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(1000.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // Convert meters to kilometers: CONVERT(A1, 1km)
        let (value, unit) = sheet.evaluate_formula("=CONVERT(A1, 1km)").unwrap();
        assert_eq!(value, CellValue::Number(1.0)); // 1000m = 1km
        assert_eq!(unit.canonical(), "km");

        // Convert meters to feet
        let (value, unit) = sheet.evaluate_formula("=CONVERT(A1, 1ft)").unwrap();
        if let CellValue::Number(num) = value {
            assert!((num - 3280.84).abs() < 0.1); // 1000m ≈ 3280.84ft
        } else {
            panic!("Expected numeric value");
        }
        assert_eq!(unit.canonical(), "ft");
    }

    #[test]
    fn test_convert_with_cell_reference() {
        let mut sheet = Sheet::new();

        // Set up cells
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(100.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("B", 1),
                Cell::new(1.0, Unit::simple("km", BaseDimension::Length)),
            )
            .unwrap();

        // Convert A1 to the unit of B1
        let (value, unit) = sheet.evaluate_formula("=CONVERT(A1, B1)").unwrap();
        assert_eq!(value, CellValue::Number(0.1)); // 100m = 0.1km
        assert_eq!(unit.canonical(), "km");
    }

    #[test]
    fn test_count_function() {
        let mut sheet = Sheet::new();

        // Set up cells A1-A5 (only 3 have values)
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(10.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(20.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 4),
                Cell::new(40.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        // A3 and A5 are empty

        // Evaluate COUNT(A1:A5)
        let (value, unit) = sheet.evaluate_formula("=COUNT(A1:A5)").unwrap();
        assert_eq!(value, CellValue::Number(3.0));
        assert!(unit.is_dimensionless());
    }

    #[test]
    fn test_count_with_individual_args() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(10.0, Unit::dimensionless()),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(20.0, Unit::dimensionless()),
            )
            .unwrap();

        // Evaluate COUNT(A1, A2, 5)
        let (value, unit) = sheet.evaluate_formula("=COUNT(A1, A2, 5)").unwrap();
        assert_eq!(value, CellValue::Number(3.0));
        assert!(unit.is_dimensionless());
    }

    #[test]
    fn test_min_function() {
        let mut sheet = Sheet::new();

        // Set up cells A1-A4
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(30.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(10.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 3),
                Cell::new(40.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 4),
                Cell::new(20.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // Evaluate MIN(A1:A4)
        let (value, unit) = sheet.evaluate_formula("=MIN(A1:A4)").unwrap();
        assert_eq!(value, CellValue::Number(10.0));
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_min_with_unit_conversion() {
        let mut sheet = Sheet::new();

        // Set up cells with different but compatible units
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(100.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(50.0, Unit::simple("cm", BaseDimension::Length)),
            )
            .unwrap();

        // Evaluate MIN(A1, A2) - should convert cm to m and compare
        let (value, unit) = sheet.evaluate_formula("=MIN(A1, A2)").unwrap();
        assert_eq!(value, CellValue::Number(0.5)); // 50cm = 0.5m, which is the minimum
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_max_function() {
        let mut sheet = Sheet::new();

        // Set up cells A1-A4
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(30.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(10.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 3),
                Cell::new(40.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 4),
                Cell::new(20.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // Evaluate MAX(A1:A4)
        let (value, unit) = sheet.evaluate_formula("=MAX(A1:A4)").unwrap();
        assert_eq!(value, CellValue::Number(40.0));
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_max_with_unit_conversion() {
        let mut sheet = Sheet::new();

        // Set up cells with different but compatible units
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(1.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(150.0, Unit::simple("cm", BaseDimension::Length)),
            )
            .unwrap();

        // Evaluate MAX(A1, A2) - should convert cm to m and compare
        let (value, unit) = sheet.evaluate_formula("=MAX(A1, A2)").unwrap();
        assert_eq!(value, CellValue::Number(1.5)); // 150cm = 1.5m, which is the maximum
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_abs_function() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(-42.5, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // Evaluate ABS(A1)
        let (value, unit) = sheet.evaluate_formula("=ABS(A1)").unwrap();
        assert_eq!(value, CellValue::Number(42.5));
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_abs_with_positive_value() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(42.5, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // Evaluate ABS(A1)
        let (value, unit) = sheet.evaluate_formula("=ABS(A1)").unwrap();
        assert_eq!(value, CellValue::Number(42.5));
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_round_function() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(42.567, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // Evaluate ROUND(A1)
        let (value, unit) = sheet.evaluate_formula("=ROUND(A1)").unwrap();
        assert_eq!(value, CellValue::Number(43.0));
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_round_with_decimals() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(42.567, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // Evaluate ROUND(A1, 2)
        let (value, unit) = sheet.evaluate_formula("=ROUND(A1, 2)").unwrap();
        assert_eq!(value, CellValue::Number(42.57));
        assert_eq!(unit.canonical(), "m");

        // Evaluate ROUND(A1, 1)
        let (value, unit) = sheet.evaluate_formula("=ROUND(A1, 1)").unwrap();
        assert_eq!(value, CellValue::Number(42.6));
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_floor_function() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(42.9, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // Evaluate FLOOR(A1)
        let (value, unit) = sheet.evaluate_formula("=FLOOR(A1)").unwrap();
        assert_eq!(value, CellValue::Number(42.0));
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_ceil_function() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(42.1, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // Evaluate CEIL(A1)
        let (value, unit) = sheet.evaluate_formula("=CEIL(A1)").unwrap();
        assert_eq!(value, CellValue::Number(43.0));
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_trunc_function() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(42.9, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // Evaluate TRUNC(A1)
        let (value, unit) = sheet.evaluate_formula("=TRUNC(A1)").unwrap();
        assert_eq!(value, CellValue::Number(42.0));
        assert_eq!(unit.canonical(), "m");

        // Test negative value
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(-42.9, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        let (value, unit) = sheet.evaluate_formula("=TRUNC(A2)").unwrap();
        assert_eq!(value, CellValue::Number(-42.0));
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_mod_function() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(17.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(5.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // Evaluate MOD(A1, A2)
        let (value, unit) = sheet.evaluate_formula("=MOD(A1, A2)").unwrap();
        assert_eq!(value, CellValue::Number(2.0)); // 17 % 5 = 2
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_mod_with_unit_conversion() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(100.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(30.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // Evaluate MOD(A1, A2)
        let (value, unit) = sheet.evaluate_formula("=MOD(A1, A2)").unwrap();
        assert_eq!(value, CellValue::Number(10.0)); // 100 % 30 = 10
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_sign_function() {
        let mut sheet = Sheet::new();

        // Positive value
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(42.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        let (value, unit) = sheet.evaluate_formula("=SIGN(A1)").unwrap();
        assert_eq!(value, CellValue::Number(1.0));
        assert!(unit.is_dimensionless());

        // Negative value
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(-42.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        let (value, unit) = sheet.evaluate_formula("=SIGN(A2)").unwrap();
        assert_eq!(value, CellValue::Number(-1.0));
        assert!(unit.is_dimensionless());

        // Zero
        sheet
            .set(
                CellAddr::new("A", 3),
                Cell::new(0.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        let (value, unit) = sheet.evaluate_formula("=SIGN(A3)").unwrap();
        assert_eq!(value, CellValue::Number(0.0));
        assert!(unit.is_dimensionless());
    }

    #[test]
    fn test_min_incompatible_units() {
        let mut sheet = Sheet::new();

        // Set up cells with incompatible units
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(100.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(50.0, Unit::simple("kg", BaseDimension::Mass)),
            )
            .unwrap();

        // Evaluate MIN(A1, A2) - should error
        let result = sheet.evaluate_formula("=MIN(A1, A2)");
        assert!(result.is_err());
    }

    #[test]
    fn test_max_incompatible_units() {
        let mut sheet = Sheet::new();

        // Set up cells with incompatible units
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(100.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(50.0, Unit::simple("kg", BaseDimension::Mass)),
            )
            .unwrap();

        // Evaluate MAX(A1, A2) - should error
        let result = sheet.evaluate_formula("=MAX(A1, A2)");
        assert!(result.is_err());
    }

    #[test]
    fn test_mod_incompatible_units() {
        let mut sheet = Sheet::new();

        // Set up cells with incompatible units
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(100.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(50.0, Unit::simple("kg", BaseDimension::Mass)),
            )
            .unwrap();

        // Evaluate MOD(A1, A2) - should error
        let result = sheet.evaluate_formula("=MOD(A1, A2)");
        assert!(result.is_err());
    }

    #[test]
    fn test_sqrt_dimensionless() {
        let sheet = Sheet::new();
        // SQRT(4) = 2 (dimensionless)
        let (value, unit) = sheet.evaluate_formula("=SQRT(4)").unwrap();
        assert_eq!(value, CellValue::Number(2.0));
        assert!(unit.is_dimensionless());
    }

    #[test]
    fn test_sqrt_area_to_length() {
        let mut sheet = Sheet::new();
        // Create a compound unit for area: m²
        let area_unit = Unit::compound("m²", vec![(BaseDimension::Length, 2)], vec![]);

        sheet
            .set(CellAddr::new("A", 1), Cell::new(100.0, area_unit))
            .unwrap();

        // SQRT(100 m²) = 10 m
        let (value, unit) = sheet.evaluate_formula("=SQRT(A1)").unwrap();
        assert_eq!(value, CellValue::Number(10.0));
        // Result should be length with exponent 1
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_sqrt_compound_units() {
        let mut sheet = Sheet::new();
        // Create m²/s²
        let compound_unit = Unit::compound(
            "m²/s²",
            vec![(BaseDimension::Length, 2)],
            vec![(BaseDimension::Time, 2)],
        );

        sheet
            .set(CellAddr::new("A", 1), Cell::new(100.0, compound_unit))
            .unwrap();

        // SQRT(100 m²/s²) = 10 m/s
        let (value, unit) = sheet.evaluate_formula("=SQRT(A1)").unwrap();
        assert_eq!(value, CellValue::Number(10.0));
        // Result should be m/s
        assert_eq!(unit.canonical(), "m/s");
    }

    #[test]
    fn test_sqrt_fractional_exponent() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(100.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // SQRT(100 m) = 10 m^0.5 (fractional exponent)
        let (value, unit) = sheet.evaluate_formula("=SQRT(A1)").unwrap();
        assert_eq!(value, CellValue::Number(10.0));
        // Result should have fractional exponent (m^0.5)
        // The unit system will represent this as a compound unit
        assert!(!unit.is_dimensionless());
    }

    #[test]
    fn test_sqrt_negative_error() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(-4.0, Unit::dimensionless()),
            )
            .unwrap();

        // SQRT(-4) should error
        let result = sheet.evaluate_formula("=SQRT(A1)");
        assert!(result.is_err());
    }

    #[test]
    fn test_power_integer_exponent() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(5.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // POWER(5 m, 2) = 25 m²
        let (value, unit) = sheet.evaluate_formula("=POWER(A1, 2)").unwrap();
        assert_eq!(value, CellValue::Number(25.0));
        // Result should be m² (or m^2 depending on canonical form)
        assert!(unit.canonical() == "m²" || unit.canonical() == "m^2");
    }

    #[test]
    fn test_power_cube() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(2.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // POWER(2 m, 3) = 8 m³
        let (value, unit) = sheet.evaluate_formula("=POWER(A1, 3)").unwrap();
        assert_eq!(value, CellValue::Number(8.0));
        // Result should be m³ (or m^3 depending on canonical form)
        assert!(unit.canonical() == "m³" || unit.canonical() == "m^3");
    }

    #[test]
    fn test_power_fractional_exponent() {
        let mut sheet = Sheet::new();
        // Create m³
        let volume_unit = Unit::compound("m³", vec![(BaseDimension::Length, 3)], vec![]);

        sheet
            .set(CellAddr::new("A", 1), Cell::new(8.0, volume_unit))
            .unwrap();

        // POWER(8 m³, 1/3) = 2 m (cube root)
        // Note: 1/3 evaluates to approximately 0.333...
        let (value, unit) = sheet.evaluate_formula("=POWER(A1, 0.333333333)").unwrap();
        if let CellValue::Number(num) = value {
            assert!((num - 2.0).abs() < 0.001); // Allow small floating point error
        } else {
            panic!("Expected numeric value");
        }
        // Result should be m
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_power_negative_exponent() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(2.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // POWER(2 m, -1) = 0.5 m⁻¹
        let (value, unit) = sheet.evaluate_formula("=POWER(A1, -1)").unwrap();
        assert_eq!(value, CellValue::Number(0.5));
        // Result should have negative exponent (m⁻¹ or 1/m)
        assert!(!unit.is_dimensionless());
    }

    #[test]
    fn test_power_zero_exponent() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(5.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // POWER(5 m, 0) = 1 (dimensionless, any unit^0 = 1)
        let (value, unit) = sheet.evaluate_formula("=POWER(A1, 0)").unwrap();
        assert_eq!(value, CellValue::Number(1.0));
        assert!(unit.is_dimensionless());
    }

    #[test]
    fn test_power_dimensionless() {
        let sheet = Sheet::new();
        // POWER(2, 3) = 8
        let (value, unit) = sheet.evaluate_formula("=POWER(2, 3)").unwrap();
        assert_eq!(value, CellValue::Number(8.0));
        assert!(unit.is_dimensionless());
    }

    #[test]
    fn test_power_compound_units() {
        let mut sheet = Sheet::new();
        // Create kg/s
        let compound_unit = Unit::compound(
            "kg/s",
            vec![(BaseDimension::Mass, 1)],
            vec![(BaseDimension::Time, 1)],
        );

        sheet
            .set(CellAddr::new("A", 1), Cell::new(2.0, compound_unit))
            .unwrap();

        // POWER(2 kg/s, 3) = 8 kg³/s³
        let (value, unit) = sheet.evaluate_formula("=POWER(A1, 3)").unwrap();
        assert_eq!(value, CellValue::Number(8.0));
        // Result should be kg³/s³ (or kg^3/s^3 depending on canonical form)
        assert!(unit.canonical() == "kg³/s³" || unit.canonical() == "kg^3/s^3");
    }

    #[test]
    fn test_power_exponent_must_be_dimensionless() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(2.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("B", 1),
                Cell::new(2.0, Unit::simple("s", BaseDimension::Time)),
            )
            .unwrap();

        // POWER(2 m, 2 s) should error - exponent must be dimensionless
        let result = sheet.evaluate_formula("=POWER(A1, B1)");
        assert!(result.is_err());
    }

    #[test]
    fn test_sqrt_as_power_half() {
        let mut sheet = Sheet::new();
        // Create m²
        let area_unit = Unit::compound("m²", vec![(BaseDimension::Length, 2)], vec![]);

        sheet
            .set(CellAddr::new("A", 1), Cell::new(100.0, area_unit))
            .unwrap();

        // SQRT(100 m²) should equal POWER(100 m², 0.5)
        let (sqrt_value, sqrt_unit) = sheet.evaluate_formula("=SQRT(A1)").unwrap();
        let (power_value, power_unit) = sheet.evaluate_formula("=POWER(A1, 0.5)").unwrap();

        assert_eq!(sqrt_value, power_value);
        assert_eq!(sqrt_unit.canonical(), power_unit.canonical());
    }

    // ========== Comparison and Logic Function Tests ==========

    #[test]
    fn test_gt_function_same_units() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(10.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(5.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // 10m > 5m → TRUE (1.0)
        let (value, unit) = sheet.evaluate_formula("=GT(A1, A2)").unwrap();
        assert_eq!(value, CellValue::Number(1.0));
        assert!(unit.is_dimensionless());

        // 5m > 10m → FALSE (0.0)
        let (value, unit) = sheet.evaluate_formula("=GT(A2, A1)").unwrap();
        assert_eq!(value, CellValue::Number(0.0));
        assert!(unit.is_dimensionless());
    }

    #[test]
    fn test_lt_function_same_units() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(10.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(5.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // 5m < 10m → TRUE (1.0)
        let (value, unit) = sheet.evaluate_formula("=LT(A2, A1)").unwrap();
        assert_eq!(value, CellValue::Number(1.0));
        assert!(unit.is_dimensionless());

        // 10m < 5m → FALSE (0.0)
        let (value, unit) = sheet.evaluate_formula("=LT(A1, A2)").unwrap();
        assert_eq!(value, CellValue::Number(0.0));
        assert!(unit.is_dimensionless());
    }

    #[test]
    fn test_gte_function() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(10.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(10.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 3),
                Cell::new(5.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // 10m >= 10m → TRUE (1.0)
        let (value, _) = sheet.evaluate_formula("=GTE(A1, A2)").unwrap();
        assert_eq!(value, CellValue::Number(1.0));

        // 10m >= 5m → TRUE (1.0)
        let (value, _) = sheet.evaluate_formula("=GTE(A1, A3)").unwrap();
        assert_eq!(value, CellValue::Number(1.0));

        // 5m >= 10m → FALSE (0.0)
        let (value, _) = sheet.evaluate_formula("=GTE(A3, A1)").unwrap();
        assert_eq!(value, CellValue::Number(0.0));
    }

    #[test]
    fn test_lte_function() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(10.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(10.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 3),
                Cell::new(15.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // 10m <= 10m → TRUE (1.0)
        let (value, _) = sheet.evaluate_formula("=LTE(A1, A2)").unwrap();
        assert_eq!(value, CellValue::Number(1.0));

        // 10m <= 15m → TRUE (1.0)
        let (value, _) = sheet.evaluate_formula("=LTE(A1, A3)").unwrap();
        assert_eq!(value, CellValue::Number(1.0));

        // 15m <= 10m → FALSE (0.0)
        let (value, _) = sheet.evaluate_formula("=LTE(A3, A1)").unwrap();
        assert_eq!(value, CellValue::Number(0.0));
    }

    #[test]
    fn test_eq_function() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(10.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(10.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 3),
                Cell::new(5.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // 10m == 10m → TRUE (1.0)
        let (value, _) = sheet.evaluate_formula("=EQ(A1, A2)").unwrap();
        assert_eq!(value, CellValue::Number(1.0));

        // 10m == 5m → FALSE (0.0)
        let (value, _) = sheet.evaluate_formula("=EQ(A1, A3)").unwrap();
        assert_eq!(value, CellValue::Number(0.0));
    }

    #[test]
    fn test_ne_function() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(10.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(10.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 3),
                Cell::new(5.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // 10m != 5m → TRUE (1.0)
        let (value, _) = sheet.evaluate_formula("=NE(A1, A3)").unwrap();
        assert_eq!(value, CellValue::Number(1.0));

        // 10m != 10m → FALSE (0.0)
        let (value, _) = sheet.evaluate_formula("=NE(A1, A2)").unwrap();
        assert_eq!(value, CellValue::Number(0.0));
    }

    #[test]
    fn test_gt_with_unit_conversion() {
        let mut sheet = Sheet::new();

        // 1000m vs 1km
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(1000.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(1.0, Unit::simple("km", BaseDimension::Length)),
            )
            .unwrap();

        // 1000m > 1km → FALSE (1000m == 1km)
        let (value, _) = sheet.evaluate_formula("=GT(A1, A2)").unwrap();
        assert_eq!(value, CellValue::Number(0.0));

        // 50cm vs 1m
        sheet
            .set(
                CellAddr::new("B", 1),
                Cell::new(50.0, Unit::simple("cm", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("B", 2),
                Cell::new(1.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // 50cm < 1m → TRUE
        let (value, _) = sheet.evaluate_formula("=LT(B1, B2)").unwrap();
        assert_eq!(value, CellValue::Number(1.0));
    }

    #[test]
    fn test_comparison_incompatible_units() {
        let mut sheet = Sheet::new();

        // 10m vs 5s (incompatible)
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(10.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(5.0, Unit::simple("s", BaseDimension::Time)),
            )
            .unwrap();

        // GT(10m, 5s) → Error
        let result = sheet.evaluate_formula("=GT(A1, A2)");
        assert!(result.is_err());

        // LT(10m, 5s) → Error
        let result = sheet.evaluate_formula("=LT(A1, A2)");
        assert!(result.is_err());

        // EQ(10m, 5s) → Error
        let result = sheet.evaluate_formula("=EQ(A1, A2)");
        assert!(result.is_err());
    }

    #[test]
    fn test_if_function_true_branch() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(100.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(200.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // IF(1, 100m, 200m) → 100m
        let (value, unit) = sheet.evaluate_formula("=IF(1, A1, A2)").unwrap();
        assert_eq!(value, CellValue::Number(100.0));
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_if_function_false_branch() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(100.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(200.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // IF(0, 100m, 200m) → 200m
        let (value, unit) = sheet.evaluate_formula("=IF(0, A1, A2)").unwrap();
        assert_eq!(value, CellValue::Number(200.0));
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_if_with_comparison() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(10.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(5.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 3),
                Cell::new(100.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 4),
                Cell::new(200.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // IF(GT(A1, A2), A3, A4) → IF(10m > 5m, 100m, 200m) → 100m
        let (value, unit) = sheet.evaluate_formula("=IF(GT(A1, A2), A3, A4)").unwrap();
        assert_eq!(value, CellValue::Number(100.0));
        assert_eq!(unit.canonical(), "m");

        // IF(LT(A1, A2), A3, A4) → IF(10m < 5m, 100m, 200m) → 200m
        let (value, unit) = sheet.evaluate_formula("=IF(LT(A1, A2), A3, A4)").unwrap();
        assert_eq!(value, CellValue::Number(200.0));
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_and_function_all_true() {
        let sheet = Sheet::new();

        // AND(1, 1, 1) → 1.0
        let (value, unit) = sheet.evaluate_formula("=AND(1, 1, 1)").unwrap();
        assert_eq!(value, CellValue::Number(1.0));
        assert!(unit.is_dimensionless());
    }

    #[test]
    fn test_and_function_one_false() {
        let sheet = Sheet::new();

        // AND(1, 0, 1) → 0.0
        let (value, unit) = sheet.evaluate_formula("=AND(1, 0, 1)").unwrap();
        assert_eq!(value, CellValue::Number(0.0));
        assert!(unit.is_dimensionless());
    }

    #[test]
    fn test_and_function_all_false() {
        let sheet = Sheet::new();

        // AND(0, 0, 0) → 0.0
        let (value, unit) = sheet.evaluate_formula("=AND(0, 0, 0)").unwrap();
        assert_eq!(value, CellValue::Number(0.0));
        assert!(unit.is_dimensionless());
    }

    #[test]
    fn test_or_function_all_true() {
        let sheet = Sheet::new();

        // OR(1, 1, 1) → 1.0
        let (value, unit) = sheet.evaluate_formula("=OR(1, 1, 1)").unwrap();
        assert_eq!(value, CellValue::Number(1.0));
        assert!(unit.is_dimensionless());
    }

    #[test]
    fn test_or_function_one_true() {
        let sheet = Sheet::new();

        // OR(0, 1, 0) → 1.0
        let (value, unit) = sheet.evaluate_formula("=OR(0, 1, 0)").unwrap();
        assert_eq!(value, CellValue::Number(1.0));
        assert!(unit.is_dimensionless());
    }

    #[test]
    fn test_or_function_all_false() {
        let sheet = Sheet::new();

        // OR(0, 0, 0) → 0.0
        let (value, unit) = sheet.evaluate_formula("=OR(0, 0, 0)").unwrap();
        assert_eq!(value, CellValue::Number(0.0));
        assert!(unit.is_dimensionless());
    }

    #[test]
    fn test_not_function_true() {
        let sheet = Sheet::new();

        // NOT(1) → 0.0
        let (value, unit) = sheet.evaluate_formula("=NOT(1)").unwrap();
        assert_eq!(value, CellValue::Number(0.0));
        assert!(unit.is_dimensionless());
    }

    #[test]
    fn test_not_function_false() {
        let sheet = Sheet::new();

        // NOT(0) → 1.0
        let (value, unit) = sheet.evaluate_formula("=NOT(0)").unwrap();
        assert_eq!(value, CellValue::Number(1.0));
        assert!(unit.is_dimensionless());
    }

    #[test]
    fn test_complex_logical_expression() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(10.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(5.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 3),
                Cell::new(15.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // AND(GT(A1, A2), LT(A1, A3)) → AND(10 > 5, 10 < 15) → AND(1, 1) → 1.0
        let (value, _) = sheet
            .evaluate_formula("=AND(GT(A1, A2), LT(A1, A3))")
            .unwrap();
        assert_eq!(value, CellValue::Number(1.0));

        // OR(GT(A1, A3), LT(A2, A1)) → OR(10 > 15, 5 < 10) → OR(0, 1) → 1.0
        let (value, _) = sheet
            .evaluate_formula("=OR(GT(A1, A3), LT(A2, A1))")
            .unwrap();
        assert_eq!(value, CellValue::Number(1.0));

        // IF(AND(GT(A1, A2), LT(A1, A3)), A1, A3) → IF(1, 10m, 15m) → 10m
        let (value, unit) = sheet
            .evaluate_formula("=IF(AND(GT(A1, A2), LT(A1, A3)), A1, A3)")
            .unwrap();
        assert_eq!(value, CellValue::Number(10.0));
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_nested_if() {
        let mut sheet = Sheet::new();

        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(10.0, Unit::dimensionless()),
            )
            .unwrap();

        // IF(GT(A1, 5), IF(GT(A1, 15), 100, 50), 0)
        // A1=10 → GT(10, 5)=1 → GT(10, 15)=0 → 50
        let (value, _) = sheet
            .evaluate_formula("=IF(GT(A1, 5), IF(GT(A1, 15), 100, 50), 0)")
            .unwrap();
        assert_eq!(value, CellValue::Number(50.0));
    }

    #[test]
    fn test_median_function() {
        let sheet = Sheet::new();

        // MEDIAN(1, 2, 3, 4, 5) = 3
        let (value, unit) = sheet.evaluate_formula("=MEDIAN(1, 2, 3, 4, 5)").unwrap();

        assert_eq!(value, CellValue::Number(3.0));
        assert!(unit.is_dimensionless());
    }

    #[test]
    fn test_median_with_units() {
        let mut sheet = Sheet::new();

        // Set up cells with units
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(100.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(200.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 3),
                Cell::new(300.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        // MEDIAN(A1:A3) = 200m
        let (value, unit) = sheet.evaluate_formula("=MEDIAN(A1:A3)").unwrap();

        assert_eq!(value, CellValue::Number(200.0));
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_median_with_conversion() {
        let mut sheet = Sheet::new();

        // 1m, 200cm, 3m → should convert 200cm to 2m → median is 2m
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(1.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(200.0, Unit::simple("cm", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 3),
                Cell::new(3.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        let (value, unit) = sheet.evaluate_formula("=MEDIAN(A1:A3)").unwrap();

        assert_eq!(value, CellValue::Number(2.0));
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_median_even_count() {
        let sheet = Sheet::new();

        // MEDIAN(1, 2, 3, 4) = 2.5
        let (value, _) = sheet.evaluate_formula("=MEDIAN(1, 2, 3, 4)").unwrap();

        assert_eq!(value, CellValue::Number(2.5));
    }

    #[test]
    fn test_stdev_function() {
        let sheet = Sheet::new();

        // Test standard deviation with known values
        // For [2, 4, 4, 4, 5, 5, 7, 9], sample stdev (n-1) ≈ 2.138
        let (value, _) = sheet
            .evaluate_formula("=STDEV(2, 4, 4, 4, 5, 5, 7, 9)")
            .unwrap();

        // Standard deviation should be approximately 2.138
        if let CellValue::Number(num) = value {
            assert!((num - 2.138).abs() < 0.01);
        } else {
            panic!("Expected numeric value");
        }
    }

    #[test]
    fn test_stdev_with_units() {
        let mut sheet = Sheet::new();

        // Set up cells with units
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(100.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(200.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 3),
                Cell::new(300.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        let (value, unit) = sheet.evaluate_formula("=STDEV(A1:A3)").unwrap();

        // Standard deviation of [100, 200, 300] ≈ 100
        if let CellValue::Number(num) = value {
            assert!((num - 100.0).abs() < 1.0);
        } else {
            panic!("Expected numeric value");
        }
        assert_eq!(unit.canonical(), "m");
    }

    #[test]
    fn test_var_function() {
        let sheet = Sheet::new();

        // Variance of [2, 4, 4, 4, 5, 5, 7, 9], sample variance (n-1) ≈ 4.571
        let (value, _) = sheet
            .evaluate_formula("=VAR(2, 4, 4, 4, 5, 5, 7, 9)")
            .unwrap();

        // Variance should be approximately 4.571
        if let CellValue::Number(num) = value {
            assert!((num - 4.571).abs() < 0.01);
        } else {
            panic!("Expected numeric value");
        }
    }

    #[test]
    fn test_var_squared_units() {
        let mut sheet = Sheet::new();

        // Variance of meters should have m² units
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(1.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(2.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 3),
                Cell::new(3.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();

        let (value, unit) = sheet.evaluate_formula("=VAR(A1:A3)").unwrap();

        // Sample variance (n-1) of [1, 2, 3] = 1.0
        if let CellValue::Number(num) = value {
            assert!((num - 1.0).abs() < 0.01);
        } else {
            panic!("Expected numeric value");
        }

        // Unit should be m² (squared)
        let canonical = unit.canonical();
        assert!(
            canonical.contains("m")
                && (canonical.contains("²") || canonical.contains("^2") || canonical.contains("2"))
        );
    }

    #[test]
    fn test_var_with_units() {
        let mut sheet = Sheet::new();

        // Set up cells with units
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(10.0, Unit::simple("kg", BaseDimension::Mass)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(20.0, Unit::simple("kg", BaseDimension::Mass)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 3),
                Cell::new(30.0, Unit::simple("kg", BaseDimension::Mass)),
            )
            .unwrap();

        let (value, unit) = sheet.evaluate_formula("=VAR(A1:A3)").unwrap();

        // Sample variance (n-1) of [10, 20, 30] = 100.0
        if let CellValue::Number(num) = value {
            assert!((num - 100.0).abs() < 0.1);
        } else {
            panic!("Expected numeric value");
        }

        // Unit should be kg² (squared)
        let canonical = unit.canonical();
        assert!(canonical.contains("kg"));
    }

    #[test]
    fn test_median_error_empty() {
        let sheet = Sheet::new();

        // MEDIAN with no arguments should fail
        let result = sheet.evaluate_formula("=MEDIAN()");
        assert!(result.is_err());
    }

    #[test]
    fn test_stdev_error_insufficient_values() {
        let sheet = Sheet::new();

        // STDEV requires at least 2 values
        let result = sheet.evaluate_formula("=STDEV(5)");
        assert!(result.is_err());
    }

    #[test]
    fn test_var_error_insufficient_values() {
        let sheet = Sheet::new();

        // VAR requires at least 2 values
        let result = sheet.evaluate_formula("=VAR(5)");
        assert!(result.is_err());
    }

    #[test]
    fn test_median_incompatible_units() {
        let mut sheet = Sheet::new();

        // Mix meters and kilograms (incompatible)
        sheet
            .set(
                CellAddr::new("A", 1),
                Cell::new(100.0, Unit::simple("m", BaseDimension::Length)),
            )
            .unwrap();
        sheet
            .set(
                CellAddr::new("A", 2),
                Cell::new(200.0, Unit::simple("kg", BaseDimension::Mass)),
            )
            .unwrap();

        let result = sheet.evaluate_formula("=MEDIAN(A1:A2)");
        assert!(result.is_err());
    }
}

/// Convert compound unit values for formula evaluation (e.g., $/quarter -> $/year)
/// This is similar to the convert_compound_unit in commands/workbook.rs but
/// adapted for use in formula evaluation
fn convert_compound_unit_for_formula(
    value: f64,
    from_unit: &str,
    to_unit: &str,
    library: &UnitLibrary,
) -> Option<f64> {
    // Handle division (e.g., $/quarter -> $/year)
    if let (Some(from_pos), Some(to_pos)) = (from_unit.find('/'), to_unit.find('/')) {
        let from_left = &from_unit[..from_pos];
        let from_right = &from_unit[from_pos + 1..];
        let to_left = &to_unit[..to_pos];
        let to_right = &to_unit[to_pos + 1..];

        // Convert numerator (e.g., $ -> $, both are Currency)
        let factor_left = if from_left == to_left {
            1.0
        } else {
            library.convert(1.0, from_left, to_left)?
        };

        // Convert denominator (e.g., quarter -> year)
        let factor_right = library.convert(1.0, from_right, to_right)?;

        // For division, divide the factors
        // Example: $/quarter -> $/year
        // factor_left = 1.0 ($ -> $)
        // factor_right = 0.25 (quarter -> year, since 1 quarter = 0.25 years)
        // combined_factor = 1.0 / 0.25 = 4.0... wait, that's wrong
        //
        // Actually, let me think about this:
        // 30000 $/quarter means $30000 per quarter
        // To convert to $/year: how many $ per year?
        // If you earn $30000 per quarter, you earn $30000 * 4 per year = $120000/year
        //
        // So the conversion is: value * (quarters per year) = value * 4
        // But factor_right is quarter->year = 0.25 (1 quarter = 0.25 years)
        // So we need: value / factor_right = value / 0.25 = value * 4
        //
        // Let me reconsider:
        // $/quarter -> $/year means ($ / quarter) -> ($ / year)
        // = $ * (1 / quarter) -> $ * (1 / year)
        // = $ * (year / quarter)
        // year/quarter = 1/0.25 = 4
        //
        // So: numerator_factor / denominator_factor
        let combined_factor = factor_left / factor_right;
        return Some(value * combined_factor);
    }

    // Handle power notation (e.g., ft^2 -> m^2)
    if let (Some(from_pos), Some(to_pos)) = (from_unit.find('^'), to_unit.find('^')) {
        let from_base = &from_unit[..from_pos];
        let from_power_str = &from_unit[from_pos + 1..];
        let to_base = &to_unit[..to_pos];
        let to_power_str = &to_unit[to_pos + 1..];

        // Parse the power
        if let (Ok(from_power), Ok(to_power)) =
            (from_power_str.parse::<i32>(), to_power_str.parse::<i32>())
        {
            if from_power == to_power {
                // Get conversion factor for base unit
                if let Some(base_factor) = library.convert(1.0, from_base, to_base) {
                    // Raise to the power
                    let combined_factor = base_factor.powi(from_power);
                    return Some(value * combined_factor);
                }
            }
        }
    }

    None
}
