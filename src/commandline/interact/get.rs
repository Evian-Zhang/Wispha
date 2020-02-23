use libwispha::core::{Tree, NodePath};
use structopt::StructOpt;

use std::error;
use std::fmt;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct GetOptions {
    #[structopt(long, short)]
    key: String,

    /// Node path for the node
    #[structopt(long, short)]
    path: String,
}

impl GetOptions {
    pub fn run(self, tree: &Tree) -> Result<(), Box<dyn error::Error>> {
        let node_path = NodePath::from(&self.path, &tree)?;
        tree.resolve_node(&node_path, &crate::layout_templates::resolve_handler, &*crate::PRESERVED_KEYS)?;

        // After tree's resolving node, there must be a direct node at `node_path`
        let node = tree.get_node(&node_path).unwrap();
        if let Some(property) = node.borrow().get_direct().unwrap().properties.get(&self.key).clone() {
            println!("{}", property);
        } else {
            let key: &str = &self.key;
            match key {
                "path" => {
                    let path = tree.get_path_buf(&node_path)?;
                    println!("{}", path.to_str().unwrap());
                },
                "children" => {
                    let node = node.borrow();
                    let children = &node.get_direct().unwrap().children;
                    let children_str = children.iter()
                        .map(|child| child.to_string())
                        .collect::<Vec<String>>()
                        .join("\n");
                    println!("{}", children_str);
                },
                "parent" => {
                    let node = node.borrow();
                    if let Some(parent) = &node.get_direct().unwrap().node_properties.parent {
                        println!("{}", parent);
                    } else {
                        return Err(Box::new(Error::NoParent));
                    }
                },
                "record_file" => {
                    let node = node.borrow();
                    let record_file = &node.get_direct().unwrap().node_properties.record_file;
                    println!("{}", record_file.to_str().unwrap())
                }
                _ => {
                    return Err(Box::new(Error::PropertyInexist(self.key.clone())));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    PropertyInexist(String),
    NoParent,
}

impl error::Error for Error { }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Error::*;
        let message = match &self {
            PropertyInexist(key) => format!("The node does not have a property with key {}.", key),
            NoParent => String::from("This node is root, who has no parent."),
        };
        write!(f, "{}", message)
    }
}