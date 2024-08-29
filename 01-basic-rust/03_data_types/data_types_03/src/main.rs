// Convert a number from one type to another and print the result.

fn main() {
    let num_u32 : u32 = 34;
    let num_i32 : i32 = num_u32 as i32;
    println!("Number: {}", num_i32);
}
