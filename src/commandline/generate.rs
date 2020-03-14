use libwispha::core::*;
use crate::commandline::CommandlineOption;

use serde_json;
use structopt::StructOpt;

use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::env;
use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;
use std::collections::{HashMap, VecDeque};

type Result<T> = std::result::Result<T, Error>;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct GenerateOptions {
    /// Project path for generating. "." by default
    #[structopt(long, short)]
    path: Option<PathBuf>,

    /// Default JSON file name. "LOOKME.json" by default
    #[structopt(long, short = "n")]
    file_name: Option<String>,
}

struct GenerateConfig {
    path: PathBuf,
    file_name: String
}

impl GenerateConfig {
    fn from_opt(opt: GenerateOptions) -> Result<Self> {
        let path = if let Some(path) = opt.path {
            if path.is_absolute() {
                path
            } else {
                env::current_dir()
                    .map_err(|io_error| Error::CurrentDirectoryNotAvailable(io_error))?
                    .join(path)
            }
        } else {
            env::current_dir()
                .map_err(|io_error| Error::CurrentDirectoryNotAvailable(io_error))?
        };

        let file_name = if let Some(file_name) = opt.file_name {
            file_name
        } else {
            String::from("LOOKME.json")
        };

        Ok(GenerateConfig {
            path,
            file_name
        })
    }
}

impl CommandlineOption for GenerateOptions {
    fn run(self) -> std::result::Result<(), Box<dyn error::Error>> {
        let config = GenerateConfig::from_opt(self)?;
        generate_file(config)?;
        Ok(())
    }
}

fn generate_file(config: GenerateConfig) -> Result<()> {
    let root = config.path.clone();
    let project_name = root.file_name()
        .map(|os_str| os_str.to_str().unwrap())
        .unwrap_or(".")
        .to_string();

    let tree = Tree::new(&TreeConfig {
        project_name
    });

    let root_path = NodePath::new(&tree);

    let mut path_queue = VecDeque::new();
    path_queue.push_front((root_path, root));

    while let Some((node_path, path)) = path_queue.pop_front() {
        let (node,children_paths) = generate_direct_node(&tree, node_path, path.clone(), &config)?;
        let json_file = serde_json::to_string_pretty(&node).unwrap();
        let file_path = node.borrow().node_properties().record_file.clone();
        fs::write(&file_path, json_file)
            .or_else(|io_error| Err(Error::CannotWrite((file_path.clone(), io_error))))?;
        for child_path in children_paths {
            path_queue.push_back(child_path);
        }
    }

    Ok(())
}

fn generate_direct_node(tree: &Tree,
                        base_node_path: NodePath,
                        dir_path: PathBuf,
                        config: &GenerateConfig) -> Result<(Rc<RefCell<Node>>, Vec<(NodePath, PathBuf)>)> {
    let name = base_node_path.name().unwrap_or(tree.config().project_name);
    let record_file = dir_path.join(&config.file_name);
    let node_properties = NodeProperties {
        name,
        parent: base_node_path.parent(),
        record_file: record_file.clone()
    };

    let mut dir_children_paths = vec![];
    let mut children = vec![];

    for child in fs::read_dir(&dir_path)
        .or_else(|io_error| Err(Error::CannotRead((dir_path.clone(), io_error))))? {
        let child = child.unwrap();
        let child_name = child.file_name().to_str().unwrap().to_owned();
        let child_path = base_node_path.push(child_name.clone());
        let metadata = child.metadata()
            .or_else(|io_error| Err(Error::CannotRead((dir_path.clone(), io_error))))?;
        if metadata.is_file() {
            let child_direct_node = DirectNode {
                children: vec![],
                node_properties: NodeProperties {
                    name: child_name,
                    parent: Some(base_node_path.clone()),
                    record_file: record_file.clone()
                },
                properties: HashMap::new()
            };
            let child_node = Rc::new(RefCell::new(Node::Direct(child_direct_node)));
            tree.insert_node(child_path.clone(), child_node);
        } else {
            let target = PathBuf::from(child.file_name()).join(config.file_name.clone());
            let child_link_node = LinkNode {
                target,
                node_properties: NodeProperties {
                    name: child.file_name().to_str().unwrap().to_owned(),
                    parent: Some(base_node_path.clone()),
                    record_file: record_file.clone()
                }
            };
            let child_node = Rc::new(RefCell::new(Node::Link(child_link_node)));
            tree.insert_node(child_path.clone(), child_node);
            dir_children_paths.push((child_path.clone(), child.path()));
        }
        children.push(child_path);
    }

    let direct_node = DirectNode {
        children,
        node_properties,
        properties: HashMap::new()
    };

    let node = Rc::new(RefCell::new(Node::Direct(direct_node)));

    tree.insert_node(base_node_path, Rc::clone(&node));

    Ok((node, dir_children_paths))
}

#[derive(Debug)]
pub enum Error {
    CannotRead((PathBuf, io::Error)),
    CannotWrite((PathBuf, io::Error)),
    CurrentDirectoryNotAvailable(io::Error)
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        use Error::*;
        let message = match &self {
            CannotRead((path, io_error)) => format!("Cannot open {}: {}", path.to_str().unwrap(), io_error),
            CannotWrite((path, io_error)) => format!("Cannot write to {}: {}", path.to_str().unwrap(), io_error),
            CurrentDirectoryNotAvailable(io_error) => format!("Can not access current directory: {}", io_error),
        };
        write!(f, "{}", message)
    }
}