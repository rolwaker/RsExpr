use crate::number::Number;

pub enum Token {
    Ident(String),
    Number(Number),
    LeftParen,
    RightParen,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    And,
    Or,
    Hat,
    Assign,
}
