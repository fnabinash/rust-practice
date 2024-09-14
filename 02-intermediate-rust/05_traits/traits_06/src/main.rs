// Write a program that creates a trait called Cloneable with a method clone and implement it for a struct representing a document.

trait Clonebale {
    fn clone(&self) -> Self;
}

#[derive(Debug)]
struct Nothing {
    something: u32,
}

impl Clonebale for Nothing {
    fn clone(&self) -> Self {
        Self {
            something: self.something
        }
    }
}

fn main() {
    let nothing: Nothing = Nothing { something: 98 };
    let nothing_clone: Nothing = nothing.clone();
    println!("Nothing: {:?}", nothing);
    println!("Nothing clone: {:?}", nothing_clone);
}
