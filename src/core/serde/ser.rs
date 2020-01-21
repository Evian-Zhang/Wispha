use std::error;
use std::fmt;

use serde::ser::{Serializer, SerializeMap};
use serde::Serialize;

use crate::strings::*;
use crate::core::structs::*;

impl Serialize for TypedProperty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        use TypedProperty::*;
        match &self {
            String(string) => string.serialize(serializer),
            Date(date) => super::date_format::serialize(date, serializer),
            Int(int) => int.serialize(serializer),
            Double(double) => double.serialize(serializer),
        }
    }
}

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

impl Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        use Node::*;
        match &self {
            Direct(direct_node) => direct_node.serialize(serializer),
            Link(link_node) => link_node.serialize(serializer)
        }
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

#[derive(Debug)]
enum Error {
    PathNotFound(NodePath)
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Error::*;
        let message = match &self {
            PathNotFound(path) => format!("Path {} not found.", path)
        };
        write!(f, "{}", message)
    }
}