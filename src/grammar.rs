use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Prog<'a> {
    pub patterns: Patterns<'a>,
    pub filters: Filters<'a>,
    pub action: Action<'a>,
}

#[derive(Debug, Default, PartialEq)]
pub struct Patterns<'a>(pub Vec<Pattern<'a>>);

impl<'a> Patterns<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug, PartialEq)]
pub struct Pattern<'a> {
    pub from_node: Identifier<'a>,
    pub to_node: Identifier<'a>,
    pub relationship_type: Relationship<'a>,
}

#[derive(Debug, Default, PartialEq)]
pub struct Filters<'a>(pub Vec<Filter<'a>>);

impl<'a> Filters<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug, PartialEq)]
pub enum Filter<'a> {
    Property(Identifier<'a>, Identifier<'a>, Value<'a>), // xyz.abc == 5, xyz.b == 5, x.a == k
}

#[derive(Debug, PartialEq)]
pub enum Action<'a> {
    None,
    Property(Identifier<'a>, Identifier<'a>), // xyz.a, xyz.b
    CallUdf(Identifier<'a>, Identifier<'a>),  // f(a)
}

impl<'a> fmt::Display for Action<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::None => write!(f, "none"),
            Action::Property(id, p) => {
                let mut result = String::new();
                result.push_str(&id.to_string());
                result.push_str(".");
                result.push_str(&p.to_string());
                write!(f, "{}", result)
            }
            Action::CallUdf(id, p) => {
                let mut result = String::new();
                result.push_str(&id.to_string());
                result.push_str("(");
                result.push_str(&p.to_string());
                result.push_str(")");
                write!(f, "{}", result)
            }
        }
    }
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

impl<'a> fmt::Display for Identifier<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id_name)
    }
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
