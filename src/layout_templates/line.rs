use libwispha::core::*;

use crate::layouter::*;
use super::resolve_handler;

use std::error;

pub struct LineLayout { }

impl LineLayout {
    pub fn new() -> LineLayout {
        LineLayout { }
    }

    // if `depth` < `max`, return `Some`, else return `None`
    fn layout_helper(&self,
                     tree: &Tree,
                     node_path: &NodePath,
                     depth: usize,
                     max: usize,
                     finished: &mut Vec<bool>,
                     is_last: bool,
                     keys: &Vec<String>,
                     hide_key: bool) -> Option<String> {
        if depth <= max {
            let mut line = String::new();
            // Can safely unwrap because of the effect of `resolve_node`
            let node = tree.get_node(node_path).unwrap();
            let node = node.borrow();
            let direct_node = node.get_direct().unwrap();

            // `depth == 0` means the node is at root
            if depth > 0 {
                for step in 1..depth {
                    if finished[step - 1] {
                        line += "    ";
                    } else {
                        line += "│   ";
                    }
                }
                if is_last {
                    line += "└── ";
                } else {
                    line += "├── ";
                }
            }

            line += &direct_node.node_properties.name;

            if keys.len() == 1 {
                let key = &keys[0];
                if let Some(property) = direct_node.properties.get(key) {
                    line += "\t\t";
                    if !hide_key {
                        line += key;
                        line += ": ";
                    }
                    line += property;
                }
            } else if keys.len() > 1 {
                for key in keys {
                    if let Some(property) = direct_node.properties.get(key) {
                        line += "\t\t";
                        line += key;
                        line += ": ";
                        line += property;
                    }
                }
            }

            let mut sub_lines = if let Some((last_child, remain)) = direct_node.children.split_last() {
                let new_depth = depth + 1;

                let mut strings = remain.iter().filter_map(|child_path| {
                    self.layout_helper(tree, child_path, new_depth, max, finished, false, keys, hide_key)
                }).collect::<Vec<String>>();

                if depth > 0 {
                    finished[depth] = true;
                }

                if let Some(last_string) = self.layout_helper(tree, last_child, new_depth, max, finished, true, keys, hide_key) {
                    strings.push(last_string);
                }

                // Restore for next parent
                if depth > 0 {
                    finished[depth] = false;
                }
                strings
            } else {
                vec![]
            };

            sub_lines.insert(0, line);

            Some(sub_lines.join("\n"))
        } else {
            None
        }
    }
}

impl Layout for LineLayout {
    fn info() -> LayoutInfo {
        LayoutInfo {
            name: "line".to_string(),
            version: "1.0".to_string()
        }
    }

    fn layout(&self,
              tree: &Tree,
              node_path: &NodePath,
              depth: usize,
              keys: &Vec<String>,
              hide_key: bool) -> Result<String, Box<dyn error::Error>> {
        tree.resolve_node(node_path, &resolve_handler, &*crate::PRESERVED_KEYS)?;
        tree.resolve_in_depth(node_path, depth, &resolve_handler, &*crate::PRESERVED_KEYS)?;
        let mut finished = vec![false; depth];
        Ok(self.layout_helper(tree,
                              node_path,
                              0,
                              depth,
                              &mut finished,
                              false,
                              keys,
                              hide_key).unwrap())
    }
}