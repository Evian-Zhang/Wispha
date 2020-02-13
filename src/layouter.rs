use libwispha::core::*;

use std::error;
use std::collections::HashMap;
use std::fmt;

pub struct LayoutInfo {
    pub name: String,
    pub version: String
}

pub trait Layout {
    fn info(&self) -> LayoutInfo;

    fn manual(&self) -> String;

    fn layout(&self,
              tree: &Tree,
              node_path: &NodePath,
              depth: usize,
              keys: &Vec<String>,
              hide_key: bool) -> Result<String, Box<dyn error::Error>>;
}

pub struct LayoutManager {
    layouts: HashMap<String, Box<dyn Layout>>
}

impl LayoutManager {
    pub fn new(layouts: Vec<Box<dyn Layout>>) -> LayoutManager {
        let layouts = layouts.into_iter().map(|layout| {
            (layout.info().name.clone(), layout)
        }).collect::<HashMap<String, Box<dyn Layout>>>();

        LayoutManager {
            layouts
        }
    }

    pub fn layout(&self,
                  name: &str,
                  tree: &Tree,
                  node_path: &NodePath,
                  depth: usize,
                  keys: &Vec<String>,
                  hide_key: bool) -> Result<String, Box<dyn error::Error>> {
        let layout = self.layouts.get(name).ok_or(Box::new(Error::LayoutNotFound(name.to_string())))?;
        layout.layout(tree, node_path, depth, keys, hide_key)
    }
}

#[derive(Debug)]
pub enum Error {
    LayoutNotFound(String)
}

impl error::Error for Error { }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Error::*;
        let message = match &self {
            LayoutNotFound(name) => format!("Can't find a layout named {}.", name)
        };
        write!(f, "{}", message)
    }
}