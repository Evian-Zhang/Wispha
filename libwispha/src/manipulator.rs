use crate::core::*;
use crate::strings::*;

use std::fmt;
use std::error;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl NodePath {
    pub fn new(tree: &Tree) -> NodePath {
        NodePath {
            components: vec![],
            tree: Rc::downgrade(&tree.0)
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

    pub(crate) fn tree(&self) -> Tree {
        Tree(self.tree.upgrade().unwrap())
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

impl InnerTree {
    fn new(config: &TreeConfig) -> InnerTree {
        InnerTree {
            nodes: HashMap::new(),
            config: config.clone()
        }
    }

    pub(crate) fn root(&self) -> Option<Rc<RefCell<Node>>> {
        let root_path = vec![];
        self.nodes.get(&root_path).map(|node_ref| Rc::clone(&node_ref))
    }

    fn get_node(&self, node_path: &NodePath) -> Option<Rc<RefCell<Node>>> {
        self.nodes.get(&node_path.components)
            .map(|node_ref| Rc::clone(node_ref))
    }

    fn insert_node(&mut self, node_path: NodePath, node: Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
        self.nodes.insert(node_path.components, node)
    }
}

impl Tree {
    pub fn new(config: &TreeConfig) -> Tree {
        Tree(Rc::new(RefCell::new(InnerTree::new(config))))
    }

    /// Get root. If the tree has no node, return `None`.
    pub fn root(&self) -> Option<Rc<RefCell<Node>>> {
        self.0.borrow().root()
    }

    pub fn config(&self) -> TreeConfig {
        self.0.borrow().config.clone()
    }

    /// Get node from the node_path
    pub fn get_node(&self, node_path: &NodePath) -> Option<Rc<RefCell<Node>>> {
        self.0.borrow().get_node(node_path)
    }

    /// Insert node with `node_path` and `node`.
    /// If the `node_path` has already existed, the node is updated, and the old one is returned.
    pub fn insert_node(&self, node_path: NodePath, node: Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
        self.0.borrow_mut().insert_node(node_path, node)
    }

    /// Get the node path of `node` in `tree`
    pub fn get_node_path(&self, node: Rc<RefCell<Node>>) -> NodePath {
        if let Some(parent) = node.borrow().node_properties().parent {
            parent.push(node.borrow().node_properties().name.clone())
        } else {
            NodePath::new(&self)
        }
    }

    /// Update the `tree`'s `nodes`, starting from `node_path`, with depth `depth`, to direct node,
    /// using `resolve_handler` to convert from `PathBuf` to `Node`.
    ///
    /// `resolve_handler`'s error return type can be `Error::Custom`
    pub fn resolve_in_depth<F>(&self,
                               node_path: &NodePath,
                               depth: usize,
                               resolve_handler: &F) -> Result<(), Error>
        where
            F: Fn(&Tree, &LinkNode) -> Result<Rc<RefCell<Node>>, Error> {
        if depth > 0 {
            let node = self.get_node(node_path).ok_or(Error::PathNotFound(node_path.clone()))?;
            let node = &*node.borrow();
            match node {
                Node::Direct(direct_node) => {
                    for child in &direct_node.children {
                        self.resolve_in_depth(child, depth - 1, resolve_handler)?;
                    }
                },
                Node::Link(link_node) => {
                    let node = resolve_handler(&self, link_node)?;
                    self.insert_node(node_path.clone(), node);
                    self.resolve_in_depth(node_path, depth, resolve_handler)?;
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