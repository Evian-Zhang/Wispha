use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error;
use std::fmt::{self, Display};
use std::path::PathBuf;

use serde::Deserialize;
use serde_json;
use serde_yaml;
use toml;

use crate::core::structs::*;
use crate::core::serde::DataFormat;

type Result<T> = std::result::Result<T, Error>;

#[derive(Deserialize, Debug)]
struct InnerNodeProperties {
    name: Option<String>
}

#[derive(Deserialize, Debug)]
struct InnerDirectNode {
    children: Vec<Rc<RefCell<InnerNode>>>,
    #[serde(flatten)]
    node_properties: InnerNodeProperties,
    #[serde(flatten)]
    properties: HashMap<String, TypedProperty>,
}

#[derive(Deserialize, Debug)]
struct InnerLinkNode {
    #[serde(flatten)]
    node_properties: InnerNodeProperties
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum InnerNode {
    Direct(InnerDirectNode),
    Link(InnerLinkNode),
}

impl Node {
    // external call:
    //      if is root, no parent, give tree.config.project_name as given_name;
    //      else give parent, give link_node.node_properties.name as given_name
    // recursive call:
    //      give parent, give given_name
    // The upmost node is in the tail of returned vec
    fn from_inner_node(inner_node: &Rc<RefCell<InnerNode>>,
                       parent: Option<NodePath>,
                       given_name: Option<String>,
                       tree: &Weak<RefCell<Tree>>,
                       record_file: &PathBuf) -> Result<Vec<(NodePath, Rc<RefCell<Node>>)>> {
        match &*inner_node.borrow() {
            InnerNode::Direct(direct_node) => {
                // given name is prior to the recorded name
                let name = if let Some(name) = given_name {
                    name
                } else {
                    direct_node.node_properties.name.clone().ok_or(Error::LackName(record_file.clone()))?
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

                // see https://stackoverflow.com/questions/59852161/how-to-handle-result-in-flat-map
                let mut nodes = direct_node.children.iter()
                                           .map(|sub_node| -> Result<_> {
                                               Node::from_inner_node(sub_node, Some(path.clone()), None, tree, record_file)
                                           })
                                           .flat_map(|result| {
                                               match result {
                                                   Ok(items) => items.into_iter()
                                                                     .map(|item| Ok(item))
                                                                     .collect(),
                                                   Err(error) => vec![Err(error)],
                                               }
                                           })
                                           .collect::<Result<Vec<_>>>()?;

                let node = Rc::new(RefCell::new(Node::Direct(DirectNode {
                    parent,
                    children: nodes.iter().map(|(node_path, _)| node_path.clone()).collect::<Vec<_>>(),
                    node_properties,
                    properties: direct_node.properties.clone(),
                })));

                nodes.push((path, node));
                Ok(nodes)
            }
            InnerNode::Link(link_node) => {
                let name = if let Some(name) = given_name {
                    name
                } else {
                    link_node.node_properties.name.clone().ok_or(Error::LackName(record_file.clone()))?
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
                    node_properties
                })));
                Ok(vec![(path, link_node)])
            }
        }
    }
}

impl Tree {
    fn validate(&self, node: Rc<RefCell<Node>>) -> Result<()> {
        match &*node.borrow() {
            Node::Direct(direct_node) => {
                for (property, value) in &direct_node.properties {
                    if let Some(property_type) = self.config.custom_properties.get(property) {
                        if value.is_compatible(property_type) {
                            return Ok(());
                        } else {
                            return Err(Error::IncompatibleType(property.clone(), direct_node.node_properties.record_file.clone()));
                        }
                    } else {
                        return Err(Error::UnknownProperty(property.clone(), direct_node.node_properties.record_file.clone()));
                    }
                }
            },
            Node::Link(_) => {},
        }
        Ok(())
    }

    pub fn insert_node_from_str(tree: Rc<RefCell<Tree>>,
                                node_str: &str,
                                data_format: DataFormat,
                                recorded_file: PathBuf,
                                is_root: bool) -> Result<()> {
        let inner_node = match data_format {
            DataFormat::Json => {
                serde_json::from_str::<InnerNode>(node_str).map_err(|error| Error::DeserializeFailed(Box::new(error)))?
            },
            DataFormat::Yaml => {
                serde_yaml::from_str::<InnerNode>(node_str).map_err(|error| Error::DeserializeFailed(Box::new(error)))?
            },
            DataFormat::Toml => {
                toml::from_str::<InnerNode>(node_str).map_err(|error| Error::DeserializeFailed(Box::new(error)))?
            },
        };
        let inner_node = Rc::new(RefCell::new(inner_node));
        let nodes = Node::from_inner_node(&inner_node,
                                          None,
                                          Some(tree.borrow().config.project_name.clone()),
                                          &Rc::downgrade(&tree),
                                          &recorded_file)?;
        let mut tree_ref_mut = tree.borrow_mut();
        if is_root {
            tree_ref_mut.root = Rc::downgrade(&nodes.last().unwrap().1);
        }
        for (path, node) in nodes {
            tree_ref_mut.validate(Rc::clone(&node))?;
            tree_ref_mut.nodes.insert(path.components, node);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    DeserializeFailed(Box<dyn error::Error>),
    LackName(PathBuf),
    UnknownProperty(String, PathBuf),
    IncompatibleType(String, PathBuf)
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        use Error::*;
        let message = match &self {
            DeserializeFailed(error) => format!("Deserialize error: {}", error),
            LackName(path) => format!("In file {}, a node lacks name", path.to_str().unwrap()),
            UnknownProperty(property_name, path) => format!("In file {}, a node has unknown property name {}", path.to_str().unwrap(), property_name),
            IncompatibleType(property_name, path) => format!("In file {}, a node has incompatible property {}", path.to_str().unwrap(), property_name),
        };
        write!(f, "{}", message)
    }
}