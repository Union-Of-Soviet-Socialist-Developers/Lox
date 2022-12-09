#[allow(unused_imports)]
use std::{env, fs};
#[allow(unused_imports)]
use std::collections::LinkedList;

mod lexer;
mod errors;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     if args.len() < 2 || args.len() > 2 {
//         println!("lox [file]");
//         return;
//     }
//     if !args[1].ends_with(".lox") {
//         println!("file extension must be .lox");
//         return;
//     }
//     if args.len() == 1  { run_file(&args[1]); }
//     //TODO: implement a interactive interpreter experience. else { run_prompt(); }
// }

// fn run_file(path: &String) {
//     let contents = fs::read_to_string(path);
//     todo!();
// }

fn main() {
    let code = r#"""
        'Hello'
        'Hello'
        'Hello'
    """#;

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
}
