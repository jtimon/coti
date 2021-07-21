//! The contact module manages contact lists.

use std::collections::HashMap;

struct Contact {

    #[allow(dead_code)]
    persona_pubk: String, // TODO use real cryptography
    #[allow(dead_code)]
    local_name: String, // This must be unique per contact list, it is equal to persona_name by default
    // signed names are public names that can be changed, but once you publicly signed a name,
    // everyone will know you signed it once (with that persona).
    #[allow(dead_code)]
    signed_name: HashMap<String, String>,
    // Examples of fields could be phone, email, etc
    #[allow(dead_code)]
    signed: HashMap<String, String>,
}

impl Contact {
    pub fn new(persona_pubk: String, local_name: String) -> Contact {
        Contact{persona_pubk, local_name, signed_name: HashMap::new(), signed: HashMap::new()}
    }
}

pub struct ContactMan {
    contacts: HashMap<String, Contact>,
}

impl ContactMan {
    pub fn new() -> ContactMan {
        ContactMan{contacts: HashMap::new()}
    }

    pub fn new_contact(&mut self, persona_pubk: String, local_name: String) {
        self.contacts.insert(persona_pubk.clone(), Contact::new(persona_pubk, local_name));
    }
}
