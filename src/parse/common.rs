use std::collections::HashMap;
use std::slice::Iter;
use std::iter::Peekable;
use crate::token::Token;

fn parse_literal(vars: &Option<&mut HashMap<String, i64>>, toks: &mut Peekable<Iter<Token>>) -> Result<i64, String> {
    match toks.next() {
        Some(Token::Ident(str)) => {
            match vars {
                Some(map) => {
                    match map.get(str.as_str()) {
                        Some(val) => Ok(*val),
                        None => Err(format!("'{str}' has not been defined"))
                    }
                },
                None => Err("variables cannot be used in this mode".to_string())
            }
        },
        Some(Token::Number(val)) => Ok(*val),
        Some(Token::LeftParen) => {
            let num = match parse_bit(vars, toks) {
                Ok(val) => val,
                Err(msg) => return Err(msg)
            };
            match toks.next() {
                Some(Token::RightParen) => Ok(num),
                _ => Err("expected ')'".to_string())
            }
        },
        _ => Err("expected an integer or '('".to_string())
    }
}

fn parse_prefix(vars: &Option<&mut HashMap<String, i64>>, toks: &mut Peekable<Iter<Token>>) -> Result<i64, String> {
    match toks.peek() {
        Some(Token::Add) => {
            toks.next();
            parse_literal(vars, toks)
        },
        Some(Token::Subtract) => {
            toks.next();
            match parse_literal(vars, toks) {
                Ok(val) => Ok(-val),
                Err(msg) => Err(msg)
            }
        },
        Some(Token::Invert) => {
            toks.next();
            match parse_literal(vars, toks) {
                Ok(val) => Ok(!val),
                Err(msg) => Err(msg)
            }
        },
        _ => parse_literal(vars, toks)
    }
}

fn parse_product(vars: &Option<&mut HashMap<String, i64>>, toks: &mut Peekable<Iter<Token>>) -> Result<i64, String> {
    let mut num = match parse_prefix(vars, toks) {
        Ok(val) => val,
        Err(msg) => return Err(msg)
    };
    
    loop {
        match toks.peek() {
            Some(Token::Multiply) => {
                toks.next();
                match parse_prefix(vars, toks) {
                    Ok(val) => num *= val,
                    Err(msg) => return Err(msg),
                }
            },
            Some(Token::Divide) => {
                toks.next();
                match parse_prefix(vars, toks) {
                    Ok(val) => num /= val,
                    Err(msg) => return Err(msg),
                }
            },
            Some(Token::Modulo) => {
                toks.next();
                match parse_prefix(vars, toks) {
                    Ok(val) => num %= val,
                    Err(msg) => return Err(msg),
                }
            },
            _ => return Ok(num)
        }
    }
}

fn parse_term(vars: &Option<&mut HashMap<String, i64>>, toks: &mut Peekable<Iter<Token>>) -> Result<i64, String> {
    let mut num = match parse_product(vars, toks) {
        Ok(val) => val,
        Err(msg) => return Err(msg)
    };
    
    loop {
        match toks.peek() {
            Some(Token::Add) => {
                toks.next();
                match parse_product(vars, toks) {
                    Ok(val) => num += val,
                    Err(msg) => return Err(msg),
                }
            },
            Some(Token::Subtract) => {
                toks.next();
                match parse_product(vars, toks) {
                    Ok(val) => num -= val,
                    Err(msg) => return Err(msg),
                }
            },
            _ => return Ok(num)
        }
    }
}

pub fn parse_bit(vars: &Option<&mut HashMap<String, i64>>, toks: &mut Peekable<Iter<Token>>) -> Result<i64, String> {
    let mut num = match parse_term(vars, toks) {
        Ok(val) => val,
        Err(msg) => return Err(msg)
    };
    
    loop {
        match toks.peek() {
            Some(Token::And) => {
                toks.next();
                match parse_term(vars, toks) {
                    Ok(val) => num &= val,
                    Err(msg) => return Err(msg),
                }
            },
            Some(Token::Ior) => {
                toks.next();
                match parse_term(vars, toks) {
                    Ok(val) => num |= val,
                    Err(msg) => return Err(msg),
                }
            },
            Some(Token::Xor) => {
                toks.next();
                match parse_term(vars, toks) {
                    Ok(val) => num ^= val,
                    Err(msg) => return Err(msg),
                }
            },
            _ => return Ok(num)
        }
    }
}
