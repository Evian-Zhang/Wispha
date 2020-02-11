use libwispha::core::*;
use libwispha::serde::de::*;

use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;

#[test]
fn empty_str() {
    let toml_str = "";
    let mut tree = Tree::new(&TreeConfig {
        project_name: "Project".to_string()
    });
    let res = tree.insert_nodes_from_str(toml_str,
                                               PathBuf::from("LOOKME.toml"),
                                               None);
    assert!(res.is_ok())
}

#[test]
fn default_type() {
    let toml_str = r#"description = "Root file""#;
    let mut tree = Tree::new(&TreeConfig {
        project_name: "Project".to_string()
    });
    tree.insert_nodes_from_str(toml_str,
                                     PathBuf::from("LOOKME.toml"),
                                     None).unwrap();
    if let Node::Direct(_) = &*tree.root().unwrap().borrow() {
        assert!(true);
    } else {
        assert!(false);
    }
}

#[test]
fn lack_target() {
    let toml_str = r#"type = "Link""#;
    let mut tree = Tree::new(&TreeConfig {
        project_name: "Project".to_string()
    });
    let res = tree.insert_nodes_from_str(toml_str,
                                               PathBuf::from("LOOKME.toml"),
                                               None);
    if let Err(error) = res {
        match error {
            Error::LackTarget => assert!(true),
            _ => assert!(false)
        }
    } else {
        assert!(false)
    }
}
