use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, Weak};

use chrono::{DateTime, Utc};
use serde::ser::{Serializer, SerializeStruct};
use serde::{Serialize, Deserialize};

/// The properties that are related to the node itself, but not the truly valuable information.
#[derive(Clone, Serialize)]
pub struct NodeProperties {
    pub name: String,
    #[serde(skip)]
    pub record_file: PathBuf,
    #[serde(skip)]
    tree: Weak<Mutex<Tree>>,
}

pub enum PropertyType {
    String,
    Date,
    Int,
    Double
}

/// The property with type. Support `String`, `Date` (stores in UTC), `Int` (with the same range as `isize` of Rust), `Double` (with the same range and precision as `f64` of Rust)
#[derive(Serialize)]
pub enum TypedProperty {
    String(String),

    #[serde(with = "date_format")]
    Date(DateTime<Utc>),

    /// property with `Int` type, with the same range as `isize` of Rust
    Int(isize),

    /// property with `Double` type, with the same range and precision as `f64` of Rust
    Double(f64),
}

// Auxiliary module for serde with chrono
mod date_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
        where
            D: Deserializer<'de>, {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

/// Direct node structure, i.e. the node that truly has valuable values
#[derive(Serialize)]
pub struct DirectNode {
    /// If a Wispha node doesn't have parent (for example, `root` in a Wispha tree), this field is `None`
    pub parent: Option<PathBuf>,

    /// If a Wispha node doesn't have any child, this field is an vector with length 0
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty", serialize_with = "")]
    pub children: Vec<PathBuf>,

    /// The properties that are related to the node itself, but not the truly valuable information.
    #[serde(flatten)]
    pub node_properties: NodeProperties,

    /// Customized properties in a direct node, supporting `String`, `Date`, `Int` and `Double`.
    #[serde(flatten)]
    pub properties: HashMap<String, TypedProperty>,
}

#[derive(Serialize)]
pub struct LinkNode {
    /// The properties that are related to the node itself, but not the truly valuable information.
    #[serde(flatten)]
    pub node_properties: NodeProperties,
}

/// Wispha node structure
#[derive(Serialize)]
pub enum Node {
    Direct(DirectNode),
    Link(LinkNode),
}

impl Node {
    pub fn node_properties(&self) -> NodeProperties {
        use Node::*;
        match &self {
            Direct(direct_node) => direct_node.node_properties.clone(),
            Link(link_node) => link_node.node_properties.clone(),
        }
    }
}

//impl Serialize for Node {
//    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//        where
//            S: Serializer {
//        use Node::*;
//        let mut state = serializer.serialize_struct("Node", 0)?;
//        match &self {
//            Direct(direct_node) => {
//                state.serialize_field("name", &direct_node.node_properties.name);
//
//            },
//            Link(link_node) => {
//                state.serialize_field("name", &link_node.node_properties.name);
//
//            }
//        }
//        state.end()
//    }
//}

/// Wispha tree structure
pub struct Tree {
    pub nodes: HashMap<PathBuf, Arc<Mutex<Node>>>,
    pub root: Weak<Mutex<Node>>,
    pub custom_properties: HashMap<String, PropertyType>,
}

impl Serialize for Tree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        if !self.custom_properties.is_empty() {
            let mut state = serializer.serialize_struct("Tree", 0)?;
            let root = self.root.upgrade().unwrap();
            state.serialize_field("name", &root.lock().unwrap().node_properties().name)?;
            state.end()
        } else {
            serializer.serialize_none()
        }
    }
}
