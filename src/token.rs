use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    // Variants that take an argument
    Identifier(&'a str),
    Value(u32),

    // Keywords
    Match,
    Where,
    Return,

    // Operators
    Edge,
    Path,
    Colon,
    Equals,
    Period,

    // Separators
    Comma,
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Identifier(id) => write!(f, "Identifier({})", id),
            Token::Value(v) => write!(f, "Value({})", v),
            Token::Match => write!(f, "MATCH"),
            Token::Where => write!(f, "WHERE"),
            Token::Return => write!(f, "RETURN"),
            Token::Edge => write!(f, "-->"),
            Token::Path => write!(f, "-\\*>"),
            Token::Colon => write!(f, ":"),
            Token::Equals => write!(f, "=="),
            Token::Period => write!(f, "."),
            Token::Comma => write!(f, ","),
        }
    }
}
