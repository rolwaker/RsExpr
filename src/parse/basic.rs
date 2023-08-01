use crate::token::Token;
use super::common::parse_bit;

pub fn parse_tokens(toks: &Vec<Token>) -> Result<i64, String> {
    parse_bit(&mut None, &mut toks.iter().peekable())
}
