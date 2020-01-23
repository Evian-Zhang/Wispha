use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;

use chrono::{DateTime, Utc, TimeZone};

use libwispha::core::structs::*;

pub fn normal_tree() -> Rc<RefCell<Tree>> {
    let mut custom_properties = HashMap::new();
    custom_properties.insert("Description".to_string(), PropertyType::String);
    custom_properties.insert("Last change".to_string(), PropertyType::Date);
    custom_properties.insert("Committer number".to_string(), PropertyType::Int);

    let mut config = TreeConfig {
        custom_properties,
        project_name: String::from("TestProject")
    };

    let tree = Rc::new(RefCell::new(Tree {
        nodes: HashMap::new(),
        root: Weak::new(),
        config
    }));

    let mut root_properties = HashMap::new();
    root_properties.insert("Description".to_string(), TypedProperty::String("Project directory".to_string()));
    root_properties.insert("Last change".to_string(), TypedProperty::Date(Utc.datetime_from_str("2020-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()));
    root_properties.insert("Committer number".to_string(), TypedProperty::Int(3));

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

    tree
}