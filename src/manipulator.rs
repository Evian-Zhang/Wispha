use crate::core::*;
use crate::strings::*;
use crate::serde::de;

use std::fmt;
use std::error;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::HashMap;

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

impl fmt::Display for NodePath {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_string())
    }
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

impl Tree {
    pub fn new(config: &TreeConfig) -> Tree {
        Tree {
            nodes: HashMap::new(),
            root: Weak::new(),
            config: config.clone()
        }
    }

    /// Get node from the node_path
    pub fn get_node(&self, node_path: &NodePath) -> Option<Rc<RefCell<Node>>> {
        self.nodes.get(&node_path.components)
            .map(|node_ref| Rc::clone(node_ref))
    }

    /// Insert node with `node_path` and `node`.
    /// If the `node_path` has already existed, the node is updated, and the old one is returned.
    pub fn insert_node(&mut self, node_path: NodePath, node: Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
        self.nodes.insert(node_path.components, node)
    }

    /// Get the node path of `node` in `tree`
    pub fn get_node_path(tree: Rc<RefCell<Tree>>, node: Rc<RefCell<Node>>) -> NodePath {
        if let Some(parent) = node.borrow().node_properties().parent {
            parent.push(node.borrow().node_properties().name.clone())
        } else {
            NodePath::new(&Rc::downgrade(&tree))
        }
    }

    /// Update the `tree`'s `nodes`, starting from `node_path`, with depth `depth`, to direct node,
    /// using `resolve_handler` to convert from `PathBuf` to `Node`.
    ///
    /// `resolve_handler`'s error return type can be `Error::Custom`
    pub fn resolve_in_depth<F>(tree: Rc<RefCell<Tree>>,
                               node_path: &NodePath,
                               depth: usize,
                               resolve_handler: &F) -> Result<(), Error>
        where
            F: Fn(Rc<RefCell<Tree>>, &LinkNode) -> Result<Rc<RefCell<Node>>, Error> {
        if depth > 0 {
            let node = tree.borrow().get_node(node_path).ok_or(Error::PathNotFound(node_path.clone()))?;
            let node = &*node.borrow();
            match node {
                Node::Direct(direct_node) => {
                    for child in &direct_node.children {
                        Tree::resolve_in_depth(Rc::clone(&tree), child, depth - 1, resolve_handler)?;
                    }
                },
                Node::Link(link_node) => {
                    let node = resolve_handler(Rc::clone(&tree), link_node)?;
                    tree.borrow_mut().insert_node(node_path.clone(), node);
                    Tree::resolve_in_depth(Rc::clone(&tree), node_path, depth, resolve_handler)?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    PathNotFound(NodePath),
    Custom(Box<dyn error::Error>)
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Error::*;
        let message = match &self {
            PathNotFound(path) => format!("Path {} not found.", path),
            Custom(error) => format!("Custom error: {}", error)
        };
        write!(f, "{}", message)
    }
}