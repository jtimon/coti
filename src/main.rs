
use coti::ui;
use coti::persona::PersonaMan;

fn print_coti_options() {
    println!("Options:");
    println!("1 Manage personas");
    println!("2 Manage contacts");
    println!("3 Publish message");
    println!("0 Quit");
}

fn print_manage_personas(personaman: &mut PersonaMan) {
    println!("Managing personas\n");
    personaman.print();
    println!("Options:");
    println!("1 Create persona");
    println!("2 Delete persona");
    println!("0 Back");
}

fn print_manage_contacts() {
    println!("Managing contacts");
    println!("Options:");
    println!("1 Create contact");
    println!("2 Delete contact");
    println!("0 Back");
}

fn print_publish_message() {
    println!("Publish message");
    println!("1 Send message");
    println!("0 Back");
}

fn create_persona(personaman: &mut PersonaMan) {
    println!("Messages for this new persona will be signed under this name.");
    println!("This is not unique, other personas can always sign with the same name (but with a different key)");
    println!("Persona name:");
    personaman.new_persona(ui::input_string(0, 256));
}

fn manage_personas(personaman: &mut PersonaMan) {

    let mut back = false;
    while !back {
        print_manage_personas(personaman);
        let sel_option : usize = ui::input_u32(0, 3) as usize;

        match sel_option {
            0 => back = true,
            1 => create_persona(personaman),
            2 => println!("ERROR: Delete persona is not implemented"),
            _ => println!("ERROR in manage_personas"),
        }
    }
}

fn manage_contacts() {
    print_manage_contacts();

    let mut back = false;
    while !back {
        let sel_option : usize = ui::input_u32(0, 3) as usize;

        match sel_option {
            0 => back = true,
            1 => println!("ERROR: Create contact is not implemented"),
            2 => println!("ERROR: Delete contact is not implemented"),
            _ => println!("ERROR in manage_contacts"),
        }
    }
}

fn publish_message() {
    print_publish_message();

    let mut back = false;
    while !back {
        let sel_option : usize = ui::input_u32(0, 2) as usize;

        match sel_option {
            0 => back = true,
            1 => println!("ERROR: Send message is not implemented"),
            _ => println!("ERROR in publish_message"),
        }
    }
}

fn main() {
    println!("Coti vPreAlpha\n");

    let mut personaman = PersonaMan::new();
    let mut exit_coti = false;
    while !exit_coti {
        print_coti_options();
        let sel_option : usize = ui::input_u32(0, 3) as usize;

        match sel_option {
            1 => manage_personas(&mut personaman),
            2 => manage_contacts(),
            3 => publish_message(),
            0 => exit_coti = true,
            _ => println!("ERROR in main menu"),
        }
    }
}
