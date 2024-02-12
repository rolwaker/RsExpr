# RsExpr

A simple expression evaluator written in Rust.

## Usage

- if ran with a single argument that doesn't start with a digit, it is taken
as a path to a file that should be evaluated one line at a time.
- if ran with no arguments, it will read lines from stdin and evaluate those
one at a time.
- if ran with more arguments or a single argument starting with a digit, they
will be joined and evaluated as a single logical line.
- when reading multiple lines from a file or from stdin it will parse in a
stateful mode, meaning that you can assign to and use variables in your
expressions.

## Operator Precedence

- 34  (...)
- +x  -x  ^x
- x*y x/y x%y
- x+y x-y
- x&y x|y x^y
- x=...

## Arithmetic

All math is performed on 128-bit integers, either as a single bitmask or as a
fraction of two 128-bit integers.
