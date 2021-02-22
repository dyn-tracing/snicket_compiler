#![feature(try_blocks)]
extern crate lazy_static;
extern crate regex;
extern crate serde;
extern crate strum;
extern crate strum_macros;

#[allow(clippy::all)]
pub mod lexer;
pub mod listener;
#[allow(clippy::all)]
pub mod parser;
pub mod to_ir;
pub mod visitor;
