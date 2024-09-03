// Create a program that uses Option to handle missing values and displays a default value if a value is None.

struct Person {
    name: String,
    age: Option<u8>
}

impl Person {
    fn new(name: String, age: Option<u8>) -> Self {
        Self { name, age }
    }

    fn display(&self) {
        println!("Name: {:?}, age: {}", self.name, self.age.unwrap_or_default());
    }
}

fn main() {
    let kesab: Person = Person::new("Kesab".to_string(), Some(45));
    let hars: Person = Person::new("Hars".to_string(), None);

    kesab.display();
    hars.display();
}
