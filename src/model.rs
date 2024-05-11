#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GraphDocument {
    pub graphs: Vec<Graph>,
    pub meta: Option<Box<Meta>>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Graph {
    pub id: String,
    pub lbl: Option<String>,
    pub meta: Option<Box<Meta>>,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    // TODO: continue with adding the following:
    //  - equivalentNodesSets
    //  - logicalDefinitionAxioms
    //  - domainRangeAxioms
    //  - propertyChainAxioms
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RdfTypes {
    #[cfg_attr(feature = "serde", serde(rename(serialize = "CLASS", deserialize = "CLASS")))]
    Class,
    #[cfg_attr(feature = "serde", serde(rename(serialize = "INDIVIDUAL", deserialize = "INDIVIDUAL")))]
    Individual,
    #[cfg_attr(feature = "serde", serde(rename(serialize = "PROPERTY", deserialize = "PROPERTY")))]
    Property,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Node {
    pub id: String,
    pub lbl: Option<String>,
    #[cfg_attr(feature = "serde", serde(rename(serialize = "type", deserialize = "type")))]
    pub rdf_type: Option<RdfTypes>,
    pub meta: Option<Meta>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Edge {
    pub sub: String,
    pub pred: String,
    pub obj: String,
    pub meta: Option<Meta>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Meta {
    pub definition: Option<DefinitionPropertyValue>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub comments: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub subsets: Vec<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub synonyms: Vec<SynonymPropertyValue>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub xrefs: Vec<XrefPropertyValue>,
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            rename(serialize = "basicPropertyValues", deserialize = "basicPropertyValues")
        )
    )]
    pub basic_property_values: Vec<BasicPropertyValue>,
    pub version: Option<String>,
    pub deprecated: Option<bool>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BasicPropertyValue {
    pub pred: String,
    pub val: String,
    pub xrefs: Option<Vec<String>>,
    pub meta: Option<Box<Meta>>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DefinitionPropertyValue {
    pub pred: Option<String>,
    pub val: String,
    pub xrefs: Option<Vec<String>>,
    pub meta: Option<Box<Meta>>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SynonymPropertyValue {
    pub pred: String,
    pub val: String,
    pub xrefs: Option<Vec<String>>,
    pub meta: Option<Box<Meta>>,
    #[cfg_attr(
        feature = "serde",
        serde(rename(serialize = "synonymType", deserialize = "synonymType"))
    )]
    pub synonym_type: Option<String>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct XrefPropertyValue {
    pub pred: Option<String>,
    pub val: String,
    pub xrefs: Option<Vec<String>>,
    pub meta: Option<Box<Meta>>,
    pub lbl: Option<String>,
}
