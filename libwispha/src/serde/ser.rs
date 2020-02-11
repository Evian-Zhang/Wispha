use std::error;
use std::fmt;

use crate::core::*;
use crate::strings::*;

use serde::ser::{Serializer, SerializeMap};
use serde::Serialize;
use toml;

impl Serialize for DirectNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        // TODO: find a more graceful way
        // WARN: THIS IS NOT PRESERVING ORDER
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry(NAME, &self.node_properties.name)?;
        for (property, value) in self.properties.iter() {
            map.serialize_entry(property, value)?;
        }
        if !self.children.is_empty() {
            let children = self.children.iter()
                               .map(|node_path| -> Result<_, S::Error> {
                                   Ok(node_path.tree()
                                               .get_node(node_path)
                                               .ok_or(Error::PathNotFound(node_path.clone()))
                                               .map_err(serde::ser::Error::custom)?)
                               })
                               .collect::<Result<Vec<_>, S::Error>>()?;
            map.serialize_entry(CHILDREN, &children)?;
        }
        map.end()
    }
}

impl Serialize for InnerTree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        if !self.nodes.is_empty() {
            self.root().unwrap().borrow().serialize(serializer)
        } else {
            serializer.serialize_none()
        }
    }
}

impl Tree {
    /// Convert tree to TOML syntax
    pub fn to_string(&self) -> Result<String, Error> {
        if self.0.borrow().nodes.is_empty() {
            Err(Error::EmptyTree)
        } else {
            toml::to_string(&*self.0.borrow()).map_err(|error| Error::SerializeFailed(Box::new(error)))
        }
    }
}

#[derive(Debug)]
pub enum Error {
    PathNotFound(NodePath),
    SerializeFailed(Box<dyn error::Error>),
    /// Tree has no nodes
    EmptyTree
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Error::*;
        let message = match &self {
            PathNotFound(path) => format!("Path {} not found.", path),
            SerializeFailed(error) => format!("Serialize error: {}", error),
            EmptyTree => String::from("Tree is empty.")
        };
        write!(f, "{}", message)
    }
}