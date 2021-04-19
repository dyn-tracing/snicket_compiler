/***********************************/
// IR Structs
/***********************************/
use indexmap::map::IndexMap;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct StructuralFilter {
    pub vertices: Vec<String>,
    pub edges: Vec<(String, String)>,
    pub properties: IndexMap<String, IndexMap<String, String>>, // attribute, value
}
impl Default for StructuralFilter {
    fn default() -> Self {
        StructuralFilter {
            vertices: Vec::new(),
            edges: Vec::new(),
            properties: IndexMap::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct AttributeFilter {
    pub node: String,
    pub property: String,
    pub value: String,
}
impl Default for AttributeFilter {
    fn default() -> Self {
        AttributeFilter {
            node: String::new(),
            property: String::new(),
            value: String::new(),
        }
    }
}

impl AttributeFilter {
    #[allow(dead_code)]
    pub fn insert_values(&mut self, node: String, property: String, value: String) {
        self.node = node;
        self.property = property;
        self.value = value;
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct IrReturn {
    pub entity: String,
    pub property: String,
}

impl IrReturn {
    pub fn new_with_items(entity: String, property: String) -> Self {
        IrReturn { entity, property }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct VisitorResults {
    pub struct_filters: Vec<StructuralFilter>,
    pub attr_filters: Vec<AttributeFilter>,
    pub return_expr: Option<IrReturn>,
    pub aggregate: Option<Aggregate>,
    pub maps: Vec<String>,
    pub root_id: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Aggregate {
    pub udf_id: String,
    pub entity: String,
    pub property: String,
}
impl Aggregate {
    pub fn new_with_items(entity: String, property: String, udf: String) -> Self {
        Aggregate {
            udf_id: udf,
            entity,
            property,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UdfCall {
    pub id: String,
    //TODO: Args may also be UDF calls
    pub args: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Property {
    pub parent: String,
    //TODO: Args may also be UDF calls
    pub members: Vec<String>,
}

type EntityReference = String;
