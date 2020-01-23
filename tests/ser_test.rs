mod common;

use libwispha::core::{structs::*, serde::{*, ser::*}};

#[test]
fn to_json_test() {
    let tree = common::normal_tree();
    let json = tree.borrow().to_string(DataFormat::Json);
    assert!(json.is_ok())
}