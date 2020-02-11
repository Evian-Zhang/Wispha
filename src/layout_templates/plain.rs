use libwispha::core::*;

use crate::layouter::*;
use super::resolve_handler;

use std::error;

pub struct PlainLayout { }

impl PlainLayout {
    pub fn new() -> PlainLayout {
        PlainLayout { }
    }

    fn layout_helper(&self,
                     tree: &Tree,
                     node_path: &NodePath,
                     depth: usize,
                     max: usize,
                     finish_depth: usize,
                     is_last: bool) -> String {
        let mut line = String::new();
        if depth < max {
            let node = tree.get_node(node_path).unwrap();
            let node = node.borrow();
            let direct_node = node.get_direct().unwrap();

            for indent in 0..finish_depth {
                line += "    ";
            }

            for indent in finish_depth..depth {
                line += "│   ";
            }

            if is_last && depth > 0 {
                line += "└── ";
            } else {
                line += "├── ";
            }

            line += &direct_node.node_properties.name;

            direct_node.children.iter().filter_map(|child_path| {
                if let Some(child) = tree.get_node(child_path) {
                    let child = child.borrow();
                    if let Some(direct_node) = child.get_direct() {
                        Some(self.layout_helper(tree, direct_node, depth + 1, max, finish_depth, is_last))
                    } else {
                        None
                    }
                } else {
                    None
                }
            });
        }
        line
    }
}

impl Layout for PlainLayout {
    fn info(&self) -> LayoutInfo {
        LayoutInfo {
            name: "plain".to_string(),
            version: "1.0".to_string()
        }
    }

    fn manual(&self) -> String {
        "".to_string()
    }

    fn layout(&self, tree: &Tree, node_path: &NodePath, depth: usize) -> Result<String, Box<dyn error::Error>> {
        tree.resolve_in_depth(node_path, depth, &resolve_handler).map_err(|error| Box::new(error))?;
        let root = tree.get_node(node_path).unwrap();
        let root = root.borrow();
        let direct_node = root.get_direct().unwrap();
        Ok(self.layout_helper(direct_node, depth))
    }
}