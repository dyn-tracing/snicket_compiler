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
