pub mod plain;

use libwispha::core::*;
use libwispha::manipulator;
use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

fn resolve_handler(tree: &Tree, link_node: &LinkNode) -> Result<Rc<RefCell<Node>>, manipulator::Error> {
    let path = link_node.node_properties.record_file.parent().unwrap()
        .join(link_node.target.clone());
    let node_str = fs::read_to_string(&path)
        .map_err(|io_error| manipulator::Error::Custom(Box::new(io_error)))?;
    tree.insert_nodes_from_str(&node_str,
                               path,
                               link_node.node_properties.parent.clone())
        .map_err(|de_error| manipulator::Error::Custom(Box::new(de_error)))
}