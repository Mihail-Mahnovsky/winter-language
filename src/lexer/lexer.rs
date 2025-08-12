use crate::lexer::token::Token;
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
        let mut tok_type = TokenType::IntLiteral;
        let mut dot_seen = false;

        while self.pos < self.line_clone.len() {
            if self.current.is_ascii_digit() {
                res.push(self.current);
            } else if self.current == '.' && !dot_seen {
                dot_seen = true;
                tok_type = TokenType::FloatLiteral;
                res.push(self.current);
            } else {
                break;
            }
            self.advance();
        }

        if tok_type == TokenType::FloatLiteral {
            let parts: Vec<&str> = res.split('.').collect();
            if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
                panic!("Invalid float literal: '{}'", res);
            }
        }

        Token::new(res, tok_type)
    }

    fn callinger_nize(&mut self) -> Token {
        let mut res = String::new();

        while self.pos < self.line_clone.len() && self.current.is_alphabetic()
            || self.current.is_ascii_digit()
        {
            res.push(self.current);
            self.advance();
        }

        match res.as_str() {
            "fn" => Token::new(res, TokenType::Fn),
            "for" => Token::new(res, TokenType::For),
            "while" => Token::new(res, TokenType::While),
            "if" => Token::new(res, TokenType::If),
            "true" => Token::new(res, TokenType::True),
            "false" => Token::new(res, TokenType::False),
            "string" => Token::new(res, TokenType::StringType),
            "int" => Token::new(res, TokenType::IntType),
            "bool" => Token::new(res, TokenType::BoolType),
            "short" => Token::new(res, TokenType::ShortType),
            "char" => Token::new(res, TokenType::CharType),
            "float" => Token::new(res, TokenType::FloatType),
            "long" => Token::new(res, TokenType::LongType),
            "void" => Token::new(res, TokenType::VoidType),
            "return" => Token::new(res, TokenType::Return),
            _ => Token::new(res, TokenType::ID),
        }
    }

    fn string_nize(&mut self) -> Token {
        let quote_char = self.current;

        if quote_char == '\'' {
            self.advance();

            if self.current == '\\' {
                self.advance();
                let esc = self.current;
                self.advance();

                if self.current != '\'' {
                    panic!("Unterminated char literal");
                }
                self.advance();

                let escaped_char = match esc {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '\\' => '\\',
                    '\'' => '\'',
                    _ => panic!("Unknown escape sequence"),
                };

                Token::new(escaped_char.to_string(), TokenType::CharLiteral)
            } else {
                let res = self.current;
                self.advance();

                if self.current != '\'' {
                    panic!("Char literal ");
                }

                self.advance();
                Token::new(res.to_string(), TokenType::CharLiteral)
            }
        } else {
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
                '#' => {self.advance(); continue},
                '>' => {self.advance(); continue},
                '+' => tokens.push(Token::new("+".to_string(), TokenType::Operator)),
                '-' =>  if self.line_clone[self.pos + 1] == '>' { tokens.push(Token::new("->".to_string(), TokenType::RetOp)); self.advance() } else { tokens.push(Token::new("-".to_string(), TokenType::Operator))},
                '*' => tokens.push(Token::new("*".to_string(), TokenType::Operator)),
                '/' => tokens.push(Token::new("/".to_string(), TokenType::Operator)),
                '=' => tokens.push(Token::new("=".to_string(), TokenType::Assignment)),
                '(' => tokens.push(Token::new("(".to_string(), TokenType::LParen)),
                ')' => tokens.push(Token::new(")".to_string(), TokenType::RParen)),
                '{' => tokens.push(Token::new("{".to_string(), TokenType::LBracket)),
                '}' => tokens.push(Token::new("}".to_string(), TokenType::RBracket)),
                ';' => tokens.push(Token::new(";".to_string(), TokenType::SemiColon)),
                ':' => tokens.push(Token::new(":".to_string(), TokenType::Colon)),
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

        //for tok in &mut tokens {
        //  println!("{}", tok.get_name_of_token());
        //}

        tokens
    }
}
