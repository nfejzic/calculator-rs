pub mod parser;

#[cfg(test)]
mod tests {
    use crate::parser::calculator::parse_expression;

    #[test]
    fn check_sum() {
        let two_terms = "1 + 2";
        let res = parse_expression(&two_terms);

        match res {
            Ok(val) => assert!(
                (3.0 - val).abs() < f64::EPSILON,
                "Expected {} but calculated {}",
                3.0,
                val
            ),
            _ => panic!("Sum not working"),
        }
    }
}
