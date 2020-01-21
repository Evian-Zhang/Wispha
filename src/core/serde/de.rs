use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error;
use std::fmt::{self, Display};

use serde::de::{self, Deserializer, Visitor, MapAccess};
use serde::Deserialize;

use serde_json;

use crate::core::structs::*;
use serde::export::Formatter;

type Result<T> = std::result::Result<T, Error>;

#[derive(Deserialize, Debug)]
struct _DirectNode {
    children: Vec<Rc<RefCell<_Node>>>,
    #[serde(flatten)]
    node_properties: NodeProperties,
    #[serde(flatten)]
    properties: HashMap<String, TypedProperty>
}

#[derive(Deserialize, Debug)]
struct _LinkNode {
    #[serde(flatten)]
    node_properties: NodeProperties
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum _Node {
    Direct(_DirectNode),
    Link(_LinkNode)
}

impl Node {
    fn from_node(inner_node: &_Node) -> Node {
        match inner_node {
            _Node::Direct(direct_node) => {
                Node::Direct(DirectNode {
                    parent: None,
                    children: vec![],
                    node_properties: direct_node.node_properties.clone(),
                    properties: direct_node.properties.clone()
                })
            },
            _Node::Link(link_node) => {
                Node::Link(LinkNode {
                    node_properties: link_node.node_properties.clone()
                })
            },
        }
    }
}

impl Tree {
    pub fn insert_node_from_str(&mut self, node_str: &str) -> Result<()> {
        let inner_node = Rc::new(RefCell::new(serde_json::from_str::<_Node>(node_str)?));

        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    DeserializeFailed(dyn serde::de::Error)
}

impl error::Error for Error { }

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        use Error::*;
        let message = match &self {
            DeserializeFailed(error) => format!("Deserialize error: {}", error)
        };
        write!(f, "{}", message)
    }
}

//#[test]
//fn test() {
//    let str = r#"{"name": "zs"}"#;
//    let node = serde_json::from_str::<_Node>(str);
//    println!("{:?}", node);
//    assert!(false)
//}

//impl<'de> Deserialize<'de> for _Node {
//    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//        where
//            D: Deserializer<'de> {
//
//    }
//}