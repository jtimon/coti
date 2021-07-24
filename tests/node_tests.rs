
/// Test the node module.

use coti::node::{
    OwnNode,
};

#[test]
fn node_create() {
    let _node1 = OwnNode::new("aaa".to_string());
}

#[test]
fn node_connect() {
    let mut node1 = OwnNode::new("aaa".to_string());
    let node2 = OwnNode::new("bbb".to_string());

    assert!(node1.connect(node2.get_pubkey(), node2.get_address().to_string()));
}

#[test]
fn node_disconnect() {
    let mut node1 = OwnNode::new("aaa".to_string());
    let node2 = OwnNode::new("bbb".to_string());

    assert!(node1.connect(node2.get_pubkey(), node2.get_address().to_string()));
    assert!(node1.disconnect(node2.get_pubkey()));
}
