use crate::errors::LoxError;

use super::errors;


#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenType {
    STRING(String),
    INTEGER(isize),
    FLOAT(f64),
    ARRAY(Vec<TokenType>),
    
    ADD,
    SUBTRACT,
    DIVIDE,
}


#[derive(Debug)]
pub struct Token {
    pub value: TokenType,
}

const STRING_START: &[char] = &['\'', '"', '~'];

pub struct Lexer {
    source: String,
    pos: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        return Lexer {
            source,
            pos: 0,
        };
    }

    fn next_char(&mut self) {
        self.pos += 1;
    }

    fn get_char(&self) -> char {
        return self.source.clone().chars().nth(self.pos).unwrap();
    }

    pub fn parse(mut self) -> Result<Vec<Token>, errors::LoxError> {
        let mut tokens = Vec::<Token>::new();

        while self.source.len() > self.pos {
            match self.get_char() {
                // ' ' => self.next_char(),

                c if STRING_START.contains(&c) => {
                    // Start of a string
                    let mut string = String::new();
                    let mut encountered_string_end = false;

                    for i in self.source[self.pos+1..].chars() {
                        // self.next_char(); // Use the next_char() method instead of manually increasing the position
                        /* cannot borrow `self` as mutable because it is also borrowed as
                           immutable mutable borrow occurs here */
                        self.pos += 1;

                        if STRING_START.contains(&i) {  // Strings should end the same way it starts
                            encountered_string_end = true;
                            break;
                        } else {
                            string.push(i);
                        }
                    }

                    if encountered_string_end == false {
                        return Err(LoxError {message: "Unterminated string".to_owned(),
                                             hint: Some("Maybe you forgot to close the string with a quote?".to_owned())});
                    }

                    tokens.push(Token {value: TokenType::STRING(string)});
                },

                c if c.is_numeric() => {
                    // Start of a number
                    let mut number = String::new();

                    for i in self.source[self.pos..].chars() {
                        if !(i.is_numeric() || ".".contains(i)) { // If its not a number or decimal point we end
                            if number.contains(".") {
                                tokens.push(Token {value: TokenType::FLOAT(number.parse::<f64>().unwrap())});
                            } else {
                                tokens.push(Token {value: TokenType::INTEGER(number.parse::<isize>().unwrap())});
                            }
                            break;
                        }
                        number.push(i);
                        self.pos += 1;
                    }
                },

                _ => {}
            }

            self.next_char();
        }

        Ok(tokens)
    }
}
