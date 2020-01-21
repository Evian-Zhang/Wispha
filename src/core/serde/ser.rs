use std::cell::RefCell;
use std::rc::Rc;

use serde::ser::{Serializer, SerializeMap};
use serde::{Serialize, Deserialize};

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
            let children: Vec<Rc<RefCell<Node>>> = self.children.iter().map(|node_path| node_path.tree.upgrade().unwrap().borrow().get_node(node_path).unwrap()).collect();
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