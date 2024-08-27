// Print a multi-line string using raw strings.

fn main() {
    let my_raw_string: &str = r#"
        This
        is
        my
        Raw
        String."#;

    println!("{}", my_raw_string);
}
