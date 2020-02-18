use crate::layout_templates::plain;
use crate::layouter::Layout;
use super::{LayoutOptions, CommandlineOption};

use libwispha::core::*;

use std::path::PathBuf;
use std::env;
use std::fmt;
use std::error;
use std::fs;

struct LayoutConfig {
    layout: String,
    project_name: String,
    path: String,
    keys: Vec<String>,
    hide_key: bool,
    file: PathBuf,
    depth: usize,
}

impl LayoutConfig {
    fn from_opt(opt: LayoutOptions) -> Result<Self, Error> {
        let layout = if let Some(layout) = opt.layout {
            layout
        } else {
            plain::PlainLayout::new().info().name.clone()
        };

        let project_name = if let Some(project_name) = opt.project_name {
            project_name
        } else {
            ".".to_string()
        };

        let path = if let Some(path) = opt.path {
            if path.starts_with("/") {
                path
            } else {
                return Err(Error::NodePathMustBeAbsolute(path));
            }
        } else {
            "/".to_string()
        };

        let keys = if let Some(keys) = opt.keys {
            keys
        } else {
            vec![]
        };

        let hide_key = opt.hide_key.clone();

        let file = if let Some(file) = opt.file {
            if file.is_absolute() {
                file
            } else {
                env::current_dir()
                    .map_err(|io_error| Error::CurrentDirectoryNotAvailable(io_error))?
                    .join(file)
            }
        } else {
            env::current_dir()
                .map_err(|io_error| Error::CurrentDirectoryNotAvailable(io_error))?
                .join("LOOKME.toml")
        };

        let depth = if let Some(depth) = opt.depth {
            depth
        } else {
            3
        };

        Ok(LayoutConfig {
            layout,
            project_name,
            path,
            keys,
            hide_key,
            file,
            depth
        })
    }
}

impl CommandlineOption for LayoutOptions {
    fn run(self) -> Result<(), Box<dyn error::Error>> {
        let config = LayoutConfig::from_opt(self)?;

        let tree_config = TreeConfig {
            project_name: config.project_name.clone()
        };

        let tree = Tree::new(&tree_config);
        let node_str = fs::read_to_string(&config.file)
            .or(Err(Error::PathNotExist(config.file.clone())))?;
        tree.insert_nodes_from_str(&node_str, config.file.clone(), None)?;
        let node_path = NodePath::from(&config.path, &tree)?;
        let layout_str = crate::layouter::LayoutManager::layout(&config.layout,
                                                                &crate::layout_templates::layout_resolver,
                                                                &tree,
                                                                &node_path,
                                                                config.depth,
                                                                &config.keys,
                                                                config.hide_key)?;
        println!("{}", layout_str);
        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    NodePathMustBeAbsolute(String),
    CurrentDirectoryNotAvailable(std::io::Error),
    PathNotExist(PathBuf),
}

impl error::Error for Error { }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Error::*;
        let message = match &self {
            NodePathMustBeAbsolute(path) => format!("Node path must be absolute, but {} is not.", path),
            CurrentDirectoryNotAvailable(io_error) => format!("Can not access current directory: {}", io_error),
            PathNotExist(path) => format!("Can't open file at {}.", path.to_str().unwrap()),
        };
        write!(f, "{}", message)
    }
}
