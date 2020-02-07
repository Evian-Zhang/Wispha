use libwispha::core::*;
use libwispha::serde::ser::*;

use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;

#[macro_use]
use maplit::*;

#[test]
fn to_toml_test() {
    let config = TreeConfig {
        project_name: String::from("TestProject")
    };

    let tree = Rc::new(RefCell::new(Tree::new(&config)));

    let root_path = NodePath::new(&Rc::downgrade(&tree));
    let subnode1_path = root_path.push(String::from("subnode1"));

    let root = Rc::new(RefCell::new(Node::Direct(DirectNode {
        children: vec![subnode1_path.clone()],
        node_properties: NodeProperties {
            name: "TestProject".to_string(),
            parent: None,
            record_file: PathBuf::from("LOOKME.toml")
        },
        properties: hashmap!{"description".to_string() => "Project directory".to_string()}
    })));

    tree.borrow_mut().root = Rc::downgrade(&root);
    tree.borrow_mut().insert_node(root_path.clone(), root);

    let subnode1 = Rc::new(RefCell::new(Node::Direct(DirectNode {
        children: vec![],
        node_properties: NodeProperties {
            name: "subnode1".to_string(),
            parent: Some(root_path.clone()),
            record_file: PathBuf::from("LOOKME.toml")
        },
        properties: hashmap!{"description".to_string() => "subnode1".to_string()}
    })));

    tree.borrow_mut().insert_node(subnode1_path.clone(), subnode1);

    let string = tree.borrow().to_string();
    assert!(string.is_ok())
}

#[test]
fn none_tree_test() {
    let tree = Tree::new(&TreeConfig {
        project_name: "My Project".to_string()
    });

    let res = tree.to_string();
    if let Err(error) = res {
        match error {
            Error::EmptyTree => assert!(true),
            _ => assert!(false)
        }
    } else {
        assert!(false)
    }
}