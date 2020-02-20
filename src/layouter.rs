use libwispha::core::*;

use std::error;

pub struct LayoutInfo {
    pub name: String,
    pub version: String
}

/// Layout template
pub trait Layout {
    fn info() -> LayoutInfo;

    /// Display a tree layout relative to `node_path` in `depth` with `keys`.
    ///
    /// `depth`: the `node_path` itself is at depth 0. All its children whose depth <= `depth` will be displayed
    /// `keys`: the property keys that will be displayed
    /// `hide_key`: if `keys` only has one element, and `hide_key` is `true`, then the key itself will not be displayed
    fn layout(tree: &Tree,
              node_path: &NodePath,
              depth: usize,
              keys: &Vec<String>,
              hide_key: bool) -> Result<String, Box<dyn error::Error>>;
}