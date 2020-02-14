use crate::layout_templates::plain;
use crate::layouter::Layout;

use structopt::StructOpt;

use std::path::PathBuf;
use std::env;
use std::fmt;
use std::error;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct CommandlineOptions {
    #[structopt(long, short)]
    pub layout: Option<String>,

    #[structopt(long, short = "n")]
    pub project_name: Option<String>,

    #[structopt(long, short)]
    pub path: Option<String>,

    #[structopt(long, short, use_delimiter = true)]
    pub keys: Option<Vec<String>>,

    #[structopt(long, short)]
    pub hide_key: Option<bool>,

    #[structopt(long, short)]
    pub file: Option<PathBuf>,
}

#[derive(Debug)]
pub struct CommandlineConfig {
    pub layout: String,
    pub project_name: String,
    pub path: String,
    pub keys: Vec<String>,
    pub hide_key: bool,
    pub file: PathBuf
}

impl CommandlineConfig {
    pub fn from_opt(opt: CommandlineOptions) -> Result<Self, Error> {
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

        let hide_key = if let Some(hide_key) = opt.hide_key {
            hide_key
        } else {
            false
        };

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

        Ok(CommandlineConfig {
            layout,
            project_name,
            path,
            keys,
            hide_key,
            file
        })
    }
}

#[derive(Debug)]
pub enum Error {
    NodePathMustBeAbsolute(String),
    CurrentDirectoryNotAvailable(std::io::Error),
}

impl error::Error for Error { }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Error::*;
        let message = match &self {
            NodePathMustBeAbsolute(path) => format!("Node path must be absolute, but {} is not.", path),
            CurrentDirectoryNotAvailable(io_error) => format!("Can not access current directory: {}", io_error)
        };
        write!(f, "{}", message)
    }
}
