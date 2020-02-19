mod layouter;
mod layout_templates;
mod commandline;

use commandline::CommandlineOption;

use structopt::StructOpt;

use std::error;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref PRESERVED_KEYS: Vec<&'static str> = vec![
    "path",
    "parent",
    "record_file"
    ];
}

fn run() -> Result<(), Box<dyn error::Error>> {
    let opt = commandline::Commandline::from_args();
    opt.run()?;
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Err: {}", error);
        std::process::exit(1);
    }
}
