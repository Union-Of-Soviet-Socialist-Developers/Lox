use std::{env, fs};

mod lexer;
mod errors;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); // The first argument is target/debug/x something something

    if args.len() != 1 {
        println!("lox [file]");
        return;
    }
    if !args[0].ends_with(".lox") {
        println!("file extension must be .lox");
        return;
    }
    if args.len() == 1  { run_file(&args[0]); }
    //TODO: implement a interactive interpreter experience. else { run_prompt(); }
}

fn run_file(path: &String) {
    let contents = fs::read_to_string(path);
    
    
    match contents {
        Ok(code) => {
            println!("{:?}", code.chars());
            let lexer_ = lexer::Lexer::new(code.to_owned());
    
            match lexer_.parse() {
                Ok(tokens) => {
                    println!("{:?}", tokens);
                },
                Err(e) => {
                    println!("{}", e.message);
                    if e.hint.is_some() {
                        println!("Hint: {}", e.hint.unwrap());
                    }
                }
            }        
        },
        Err(e) => {
            println!("Failed to open file {}", path);
            println!("Details: {}", e);
        },
    }
}
