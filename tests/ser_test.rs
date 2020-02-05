use libwispha::core::structs::*;
use libwispha::serde::ser::*;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::path::PathBuf;

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

    let mut root_properties = HashMap::new();
    root_properties.insert("description".to_string(), "Project directory".to_string());

    let root = Rc::new(RefCell::new(Node::Direct(DirectNode {
        parent: None,
        children: vec![],
        node_properties: NodeProperties {
            name: "TestProject".to_string(),
            record_file: PathBuf::from("LOOKME.json")
        },
        properties: root_properties
    })));

    let root_path = NodePath::new(&Rc::downgrade(&tree));

    tree.borrow_mut().root = Rc::downgrade(&root);
    tree.borrow_mut().nodes.insert(root_path.components, root);

    let string = tree.borrow().to_string().unwrap();
    println!("{}", string);
    assert!(false)
}