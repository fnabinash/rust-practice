// Shadow a variable and print both values.

fn main() {
    let value: &str = "You are not brave.";
    println!("{}", value);
    
    let value: &str = "Men are brave.";
    println!("{}", value);
}
