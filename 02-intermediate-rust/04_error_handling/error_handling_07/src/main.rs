// Write a program that attempts to divide two numbers and handles division by zero errors.

fn divide(a: u32, b: u32) -> Result<u32, String> {
    if b == 0 {
        return Err("Can not divide by 0.".to_string());
    }

    Ok(a / b)
}

fn main() {
    match divide(6, 0) {
        Ok(ans) => println!("{} / {} = {}", 6, 0, ans),
        Err(e) => println!("Error: {:?}", e),
    }
}
