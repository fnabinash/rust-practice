/// This is a simple "Hello, World!" program in Rust.
/// 
/// The main function is the entry point of the program.
/// 
/// When executed, it prints "Hello, world!" to the console.
fn main() {
    println!("{}", print_hello());
}

/// Returns the string "Hello, world!".
///
/// # Returns
///
/// A `String` containing "Hello, world!"
fn print_hello() -> String {
    "Hello, world!".to_string()
}

// Run`cargo test` to compile and run the tests defined by the #[cfg(test)] attribute.
#[cfg(test)]
mod tests {
    // This brings all items from the parent module into scope,
    // allowing us to use the `print_hello()` function in our tests.
    use super::*;

    /// Test to ensure that the `print_hello()` function returns the correct string.
    #[test]
    fn test_hello() {
        assert_eq!(print_hello(), "Hello, world!");
    }
}