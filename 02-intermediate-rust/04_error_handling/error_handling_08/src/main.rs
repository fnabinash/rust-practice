// Implement a function that performs a calculation and returns a Result type with a custom error message if the calculation fails.

#[allow(dead_code)]
#[derive(Debug)]
enum CalculationError {
    DivisionByZero,
    NegativeNumber,
    Overflow,
    Other(String),
}

fn perform_calculation(a: i32, b: i32) -> Result<i32, CalculationError> {
    if b == 0 {
        return Err(CalculationError::DivisionByZero);
    }

    let result = a.checked_div(b).ok_or(CalculationError::Overflow)?;

    if result < 0 {
        return Err(CalculationError::NegativeNumber);
    }

    Ok(result)
}

fn main() {
    match perform_calculation(10, 0) {
        Ok(result) => println!("Calculation successful: {}", result),
        Err(e) => match e {
            CalculationError::DivisionByZero => println!("Error: Division by zero is not allowed."),
            CalculationError::NegativeNumber => println!("Error: The result is a negative number."),
            CalculationError::Overflow => println!("Error: Overflow occurred during the calculation."),
            CalculationError::Other(msg) => println!("Error: {}", msg),
        },
    }
}

