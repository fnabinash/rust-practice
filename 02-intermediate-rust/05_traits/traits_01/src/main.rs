// Create a trait called Printable with a method print and implement it for different types (e.g., struct, enum).

trait Printable {
    fn print(&self);
}

struct Empty;

enum DoNothing {
    Nothing
}

impl Printable for Empty {
    fn print(&self) {
        println!("The strcut is empty.");
    }
}

impl Printable for DoNothing {
    fn print(&self) {
        println!("This does nothing.");
    }
}

fn main() {
    let empty: Empty = Empty {};
    empty.print();

    let nothing: DoNothing = DoNothing::Nothing;
    nothing.print();
}
