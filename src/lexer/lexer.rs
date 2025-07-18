use crate::lexer::token::{self, Token};
use crate::lexer::token_type::*;

pub struct Lexer {
    pos: usize,
    current: char,
    line_clone: Vec<char>,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            pos: 0,
            current: ' ',
            line_clone: Vec::new(),
        }
    }

        fn advance(&mut self) {
        self.pos += 1;
        if self.pos < self.line_clone.len() {
            self.current = self.line_clone[self.pos];
        }
    }

    fn number_nize(&mut self) -> Token {
        let mut res = String::new();

        while self.pos < self.line_clone.len() && self.current.is_ascii_digit() {
            res.push(self.current);
            self.advance();
        }

        Token::new(res, TokenType::IntLiteral)
    }

    fn callinger_nize(&mut self) -> Token{
        let mut res = String::new();

        while self.pos < self.line_clone.len() && self.current.is_alphabetic() {
            res.push(self.current);
            self.advance();
        }

        match res.as_str() {
            "fn"=>Token::new(res, TokenType::Fn),
            "for"=>Token::new(res, TokenType::For),
            "while"=>Token::new(res, TokenType::While),
            "if"=>Token::new(res, TokenType::If),
            "true"=>Token::new(res, TokenType::True),
            "false"=>Token::new(res, TokenType::False),
            "string"=>Token::new(res, TokenType::StringType),
            "int"=>Token::new(res, TokenType::IntType),
            "bool"=>Token::new(res, TokenType::BoolType),
            "void"=>Token::new(res, TokenType::VoidType),
            //for test
            "echo" => Token::new(res, TokenType::Echo),
            _ => Token::new(res, TokenType::ID),
        }
    }

    fn string_nize(&mut self) -> Token {
        let quote_char = self.current; 
        self.advance(); 

        let mut res = String::new();

        while self.pos < self.line_clone.len() && self.current != quote_char {
            res.push(self.current);
            self.advance();
        }

        if self.current != quote_char {
            panic!("Unterminated string literal");
        }

        self.advance(); 

        Token::new(res, TokenType::StringLiteral)
    }


    pub fn token_nize(&mut self, line: String) -> Vec<Token> {
        self.line_clone = line.chars().collect();
        self.pos = 0;
        let mut tokens: Vec<Token> = Vec::new();

        if !self.line_clone.is_empty() {
            self.current = self.line_clone[0];
        } else {
            return tokens;
        }

    while self.pos < self.line_clone.len() {
        match self.current {
            '#' => continue,
            '+' => tokens.push(Token::new("+".to_string(), TokenType::Operator)),
            '-' => tokens.push(Token::new("-".to_string(), TokenType::Operator)),
            '*' => tokens.push(Token::new("*".to_string(), TokenType::Operator)),
            '/' => tokens.push(Token::new("/".to_string(), TokenType::Operator)),
            '=' => tokens.push(Token::new("=".to_string(), TokenType::Assignment)),
            '(' => tokens.push(Token::new("(".to_string(), TokenType::LParen)),
            ')' => tokens.push(Token::new(")".to_string(), TokenType::RParen)),
            '{' => tokens.push(Token::new("{".to_string(), TokenType::LBracket)),
            '}' => tokens.push(Token::new("}".to_string(), TokenType::RBracket)),
            ';' => tokens.push(Token::new(";".to_string(), TokenType::SEMICOLON)),
            ':' => tokens.push(Token::new(":".to_string(), TokenType::COLON)),
            ',' => tokens.push(Token::new(",".to_string(), TokenType::Coma)),


            '"' | '\'' => {
                tokens.push(self.string_nize());
                continue;
            }

            c if c.is_alphabetic() => {
                tokens.push(self.callinger_nize());
                continue;
            }
            c if c.is_ascii_digit() => {
                tokens.push(self.number_nize());
                continue;
            }
            ' ' | '\t' | '\n' => {
                self.advance();
                continue;
            }

            _ => {
                println!("error: unexpected xyeta '{}'", self.current);
                self.advance();
                continue;
            }
        }
            self.advance(); 
        }


        //for tok in &mut tokens{
        //    println!("{}",tok.get_name_of_token());
        //}

        tokens
    }
}