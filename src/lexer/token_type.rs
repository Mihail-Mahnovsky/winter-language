#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Operator,

    LParen,
    RParen,

    IntLiteral,
    StringLiteral,

    Fn,
    If,
    For,
    While,
    ID,

    True,
    False,

    //this echo add for test! in full version of language this token type need for deleting
    Echo,

    Assignment,
    COLON,
    SEMICOLON,
}