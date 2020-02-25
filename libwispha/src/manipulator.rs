use crate::core::*;
use crate::strings::*;

use std::fmt;
use std::error;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;

impl NodePath {
    /// Create a root node path
    pub fn new(tree: &Tree) -> NodePath {
        NodePath {
            components: vec![],
            tree: Rc::downgrade(&tree.0)
        }
    }

    /// Convert the node path to an absolute unix-style path string
    pub fn to_string(&self) -> String {
        format!("{root}{components}", root=ROOT, components=self.components.join(PATH_SEPARATOR))
    }

    /// Convert an absolute unix-style path string to node path.
    ///
    /// If path string is not absolute, return `Error::NodePathMustBeAbsolute`
    pub fn from(raw_path: &String, tree: &Tree) -> Result<NodePath, Error> {
        let mut path = raw_path.clone();
        if path.starts_with("/") {
            path = path[1..].to_string();
            if path.ends_with("/") {
                path.pop();
            }
            let components = if path.is_empty() {
                vec![]
            } else {
                path.split("/").map(|component| component.to_string()).collect::<Vec<String>>()
            };
            Ok(NodePath {
                components,
                tree: Rc::downgrade(&tree.0)
            })
        } else {
            Err(Error::NodePathMustBeAbsolute(raw_path.clone()))
        }
    }

    /// Push a component to the node path
    pub fn push(&self, component: String) -> NodePath {
        let mut components = self.components.clone();
        components.push(component);
        NodePath {
            components,
            tree: self.tree.clone()
        }
    }

    /// The parent node path of current node path. Return `None` if current node path is root
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

    /// Return the last component of current node path. Return `None` if current node path is root
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

    pub fn get_direct(&self) -> Option<&DirectNode> {
        if let Node::Direct(direct_node) = &self {
            Some(direct_node)
        } else {
            None
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

    fn clear(&mut self) {
        self.nodes.clear()
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

    /// Get the os-related path of `node` in `tree`
    pub fn get_path_buf(&self, node_path: &NodePath) -> Result<PathBuf, Error> {
        let node = self.get_node(node_path).ok_or(Error::PathNotFound(node_path.clone()))?;
        let node = node.borrow();
        if let Some(parent) = &node.node_properties().parent {
            Ok(self.get_path_buf(parent)?.join(node.node_properties().name))
        } else {
            // is root
            let dir = node.node_properties().record_file.parent().unwrap().to_path_buf();
            Ok(dir)
        }
    }

    /// Clear all the nodes in the tree
    pub fn clear(&self) {
        self.0.borrow_mut().clear()
    }

    /// Resolve to make sure tree has a direct node value of key `node_path`.
    ///
    /// `resolve_handler` does two things:
    /// * Convert `link_node`'s `target` to a node_str contains the `target`'s content
    /// * Check if a direct loop exists, i.e. the `link_node`'s `target` is its `record_file`
    pub fn resolve_node<F>(&self,
                           node_path: &NodePath,
                           resolve_handler: &F,
                           preserved_keys: &Vec<&'static str>) -> Result<(), Error>
        where
            F: Fn(&LinkNode) -> Result<(PathBuf, String), Box<dyn error::Error>> {
        if let Some(node) = self.get_node(node_path) {
            if let Node::Link(link_node) = &*node.borrow() {
                let (path, node_str) = resolve_handler(link_node).map_err(|error| Error::Custom(error))?;
                let parent_and_given_name = link_node.node_properties.parent.clone()
                                                     .map(|parent| (parent, link_node.node_properties.name.clone()));
                self.insert_nodes_from_str(&node_str,
                                           path,
                                           parent_and_given_name,
                                           preserved_keys)
                    .map_err(|de_error| Error::Custom(Box::new(de_error)))?;
                // in case of `target` of `link_node` is still a link node
                self.resolve_node(node_path, resolve_handler, preserved_keys)?;
            }
            Ok(())
        } else {
            if let Some(parent) = &node_path.parent() {
                self.resolve_node(&parent, resolve_handler, preserved_keys)?;
                if let Some(node) = self.get_node(node_path) {
                    if let Node::Link(link_node) = &*node.borrow() {
                        let (path, node_str) = resolve_handler(link_node).map_err(|error| Error::Custom(error))?;
                        let parent_and_given_name = link_node.node_properties.parent.clone()
                                                             .map(|parent| (parent, link_node.node_properties.name.clone()));
                        self.insert_nodes_from_str(&node_str,
                                                   path,
                                                   parent_and_given_name,
                                                   preserved_keys)
                            .map_err(|de_error| Error::Custom(Box::new(de_error)))?;
                        // in case of `target` of `link_node` is still a link node
                        self.resolve_node(node_path, resolve_handler, preserved_keys)?;
                    }
                    Ok(())
                } else {
                    Err(Error::PathNotFound(node_path.clone()))
                }
            } else {
                Err(Error::PathNotFound(node_path.clone()))
            }
        }
    }

    /// Update the `tree`'s `nodes`, starting from `node_path`, with depth `depth`, to direct node,
    /// using `resolve_handler` to convert from `PathBuf` to `Node`.
    ///
    /// If this function returns `Ok`, it means two things:
    /// * The `node_path` does exist in the tree
    /// * The children of `node_path` in `depth` (if exists) are of type `DirectNode`
    ///
    /// The `node_path` itself's `depth` is 0
    ///
    /// `resolve_handler` does two things:
    /// * Convert `link_node`'s `target` to a node_str contains the `target`'s content
    /// * Check if a direct loop exists, i.e. the `link_node`'s `target` is its `record_file`
    pub fn resolve_in_depth<F>(&self,
                               node_path: &NodePath,
                               depth: usize,
                               resolve_handler: &F,
                               preserved_keys: &Vec<&'static str>) -> Result<(), Error>
        where
            F: Fn(&LinkNode) -> Result<(PathBuf, String), Box<dyn error::Error>> {
        let node = self.get_node(node_path).ok_or(Error::PathNotFound(node_path.clone()))?;
        let node = &*node.borrow();
        match node {
            Node::Direct(direct_node) => {
                if depth > 0 {
                    for child in &direct_node.children {
                        self.resolve_in_depth(child,
                                              depth - 1,
                                              resolve_handler,
                                              preserved_keys)?;
                    }
                }
            },
            Node::Link(link_node) => {
                let (path, node_str) = resolve_handler(link_node).map_err(|error| Error::Custom(error))?;
                let parent_and_given_name = link_node.node_properties.parent.clone()
                                                     .map(|parent| (parent, link_node.node_properties.name.clone()));
                self.insert_nodes_from_str(&node_str,
                                           path,
                                           parent_and_given_name,
                                           preserved_keys)
                    .map_err(|de_error| Error::Custom(Box::new(de_error)))?;
                // in case of `target` of `link_node` is still a link node
                self.resolve_in_depth(node_path, depth, resolve_handler,
                                      preserved_keys)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    PathNotFound(NodePath),
    Custom(Box<dyn error::Error>),
    NodePathMustBeAbsolute(String)
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Error::*;
        let message = match &self {
            PathNotFound(path) => format!("Path {} not found.", path),
            Custom(error) => format!("{}", error),
            NodePathMustBeAbsolute(path) => format!("Node path must be absolute, but {} is not.", path)
        };
        write!(f, "{}", message)
    }
}