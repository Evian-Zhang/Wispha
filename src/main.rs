mod layouter;
mod layout_templates;
mod commandline;

use commandline::CommandlineOption;

use structopt::StructOpt;

use std::error;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref PRESERVED_KEYS: Vec<&'static str> = vec![
    "path",
    "parent",
    "record_file"
    ];
}

fn run() -> Result<(), Box<dyn error::Error>> {
    let opt = commandline::Commandline::from_args();
    opt.run()?;
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Err: {}", error);
        std::process::exit(1);
    }
//    let node_str = r#"
//    description = "Main project folder"
//    [[children]]
//    name="child1"
//    description = "The first child"
//    [[children]]
//    name="child2"
//    [[children.children]]
//    name="child3"
//    [[children.children.children]]
//    name="child4"
//    description = "The fourth child"
//    [[children.children]]
//    name="child5"
//    "#;
//    let tree = Tree::new(&TreeConfig {
//        project_name: "My Project".to_string()
//    });
//    tree.insert_nodes_from_str(node_str, std::env::current_dir().unwrap().join("LOOKME.toml"), None);
//    let plain = layout_templates::plain::PlainLayout::new();
//    let plain_str = plain.layout(&tree, &NodePath::new(&tree), 3, &vec!["description".to_string()], false).unwrap();
//    println!("{}", plain_str);
}
