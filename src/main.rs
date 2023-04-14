use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }
    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_lowercase()
}

fn was_person_invited(name: &str) {
    let visitors = [
        Visitor::new("Bob", "Hello, Bob!"),
        Visitor::new("Alice", "Hello, Alice!"),
        Visitor::new("John", "Hello, John!"),
        Visitor::new("Jane", "Hello, Jane!"),
    ];
    let known_visitor = visitors.iter().find(|visitor| visitor.name == name);
    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("You are not invited!")
    }
}

fn main() {
    println!("Hello, what's your name?");
    let name = what_is_your_name();
    was_person_invited(&name);
    let mut name2 = name;
    name2.push_str(" is cool");
    println!("{name2}");
}
