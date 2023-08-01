use std::collections::HashMap;
use std::slice::Iter;
use std::iter::Peekable;
use crate::token::Token;
use super::common::parse_bit;

fn has_assign(it: &mut Peekable<Iter<'_, Token>>) -> Option<String> {
    match it.peek() {
        Some(Token::Ident(key)) => {
            it.next();
            match it.peek() {
                Some(Token::Assign) => {
                    it.next();
                    Some(key.clone())
                },
                _ => None
            }
        },
        _ => None
    }
}

pub fn parse_tokens(vars: &mut HashMap<String, i64>, toks: &Vec<Token>) -> Result<i64, String> {
    let mut it = toks.iter().peekable();
    match has_assign(&mut it) {
        Some(key) => {
            let result = parse_bit(&Some(vars), &mut it);
            if let Ok(val) = result {
                vars.insert(key, val);
            }
            result
        },
        None => {
            it = toks.iter().peekable();
            parse_bit(&Some(vars), &mut it)
        }
    }
}
