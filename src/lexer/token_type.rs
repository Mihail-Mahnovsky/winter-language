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
    VoidType,

    True,
    False,

    LBracket,
    RBracket,

    Assignment,
    COLON,
    SEMICOLON,
    Coma,
}
