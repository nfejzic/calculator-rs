use super::checker::*;
use super::parsing_error::*;

type CalcRes = (f64, usize);

#[derive(PartialEq, Eq)]
enum Operation {
    Add,
    Subt,
    Mult,
    Div,
    Mod,
}

pub fn parse_expression(exp: &str) -> Result<f64, ParsingError> {
    let no_whitespaces = remove_whitespaces(exp)?;

    Ok(calculate(&no_whitespaces, 0)?.0)
}

fn remove_whitespaces(text: &str) -> Result<String, ParsingError> {
    let mut res: String = "".into();
    for (i, ch) in text.chars().enumerate() {
        if is_number(ch) || is_binary_arithmetic_symbol(ch) || is_parentheses(ch) || ch == '.' {
            res = res + &ch.to_string();
        } else if is_whitespace(ch) {
            continue;
        } else {
            // Syntax error!
            return Err(ParsingError {
                text: String::from(text),
                index: i,
            });
        }
    }

    Ok(res)
}

fn calculate(text: &str, index: usize) -> Result<CalcRes, ParsingError> {
    let (mut res, mut i) = calculate_term(&text, index)?;

    while i < text.len() {
        let operation;

        if let Some(ch) = text.chars().nth(i) {
            if ch == '+' {
                operation = Operation::Add;
            } else if ch == '-' {
                operation = Operation::Subt;
            } else {
                break;
            }

            i += 1;
            let (new_val, new_index) = calculate_term(&text, i)?;

            i = new_index;
            res = if operation == Operation::Add {
                res + new_val
            } else {
                res - new_val
            };
        }
    }

    Ok((res, i))
}

fn calculate_term(text: &str, index: usize) -> Result<CalcRes, ParsingError> {
    let (mut res, mut i) = calculate_factor(&text, index)?;

    while i < text.len() {
        let operation;

        if let Some(ch) = text.chars().nth(i) {
            if ch == '*' {
                operation = Operation::Mult;
            } else if ch == '/' {
                operation = Operation::Div;
            } else if ch == '%' {
                operation = Operation::Mod;
            } else {
                break;
            }

            i += 1;

            let (new_val, new_index) = calculate_factor(&text, i)?;
            i = new_index;
            res = if operation == Operation::Mult {
                res * new_val
            } else if operation == Operation::Div {
                res / new_val
            } else {
                res % new_val
            };
        }
    }

    Ok((res, i))
}

fn calculate_factor(text: &str, index: usize) -> Result<CalcRes, ParsingError> {
    let mut res = 0.0;
    let mut i = index;

    if let Some(ch) = text.chars().nth(index) {
        if is_number(ch) {
            let (number, index) = parse_number(text, i)?;
            res = number;
            i = index;
        } else if is_parentheses(ch) {
            i += 1;
            let (new_val, index) = calculate(&text, i)?;

            i = index;
            res += new_val;

            match text.chars().nth(i) {
                Some(ch) => {
                    if ch == ')' {
                        i += 1;
                    } else {
                        return Err(ParsingError {
                            text: String::from(text),
                            index: i,
                        });
                    }
                }

                None => {
                    return Err(ParsingError {
                        text: String::from(text),
                        index: i,
                    });
                }
            }
        }
    }

    Ok((res, i))
}

fn parse_number(text: &str, index: usize) -> Result<CalcRes, ParsingError> {
    let mut digits: String = "".into();
    let mut res: f64 = 0.0;
    let mut i = index;
    let mut is_factor = false;
    let mut has_comma = false;

    while let Some(ch) = text.chars().nth(i) {
        if is_number(ch) || ch == '.' {
            if ch == '.' {
                if has_comma || i == index {
                    return Err(ParsingError {
                        text: String::from(text),
                        index: i,
                    });
                } else {
                    has_comma = true;
                    digits = digits + &ch.to_string();
                    i += 1;
                    continue;
                }
            }

            is_factor = true;
            digits = digits + &ch.to_string();
            i += 1;
        } else {
            break;
        }
    }

    if is_factor {
        res = digits.parse::<f64>().unwrap();
    }

    Ok((res, i))
}
