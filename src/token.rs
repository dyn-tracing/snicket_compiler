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
  Arrow,

  // Grouping operators
  ParenLeft,
  ParenRight,
  BraceLeft,
  BraceRight,
  SquareLeft,
  SquareRight,

  // Binary arithmetic operators + conditional operator
  Plus,
  Minus,
  Mul,
  Div,
  Modulo,
  Cond,

  // Comparison operators
  Equal,
  NotEqual,
  LTEQOp,
  GTEQOp,
  LessThan,
  GreaterThan,
}
