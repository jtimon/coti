
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

#[test]
fn persona_delete() {
    let mut pm = PersonaMan::new();
    pm.new_persona("aaa".to_string());
    pm.delete_persona("aaa".to_string());
}

#[test]
fn persona_get_persona() {
    let mut pm = PersonaMan::new();
    pm.new_persona("aaa".to_string());
    let persona = pm.get_persona("aaa".to_string()).unwrap();
    assert_eq!(&"aaa".to_string(), persona.get_name());
}

#[test]
fn persona_get_mut_persona() {
    let mut pm = PersonaMan::new();
    pm.new_persona("aaa".to_string());
    let persona = pm.get_mut_persona("aaa".to_string()).unwrap();
    assert_eq!(&"aaa".to_string(), persona.get_name());
}

#[test]
fn persona_get_persona_not_found() {
    let mut pm = PersonaMan::new();
    let persona = pm.get_persona("aaa".to_string());
    assert!(persona.is_none());
}

#[test]
fn persona_get_mut_persona_not_found() {
    let mut pm = PersonaMan::new();
    let persona = pm.get_mut_persona("aaa".to_string());
    assert!(persona.is_none());
}
