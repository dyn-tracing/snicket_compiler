use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Prog<'a> {
    pub patterns: Patterns<'a>,
    pub filters: Filters<'a>,
    pub actions: Actions<'a>,
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
    Property(Identifier<'a>, Vec<Identifier<'a>>, Value<'a>), // xyz.abc == 5, xyz.a.b.c == 5, x.a == k
}

#[derive(Debug, Default, PartialEq)]
pub struct Actions<'a>(pub Vec<Action<'a>>);

impl<'a> Actions<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug, PartialEq)]
pub enum Action<'a> {
    Property(Identifier<'a>, Vec<Identifier<'a>>), // xyz.abc, xyz.a.b.c
}

impl<'a> fmt::Display for Action<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Action::Property(id, id_vec) = self;
        let mut result = String::new();
        result.push_str(&id.to_string());
        for i in id_vec {
            result.push_str(".");
            result.push_str(&i.to_string());
        }
        write!(f, "{}", result)
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
