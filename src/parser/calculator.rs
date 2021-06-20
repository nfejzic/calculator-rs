use super::{checker::*, operations::*, parsing_error::*};

/// Abstraction of tuple returned by calculation function.
type CalcRes = (f64, usize);

/// Parses the expression and returns f64 as a result of calculation or <br> ParsingError if invalid expression encountered
///
/// # Arguments
///
/// * `exp` - String reference containing the expressin to be evaluated
/// # Example
/// ```
/// let expression = "2 + 2 * 3"
/// let result = parse_expression(&expression);
/// match result {
///     Ok(val) => println!("{}", val),
///     Err(parsing_error) => println!("{}", parsing_error),
/// }
/// ```
pub fn parse_expression(exp: &str) -> Result<f64, ParsingError> {
    let no_whitespaces = remove_whitespaces(exp)?;

    let (result, index) = calculate(&no_whitespaces, 0)?;

    if index < no_whitespaces.len() {
        return Err(ParsingError {
            text: no_whitespaces,
            index,
        });
    }

    Ok(result)
}

/// Removes whitespaces in a given valid expression.
///
/// Returns new String, or ParsingError if expression contains invalid characters
fn remove_whitespaces(text: &str) -> Result<String, ParsingError> {
    let mut res: String = "".into();

    for (i, ch) in text.chars().enumerate() {
        if is_valid_character(ch) {
            res = if ch.is_whitespace() {
                res
            } else {
                res + &ch.to_string()
            };
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

/// Parse and calculate the given expression. <br>
/// Returns either Tuple containing calculated result as f64 and index of the next character to observe as usize
/// or ParsingError in case something goes wrong (like missing closing parentheses or number with multiple periods etc.)
/// # Arguments
/// * `text` - Expression as a string reference
/// * `index` - The index of character to observe next
fn calculate(text: &str, index: usize) -> Result<CalcRes, ParsingError> {
    let (mut res, mut i) = calculate_term(&text, index)?;

    while i < text.len() {
        if let Some(ch) = text.chars().nth(i) {
            let operation = Operation::Operator(ch).from_operator();

            if operation.order() != OperationOrder::Second {
                break;
            }

            let (new_val, new_index) = calculate_term(&text, i + 1)?;

            i = new_index;
            match operation {
                Operation::Add => res += new_val,
                Operation::Subt => res -= new_val,
                _ => break,
            }
        }
    }

    Ok((res, i))
}

/// Parse and calculate a term, i.e. a multiplication is considered one term.
/// Example: `28 * (3 + 4)` would all be considered one term. <br>
/// Returns either Tuple containing calculated result as f64 and index of the next character to observe as usize
/// or ParsingError in case something goes wrong (like missing closing parentheses or number with multiple periods etc.)
/// # Arguments
/// * `text` - Expression as a string reference
/// * `index` - The index of character to observe next
fn calculate_term(text: &str, index: usize) -> Result<CalcRes, ParsingError> {
    let (mut res, mut i) = calculate_factor(&text, index)?;

    while i < text.len() {
        if let Some(ch) = text.chars().nth(i) {
            let operation = Operation::Operator(ch).from_operator();

            if operation.order() != OperationOrder::First {
                break;
            }

            let (new_val, new_index) = calculate_factor(&text, i + 1)?;

            i = new_index;

            match operation {
                Operation::Mult => res *= new_val,
                Operation::Div => res /= new_val,
                Operation::Mod => res %= new_val,
                _ => break,
            }
        }
    }

    Ok((res, i))
}

/// Parse and calculate a given factor. A factor can be a number, or an expression inside parentheses. <br>
/// Returns either Tuple containing calculated result as f64 and index of the next character to observe as usize
/// or ParsingError in case something goes wrong (like missing closing parentheses or number with multiple periods etc.)
/// # Arguments
/// * `text` - Expression as a string reference
/// * `index` - The index of character to observe next
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
            println!("Here with char {}", ch);

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
    } else {
        return Err(ParsingError {
            text: String::from(text),
            index: i,
        });
    }

    Ok((res, i))
}

/// Parse the given number. <br>
/// Returns either Tuple containing the number as f64 and index of the next character to observe as usize
/// or ParsingError in case something goes wrong (like missing closing parentheses or number with multiple periods etc.)
/// # Arguments
/// * `text` - Expression as a string reference
/// * `index` - The index of character to observe next
fn parse_number(text: &str, index: usize) -> Result<CalcRes, ParsingError> {
    let mut digits = String::new();
    let mut res = 0.0;
    let mut i = index;
    let mut is_factor = false;
    let mut has_comma = false;

    while let Some(ch) = text.chars().nth(i) {
        if is_number(ch) || ch == '.' {
            if ch == '.' {
                if has_comma {
                    // multiple dots
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
