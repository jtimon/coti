//! The persona module manages personas.

use std::collections::HashMap;

use crate::crypto::{
    CryptoAlgorithm,
    PrivKey,
    PubKey,
};

pub struct Persona {
    pubk: PubKey,
    prik: PrivKey,
    persona_name: String, // This is not unique, other personas can always sign with the same name
}

impl Persona {
    pub fn new(persona_name: String) -> Persona {
        // TODO use real cryptography
        let prik = PrivKey::new(CryptoAlgorithm::DUMMY).unwrap();
        let pubk = prik.get_pub();
        Persona{pubk, prik, persona_name}
    }

    pub fn get_name(&self) -> &String {
        &self.persona_name
    }
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
