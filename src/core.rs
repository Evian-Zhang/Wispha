use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::path::PathBuf;

use serde::Serialize;

type NodePathComponents = Vec<String>;

#[derive(Clone, Debug, Default)]
pub struct NodePath {
    pub(crate) components: NodePathComponents,
    pub(crate) tree: Weak<RefCell<InnerTree>>
}

/// The properties that are shared by all type nodes
#[derive(Clone, Serialize, Debug)]
pub struct NodeProperties {
    pub name: String,
    /// If a Wispha node doesn't have parent (for example, `root` in a Wispha tree), this field is `None`
    #[serde(skip)]
    pub parent: Option<NodePath>,
    #[serde(skip)]
    pub record_file: PathBuf,
}

/// Direct node structure, i.e. the node that truly has valuable values
#[derive(Debug)]
pub struct DirectNode {
    /// If a Wispha node doesn't have any child, this field is an vector with length 0
    pub children: Vec<NodePath>,

    /// The properties that are related to the node itself, but not the truly valuable information.
    pub node_properties: NodeProperties,

    /// Customized properties in a direct node.
    pub properties: HashMap<String, String>,
}

/// Link node structure. Links to another Wispha file
#[derive(Serialize, Debug)]
pub struct LinkNode {
    /// The path of linked Wispha file, e.g. subdir/LOOKME.toml. Same in memory as in Wispha file.
    pub target: PathBuf,

    /// The properties that are related to the node itself, but not the truly valuable information.
    #[serde(flatten)]
    pub node_properties: NodeProperties,
}

/// Wispha node structure
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Node {
    Direct(DirectNode),
    Link(LinkNode),
}

#[derive(Debug, Clone)]
pub struct TreeConfig {
    pub project_name: String,
}

/// Wispha tree structure
#[derive(Debug)]
pub(crate) struct InnerTree {
    pub nodes: HashMap<NodePathComponents, Rc<RefCell<Node>>>,
    pub config: TreeConfig
}

pub struct Tree(pub(crate) Rc<RefCell<InnerTree>>);