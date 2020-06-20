#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token<'a> {
  // Variants that take an argument
  Identifier(&'a str),
  Value(u32),

  // Keywords: TODO

  // Separators
  Colon,
  Comma,
  Edge,
  Path,

  // Grouping operators
  ParenLeft,
  ParenRight,
  BraceLeft,
  BraceRight,
}
