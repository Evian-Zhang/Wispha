mod commandline_parser;
mod layout;
mod get;

use super::CommandlineOption;
use crate::layout_templates::LayoutManager;

use libwispha::core::*;
use structopt::StructOpt;
use rustyline;

use std::error;
use std::path::PathBuf;
use std::env;
use std::fmt;
use std::fs;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct InteractOptions {
    /// Name for the project, used for the name of top directory. "." by default
    #[structopt(long, short = "n")]
    project_name: Option<String>,

    /// File path for the project's root JSON file. `LOOKME.json` By default
    #[structopt(long, short)]
    file: Option<PathBuf>,
}

struct InteractConfig {
    project_name: String,
    file: PathBuf,
}

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Subcommand {
    /// Display a project layout
    Layout(layout::LayoutOptions),

    /// Get a key for a node
    Get(get::GetOptions),

    /// Refresh nodes cache
    Refresh,
    Quit,
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
                .join("LOOKME.json")
        };

        Ok(InteractConfig {
            project_name,
            file
        })
    }

    fn run_helper(&self, line: &String, tree: &Tree, manager: &LayoutManager) -> Result<bool, Box<dyn error::Error>> {
        use Subcommand::*;

        let args = commandline_parser::to_args(&line)?;
        let interact_opt = Subcommand::from_iter_safe(args)?;
        match interact_opt {
            Layout(layout_options) => {
                layout_options.run(tree, manager)?;
            },
            Get(get_options) => {
                get_options.run(tree)?;
            },
            Refresh => {
                let node_str = fs::read_to_string(&self.file)
                    .or(Err(Error::PathNotExist(self.file.clone())))?;

                // Clear after read to string successfully
                tree.clear();

                tree.insert_nodes_from_str(&node_str, self.file.clone(), None, &*crate::PRESERVED_KEYS)?;
            },
            Quit => return Ok(true),
        }
        Ok(false)
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
        tree.insert_nodes_from_str(&node_str, config.file.clone(), None, &*crate::PRESERVED_KEYS)?;

        let layout_manager = LayoutManager::new();

        let mut rl = rustyline::Editor::<()>::new();
        let mut line;

        loop {
            line = rl.readline("(wispha) ");
            match &line {
                Ok(line) => {
                    match config.run_helper(&line, &tree, &layout_manager) {
                        Ok(will_quit) => if will_quit { break } else { continue },
                        Err(error) => eprintln!("{}", error)
                    }
                },
                Err(error) => {
                    eprintln!("{}", error);
                    break;
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    CurrentDirectoryNotAvailable(std::io::Error),
    PathNotExist(PathBuf),
}

impl error::Error for Error { }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Error::*;
        let message = match &self {
            CurrentDirectoryNotAvailable(io_error) => format!("Can not access current directory: {}", io_error),
            PathNotExist(path) => format!("Can't open file at {}.", path.to_str().unwrap()),
        };
        write!(f, "{}", message)
    }
}
