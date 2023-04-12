use std::io::stdin;

fn what_is_your_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_lowercase()
}

fn was_person_invited(name: &String) -> bool {
    let mut allow_them_in = false;
    let visitors = ["bert", "steve", "freed"];
    for visitor in &visitors {
        if visitor == name {
            allow_them_in = true;
            break;
        }
    }
    allow_them_in
}

fn main() {
    println!("Hello, what's your name?");
    let name = what_is_your_name();
    println!("Hello, {name:?}");
    let was_invited = was_person_invited(&name);
    if was_invited {
        println!("Welcome to the party!");
    } else {
        println!("You're not invited!");
    }
}
