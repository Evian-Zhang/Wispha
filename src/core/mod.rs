use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, Weak};

use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer, SerializeStruct, Deserialize};

/// The properties that are related to the node itself, but not the truly valuable information.
#[derive(Clone)]
pub struct NodeProperties {
    pub name: String,
    pub record_file: PathBuf,
}

pub enum PropertyType {
    String,
    Date,
    Int,
    Double
}

/// The property with type. Support `String`, `Date` (stores in UTC), `Int` (with the same range as `isize` of Rust), `Double` (with the same range and precision as `f64` of Rust)
pub enum TypedProperty {
    String(String),
    Date(DateTime<Utc>),
    /// property with `Int` type, with the same range as `isize` of Rust
    Int(isize),
    /// property with `Double` type, with the same range and precision as `f64` of Rust
    Double(f64),
}

/// Direct node structure, i.e. the node that truly has valuable values
pub struct DirectNode {
    /// If a Wispha node doesn't have parent (for example, `root` in a Wispha tree), this field is `None`
    pub parent: Option<PathBuf>,

    /// If a Wispha node doesn't have any child, this field is an vector with length 0
    pub children: Vec<PathBuf>,

    /// The properties that are related to the node itself, but not the truly valuable information.
    pub node_properties: NodeProperties,

    /// Customized properties in a direct node, supporting `String`, `Date`, `Int` and `Double`.
    pub properties: HashMap<String, TypedProperty>,
}

pub struct LinkNode {
    /// The properties that are related to the node itself, but not the truly valuable information.
    pub node_properties: NodeProperties,
}

/// Wispha node structure
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
        let mut state = serializer.serialize_struct("Tree")?;
        if !self.custom_properties.is_empty() {
            let root = self.root.upgrade().unwrap();

        }
        state.end()
    }
}
