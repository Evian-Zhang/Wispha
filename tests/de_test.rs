use libwispha::core::structs::*;
use libwispha::serde::de::*;

use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;

#[test]
fn empty_str() {
    let toml_str = "";
    let tree = Rc::new(RefCell::new(Tree::new(&TreeConfig {
        project_name: "Project".to_string()
    })));
    let res = Tree::insert_nodes_from_str(Rc::clone(&tree),
                                          toml_str,
                                          PathBuf::from("LOOKME.toml"),
                                          None);
    assert!(res.is_ok())
}

#[test]
fn default_type() {
    let toml_str = r#"description = "Root file""#;
    let tree = Rc::new(RefCell::new(Tree::new(&TreeConfig {
        project_name: "Project".to_string()
    })));
    Tree::insert_nodes_from_str(Rc::clone(&tree),
                                toml_str,
                                PathBuf::from("LOOKME.toml"),
                                None).unwrap();
    let tree = tree.borrow();
    if let Node::Direct(_) = &*tree.root.upgrade().unwrap().borrow() {
        assert!(true);
    } else {
        assert!(false);
    }
}
