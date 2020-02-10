use crate::core::*;

use std::error;

pub struct PluginInfo {
    pub name: String,
    pub version: String
}

pub trait Plugin {
    fn info() -> PluginInfo;

    fn display(tree: &Tree, node_path: &NodePath, depth: usize) -> Result<String, Box<dyn error::Error>>;
}