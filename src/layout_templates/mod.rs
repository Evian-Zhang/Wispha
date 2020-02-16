pub mod plain;

use libwispha::core::*;
use libwispha::manipulator;

use crate::layouter::Layout;

use std::fs;
use std::error;
use std::fmt;
use std::path::PathBuf;

fn resolve_handler(link_node: &LinkNode) -> Result<(PathBuf, String), manipulator::Error> {
    let path = if link_node.target.is_absolute() {
        link_node.target.clone()
    } else {
        link_node.node_properties.record_file.parent().unwrap()
                 .join(link_node.target.clone())
    };
    let node_str = fs::read_to_string(&path)
        .or(Err(manipulator::Error::Custom(Box::new(Error::PathNotExist(path.clone())))))?;
    Ok((path, node_str))
}

#[derive(Debug)]
enum Error {
    PathNotExist(PathBuf)
}

impl error::Error for Error { }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Error::*;
        let message = match &self {
            PathNotExist(path) => format!("Can't open file at {}.", path.to_str().unwrap())
        };
        write!(f, "{}", message)
    }
}

pub fn layout_resolver(name: &str) -> Option<Box<dyn Layout>> {
    match name {
        "plain" => Some(Box::new(plain::PlainLayout::new())),
        _ => None
    }
}