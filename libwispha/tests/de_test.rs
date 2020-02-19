use libwispha::core::*;
use libwispha::serde::de::*;

use std::path::PathBuf;

#[test]
fn empty_str() {
    let json_str = "";
    let tree = Tree::new(&TreeConfig {
        project_name: "Project".to_string()
    });
    let res = tree.insert_nodes_from_str(json_str,
                                         PathBuf::from("LOOKME.json"),
                                         None,
                                         &vec![]);
    assert!(res.is_err())
}

#[test]
fn default_type() {
    let json_str = r#"{"description": "root file"}"#;
    let tree = Tree::new(&TreeConfig {
        project_name: "Project".to_string()
    });
    tree.insert_nodes_from_str(json_str,
                               PathBuf::from("LOOKME.json"),
                               None,
                               &vec![]).unwrap();
    if let Node::Direct(_) = &*tree.root().unwrap().borrow() {
        assert!(true);
    } else {
        assert!(false);
    }
}

#[test]
fn lack_target() {
    let json_str = r#"{"type": "Link"}"#;
    let tree = Tree::new(&TreeConfig {
        project_name: "Project".to_string()
    });
    let res = tree.insert_nodes_from_str(json_str,
                                         PathBuf::from("LOOKME.json"),
                                         None,
                                         &vec![]);
    if let Err(error) = res {
        match error {
            Error::LackTarget => assert!(true),
            _ => assert!(false)
        }
    } else {
        assert!(false)
    }
}
