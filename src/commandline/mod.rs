use structopt::StructOpt;

use std::path::PathBuf;
use std::error;

mod layout;
mod interact;

pub trait CommandlineOption {
    fn run(self) -> Result<(), Box<dyn error::Error>>;
}

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct LayoutOptions {
    #[structopt(long, short)]
    layout: Option<String>,

    #[structopt(long, short = "n")]
    project_name: Option<String>,

    #[structopt(long, short)]
    path: Option<String>,

    #[structopt(long, short, use_delimiter = true)]
    keys: Option<Vec<String>>,

    #[structopt(long, short)]
    hide_key: Option<bool>,

    #[structopt(long, short)]
    file: Option<PathBuf>,

    #[structopt(long, short)]
    depth: Option<usize>,
}

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct InteractOptions {
    #[structopt(long, short = "n")]
    project_name: Option<String>,

    #[structopt(long, short)]
    file: Option<PathBuf>,
}

#[derive(StructOpt)]
pub enum Commandline {
    Layout(LayoutOptions),
    Interact(InteractOptions)
}

impl CommandlineOption for Commandline {
    fn run(self) -> Result<(), Box<dyn error::Error>> {
        use Commandline::*;
        match self {
            Layout(layout_options) => layout_options.run(),
            Interact(interact_options) => interact_options.run(),
        }
    }
}
