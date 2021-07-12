/***********************************/
// IR Structs
/***********************************/
use indexmap::IndexSet;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct StructuralFilter {
    pub vertices: IndexSet<String>,
    pub edges: IndexSet<(String, String)>,
}
impl Default for StructuralFilter {
    fn default() -> Self {
        StructuralFilter {
            vertices: IndexSet::new(),
            edges: IndexSet::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
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
    pub root_id: String,
    pub struct_filters: Vec<StructuralFilter>,
    pub attr_filters: Vec<AttributeFilter>,
    pub return_expr: IrReturnEnum,
    pub properties: IndexSet<Property>,
    pub udf_calls: IndexSet<UdfCall>,
}

pub trait Expression {}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct UdfCall {
    pub id: String,
    //TODO: Args may also be UDF calls
    pub args: Vec<String>,
}

/*
//TODO: what do to with this?
impl UdfCall {
    pub fn to_ref_str(&self) -> String {
        // TODO: Make this a little bit less stringbuildery
        let mut udf_str = self.id.clone();
        udf_str.push('(');
        if let Some((last, front)) = self.args.split_last() {
            for arg in front {
                udf_str.push_str(&arg);
                udf_str.push_str(", ");
            }
            udf_str.push_str(last);
        }
        udf_str.push(')');
        udf_str
    }
}
*/

impl Expression for UdfCall {}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct Property {
    pub parent: String,
    pub members: Vec<String>,
}

impl Property {
    pub fn as_vec_str(&self) -> String {
        // TODO: Make this a little bit less stringbuildery
        let mut lst_str = "vec![".to_string();
        for member in &self.members {
            lst_str.push('\"');
            lst_str.push_str(member);
            lst_str.push_str("\", ");
        }
        lst_str.push(']');
        lst_str
    }
    pub fn to_dot_string(&self) -> String {
        // TODO: Make this a little bit less stringbuildery
        let mut udf_str = String::new();
        if let Some((last, front)) = self.members.split_last() {
            for member in front {
                udf_str.push_str(member);
                udf_str.push('.');
            }
            udf_str.push_str(last);
        }
        udf_str
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

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Aggregate {
    pub udf_reference: UdfCall,
    pub args: Vec<PropertyOrUDF>,
}
impl Aggregate {
    pub fn new_with_items(udf_reference: UdfCall, args: Vec<PropertyOrUDF>) -> Self {
        Aggregate {
            udf_reference,
            args,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize)]
pub enum PropertyOrUDF {
    Property(Property),
    UdfCall(UdfCall),
}

impl Default for PropertyOrUDF {
    fn default() -> Self {
        PropertyOrUDF::Property(Property::default())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum IrReturnEnum {
    Aggregate(Aggregate),
    PropertyOrUDF(PropertyOrUDF),
}

impl Default for IrReturnEnum {
    fn default() -> Self {
        IrReturnEnum::PropertyOrUDF(PropertyOrUDF::default())
    }
}

pub type EntityReference = String;
