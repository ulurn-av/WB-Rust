trait Action {
    fn say(&self) -> ();
}

struct Person {
    name: String,
}

impl Action for Person {
    fn say(&self) -> () {
        println!("Hello, {}", self.name);
    }
}

fn main() {
    let person: Person = Person {
        name: "Andrey".to_string(),
    };

    person.say();
}
