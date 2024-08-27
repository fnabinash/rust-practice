// Print the result of basic arithmetic operations.

fn main() {
    let first_number: i32 = 34;
    let second_number: i32 = 98;

    let addition: i32 = first_number + second_number;
    let substraction: i32 = first_number - second_number;
    let multiplication: i32 = first_number * second_number;
    let division: f32 = first_number as f32 / second_number as f32;

    println!("The additon of {} and {} is {}.", first_number, second_number, addition);
    println!("The substraction of {} from {} is {}.", second_number, first_number, substraction);
    println!("The multiplication of {} with {} is {}.", first_number, second_number, multiplication);
    println!("The division of {} by {} is {}.", first_number, second_number, division);
}
