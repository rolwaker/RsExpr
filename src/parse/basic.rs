use crate::token::Token;
use super::common::parse_bit;

pub fn parse_tokens(toks: &Vec<Token>) -> Result<i64, String> {
    let mut it = toks.iter().peekable();
    let novars = None;
    parse_bit(&novars, &mut it)
}
