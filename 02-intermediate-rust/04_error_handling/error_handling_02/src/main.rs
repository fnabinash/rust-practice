// Implement a function that returns a Result type and handles both success and error cases.

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
