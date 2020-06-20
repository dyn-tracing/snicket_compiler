#[derive(Debug)]
#[derive(PartialEq)]
pub struct Prog<'a> {
  pub patterns : Patterns<'a>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Patterns<'a> {
  pub pattern_vector : Vec<Pattern<'a>>
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Pattern<'a> {
  pub from_node         : QualifiedIdentifier<'a>,
  pub to_node           : QualifiedIdentifier<'a>,
  pub relationship_type : Relationship<'a>
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct QualifiedIdentifier<'a> {
  pub labels     : Labels<'a>,
  pub properties : Properties<'a>,
  pub identifier : Identifier<'a>
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Labels<'a> {
  pub label_vector   : Vec<Label<'a>>
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Properties<'a> {
  pub property_vector   : Vec<Property<'a>>
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Label<'a> {
  pub label : Identifier<'a>
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Property<'a> {
  pub key : Identifier<'a>,
  pub val : Value<'a>
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Relationship<'a> {
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
