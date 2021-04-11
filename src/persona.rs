//! The persona module manages personas.

struct Persona {
    pubk: Vec<u8>, // TODO use real cryptography
    prik: Vec<u8>, // TODO use real cryptography
    persona_name: String, // This is not unique, other personas can always sign with the same name
}

impl Persona {
    pub fn new(persona_name: String) -> Persona {
        // TODO use real cryptography
        let pubk = Vec::<u8>::new();
        let prik = Vec::<u8>::new();
        Persona{pubk, prik, persona_name}
    }
}

pub struct PersonaMan {
    personas: Vec<Persona>,
}

impl PersonaMan {
    pub fn new() -> PersonaMan {
        PersonaMan{personas: Vec::<Persona>::new()}
    }

    pub fn new_persona(&mut self, persona_name: String) {
        self.personas.push(Persona::new(persona_name));
    }

    pub fn print(&self) {
        for p in &self.personas {
            println!("persona_name: {}, prik: {:?}", p.persona_name, p.prik);
        }
    }
}
