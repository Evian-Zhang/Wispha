use libwispha::core::*;

use std::error;

pub struct LayoutInfo {
    pub name: String,
    pub version: String
}

pub trait Layout {
    fn info(&self) -> LayoutInfo;

    fn manual(&self) -> String;

    fn layout(&self, tree: &Tree, node_path: &NodePath, depth: usize) -> Result<String, Box<dyn error::Error>>;
}