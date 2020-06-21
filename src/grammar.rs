#[derive(Debug)]
#[derive(PartialEq)]
pub struct Prog<'a> {
  pub patterns : Patterns<'a>,
  pub filters  : Filters<'a>
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Patterns<'a> {
  pub pattern_vector : Vec<Pattern<'a>>
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Pattern<'a> {
  pub from_node         : Identifier<'a>,
  pub to_node           : Identifier<'a>,
  pub relationship_type : Relationship
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Filters<'a> {
  pub filter_vector : Vec<Filter<'a>>
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Filter<'a> {
  Label(Identifier<'a>, Identifier<'a>), // xyz : Person
  Property(Identifier<'a>, Identifier<'a>, Value) // xyz.abc == 5
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Relationship {
  Path(),
  Edge()
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Identifier<'a> {
  pub id_name : &'a str,
}

impl<'a> Identifier<'a> {
  pub fn get_str(&self) -> &str{
    return self.id_name;
  }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Value {
  pub value : u32
}

impl Value {
  pub fn get_string(&self) -> String {
    return self.value.to_string();
  }
}
