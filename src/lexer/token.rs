use crate::lexer::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    tok_type: TokenType,
    tok_value: String,
}

impl Token {
    pub fn new(value: String, token_type: TokenType) -> Self {
        Self {
            tok_type: token_type,
            tok_value: value,
        }
    }

    pub fn get_type(&self) -> TokenType {
        self.tok_type.clone()
    }

    pub fn get_value(&self) -> String {
        self.tok_value.clone()
    }

    pub fn get_name_of_token(&self) -> &str {
        match self.tok_type {
            TokenType::Operator => "operator",
            TokenType::IntLiteral => "int_literal",
            TokenType::Assignment => "assignment",
            TokenType::ID => "ID",
            TokenType::SEMICOLON => "semicolon",
            TokenType::COLON => "colon",
            TokenType::Fn => "fn",
            TokenType::False => "false",
            TokenType::True => "true",
            TokenType::For => "for",
            TokenType::If => "if",
            TokenType::LParen => "LParen",
            TokenType::RParen => "RParen",
            TokenType::While => "while",
            TokenType::Echo => "echo",
            //TokenType::Type => self.get_value(),
            _ => return "error",
        }
    }
}
