mod layouter;
mod layout_templates;
mod commandline;

use libwispha::core::*;
use structopt::StructOpt;

use std::error;
use std::fs;
use std::fmt;
use std::path::PathBuf;

fn run() -> Result<(), Box<dyn error::Error>> {
    let opt = commandline::CommandlineOptions::from_args();
    let config = commandline::CommandlineConfig::from_opt(opt)?;

    let tree_config = TreeConfig {
        project_name: config.project_name.clone()
    };

    let tree = Tree::new(&tree_config);
    let node_str = fs::read_to_string(&config.file)
        .or(Err(Error::PathNotExist(config.file.clone())))?;
    tree.insert_nodes_from_str(&node_str, config.file.clone(), None)?;
    let node_path = NodePath::from(&config.path, &tree)?;
    layouter::LayoutManager::layout(&config.layout,
                                    &layout_templates::layout_resolver,
                                    &tree,
                                    &node_path,
                                    config.depth,
                                    &config.keys,
                                    config.hide_key)?;
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

#[derive(Debug)]
enum Error {
    PathNotExist(PathBuf)
}

impl error::Error for Error { }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Error::*;
        let message = match &self {
            PathNotExist(path) => format!("Can't open file at {}.", path.to_str().unwrap())
        };
        write!(f, "{}", message)
    }
}
