pub enum TokenKind {
    /// Data Types like Int, Char
    Literal { kind: LiteralKind },
    /// =
    EqualsTo,
    /// ;
    SemiColon,
    /// :
    Colon,
    /// (
    LeftParen,
    /// )
    RightParen,
    /// {
    LeftBrace,
    /// }
    RightBrace,
    /// [
    LeftBraket,
    /// ]
    RightBracket,
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Star,
    /// /
    ForwardSlash,
}

pub enum LiteralKind {
    /// [123, 32] -> `int`
    Integer,
    /// `a`, `b`; Supports unicode as well -> char
    Character,
    /// "Hello world"; -> str
    String,
    /// True / False -> bool
    Boolean,
}
