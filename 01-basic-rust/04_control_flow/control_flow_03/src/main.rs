// Create a calculator using a match statement.

fn main() {
    let first_num: i32 = 34;
    let second_num: i32 = 29;

    let operation: char = '-';

    match operation {
        '+' => println!("Addition of {} and {} is {}.", first_num, second_num, first_num + second_num),
        '-' => println!("Substraction of {} from {} is {}.", second_num, first_num, first_num - second_num),
        '*' => println!("Multiplication of {} with {} is {}.", first_num, second_num, first_num * second_num),
        '/' => println!("Division of {} by {} is {}.", first_num, second_num, first_num as f32 / second_num as f32),
        _ => println!("Invalid operation."),
    }
}
