// Cell input parser for handling inline label definitions
//
// Supports two syntaxes:
// - `label: value` - defines a named range with a literal value (e.g., "tax_rate: 0.15")
// - `label:= formula` - defines a named range with a formula (e.g., "total:= A1+A2")

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CellInputError {
    #[error("Invalid label name: {0}")]
    InvalidLabelName(String),

    #[error("Parse error: {0}")]
    ParseError(String),
}

/// Result of parsing cell input
#[derive(Debug, Clone, PartialEq)]
pub enum CellInput {
    /// Plain value or formula (no label)
    Plain(String),

    /// Labeled value: name and the value/formula
    Labeled {
        label: String,
        content: String,
        is_formula: bool,
    },
}

/// Parse cell input to detect inline label definitions
///
/// Examples:
/// - "100" -> Plain("100")
/// - "=A1+A2" -> Plain("=A1+A2")
/// - "tax_rate: 0.15" -> Labeled { label: "tax_rate", content: "0.15", is_formula: false }
/// - "total:= A1+A2" -> Labeled { label: "total", content: "=A1+A2", is_formula: true }
pub fn parse_cell_input(input: &str) -> Result<CellInput, CellInputError> {
    let input = input.trim();

    // Check for label syntax: "label: value" or "label:= formula"
    if let Some((label_part, content_part)) = split_label(input) {
        let label = label_part.trim();
        let content = content_part.trim();

        // Validate label name
        if !is_valid_label(label) {
            return Err(CellInputError::InvalidLabelName(label.to_string()));
        }

        // Check if it's a formula (using :=)
        let is_formula = input.contains(":=");

        // If formula syntax, prepend = if not already present
        let final_content = if is_formula && !content.starts_with('=') {
            format!("={}", content)
        } else {
            content.to_string()
        };

        return Ok(CellInput::Labeled {
            label: label.to_string(),
            content: final_content,
            is_formula,
        });
    }

    // No label, return plain input
    Ok(CellInput::Plain(input.to_string()))
}

/// Split input into label and content if it matches label syntax
fn split_label(input: &str) -> Option<(String, String)> {
    // Try := first (formula syntax)
    if let Some(pos) = input.find(":=") {
        let label = input[..pos].to_string();
        let content = input[pos + 2..].to_string();
        return Some((label, content));
    }

    // Try : (value syntax)
    // Must be careful not to match time values like "12:30"
    if let Some(pos) = input.find(':') {
        let label = &input[..pos];

        // Only treat as label if it starts with a letter or underscore
        // (not a digit, which would indicate time like "12:30")
        if let Some(first_char) = label.chars().next() {
            if first_char.is_ascii_alphabetic() || first_char == '_' {
                let content = input[pos + 1..].to_string();
                return Some((label.to_string(), content));
            }
        }
    }

    None
}

/// Check if a label name is valid
/// Must start with lowercase letter or underscore, contain only alphanumerics and underscores
fn is_valid_label(label: &str) -> bool {
    if label.is_empty() {
        return false;
    }

    let mut chars = label.chars();
    let first = chars.next().unwrap();

    // Must start with lowercase letter or underscore
    if !first.is_ascii_lowercase() && first != '_' {
        return false;
    }

    // Rest must be alphanumeric or underscore
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_value() {
        let result = parse_cell_input("100").unwrap();
        assert_eq!(result, CellInput::Plain("100".to_string()));
    }

    #[test]
    fn test_plain_formula() {
        let result = parse_cell_input("=A1+A2").unwrap();
        assert_eq!(result, CellInput::Plain("=A1+A2".to_string()));
    }

    #[test]
    fn test_labeled_value() {
        let result = parse_cell_input("tax_rate: 0.15").unwrap();
        match result {
            CellInput::Labeled { label, content, is_formula } => {
                assert_eq!(label, "tax_rate");
                assert_eq!(content, "0.15");
                assert!(!is_formula);
            }
            _ => panic!("Expected Labeled"),
        }
    }

    #[test]
    fn test_labeled_formula() {
        let result = parse_cell_input("total:= A1+A2").unwrap();
        match result {
            CellInput::Labeled { label, content, is_formula } => {
                assert_eq!(label, "total");
                assert_eq!(content, "=A1+A2");
                assert!(is_formula);
            }
            _ => panic!("Expected Labeled"),
        }
    }

    #[test]
    fn test_labeled_formula_with_equals() {
        let result = parse_cell_input("total:= =A1+A2").unwrap();
        match result {
            CellInput::Labeled { label, content, is_formula } => {
                assert_eq!(label, "total");
                assert_eq!(content, "=A1+A2");
                assert!(is_formula);
            }
            _ => panic!("Expected Labeled"),
        }
    }

    #[test]
    fn test_labeled_with_currency() {
        let result = parse_cell_input("price: $15").unwrap();
        match result {
            CellInput::Labeled { label, content, is_formula } => {
                assert_eq!(label, "price");
                assert_eq!(content, "$15");
                assert!(!is_formula);
            }
            _ => panic!("Expected Labeled"),
        }
    }

    #[test]
    fn test_invalid_label_uppercase() {
        let result = parse_cell_input("Price: $15");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_label_starts_with_number() {
        // Labels starting with numbers are not recognized as labels, treated as plain
        let result = parse_cell_input("1price: $15").unwrap();
        assert_eq!(result, CellInput::Plain("1price: $15".to_string()));
    }

    #[test]
    fn test_time_not_treated_as_label() {
        // Times like "12:30" should be treated as plain values
        let result = parse_cell_input("12:30").unwrap();
        assert_eq!(result, CellInput::Plain("12:30".to_string()));
    }

    #[test]
    fn test_label_with_underscore() {
        let result = parse_cell_input("_private: 42").unwrap();
        match result {
            CellInput::Labeled { label, .. } => {
                assert_eq!(label, "_private");
            }
            _ => panic!("Expected Labeled"),
        }
    }

    #[test]
    fn test_label_with_numbers() {
        let result = parse_cell_input("value123: 42").unwrap();
        match result {
            CellInput::Labeled { label, .. } => {
                assert_eq!(label, "value123");
            }
            _ => panic!("Expected Labeled"),
        }
    }

    #[test]
    fn test_whitespace_handling() {
        let result = parse_cell_input("  tax_rate  :  0.15  ").unwrap();
        match result {
            CellInput::Labeled { label, content, .. } => {
                assert_eq!(label, "tax_rate");
                assert_eq!(content, "0.15");
            }
            _ => panic!("Expected Labeled"),
        }
    }
}
