// Write a program that creates a struct to represent a complex number and implements methods for addition and subtraction.

struct ComplexNumber {
    real: i32,
    imaginary: i32
}

impl ComplexNumber {
    fn add(&self, other: &ComplexNumber) -> Self {
        Self {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary
        }
    }

    fn subtract(&self, other: &ComplexNumber) -> Self {
        Self {
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary
        }
    }
}

fn main() {
    let complex_number1: ComplexNumber = ComplexNumber {
        real: 5,
        imaginary: 6
    };
    println!("First Complex Number: {}+{}i", complex_number1.real, complex_number1.imaginary);

    let complex_number2: ComplexNumber = ComplexNumber {
        real: 7,
        imaginary: 2
    };
    println!("Second Complex Number: {}+{}i", complex_number2.real, complex_number2.imaginary);

    let addition: ComplexNumber = complex_number1.add(&complex_number2);
    let subtract: ComplexNumber = complex_number1.subtract(&complex_number2);
    
    println!("Addition of two complex number is {}+{}i", addition.real, addition.imaginary);
    println!("Subtraction of two complex number is {}+{}i", subtract.real, subtract.imaginary);
}
