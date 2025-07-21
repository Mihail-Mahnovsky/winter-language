#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Operator,

    LParen,
    RParen,

    IntLiteral,
    StringLiteral,
    CharLiteral,
    FloatLiteral,

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
    CharType,
    ShortType,
    LongType,

    True,
    False,

    LBracket,
    RBracket,

    Assignment,
    Colon,
    SemiColon,
    Coma,
}
