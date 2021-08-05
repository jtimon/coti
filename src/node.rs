
//! The node module implements a p2p coti node.

use std::collections::HashMap;

use crate::crypto::{
    CryptoAlgorithm,
    PrivKey,
    PubKey,
};

pub struct NNode {
    pubk: PubKey,
    address: String,
}

impl NNode {
    pub fn new(pubk: PubKey, address: String) -> NNode {
        NNode{pubk, address}
    }

    pub fn get_pubkey(&self) -> &PubKey {
        &self.pubk
    }

    pub fn get_address(&self) -> &String {
        &self.address
    }
}

pub struct OwnNode {
    node: NNode,
    // served_personas: HashMap<PubKey, NNode>
    #[allow(dead_code)] // TODO Remove cheating
    prik: PrivKey,
    connections: HashMap<PubKey, NNode>,
}

impl OwnNode {

    pub fn new(address: String) -> OwnNode {
        // TODO use real cryptography
        let prik = PrivKey::new(CryptoAlgorithm::DUMMY).unwrap();
        OwnNode{node: NNode::new(prik.get_pub(), address), prik, connections: HashMap::new()}
    }

    pub fn get_pubkey(&self) -> &PubKey {
        &self.node.get_pubkey()
    }

    pub fn get_address(&self) -> &String {
        &self.node.get_address()
    }

    pub fn connect(&mut self, pubk: &PubKey, address: String) -> bool {
        // let ack = self.prik.sign("COTIv0.0_ACK".to_string());
        let nnode = NNode::new(pubk.clone(), address);
        self.connections.insert(pubk.clone(), nnode);
        true // TODO use real networking
    }

    pub fn disconnect(&mut self, pubk: &PubKey) -> bool {
        self.connections.remove(&pubk);
        true // TODO use real networking
    }

    // TODO change msg from string to something more low level for msg
    pub fn send_msg(&mut self, dest_pubk: &PubKey, msg: String) -> bool {
        match self.connections.get(dest_pubk) {
            None => false,
            Some(dest_node) => {
                let signed_msg = self.prik.sign(msg);
                let crypt_signed_msg = dest_node.get_pubkey().encrypt(signed_msg);
                // dest_node.get_address()
                true // TODO use real networking
            },
        }
    }
}