/// Checks if given number is a decimal digit.
/// Returns true if so.
/// # Arguments
/// * `ch` - a character to check
pub fn is_number(ch: char) -> bool {
    ch.is_digit(10)
}

/// Checks if given number is a valid arithmetic operation
/// Returns true if so.
/// # Arguments
/// * `ch` - a character to check
pub fn is_binary_arithmetic_symbol(ch: char) -> bool {
    ch == '+' || ch == '-' || ch == '*' || ch == '/' || ch == '%'
}

/// Checks if given number is a open or close parentheses
/// Returns true if so.
/// # Arguments
/// * `ch` - a character to check
pub fn is_parentheses(ch: char) -> bool {
    ch == '(' || ch == ')'
}

/// Checks if given number is a valid character in an expression
/// Returns true if so.
/// # Arguments
/// * `ch` - a character to check
pub fn is_valid_character(ch: char) -> bool {
    ch.is_digit(10)
        || is_parentheses(ch)
        || is_binary_arithmetic_symbol(ch)
        || ch.is_whitespace()
        || ch == '.'
}
