// Declare a variable with a tuple and access its elements.

fn main() {
    let point: (u8, u8) = (7, 9);
    let (x, y): (u8, u8) = point;

    println!("x-axis: {}, y-axis: {}", x, y);
}
