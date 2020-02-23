use crate::layout_templates::line;
use crate::layout_templates::LayoutManager;
use crate::layouter::Layout;

use libwispha::core::*;
use structopt::StructOpt;

use std::fmt;
use std::error;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct LayoutOptions {
    /// Name for layout template. For now, it's "plain", "line" or "triangle". "line" by default
    #[structopt(long, short)]
    layout: Option<String>,

    /// Node path for the node to be displayed at top level. "/" by default
    #[structopt(long, short)]
    path: Option<String>,

    /// List of keys to be displayed. Empty list by default
    #[structopt(long, short, use_delimiter = true)]
    keys: Option<Vec<String>>,

    /// If keys only have one element, whether to hide key's name. false by default
    #[structopt(long, short)]
    hide_key: bool,

    /// Project layout depth. 3 by default
    #[structopt(long, short)]
    depth: Option<usize>,
}

struct LayoutConfig {
    layout: String,
    path: String,
    keys: Vec<String>,
    hide_key: bool,
    depth: usize,
}

impl LayoutConfig {
    fn from_opt(layout_opt: LayoutOptions) -> Result<Self, Error> {
        let layout = if let Some(layout) = layout_opt.layout {
            layout
        } else {
            line::LineLayout::info().name.clone()
        };

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

        let hide_key = layout_opt.hide_key.clone();

        let depth = if let Some(depth) = layout_opt.depth {
            depth
        } else {
            3
        };

        Ok(LayoutConfig {
            layout,
            path,
            keys,
            hide_key,
            depth
        })
    }
}

impl LayoutOptions {
    pub fn run(self, tree: &Tree, manager: &LayoutManager) -> Result<(), Box<dyn error::Error>> {
        let config = LayoutConfig::from_opt(self)?;

        let node_path = NodePath::from(&config.path, &tree)?;
        let layout_str = manager.layout(&config.layout,
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
}

impl error::Error for Error { }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Error::*;
        let message = match &self {
            NodePathMustBeAbsolute(path) => format!("Node path must be absolute, but {} is not.", path),
        };
        write!(f, "{}", message)
    }
}