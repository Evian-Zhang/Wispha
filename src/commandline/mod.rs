use structopt::StructOpt;

use std::error;

mod layout;
mod interact;
mod generate;

pub trait CommandlineOption {
    fn run(self) -> Result<(), Box<dyn error::Error>>;
}

#[derive(StructOpt)]
pub enum Commandline {
    /// Display a project layout
    Layout(layout::LayoutOptions),

    /// Generate default JSON files
    Generate(generate::GenerateOptions),

    /// Enter interact mode
    Interact(interact::InteractOptions)
}

impl CommandlineOption for Commandline {
    fn run(self) -> Result<(), Box<dyn error::Error>> {
        use Commandline::*;
        match self {
            Layout(layout_options) => layout_options.run(),
            Generate(generate_options) => generate_options.run(),
            Interact(interact_options) => interact_options.run(),
        }
    }
}
