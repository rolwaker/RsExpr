use super::token::Token;
use std::str::Chars;
use std::iter::Peekable;

fn lex_identifier(buffer: &mut Peekable<Chars>) -> Result<Token, String> {
    let mut var = String::new();
    loop {
        match buffer.peek() {
            Some('a'..='z' | 'A'..='Z' | '0'..='9') => var.push(buffer.next().unwrap()),
            _ => return Ok(Token::Ident(var))
        }
    }
}

fn lex_operator(buffer: &mut Peekable<Chars>) -> Result<Token, String> {
    match buffer.next() {
        Some('(') => Ok(Token::LeftParen),
        Some(')') => Ok(Token::RightParen),
        Some('+') => Ok(Token::Add),
        Some('-') => Ok(Token::Subtract),
        Some('*') => Ok(Token::Multiply),
        Some('/') => Ok(Token::Divide),
        Some('%') => Ok(Token::Modulo),
        Some('&') => Ok(Token::And),
        Some('|') => Ok(Token::Ior),
        Some('^') => Ok(Token::Xor),
        Some('~') => Ok(Token::Invert),
        Some('=') => Ok(Token::Assign),
        _ => Err("impossible".to_string())
    }
}

fn lex_numeral(buffer: &mut Peekable<Chars>) -> Result<Token, String> {
    let mut num = 0i64;
    
    loop {
        match buffer.peek() {
            Some('0'..='9') => {
                let d = buffer.next().unwrap();
                num = num * 10 + d.to_digit(10).unwrap() as i64;
            },
            _ => break
        }
    }
    
    Ok(Token::Number(num))
}


fn lex_token(buffer: &mut Peekable<Chars>) -> Result<Token, String> {
    match buffer.peek() {
        Some('a'..='z' | 'A'..='Z') => lex_identifier(buffer),
        Some('(' | ')' | '+' | '-' | '*' | '/' | '%' | '&' | '|' | '^' | '~' | '=') => lex_operator(buffer),
        Some('0'..='9') => lex_numeral(buffer),
        Some(c) => Err(format!("unknown character: '{c}'")),
        None => Err("unexpected end-of-file".to_string())
    }
}


fn lex_whitespace(buffer: &mut Peekable<Chars>) -> bool {
    loop {
        match buffer.peek() {
            Some(' ' | '\t' | '\n') => {
                buffer.next();
            },
            Some(_) => return true,
            None => return false
        }
    }
}


pub fn lex_string(string: &str) -> Result<Vec<Token>, String> {
    let mut toks = Vec::new();
    let mut chars = string.chars().peekable();
    
    while lex_whitespace(&mut chars) {
        match lex_token(&mut chars) {
            Ok(tok) => toks.push(tok),
            Err(msg) => {
                let cnt = string.len() - chars.count();
                println!("(cnt: {cnt})");
                return Err(msg)
            }
        }
    }
    
    Ok(toks)
}
