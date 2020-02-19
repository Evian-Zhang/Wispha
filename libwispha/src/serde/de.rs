use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::path::PathBuf;

use crate::core::*;
use crate::strings::*;

use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
struct RawNode {
    #[serde(flatten)]
    properties: HashMap<String, String>,
    children: Option<Vec<Rc<RefCell<RawNode>>>>
}

enum RawNodeType {
    Direct,
    Link
}

impl RawNodeType {
    fn from_str(s: &str) -> Option<RawNodeType> {
        use RawNodeType::*;
        match s {
            "Direct" => Some(Direct),
            "Link" => Some(Link),
            _ => None
        }
    }
}

impl Default for RawNodeType {
    fn default() -> Self {
        RawNodeType::Direct
    }
}

impl RawNode {
    // external call:
    //      if is root, no parent, give tree.config.project_name as given_name;
    //      else give parent, give link_node.node_properties.name as given_name
    // recursive call:
    //      give parent, give given_name
    // The upmost node is in the tail of returned vec
    fn convert_to_nodes(raw_node: &Rc<RefCell<RawNode>>,
                        parent: Option<NodePath>,
                        given_name: Option<String>,
                        tree: &Tree,
                        record_file: &PathBuf,
                        preserved_keys: &Vec<&'static str>) -> Result<Vec<(NodePath, Rc<RefCell<Node>>)>, Error> {
        let raw_node_type = if let Some(node_type_str) = raw_node.borrow().properties.get("type") {
            RawNodeType::from_str(node_type_str).ok_or(Error::UnknownType(node_type_str.to_owned()))?
        } else {
            RawNodeType::default()
        };
        match raw_node_type {
            RawNodeType::Direct => {
                // given name is prior to the recorded name
                let name = if let Some(name) = given_name {
                    name
                } else {
                    raw_node.borrow().properties.get(NAME).cloned().ok_or(Error::LackName)?
                };
                let path = if let Some(parent) = &parent {
                    parent.push(name.clone())
                } else {
                    NodePath::new(tree)
                };

                let node_properties = NodeProperties {
                    name,
                    parent,
                    record_file: record_file.clone(),
                };

                // the vector of vector of children of children
                let sub_node_children = if let Some(children) = &raw_node.borrow().children {
                    children.iter()
                        .map(|sub_node| -> Result<_, Error> {
                            RawNode::convert_to_nodes(sub_node, Some(path.clone()), None, tree, record_file, preserved_keys)
                        })
                        .collect::<Result<Vec<_>, Error>>()?
                } else {
                    vec![]
                };

                let sub_nodes = sub_node_children.iter().map(|children| {
                    children.last().unwrap().0.clone()
                }).collect::<Vec<_>>();

                let raw_node = raw_node.borrow();
                let keys = raw_node.properties.keys();

                for key in keys {
                    if preserved_keys.contains(&key.as_str()) {
                        return Err(Error::PreservedKey(key.clone()));
                    }
                }

                let node = Rc::new(RefCell::new(Node::Direct(DirectNode {
                    children: sub_nodes,
                    node_properties,
                    properties: raw_node.properties.clone(),
                })));

                let mut nodes = sub_node_children.into_iter().flatten().collect::<Vec<_>>();

                nodes.push((path, node));
                Ok(nodes)
            },
            RawNodeType::Link => {
                let name = if let Some(name) = given_name {
                    name
                } else {
                    raw_node.borrow().properties.get("name").cloned().ok_or(Error::LackName)?
                };
                let path = if let Some(parent) = &parent {
                    parent.push(name.clone())
                } else {
                    NodePath::new(tree)
                };
                let node_properties = NodeProperties {
                    name,
                    parent,
                    record_file: record_file.clone(),
                };
                let link_node = Rc::new(RefCell::new(Node::Link(LinkNode {
                    node_properties,
                    target: PathBuf::from(raw_node.borrow().properties.get("target").cloned().ok_or(Error::LackTarget)?)
                })));
                Ok(vec![(path, link_node)])
            },
        }
    }
}

impl Tree {
    /// Insert nodes from TOML string `node_str` in `recorded_file` to `tree`.
    ///
    /// If `node_str` is the root of Wispha tree, `parent_and_given_name` should be `None`;
    /// else `parent` should be the `node_str`'s parent, `given_name` should be the link node's `name`
    pub fn insert_nodes_from_str(&self,
                                 node_str: &str,
                                 recorded_file: PathBuf,
                                 parent_and_given_name: Option<(NodePath, String)>,
                                 preserved_keys: &Vec<&'static str>) -> Result<Rc<RefCell<Node>>, Error> {
        let raw_node = toml::from_str::<RawNode>(node_str).map_err(|error| Error::ParsingFailed(error))?;
        let raw_node = Rc::new(RefCell::new(raw_node));
        let (parent, given_name) = if let Some((parent, given_name)) = parent_and_given_name {
            (Some(parent), given_name)
        } else {
            (None, self.config().project_name.clone())
        };
        let nodes = RawNode::convert_to_nodes(&raw_node,
                                              parent,
                                              Some(given_name),
                                              &self,
                                              &recorded_file,
                                              preserved_keys)?;
        let root = Rc::clone(&nodes.last().unwrap().1);
        for (path, node) in nodes {
            self.insert_node(path, node);
        }
        Ok(root)
    }
}

#[derive(Debug)]
pub enum Error {
    ParsingFailed(toml::de::Error),
    /// Type of node is unknown
    UnknownType(String),
    /// A node which is not the upmost node in a file, has no name
    LackName,
    /// A node whose type is "Link" and lacks a target
    LackTarget,
    /// Key is preserved
    PreservedKey(String),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Error::*;
        let message = match &self {
            ParsingFailed(error) => format!("TOML syntax parsing error: {}", error),
            UnknownType(type_str) => format!("Unknown type {}", type_str),
            LackName => String::from("Lack name"),
            LackTarget => String::from(r#"The node whose type is "Link" lacks target"#),
            PreservedKey(key) => format!("Key {} is preserved.", key),
        };
        write!(f, "{}", message)
    }
}

impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Self {
        Error::ParsingFailed(error)
    }
}
