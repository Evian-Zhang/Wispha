mod layouter;
mod layout_templates;

use libwispha::core::*;
use crate::layouter::Layout;

fn main() {
    let node_str = r#"
    [[children]]
    name="child1"
    [[children]]
    name="child2"
    [[children.children]]
    name="child3"
    [[children.children.children]]
    name="child4"
    [[children.children]]
    name="child5"
    "#;
    let tree = Tree::new(&TreeConfig {
        project_name: "My Project".to_string()
    });
    tree.insert_nodes_from_str(node_str, std::env::current_dir().unwrap().join("LOOKME.toml"), None);
    let plain = layout_templates::plain::PlainLayout::new();
    let plain_str = plain.layout(&tree, &NodePath::new(&tree), 3).unwrap();
    println!("{}", plain_str);
}