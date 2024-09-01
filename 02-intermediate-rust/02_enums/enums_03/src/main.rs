// Write a program that uses an enum to represent the result of a calculation (Success, Error) and uses pattern matching to handle both cases.

enum Result {
    Success(i32),
    Error(String)
}

fn devide(a: i32, b: i32) -> Result {
    if b == 0 {
        Result::Error("Can not divided by 0.".to_string())
    } else {
        Result::Success(a / b)
    }
}

fn main() {
    let result: Result = devide(5, 0);

    match result {
        Result::Success(r) => println!("Success. Value: {}", r),
        Result::Error(e) => println!("Error. {}", e),
    }
}
