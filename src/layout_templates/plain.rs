use libwispha::core::*;

use crate::layouter::*;
use super::resolve_handler;

use std::error;

pub struct PlainLayout { }

impl PlainLayout {
    fn new() -> PlainLayout {
        PlainLayout { }
    }
}

impl Layout for PlainLayout {
    fn info(&self) -> LayoutInfo {
        LayoutInfo {
            name: "".to_string(),
            version: "1.0".to_string()
        }
    }

    fn manual(&self) -> String {
        "".to_string()
    }

    fn layout(&self, tree: &Tree, node_path: &NodePath, depth: usize) -> Result<String, Box<dyn error::Error>> {
        tree.resolve_in_depth(node_path, depth, &resolve_handler).map_err(|error| Box::new(error))?;

        Ok("".to_string())
    }
}