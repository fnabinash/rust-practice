// Write a program that creates a trait called Calculable with methods for addition, subtraction, multiplication, and division, and implement it for a struct representing a calculator.

trait Calculator {
    fn addition(&self) -> i32;
    fn subtraction(&self) -> i32;
    fn multiplication(&self) -> i32;
    fn division(&self) -> i32;
}

#[derive(Debug)]
struct Values {
    first: i32,
    second: i32
}

impl Calculator for Values {
    fn addition(&self) -> i32 {
        self.first + self.second
    }

    fn subtraction(&self) -> i32 {
        self.first - self.second
    }

    fn multiplication(&self) -> i32 {
        self.first * self.second
    }

    fn division(&self) -> i32 {
        self.first / self.second
    }
}

fn main() {
    let values: Values = Values {
        first: 89,
        second: 43,
    };

    println!("Values: {:?}", values);

    println!("Addition of values: {}", values.addition());
    println!("Subtraction of values: {}", values.subtraction());
    println!("Multiplication of values: {}", values.multiplication());
    println!("Division of values: {}", values.division());
}
