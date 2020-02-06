use libwispha::core::structs::*;
use libwispha::serde::ser::*;

use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::path::PathBuf;

#[macro_use]
use maplit::*;

#[test]
fn to_toml_test() {
    let mut config = TreeConfig {
        project_name: String::from("TestProject")
    };

    let tree = Rc::new(RefCell::new(Tree {
        nodes: HashMap::new(),
        root: Weak::new(),
        config
    }));

    let root_path = NodePath::new(&Rc::downgrade(&tree));
    let subnode1_path = root_path.push(String::from("subnode1"));

    let root = Rc::new(RefCell::new(Node::Direct(DirectNode {
        parent: None,
        children: vec![subnode1_path.clone()],
        node_properties: NodeProperties {
            name: "TestProject".to_string(),
            record_file: PathBuf::from("LOOKME.json")
        },
        properties: hashmap!{"description".to_string() => "Project directory".to_string()}
    })));

    tree.borrow_mut().root = Rc::downgrade(&root);
    tree.borrow_mut().nodes.insert(root_path.components.clone(), root);

    let subnode1 = Rc::new(RefCell::new(Node::Direct(DirectNode {
        parent: Some(root_path.clone()),
        children: vec![],
        node_properties: NodeProperties {
            name: "subnode1".to_string(),
            record_file: PathBuf::from("LOOKME.json")
        },
        properties: hashmap!{"description".to_string() => "subnode1".to_string()}
    })));

    tree.borrow_mut().nodes.insert(subnode1_path.components, subnode1);

    let string = tree.borrow().to_string().unwrap();
    println!("{}", string);
    assert!(false)
}