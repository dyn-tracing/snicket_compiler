use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Prog<'a> {
    pub patterns: Patterns<'a>,
    pub filters: Filters<'a>,
}

#[derive(Debug, PartialEq)]
pub struct Patterns<'a> {
    pub pattern_vector: Vec<Pattern<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Pattern<'a> {
    pub from_node: Identifier<'a>,
    pub to_node: Identifier<'a>,
    pub relationship_type: Relationship<'a>,
}

#[derive(Debug, PartialEq)]
pub struct Filters<'a> {
    pub filter_vector: Vec<Filter<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum Filter<'a> {
    Label(Identifier<'a>, Identifier<'a>), // xyz : Person
    Property(Identifier<'a>, Vec<Identifier<'a>>, Value), // xyz.abc == 5, xyz.a.b.c == 5
}

#[derive(Debug, PartialEq)]
pub enum Relationship<'a> {
    Path(Identifier<'a>),
    Edge(Identifier<'a>),
}

#[derive(Debug, PartialEq)]
pub struct Identifier<'a> {
    pub id_name: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct Value {
    pub value: u32,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value.to_string())
    }
}
