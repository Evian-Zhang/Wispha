use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::fmt;
use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::strings::*;

type NodePathComponents = Vec<String>;

#[derive(Clone, Debug)]
pub struct NodePath {
    pub components: NodePathComponents,
    pub tree: Weak<RefCell<Tree>>
}

impl NodePath {
    pub fn to_string(&self) -> String {
        format!("{root}{components}", root=ROOT, components=self.components.join(PATH_SEPARATOR))
    }
}

impl Display for NodePath {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}

/// The properties that are related to the node itself, but not the truly valuable information.
#[derive(Clone, Serialize, Debug)]
pub struct NodeProperties {
    pub name: String,
    #[serde(skip)]
    pub record_file: NodePath,
}

#[derive(Debug)]
pub enum PropertyType {
    String,
    Date,
    Int,
    Double
}

/// The property with type. Support `String`, `Date` (stores in UTC), `Int` (with the same range as `isize` of Rust), `Double` (with the same range and precision as `f64` of Rust)
#[derive(Debug)]
pub enum TypedProperty {
    String(String),

    Date(DateTime<Utc>),

    /// property with `Int` type, with the same range as `isize` of Rust
    Int(isize),

    /// property with `Double` type, with the same range and precision as `f64` of Rust
    Double(f64),
}

/// Direct node structure, i.e. the node that truly has valuable values
#[derive(Debug)]
pub struct DirectNode {
    /// If a Wispha node doesn't have parent (for example, `root` in a Wispha tree), this field is `None`
    pub parent: Option<NodePath>,

    /// If a Wispha node doesn't have any child, this field is an vector with length 0
    pub children: Vec<NodePath>,

    /// The properties that are related to the node itself, but not the truly valuable information.
    pub node_properties: NodeProperties,

    /// Customized properties in a direct node, supporting `String`, `Date`, `Int` and `Double`.
    pub properties: HashMap<String, TypedProperty>,
}

#[derive(Serialize, Debug)]
pub struct LinkNode {
    /// The properties that are related to the node itself, but not the truly valuable information.
    #[serde(flatten)]
    pub node_properties: NodeProperties,
}

/// Wispha node structure
#[derive(Debug)]
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
#[derive(Debug)]
pub struct Tree {
    pub nodes: HashMap<NodePathComponents, Rc<RefCell<Node>>>,
    pub root: Weak<RefCell<Node>>,
    pub custom_properties: HashMap<String, PropertyType>,
}

impl Tree {
    pub fn get_node(&self, components: &NodePath) -> Option<Rc<RefCell<Node>>> {
        self.nodes.get(&components.components)
            .map(|node_ref| Rc::clone(node_ref))
    }
}