//! The persona module manages personas.

use std::collections::HashMap;

use crate::crypto::{
    CryptoAlgorithm,
    PrivKey,
    PubKey,
    SignedMsg,
};

struct Followee {
    persona_pubk: PubKey,
}

pub struct Persona {
    pubk: PubKey,
    prik: PrivKey,
    // This is not unique, other personas can always sign with the same name. It can also be changed.
    persona_name: String,
    signed_msgs: Vec<SignedMsg>,
    followees: HashMap<PubKey, Followee>,
}

impl Persona {
    pub fn new(persona_name: String) -> Persona {
        // TODO use real cryptography
        let prik = PrivKey::new(CryptoAlgorithm::DUMMY).unwrap();
        let pubk = prik.get_pub();
        Persona{pubk, prik, persona_name, signed_msgs: Vec::new(), followees: HashMap::new()}
    }

    pub fn get_name(&self) -> &String {
        &self.persona_name
    }

    pub fn get_pubkey(&self) -> &PubKey {
        &self.pubk
    }

    pub fn sign_msg(&mut self, msg: String) {
        let signed_msg = self.prik.sign(msg);
        self.signed_msgs.push(signed_msg);
    }

    pub fn get_signed_messages(&self) -> &Vec<SignedMsg> {
        &self.signed_msgs
    }

    pub fn follow(&mut self, persona_pubk: PubKey) {
        self.followees.insert(persona_pubk.clone(), Followee{persona_pubk});
    }
    // pub fn insert_contact(&mut self, persona_name: String, persona_pubk: String, local_name: String) {
    //     // TODO search first to ask to confirm replace
    //     self.personas[&persona_name].contacts.insert(local_name.clone(), Contact::new(persona_pubk, local_name));
    // }

}

pub struct PersonaMan {
    personas: HashMap<String, Persona>,
}

impl PersonaMan {
    pub fn new() -> PersonaMan {
        PersonaMan{personas: HashMap::new()}
    }

    pub fn new_persona(&mut self, persona_name: String) {
        self.personas.insert(persona_name.clone(), Persona::new(persona_name));
    }

    pub fn delete_persona(&mut self, persona_name: String) {
        self.personas.remove(&persona_name);
    }

    pub fn get_persona(&mut self, persona_name: String) -> Option<&Persona> {
        self.personas.get(&persona_name)
    }

    pub fn get_mut_persona(&mut self, persona_name: String) -> Option<&mut Persona> {
        self.personas.get_mut(&persona_name)
    }

    pub fn print(&self) {
        for (_name, p) in &self.personas {
            println!("persona_name: {}, pubk: {:?}, pubk (from prik): {:?}", p.persona_name, p.pubk.pubk, p.prik.get_pub().pubk);
        }
    }
}
