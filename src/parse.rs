use super::token::Token;
use std::slice::Iter;
use std::iter::Peekable;

fn parse_literal(toks: &mut Peekable<Iter<Token>>) -> Result<i64, String> {
    match toks.next() {
        Some(Token::Number(val)) => Ok(*val),
        Some(Token::LeftParen) => {
            let num = match parse_bit(toks) {
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

fn parse_prefix(toks: &mut Peekable<Iter<Token>>) -> Result<i64, String> {
    match toks.peek() {
        Some(Token::Add) => {
            toks.next();
            match parse_literal(toks) {
                Ok(val) => Ok(val),
                Err(msg) => Err(msg)
            }
        },
        Some(Token::Subtract) => {
            toks.next();
            match parse_literal(toks) {
                Ok(val) => Ok(-val),
                Err(msg) => Err(msg)
            }
        },
        _ => parse_literal(toks)
    }
}

fn parse_product(toks: &mut Peekable<Iter<Token>>) -> Result<i64, String> {
    let mut num = match parse_prefix(toks) {
        Ok(val) => val,
        Err(msg) => return Err(msg)
    };
    
    loop {
        match toks.peek() {
            Some(Token::Multiply) => {
                toks.next();
                match parse_prefix(toks) {
                    Ok(val) => num *= val,
                    Err(msg) => return Err(msg),
                }
            },
            Some(Token::Divide) => {
                toks.next();
                match parse_prefix(toks) {
                    Ok(val) => num /= val,
                    Err(msg) => return Err(msg),
                }
            },
            Some(Token::Modulo) => {
                toks.next();
                match parse_prefix(toks) {
                    Ok(val) => num %= val,
                    Err(msg) => return Err(msg),
                }
            },
            _ => return Ok(num)
        }
    }
}

fn parse_term(toks: &mut Peekable<Iter<Token>>) -> Result<i64, String> {
    let mut num = match parse_product(toks) {
        Ok(val) => val,
        Err(msg) => return Err(msg)
    };
    
    loop {
        match toks.peek() {
            Some(Token::Add) => {
                toks.next();
                match parse_product(toks) {
                    Ok(val) => num += val,
                    Err(msg) => return Err(msg),
                }
            },
            Some(Token::Subtract) => {
                toks.next();
                match parse_product(toks) {
                    Ok(val) => num -= val,
                    Err(msg) => return Err(msg),
                }
            },
            _ => return Ok(num)
        }
    }
}

fn parse_bit(toks: &mut Peekable<Iter<Token>>) -> Result<i64, String> {
    let mut num = match parse_term(toks) {
        Ok(val) => val,
        Err(msg) => return Err(msg)
    };
    
    loop {
        match toks.peek() {
            Some(Token::And) => {
                toks.next();
                match parse_term(toks) {
                    Ok(val) => num &= val,
                    Err(msg) => return Err(msg),
                }
            },
            Some(Token::Ior) => {
                toks.next();
                match parse_term(toks) {
                    Ok(val) => num |= val,
                    Err(msg) => return Err(msg),
                }
            },
            Some(Token::Xor) => {
                toks.next();
                match parse_term(toks) {
                    Ok(val) => num ^= val,
                    Err(msg) => return Err(msg),
                }
            },
            _ => return Ok(num)
        }
    }
}


pub fn parse_tokens(toks: &Vec<Token>) -> Result<i64, String> {
    let mut it = toks.iter().peekable();
    parse_bit(&mut it)
}
