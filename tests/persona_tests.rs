
/// PersonaMan can create and manage personas.

use coti::persona::{
    PersonaMan,
};

#[test]
fn persona_create() {
    let mut pm = PersonaMan::new();
    pm.new_persona("aaa".to_string());
    pm.print();
}
