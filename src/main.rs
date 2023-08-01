use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read, BufRead};
use std::path::Path;

mod token;
mod lex;
mod parse;

use token::Token;

fn eval_print<Parse>(string: &str, mut parse: Parse) -> Result<(), i32>
        where Parse: FnMut(&Vec<Token>) -> Result<i64, String> {
    match lex::lex_string(string) {
        Ok(toks) => {
            match parse(&toks) {
                Ok(value) => {
                    println!("{value}");
                    Ok(())
                },
                Err(msg) => {
                    println!("parse error: {msg}!");
                    Err(1)
                }
            }
        },
        Err(msg) => {
            println!("lex error: {msg}!");
            Err(1)
        }
    }
}

fn main() -> Result<(), i32> {
    let args: Vec<String> = env::args().collect();
    let mut buffer;
    
    if args.len() < 2 {
        let stdin = io::stdin();
        let mut vars = HashMap::new();
        
        loop {
            buffer = stdin.lock().lines().next().unwrap().unwrap();
            
            match buffer.as_str() {
                "exit" | "quit" | "q" => return Ok(()),
                _ => _ = eval_print(&buffer, |toks| parse::state::parse_tokens(&mut vars, toks))
            }
        }
    } else if args.len() == 2 && !args.get(1).unwrap().chars().nth(0).unwrap_or(' ').is_digit(10) {
        buffer = String::new();
        let path = Path::new(args.get(1).unwrap());
        let disp = path.display();
        
        match File::open(path) {
            Ok(mut file) => {
                match file.read_to_string(&mut buffer) {
                    Ok(_) => {
                        let mut ec = 0;
                        let mut vars = HashMap::new();
                        
                        for line in buffer.lines() {
                            match eval_print(line, |toks| parse::state::parse_tokens(&mut vars, toks)) {
                                Err(c) => {
                                    ec = c;
                                },
                                Ok(_) => {}
                            }
                        }
                        
                        if ec != 0 {
                            Err(ec)
                        } else {
                            Ok(())
                        }
                    },
                    Err(msg) => {
                        println!("could not read file '{disp}', error: {msg}!");
                        Err(-1)
                    }
                }
            },
            Err(msg) => {
                println!("could not open file '{disp}', error: {msg}!");
                Err(-1)
            }
        }
    } else {
        buffer = (&args[1..]).join(" ");
        eval_print(&buffer, parse::basic::parse_tokens)
    }
}
