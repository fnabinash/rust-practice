// Create an enum to represent different operations (Add, Subtract, Multiply, Divide) and use pattern matching to implement a basic calculator.

enum Calculator {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl Calculator {
    fn perform(&self, a: i32, b: i32) {
        match self {
            Self::Add => println!("{} + {} = {}", a, b, a + b),
            Self::Subtract => println!("{} - {} = {}", a, b, a - b),
            Self::Multiply => println!("{} * {} = {}", a, b, a * b),
            Self::Divide if b == 0 => println!("Can not devide by 0."),
            Self::Divide => println!("{} / {} = {}", a, b, a / b)
        }
    }
}

fn main() {
    Calculator::Add.perform(5, 7);
    Calculator::Subtract.perform(5, 7);
    Calculator::Multiply.perform(5, 7);
    Calculator::Divide.perform(5, 0);
}
