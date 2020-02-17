use std::error;
use std::fmt;

pub fn to_args(input: &String) -> Result<Vec<String>, Error> {
    let mut args = vec!["wispha".to_string()];

    for arg in input.split(" ") {
        args.push(arg.to_string());
    }
    Ok(args)
}

#[derive(Debug)]
pub enum Error {

}

impl error::Error for Error { }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        Ok(())
    }
}
