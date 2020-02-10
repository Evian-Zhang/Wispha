use libwispha::core::*;
use libwispha::plugin::*;

use std::error;

pub struct PlainDisplayer {

}

impl Plugin for PlainDisplayer {
    fn info() -> PluginInfo {
        PluginInfo {
            name: "".to_string(),
            version: "1.0".to_string()
        }
    }

    fn display(tree: &Tree, node_path: &NodePath, depth: usize) -> Result<String, Box<dyn error::Error>> {
        unimplemented!()
    }
}
