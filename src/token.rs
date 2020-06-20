#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token<'a> {
  // Variants that take an argument
  Identifier(&'a str),
  Value(u32),

  // Keywords: TODO

  // Separators
  Colon,
  SemiColon,
  Comma,
  FwdArrow,
  BwdArrow,

  // Grouping operators
  ParenLeft,
  ParenRight,
  BraceLeft,
  BraceRight,
  SquareLeft,
  SquareRight,

  // Others
  Relationship,
  Iterator,
  Ellipsis,
  Pipe,
}
