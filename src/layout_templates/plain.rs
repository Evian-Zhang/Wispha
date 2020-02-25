use libwispha::core::*;

use crate::layouter::*;
use super::resolve_handler;

use std::error;

pub struct PlainLayout { }

impl PlainLayout {
    // if `depth` < `max`, return `Some`, else return `None`
    fn layout_helper(tree: &Tree,
                     node_path: &NodePath,
                     depth: usize,
                     max: usize,
                     keys: &Vec<String>,
                     hide_key: bool) -> Option<Vec<(String, String)>> {
        if depth <= max {
            let mut line = String::new();
            // Can safely unwrap because of the effect of `resolve_node`
            let node = tree.get_node(node_path).unwrap();
            let node = node.borrow();
            let direct_node = node.get_direct().unwrap();

            line += &vec!["    "; depth].concat();

            line += &direct_node.node_properties.name;

            let mut appendix = String::new();

            if keys.len() == 1 {
                let key = &keys[0];
                if let Some(property) = direct_node.properties.get(key) {
                    if !hide_key {
                        appendix += key;
                        appendix += ": ";
                    }
                    appendix += property;
                }
            } else if keys.len() > 1 {
                for key in keys {
                    if let Some(property) = direct_node.properties.get(key) {
                        appendix += key;
                        appendix += ": ";
                        appendix += property;
                        appendix += "\t\t";
                    }
                }
            }

            let mut sub_lines = direct_node.children.iter().filter_map(|child_path| {
                PlainLayout::layout_helper(tree, child_path, depth + 1, max, keys, hide_key)
            }).flatten().collect::<Vec<(String, String)>>();

            sub_lines.insert(0, (line, appendix));

            Some(sub_lines)
        } else {
            None
        }
    }

    fn appender(strings_and_appendices: Vec<(String, String)>) -> String {
        let max_len = strings_and_appendices.iter().fold(0, |pre_len, (name, _)| {
            let len = name.chars().count();
            if pre_len > len {
                pre_len
            } else {
                len
            }
        });
        let pre_len = max_len + 4;
        strings_and_appendices.into_iter().map(|(name, appendix)| {
            let len = name.chars().count();
            let remain = pre_len - len;
            name + &" ".repeat(remain) + &appendix
        }).collect::<Vec<String>>().join("\n")
    }
}

impl Layout for PlainLayout {
    fn info() -> LayoutInfo {
        LayoutInfo {
            name: "plain".to_string(),
            version: "1.0".to_string()
        }
    }

    fn layout(tree: &Tree,
              node_path: &NodePath,
              depth: usize,
              keys: &Vec<String>,
              hide_key: bool) -> Result<String, Box<dyn error::Error>> {
        tree.resolve_node(node_path, &resolve_handler, &*crate::PRESERVED_KEYS)?;
        tree.resolve_in_depth(node_path, depth, &resolve_handler, &*crate::PRESERVED_KEYS)?;
        let strings_and_appendices = PlainLayout::layout_helper(tree,
                                                                node_path,
                                                                0,
                                                                depth,
                                                                keys,
                                                                hide_key).unwrap();
        Ok(PlainLayout::appender(strings_and_appendices))
    }
}