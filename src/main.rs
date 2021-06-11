use std::{env, process};

mod parser;
use parser::calculator::parse_expression;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Incorrect usage!");
        eprintln!("Correct usage:");
        eprintln!("\n\tcalculate \"expression\" ;\n\n\tWhere expression can contain decimal numbers, +, -, *, /, %, '(' and ')'");
        process::exit(1);
    }

    let expression = args
        .get(1)
        .expect("Args has exactly 2 arguments at this point. Index 1 is valid");

    let res = parse_expression(&&expression);

    match res {
        Ok(val) => println!("{} = {}", expression, val),
        Err(parsing_error) => println!("{}", parsing_error),
    }
}
