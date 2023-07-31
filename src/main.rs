use std::env;
use std::fs::File;
use std::io::{self, Read, BufRead};
use std::path::Path;

mod token;
mod lex;
mod parse;

fn eval_print(string: &str) -> Result<(), i32> {
    match lex::lex_file(string) {
        Ok(toks) => {
            match parse::parse_tokens(&toks) {
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
        
        loop {
            buffer = stdin.lock().lines().next().unwrap().unwrap();
            
            match buffer.as_str() {
                "exit" | "quit" | "q" => return Ok(()),
                _ => _ = eval_print(&buffer)
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
                        
                        for line in buffer.lines() {
                            match eval_print(line) {
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
        eval_print(&buffer)
    }
}
