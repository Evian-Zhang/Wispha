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
                        record_file: &PathBuf) -> Result<Vec<(NodePath, Rc<RefCell<Node>>)>, Error> {
        let inner_node_type = if let Some(node_type_str) = raw_node.borrow().properties.get("type") {
            RawNodeType::from_str(node_type_str).ok_or(Error::UnknownType(node_type_str.to_owned()))?
        } else {
            RawNodeType::default()
        };
        match inner_node_type {
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
                let mut nodes = if let Some(children) = &raw_node.borrow().children {
                    // see https://stackoverflow.com/questions/59852161/how-to-handle-result-in-flat-map
                    children.iter()
                            .map(|sub_node| -> Result<_, Error> {
                                RawNode::convert_to_nodes(sub_node, Some(path.clone()), None, tree, record_file)
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
                    children: nodes.iter().map(|(node_path, _)| node_path.clone()).collect::<Vec<_>>(),
                    node_properties,
                    properties: raw_node.borrow().properties.clone(),
                })));

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
    /// If `parent` is `None`, treat `node_str` as root, else treat `node_str` as children of `parent`.
    pub fn insert_nodes_from_str(&self,
                                 node_str: &str,
                                 recorded_file: PathBuf,
                                 parent: Option<NodePath>) -> Result<Rc<RefCell<Node>>, Error> {
        let raw_node = toml::from_str::<RawNode>(node_str).map_err(|error| Error::ParsingFailed(error))?;
        let raw_node = Rc::new(RefCell::new(raw_node));
        let nodes = RawNode::convert_to_nodes(&raw_node,
                                              parent.clone(),
                                              Some(self.config().project_name.clone()),
                                              &self,
                                              &recorded_file)?;
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
