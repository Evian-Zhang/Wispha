use libwispha::core::{structs::*};
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use serde_json::json;

fn main() {
    let mut custom_properties = HashMap::new();
    custom_properties.insert(String::from("description"), PropertyType::String);
    let tree = Tree {
        nodes: HashMap::new(),
        root: Weak::new(),
        custom_properties
    };
    let tree = Rc::new(RefCell::new(tree));
    let node_path_components2 = vec![String::from("root"), String::from("child1")];
    let node_path2 = NodePath {
        components: node_path_components2.clone(),
        tree: Rc::downgrade(&tree),
    };
    let node_2 = LinkNode {
        node_properties: NodeProperties {
            name: String::from("child1"),
            record_file: node_path2.clone()
        }
    };
    let node_path_components1 = vec![String::from("root")];
    let node_path1 = NodePath {
        components: node_path_components1.clone(),
        tree: Rc::downgrade(&tree),
    };
    let mut property_1 = HashMap::new();
    property_1.insert(String::from("description"), TypedProperty::String(String::from("root file")));
    let node_1 = DirectNode {
        parent: None,
        children: vec![node_path2.clone()],
        node_properties: NodeProperties {
            name: String::from("root"),
            record_file: node_path1.clone()
        },
        properties: property_1
    };
    let node2 = Rc::new(RefCell::new(Node::Link(node_2)));
    let node1 = Rc::new(RefCell::new(Node::Direct(node_1)));
    tree.borrow_mut().nodes.insert(node_path_components2.clone(), Rc::clone(&node2));
    tree.borrow_mut().nodes.insert(node_path_components1.clone(), Rc::clone(&node1));
    tree.borrow_mut().root = Rc::downgrade(&node1);
    let json = json!(tree);
    let string = json.to_string();
    println!("{}", string);
}