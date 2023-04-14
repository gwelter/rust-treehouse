use std::io::stdin;

#[derive(Debug)]
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

fn main() {
    let mut visitors = vec![
        Visitor::new("Bob", "Hello, Bob!"),
        Visitor::new("Alice", "Hello, Alice!"),
    ];
    loop {
        println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
        let name = what_is_your_name();

        let known_visitor = visitors.iter().find(|visitor| visitor.name == name);
        match known_visitor {
            Some(visitor) => {
                visitor.greet_visitor();
                println!("{:#?}", visitor);
            }
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("Hello, {}! I don't think we've met before.", name);
                    visitors.push(Visitor::new(&name, "New friend!"));
                }
            }
        }
    }
}
