use crate::layout_templates::line;
use crate::layouter::Layout;
use super::CommandlineOption;

use libwispha::core::*;
use structopt::StructOpt;

use std::path::PathBuf;
use std::env;
use std::fmt;
use std::error;
use std::fs;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct LayoutOptions {
    /// Name for layout template. For now, it's "plain", "line" or "triangle". "line" by default
    #[structopt(long, short)]
    layout: Option<String>,

    /// Name for the project, used for the name of top directory. "." by default
    #[structopt(long, short = "n")]
    project_name: Option<String>,

    /// Node path for the node to be displayed at top level. "/" by default
    #[structopt(long, short)]
    path: Option<String>,

    /// List of keys to be displayed. Empty list by default
    #[structopt(long, short, use_delimiter = true)]
    keys: Option<Vec<String>>,

    /// If keys only have one element, whether to hide key's name. false by default
    #[structopt(long, short)]
    hide_key: bool,

    /// File path for the project's root JSON file. `LOOKME.json` By default
    #[structopt(long, short)]
    file: Option<PathBuf>,

    /// Project layout depth. 3 by default
    #[structopt(long, short)]
    depth: Option<usize>,
}

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
            line::LineLayout::info().name.clone()
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
                .join("LOOKME.json")
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
        tree.insert_nodes_from_str(&node_str, config.file.clone(), None, &*crate::PRESERVED_KEYS)?;
        let node_path = NodePath::from(&config.path, &tree)?;

        let layout_manager = crate::layout_templates::LayoutManager::new();

        let layout_str = layout_manager.layout(&config.layout,
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
