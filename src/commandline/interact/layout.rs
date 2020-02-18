use crate::layout_templates::plain;
use crate::layouter::Layout;
use super::{LayoutOptions, InteractConfig, InteractOption};

use libwispha::core::*;

use std::path::PathBuf;
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
    fn from_opt(layout_opt: LayoutOptions, interact_conf: &InteractConfig) -> Result<Self, Error> {
        let layout = if let Some(layout) = layout_opt.layout {
            layout
        } else {
            plain::PlainLayout::new().info().name.clone()
        };

        let project_name = interact_conf.project_name.clone();

        let path = if let Some(path) = layout_opt.path {
            if path.starts_with("/") {
                path
            } else {
                return Err(Error::NodePathMustBeAbsolute(path));
            }
        } else {
            "/".to_string()
        };

        let keys = if let Some(keys) = layout_opt.keys {
            keys
        } else {
            vec![]
        };

        let hide_key = if let Some(hide_key) = layout_opt.hide_key {
            hide_key
        } else {
            false
        };

        let file = interact_conf.file.clone();

        let depth = if let Some(depth) = layout_opt.depth {
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

impl InteractOption for LayoutOptions {
    fn run(self, interact_conf: &InteractConfig) -> Result<(), Box<dyn error::Error>> {
        let config = LayoutConfig::from_opt(self, interact_conf)?;

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
    PathNotExist(PathBuf),
}

impl error::Error for Error { }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Error::*;
        let message = match &self {
            NodePathMustBeAbsolute(path) => format!("Node path must be absolute, but {} is not.", path),
            PathNotExist(path) => format!("Can't open file at {}.", path.to_str().unwrap()),
        };
        write!(f, "{}", message)
    }
}