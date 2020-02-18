use std::error;
use std::fmt;

pub fn to_args(input: &String) -> Result<Vec<&str>, Error> {
    let mut args = vec![""];

    args.append(&mut input.split(" ").collect::<Vec<&str>>());
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
