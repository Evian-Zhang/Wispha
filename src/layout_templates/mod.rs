pub mod plain;
pub mod line;
pub mod triangle;

use libwispha::core::*;

use crate::layouter::Layout;

use std::fs;
use std::error;
use std::fmt;
use std::path::PathBuf;
use std::collections::HashMap;

pub fn resolve_handler(link_node: &LinkNode) -> Result<(PathBuf, String), Box<dyn error::Error>> {
    let path = if link_node.target.is_absolute() {
        link_node.target.clone()
    } else {
        link_node.node_properties.record_file.parent().unwrap()
                 .join(link_node.target.clone())
    };
    if path == link_node.node_properties.record_file {
        Err(Box::new(Error::LoopTarget(path.clone())))
    } else {
        let node_str = fs::read_to_string(&path)
            .or(Err(Box::new(Error::PathNotExist(path.clone()))))?;
        Ok((path, node_str))
    }
}

// see https://stackoverflow.com/questions/60312249/how-to-store-structs-not-instance-that-implement-a-common-trait/60312850#60312850
pub struct LayoutManager {
    templates: HashMap<
        String,
        fn(tree: &Tree,
           node_path: &NodePath,
           depth: usize,
           keys: &Vec<String>,
           hide_key: bool) -> Result<String, Box<dyn error::Error>>
    >
}

impl LayoutManager {
    pub fn new() -> LayoutManager {
        let mut manager = LayoutManager { templates: HashMap::new() };
        manager.register_template::<plain::PlainLayout>();
        manager.register_template::<line::LineLayout>();
        manager.register_template::<triangle::TriangleLayout>();
        manager
    }

    fn register_template<T: Layout>(&mut self) {
        let name = &T::info().name;
        self.templates.insert(name.clone(), T::layout);
    }

    pub fn layout(&self,
                  template: &String,
                  tree: &Tree,
                  node_path: &NodePath,
                  depth: usize,
                  keys: &Vec<String>,
                  hide_key: bool) -> Result<String, Box<dyn error::Error>> {
        if let Some(layout) = self.templates.get(template) {
            layout(tree, node_path, depth, keys, hide_key)
        } else {
            Err(Box::new(Error::LayoutNotFound(template.clone())))
        }
    }
}

#[derive(Debug)]
enum Error {
    PathNotExist(PathBuf),
    LoopTarget(PathBuf),
    LayoutNotFound(String)
}

impl error::Error for Error { }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Error::*;
        let message = match &self {
            PathNotExist(path) => format!("Can't open file at {}.", path.to_str().unwrap()),
            LoopTarget(path) => format!("A node in file {} has target to the file itself.", path.to_str().unwrap()),
            LayoutNotFound(name) => format!("Can't find a layout named {}.", name)
        };
        write!(f, "{}", message)
    }
}