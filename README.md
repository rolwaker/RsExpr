# RsExpr
simple expression evaluator written in Rust.

## Usage
- if ran with a single argument that doesn't start with a digit, it is taken
as a path to a file that should be evaluated one line at a time.
- if ran with no arguments, it will read lines from stdin and evaluate those
one at a time.
- if ran with more arguments or a single argument starting with a digit, they
will be joined and evaluated as a single logical line.

## Operator Precedence
- 34  (...)
- +x  -x
- x*y x/y x%y
- x+y x-y
- x&y x|y x^y

## Arithmetic
- all math is performed on i64 variables (64-bit signed integers).
