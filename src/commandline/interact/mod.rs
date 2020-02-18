mod commandline_parser;
mod layout;

use super::{CommandlineOption, InteractOptions};

use libwispha::core::*;
use structopt::StructOpt;
use rustyline;

use std::error;
use std::path::PathBuf;
use std::env;
use std::fmt;
use std::fs;

struct InteractConfig {
    project_name: String,
    file: PathBuf,
}

trait InteractOption {
    fn run(self, interact_conf: &InteractConfig) -> Result<(), Box<dyn error::Error>>;
}

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct LayoutOptions {
    #[structopt(long, short)]
    layout: Option<String>,

    #[structopt(long, short)]
    path: Option<String>,

    #[structopt(long, short, use_delimiter = true)]
    keys: Option<Vec<String>>,

    #[structopt(long, short)]
    hide_key: bool,

    #[structopt(long, short)]
    depth: Option<usize>,
}

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Subcommand {
    Layout(LayoutOptions),
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
                .join("LOOKME.toml")
        };

        Ok(InteractConfig {
            project_name,
            file
        })
    }

    fn run_helper(&self, line: &String) -> Result<bool, Box<dyn error::Error>> {
        use Subcommand::*;

        let args = commandline_parser::to_args(&line)?;
        let interact_opt = Subcommand::from_iter_safe(args)?;
        match interact_opt {
            Layout(layout_options) => {
                layout_options.run(&self)?;
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
        tree.insert_nodes_from_str(&node_str, config.file.clone(), None)?;

        let mut rl = rustyline::Editor::<()>::new();
        let mut line;

        loop {
            line = rl.readline("(wispha) ");
            match &line {
                Ok(line) => {
                    match config.run_helper(&line) {
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
