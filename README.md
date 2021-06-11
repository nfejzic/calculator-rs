# Rust CLI calculator

This is a simple CLI app implemented as a Rust learning exercise.

## Compilation

Simply execute `cargo build --release`

To install, clone the repository and then run `cargo install --path .`

## Usage:

`calculator "expression"`

Expression may use any of the following:

-   Decimal numbers: `0 - 9`, decimal point numbers are allowed i.e. `2.5542`
-   Parentheses: `(`, `)`
-   Addition : `+`
-   Subtraction : `-`
-   Multiplication : `*`
-   Division : `/`
-   Modulo : `%`

### Examples:

```
calculator "2 + 2"
2 + 2 = 4

calculator "2 * 3 + 2"
2 * 3 + 2 = 8

calculator "2 * (3 + 2)"
2 * (3 + 2) = 10

```
