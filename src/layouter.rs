use libwispha::core::*;

use std::error;
use std::fmt;

pub struct LayoutInfo {
    pub name: String,
    pub version: String
}

/// Layout template
pub trait Layout {
    fn info(&self) -> LayoutInfo;

    /// Display a tree layout relative to `node_path` in `depth` with `keys`.
    ///
    /// `depth`: the `node_path` itself is at depth 0. All its children whose depth <= `depth` will be displayed
    /// `keys`: the property keys that will be displayed
    /// `hide_key`: if `keys` only has one element, and `hide_key` is `true`, then the key itself will not be displayed
    fn layout(&self,
              tree: &Tree,
              node_path: &NodePath,
              depth: usize,
              keys: &Vec<String>,
              hide_key: bool) -> Result<String, Box<dyn error::Error>>;
}

pub struct LayoutManager { }

impl LayoutManager {
    pub fn layout<F>(name: &str,
                     layout_resolver: &F,
                     tree: &Tree,
                     node_path: &NodePath,
                     depth: usize,
                     keys: &Vec<String>,
                     hide_key: bool) -> Result<String, Box<dyn error::Error>>
        where
            F: Fn(&str) -> Option<Box<dyn Layout>> {
        let layout = layout_resolver(name).ok_or(Error::LayoutNotFound(name.to_string()))?;
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