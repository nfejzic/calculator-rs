pub mod parser;

#[cfg(test)]
mod tests {
    use core::panic;

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

    #[test]
    fn check_subt() {
        let two_terms = "1 - 2";
        let res = parse_expression(&two_terms);

        match res {
            Ok(val) => assert!(
                (1.0 + val).abs() < f64::EPSILON,
                "Expected {} but calculated {}",
                -1.0,
                val
            ),
            _ => panic!("Subt not working"),
        }
    }

    #[test]
    fn check_mult() {
        let two_terms = "6 * 7";
        let res = parse_expression(&two_terms);

        match res {
            Ok(val) => assert!(
                (42.0 - val).abs() < f64::EPSILON,
                "Expected {} but calculated {}",
                42.0,
                val
            ),
            _ => panic!("Mult not working"),
        }
    }

    #[test]
    fn check_div() {
        let two_terms = "42 / 7";
        let res = parse_expression(&two_terms);

        match res {
            Ok(val) => assert!(
                (6.0 - val).abs() < f64::EPSILON,
                "Expected {} but calculated {}",
                6.0,
                val
            ),
            _ => panic!("Div not working"),
        }
    }

    #[test]
    fn check_mod() {
        let two_terms = "41 % 7";
        let res = parse_expression(&two_terms);

        match res {
            Ok(val) => assert!(
                (6.0 - val).abs() < f64::EPSILON,
                "Expected {} but calculated {}",
                6.0,
                val
            ),
            _ => panic!("Mod not working"),
        }
    }

    #[test]
    fn check_pow() {
        let two_terms = "2^4";
        let res = parse_expression(&two_terms);

        match res {
            Ok(val) => assert!(
                (16.0 - val).abs() < f64::EPSILON,
                "Expected {} but calculated {}",
                16.0,
                val
            ),
            _ => panic!("Pow not working"),
        }

        let three_terms = "3^2^2"; // = 81
        let res = parse_expression(&three_terms);

        match res {
            Ok(val) => assert!(
                (81.0 - val).abs() < f64::EPSILON,
                "Expected {} but calculated {}",
                81.0,
                val
            ),
            _ => panic!("Pow not working"),
        }
    }

    #[test]
    fn check_expression() {
        let exp = "2 + 3 - 7 * 2 + 100 / 4"; // = 16
        let res = parse_expression(exp);

        match res {
            Ok(val) => assert!(
                (16.0 - val).abs() < f64::EPSILON,
                "Expected {} but calculated {}",
                16.0,
                val
            ),
            _ => panic!("Calcuator not working"),
        }

        let exp = "2^3 + 3 - 7 * 2 + 100 / 4"; // = 22
        let res = parse_expression(exp);

        match res {
            Ok(val) => assert!(
                (22.0 - val).abs() < f64::EPSILON,
                "Expected {} but calculated {}",
                22.0,
                val
            ),
            _ => panic!("Calcuator not working"),
        }

        let exp = "(2^3 + 3) % 3 - 7 * 2 + 100 / 4"; // = 13
        let res = parse_expression(exp);

        match res {
            Ok(val) => assert!(
                (13.0 - val).abs() < f64::EPSILON,
                "Expected {} but calculated {}",
                13.0,
                val
            ),
            _ => panic!("Calcuator not working"),
        }

        let exp = "(2^3 + 3^3) % 3 - 7 * 2 + 100 / 4"; // = 13
        let res = parse_expression(exp);

        match res {
            Ok(val) => assert!(
                (13.0 - val).abs() < f64::EPSILON,
                "Expected {} but calculated {}",
                13.0,
                val
            ),
            _ => panic!("Calcuator not working"),
        }

        let exp = "(2^3 + 3^3) % (3^3 - 7 * 2) + 100 / 4"; // = 34
        let res = parse_expression(exp);

        match res {
            Ok(val) => assert!(
                (34.0 - val).abs() < f64::EPSILON,
                "Expected {} but calculated {}",
                34.0,
                val
            ),
            _ => panic!("Calcuator not working"),
        }
    }
}
