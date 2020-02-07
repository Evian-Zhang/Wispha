use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::fmt::{self, Display};
use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use crate::strings::*;

type NodePathComponents = Vec<String>;

#[derive(Clone, Debug, Default)]
pub struct NodePath {
    pub components: NodePathComponents,
    pub tree: Weak<RefCell<Tree>>
}

impl NodePath {
    pub fn new(tree: &Weak<RefCell<Tree>>) -> NodePath {
        NodePath {
            components: vec![],
            tree: tree.clone()
        }
    }

    pub fn to_string(&self) -> String {
        format!("{root}{components}", root=ROOT, components=self.components.join(PATH_SEPARATOR))
    }

    pub fn push(&self, component: String) -> NodePath {
        let mut components = self.components.clone();
        components.push(component);
        NodePath {
            components,
            tree: self.tree.clone()
        }
    }

    pub fn parent(&self) -> Option<NodePath> {
        let mut components = self.components.clone();
        if let Some(_) = components.pop() {
            Some(NodePath {
                components,
                tree: self.tree.clone()
            })
        } else {
            None
        }
    }

    pub fn name(&self) -> Option<String> {
        self.components.last().cloned()
    }
}

impl Display for NodePath {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}

/// The properties that are related to the node itself, but not the truly valuable information.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NodeProperties {
    pub name: String,
    #[serde(skip)]
    pub record_file: PathBuf,
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

    /// Customized properties in a direct node.
    pub properties: HashMap<String, String>,
}

/// Link node structure. Links to another Wispha file
#[derive(Serialize, Debug)]
pub struct LinkNode {
    /// The properties that are related to the node itself, but not the truly valuable information.
    #[serde(flatten)]
    pub node_properties: NodeProperties,
    /// The path of linked Wispha file, e.g. subdir/LOOKME.toml. Same in memory as in Wispha file.
    pub target: PathBuf
}

/// Wispha node structure
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
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

#[derive(Debug, Clone)]
pub struct TreeConfig {
    pub project_name: String,
}

/// Wispha tree structure
#[derive(Debug)]
pub struct Tree {
    pub nodes: HashMap<NodePathComponents, Rc<RefCell<Node>>>,
    pub root: Weak<RefCell<Node>>,
    pub config: TreeConfig
}

impl Tree {
    pub fn new(config: &TreeConfig) -> Tree {
        Tree {
            nodes: HashMap::new(),
            root: Weak::new(),
            config: config.clone()
        }
    }

    pub fn get_node(&self, components: &NodePath) -> Option<Rc<RefCell<Node>>> {
        self.nodes.get(&components.components)
            .map(|node_ref| Rc::clone(node_ref))
    }
}