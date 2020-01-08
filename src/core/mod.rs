use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, Weak};

/// Wispha node structure
pub struct WisphaNode {
    /// If a wispha node doesn't have parent (for example, `root` in Wispha tree), this field is `None`
    pub parent: Option<PathBuf>,

    /// If a wispha node doesn't have any child, this field is an vector with length 0
    pub children: Vec<PathBuf>
}

/// Wispha tree structure
pub struct WisphaTree {
    pub entries: HashMap<PathBuf, Arc<Mutex<WisphaNode>>>,
    pub root: Arc<Mutex<WisphaNode>>
}
