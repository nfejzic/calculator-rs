pub fn is_number(ch: char) -> bool {
    ('0'..='9').contains(&ch)
}

pub fn is_binary_arithmetic_symbol(ch: char) -> bool {
    ch == '+' || ch == '-' || ch == '*' || ch == '/' || ch == '%'
}

pub fn is_parentheses(ch: char) -> bool {
    ch == '(' || ch == ')'
}

pub fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\n' || ch == '\t'
}
