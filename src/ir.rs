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

pub struct VisitorResults {
    pub struct_filters: Vec<StructuralFilter>,
    pub attr_filters: Vec<AttributeFilter>,
    pub return_expr: IrReturnEnum,
    pub maps: Vec<String>,
    pub root_id: String,
    pub properties: Vec<Property>,
    pub udf_calls: Vec<UdfCall>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Aggregate {
    pub udf_reference: String,
    pub property: String,
}
impl Aggregate {
    pub fn new_with_items(udf_reference: String, property: String) -> Self {
        Aggregate {
            udf_reference,
            property,
        }
    }
}

pub trait Expression {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UdfCall {
    pub id: String,
    //TODO: Args may also be UDF calls
    pub args: Vec<String>,
}

impl UdfCall {
    pub fn to_ref_str(&self) -> String {
        // TODO: Make this a little bit less stringbuildery
        let mut udf_str = self.id.clone();
        udf_str.push_str("(");
        if let Some((last, front)) = self.args.split_last() {
            for arg in front {
                udf_str.push_str(&arg);
                udf_str.push_str(", ");
            }
            udf_str.push_str(last);
        }
        udf_str.push_str(")");
        return udf_str;
    }
}

impl Expression for UdfCall {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Property {
    pub parent: String,
    //TODO: Args may also be UDF calls
    pub members: Vec<String>,
}

impl Property {
    pub fn as_list_str(&self) -> String {
        // TODO: Make this a little bit less stringbuildery
        let mut lst_str = "[".to_string();
        // lst_str.push_str(&self.parent);
        // lst_str.push_str("\"");
        for member in &self.members {
            lst_str.push_str(", \"");
            lst_str.push_str(&member);
            lst_str.push_str("\"");
        }
        lst_str.push_str("]");
        return lst_str;
    }
}
impl Expression for Property {}
impl Default for Property {
    fn default() -> Self {
        Property {
            parent: "".to_string(),
            members: Vec::new(),
        }
    }
}

pub struct AggregateAlt {
    pub udf_reference: PropertyOrUDF,
    pub property: PropertyOrUDF,
}
impl AggregateAlt {
    pub fn new_with_items(udf_reference: PropertyOrUDF, property: PropertyOrUDF) -> Self {
        AggregateAlt {
            udf_reference,
            property,
        }
    }
}

#[derive(Clone, Debug)]
pub enum PropertyOrUDF {
    Property(Property),
    UdfCall(UdfCall),
}

pub enum IrReturnEnum {
    AggregateAlt(AggregateAlt),
    PropertyOrUDF(PropertyOrUDF),
}

pub type EntityReference = String;
