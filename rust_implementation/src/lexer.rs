use crate::errors::LoxError;

use super::errors;


#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Debug)]
pub enum Type {
        WHITESPACE,
        LINE_BREAK,
        NEWLINE,
        STRING,
        INTEGER,
        FLOAT,
        OPERATOR,
        STRING_QUOTE,
        SINGLE_LINE_COMMENT,
        MULTI_LINE_COMMENT_START,
        MULTI_LINE_COMMENT_END,
        UNKNOWN,
}


#[derive(Debug)]
pub struct Token {
    pub token_type: Type,
    pub value: String,
}


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
                ' ' => self.next_char(),

                '\'' | '"' => {
                    // Start of a string
                    let mut string = String::new();
                    let mut encountered_string_end = false;

                    for i in self.source[self.pos+1..].chars() {
                        // self.next_char(); // Use the next_char() method instead of manually increasing the position
                        /* cannot borrow `self` as mutable because it is also borrowed as
                           immutable mutable borrow occurs here */
                        self.pos += 1;

                        if ['\'', '"'].contains(&i) {
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

                    // println!("String: {}", string);
                    tokens.push(Token {token_type: Type::STRING, value: string});
                },

                c if c.is_numeric() => {
                    // Start of a number
                    let mut number = String::new(); // Will then be converted to a number

                    for i in self.source[self.pos..].chars() {
                        if !(i.is_numeric() || ".".contains(i)) { // if its not a number or decimal point we end
                            // TODO convert 'number' (currently as str) to int or float
                            tokens.push(Token {token_type: if number.contains(".") {Type::FLOAT} else {Type::INTEGER}, value: number});
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
