use super::{CommandlineOption, InteractOptions};

use libwispha::core::*;

use std::error;
use std::path::PathBuf;
use std::env;
use std::fmt;
use std::fs;

struct InteractConfig {
    project_name: String,
    file: PathBuf,
}

impl InteractConfig {
    fn from_opt(opt: InteractOptions) -> Result<InteractConfig, Error> {
        let project_name = if let Some(project_name) = opt.project_name {
            project_name
        } else {
            ".".to_string()
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

        Ok(InteractConfig {
            project_name,
            file
        })
    }
}

impl CommandlineOption for InteractOptions {
    fn run(self) -> Result<(), Box<dyn error::Error>> {
        let config = InteractConfig::from_opt(self)?;

        let tree_config = TreeConfig {
            project_name: config.project_name.clone()
        };

        let tree = Tree::new(&tree_config);
        let node_str = fs::read_to_string(&config.file)
            .or(Err(Error::PathNotExist(config.file.clone())))?;
        tree.insert_nodes_from_str(&node_str, config.file.clone(), None)?;

        loop {

        }

        Ok(())
    }
}



#[derive(Debug)]
pub enum Error {
    CurrentDirectoryNotAvailable(std::io::Error),
}

impl error::Error for Error { }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Error::*;
        let message = match &self {
            CurrentDirectoryNotAvailable(io_error) => format!("Can not access current directory: {}", io_error),
        };
        write!(f, "{}", message)
    }
}
