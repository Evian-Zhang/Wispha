use std::error;
use std::fmt;

use serde::ser::{Serializer, SerializeMap};
use serde::Serialize;
use serde_json;
use serde_yaml;
use toml;

use crate::strings::*;
use crate::core::structs::*;
use crate::core::serde::DataFormat;

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
                                   Ok(node_path.tree.upgrade().unwrap().borrow()
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

impl Serialize for Tree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        if !self.nodes.is_empty() {
            self.root.upgrade().unwrap().borrow().serialize(serializer)
        } else {
            serializer.serialize_none()
        }
    }
}

impl Tree {
    /// Serialize the tree into a given data format
    pub fn to_string(&self, data_format: DataFormat) -> Result<String, Error> {
        match data_format {
            DataFormat::Json => {
                serde_json::to_string(&self).map_err(|error| Error::SerializeFailed(Box::new(error)))
            },
            DataFormat::Yaml => {
                serde_yaml::to_string(&self).map_err(|error| Error::SerializeFailed(Box::new(error)))
            },
            DataFormat::Toml => {
                toml::to_string(&self).map_err(|error| Error::SerializeFailed(Box::new(error)))
            },
        }
    }
}

#[derive(Debug)]
pub enum Error {
    PathNotFound(NodePath),
    SerializeFailed(Box<dyn error::Error>)
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Error::*;
        let message = match &self {
            PathNotFound(path) => format!("Path {} not found.", path),
            SerializeFailed(error) => format!("Serialize error: {}", error)
        };
        write!(f, "{}", message)
    }
}