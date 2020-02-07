use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::path::PathBuf;

use crate::core::structs::*;
use crate::strings::*;

use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
struct InnerNode {
    #[serde(flatten)]
    properties: HashMap<String, String>,
    children: Option<Vec<Rc<RefCell<InnerNode>>>>
}

enum InnerNodeType {
    Direct,
    Link
}

impl InnerNodeType {
    fn from_str(s: &str) -> Option<InnerNodeType> {
        use InnerNodeType::*;
        match s {
            "Direct" => Some(Direct),
            "Link" => Some(Link),
            _ => None
        }
    }
}

impl Default for InnerNodeType {
    fn default() -> Self {
        InnerNodeType::Direct
    }
}

impl InnerNode {
    // external call:
    //      if is root, no parent, give tree.config.project_name as given_name;
    //      else give parent, give link_node.node_properties.name as given_name
    // recursive call:
    //      give parent, give given_name
    // The upmost node is in the tail of returned vec
    fn convert_to_nodes(inner_node: &Rc<RefCell<InnerNode>>,
                        parent: Option<NodePath>,
                        given_name: Option<String>,
                        tree: &Weak<RefCell<Tree>>,
                        record_file: &PathBuf) -> Result<Vec<(NodePath, Rc<RefCell<Node>>)>, Error> {
        let inner_node_type = if let Some(node_type_str) = inner_node.borrow().properties.get("type") {
            InnerNodeType::from_str(node_type_str).ok_or(Error::UnknownType(node_type_str.to_owned()))?
        } else {
            InnerNodeType::default()
        };
        match inner_node_type {
            InnerNodeType::Direct => {
                // given name is prior to the recorded name
                let name = if let Some(name) = given_name {
                    name
                } else {
                    inner_node.borrow().properties.get(NAME).cloned().ok_or(Error::LackName)?
                };
                let path = if let Some(parent) = &parent {
                    parent.push(name.clone())
                } else {
                    NodePath::new(tree)
                };

                let node_properties = NodeProperties {
                    name,
                    record_file: record_file.clone(),
                };
                let mut nodes = if let Some(children) = &inner_node.borrow().children {
                    // see https://stackoverflow.com/questions/59852161/how-to-handle-result-in-flat-map
                    children.iter()
                            .map(|sub_node| -> Result<_, Error> {
                                InnerNode::convert_to_nodes(sub_node, Some(path.clone()), None, tree, record_file)
                            })
                            .flat_map(|result| {
                                match result {
                                    Ok(items) => items.into_iter()
                                                      .map(|item| Ok(item))
                                                      .collect(),
                                    Err(error) => vec![Err(error)],
                                }
                            })
                            .collect::<Result<Vec<_>, Error>>()?
                } else {
                    vec![]
                };
                let node = Rc::new(RefCell::new(Node::Direct(DirectNode {
                    parent,
                    children: nodes.iter().map(|(node_path, _)| node_path.clone()).collect::<Vec<_>>(),
                    node_properties,
                    properties: inner_node.borrow().properties.clone(),
                })));

                nodes.push((path, node));
                Ok(nodes)
            },
            InnerNodeType::Link => {
                let name = if let Some(name) = given_name {
                    name
                } else {
                    inner_node.borrow().properties.get("name").cloned().ok_or(Error::LackName)?
                };
                let path = if let Some(parent) = parent {
                    parent.push(name.clone())
                } else {
                    NodePath::new(tree)
                };
                let node_properties = NodeProperties {
                    name,
                    record_file: record_file.clone(),
                };
                let link_node = Rc::new(RefCell::new(Node::Link(LinkNode {
                    node_properties,
                    target: PathBuf::from(inner_node.borrow().properties.get("target").cloned().ok_or(Error::LackTarget)?)
                })));
                Ok(vec![(path, link_node)])
            },
        }
    }
}

impl Tree {
    /// Insert nodes from TOML string `node_str` in `recorded_file` to `tree`.
    /// If `parent` is `None`, treat `node_str` as root, else treat `node_str` as children of `parent`.
    pub fn insert_nodes_from_str(tree: Rc<RefCell<Tree>>,
                                 node_str: &str,
                                 recorded_file: PathBuf,
                                 parent: Option<NodePath>) -> Result<(), Error> {
        let inner_node = toml::from_str::<InnerNode>(node_str).map_err(|error| Error::ParsingFailed(error))?;
        let inner_node = Rc::new(RefCell::new(inner_node));
        let nodes = InnerNode::convert_to_nodes(&inner_node,
                                                parent.clone(),
                                                Some(tree.borrow().config.project_name.clone()),
                                                &Rc::downgrade(&tree),
                                                &recorded_file)?;
        let mut tree_ref_mut = tree.borrow_mut();
        if parent.is_none() {
            tree_ref_mut.root = Rc::downgrade(&nodes.last().unwrap().1);
        }
        for (path, node) in nodes {
            tree_ref_mut.nodes.insert(path.components, node);
        }
        Ok(())
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
    LackTarget
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Error::*;
        let message = match &self {
            ParsingFailed(error) => format!("TOML syntax parsing error: {}", error),
            UnknownType(type_str) => format!("Unknown type {}", type_str),
            LackName => String::from("Lack name"),
            LackTarget => String::from(r#"The node whose type is "Link" lacks target"#)
        };
        write!(f, "{}", message)
    }
}

impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Self {
        Error::ParsingFailed(error)
    }
}
