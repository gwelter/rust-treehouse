use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    age: u8,
    action: VisitorAction,
}

impl Visitor {
    fn new(name: &str, age: u8, action: VisitorAction) -> Self {
        Self {
            name: name.to_lowercase(),
            age,
            action,
        }
    }
    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the treehouse party, {}!", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{}, your request is under review.", self.name),
            VisitorAction::Refuse => println!("Please leave.",),
        }
    }
}

fn what_is_your_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_lowercase()
}

fn main() {
    let mut visitors = vec![
        Visitor::new("Bob", 18, VisitorAction::Accept),
        Visitor::new(
            "Alice",
            21,
            VisitorAction::AcceptWithNote {
                note: "She likes cats".to_string(),
            },
        ),
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
                    let visitor = Visitor::new(&name, 18, VisitorAction::Probation);
                    visitor.greet_visitor();
                    visitors.push(visitor);
                }
            }
        }
    }
}
