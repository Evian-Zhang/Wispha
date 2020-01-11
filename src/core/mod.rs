use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, Weak};
use std::hash::{Hash, Hasher};
use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::ser::{Serializer, SerializeStruct, SerializeMap};
use serde::{Serialize, Deserialize};
use serde::export::Formatter;
use serde::export::fmt::Error;

type NodePathComponents = Vec<String>;

#[derive(Clone, Debug)]
pub struct NodePath<'a> {
    pub components: NodePathComponents,
    pub tree: &'a Tree<'a>,
}

impl<'a> NodePath<'a> {
    pub fn to_string(&self) -> String {
        // TODO: add a singleton String "/" instead of creating a String every time
        "/".to_owned() + &self.components.join("/")
    }
}

impl Display for NodePath<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.to_string())
    }
}

//impl Serialize for NodePath<'_> {
//    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//        where
//            S: Serializer {
//        serializer.serialize_str(&self.to_string())
//    }
//}

/// The properties that are related to the node itself, but not the truly valuable information.
#[derive(Clone, Serialize, Debug)]
pub struct NodeProperties<'a> {
    pub name: String,
    #[serde(skip)]
    pub record_file: NodePath<'a>,
}

#[derive(Debug)]
pub enum PropertyType {
    String,
    Date,
    Int,
    Double
}

/// The property with type. Support `String`, `Date` (stores in UTC), `Int` (with the same range as `isize` of Rust), `Double` (with the same range and precision as `f64` of Rust)
#[derive(Serialize, Debug)]
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
#[derive(Debug)]
pub struct DirectNode<'a> {
    /// If a Wispha node doesn't have parent (for example, `root` in a Wispha tree), this field is `None`
    pub parent: Option<NodePath<'a>>,

    /// If a Wispha node doesn't have any child, this field is an vector with length 0
    pub children: Vec<NodePath<'a>>,

    /// The properties that are related to the node itself, but not the truly valuable information.
    pub node_properties: NodeProperties<'a>,

    /// Customized properties in a direct node, supporting `String`, `Date`, `Int` and `Double`.
    pub properties: HashMap<String, TypedProperty>,
}

impl<'a> Serialize for DirectNode<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        // TODO: find a more graceful way
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("name", &self.node_properties.name)?;
        for (property, value) in self.properties.iter() {
            map.serialize_entry(property, value)?;
        }
        if !self.children.is_empty() {
            let children: Vec<Arc<Mutex<Node>>> = self.children.iter().map(|node_path| node_path.tree.get_node(node_path).unwrap()).collect();
            map.serialize_entry("children", &children)?;
        }
        map.end()
    }
}

#[derive(Serialize, Debug)]
pub struct LinkNode<'a> {
    /// The properties that are related to the node itself, but not the truly valuable information.
    #[serde(flatten)]
    pub node_properties: NodeProperties<'a>,
}

/// Wispha node structure
#[derive(Serialize, Debug)]
pub enum Node<'a> {
    Direct(DirectNode<'a>),
    Link(LinkNode<'a>),
}

impl<'a> Node<'a> {
    pub fn node_properties(&'a self) -> NodeProperties {
        use Node::*;
        match &self {
            Direct(direct_node) => direct_node.node_properties.clone(),
            Link(link_node) => link_node.node_properties.clone(),
        }
    }
}

/// Wispha tree structure
#[derive(Debug)]
pub struct Tree<'a> {
    pub nodes: HashMap<NodePathComponents, Arc<Mutex<Node<'a>>>>,
    pub root: Weak<Mutex<Node<'a>>>,
    pub custom_properties: HashMap<String, PropertyType>,
}

impl Serialize for Tree<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        if !self.custom_properties.is_empty() {
            self.root.upgrade().unwrap().lock().unwrap().serialize(serializer)
        } else {
            serializer.serialize_none()
        }
    }
}

impl<'a> Tree<'a> {
    pub fn get_node(&self, components: &NodePath) -> Option<Arc<Mutex<Node<'a>>>> {
        self.nodes.get(&components.components)
            .map(|node_ref| Arc::clone(node_ref))
    }
}
