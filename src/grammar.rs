use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Prog<'a> {
    pub patterns: Patterns<'a>,
    pub filters: Filters<'a>,
    pub actions: Actions<'a>,
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
    Property(Identifier<'a>, Vec<Identifier<'a>>, Value<'a>), // xyz.abc == 5, xyz.a.b.c == 5, x.a == k
}

#[derive(Debug, PartialEq)]
pub struct Actions<'a> {
    pub action_vector: Vec<Action<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum Action<'a> {
    Property(Identifier<'a>, Vec<Identifier<'a>>), // xyz.abc, xyz.a.b.c
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
pub enum Value<'a> {
    U32(u32),
    Str(&'a str),
}

impl<'a> fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::U32(i) => write!(f, "{}", i),
            Value::Str(s) => write!(f, "{}", s),
        }
    }
}
