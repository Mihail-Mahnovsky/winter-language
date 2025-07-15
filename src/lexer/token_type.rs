#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Operator,

    LParen,
    RParen,

    IntLiteral,
    StringLiteral,

    //lang words
    Fn,
    If,
    For,
    While,
    Return,
    Break,
    Continue,

    ID,

    IntType,
    StringType,
    FloatType,
    BoolType,

    True,
    False,

    //this echo add for test! in full version of language this token type need for deleting
    Echo,

    LBracket,
    RBracket,

    Assignment,
    COLON,
    SEMICOLON,
    Coma,
}