use std::fs::File;
use std::io::Read;
use std::path::Path;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Token {
    Eof,
    Equal,
    Equality,
    Greater,
    Identifier(String),
    Int,
    LBrace,
    LParen,
    MinusMinus,
    MinusEqual,
    NumericConstant(usize),
    PlusPlus,
    PlusEqual,
    RBrace,
    Return,
    RParen,
    Semi,
    While,
}

pub fn read_file(path : &Path) -> String {
    
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("File {} not openable, because {}", display, why),
        Ok(file) => file,
    };

    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Err(_) => panic!("Hello"),
        Ok(content) => content,
    };

    return content;
} 

pub struct Lexer {
    buf: String,
    pos: usize,
    done: bool,
    eat_whitespace_re: Regex,
    identifier_re: Regex,
    numeric_re: Regex,
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        if let Some(m) = self.eat_whitespace_re.find(&self.buf[self.pos..]) {
            self.pos += m.end();
        }

        if let Some(c) = self.buf.chars().nth(self.pos) {

            if let Some(m) = self.identifier_re.find(&self.buf[self.pos..]) {
                self.pos += m.end();
                return match m.as_str() {
                    "int" => Some(Token::Int),
                    "while" => Some(Token::While),
                    "return" => Some(Token::Return),
                    _ => Some(Token::Identifier(String::from(m.as_str()))),
                }
            }

            if let Some(m) = self.numeric_re.find(&self.buf[self.pos..]) {
                self.pos += m.end();
                return Some(Token::NumericConstant(m.as_str().parse::<usize>().unwrap()));
            }

            if let Some(_) = self.buf.chars().nth(self.pos+1) {
                let token = match &self.buf[self.pos..self.pos+2] {
                    "--" => Some(Token::MinusMinus),
                    "-=" => Some(Token::MinusEqual),
                    "++" => Some(Token::PlusPlus),
                    "+=" => Some(Token::PlusEqual),
                    "==" => Some(Token::Equality),
                    _ => None,
                };

                if token.is_some() {
                    self.pos += 2;
                    return token;
                }
            }

            let token = match c {
                '=' => Some(Token::Equal),
                '>' => Some(Token::Greater),
                ';' => Some(Token::Semi),
                '(' => Some(Token::LParen),
                ')' => Some(Token::RParen),
                '{' => Some(Token::LBrace),
                '}' => Some(Token::RBrace),
                _ => None,
            };
            self.pos += 1;
            return token;
        } else if !self.done {
            self.done = true;
            return Some(Token::Eof);
        }
        return None;
    }
}

pub fn tokenizer(path: &Path) -> Lexer {
    let content = read_file(path);
    let lex = Lexer {
        buf: content,
        pos: 0,
        done: false,
        eat_whitespace_re: Regex::new(r"^\s*").unwrap(),
        identifier_re: Regex::new(r"^[A-z]\w*").unwrap(),
        numeric_re: Regex::new(r"^[1-9]\d*").unwrap(),
    };
    return lex;
}
