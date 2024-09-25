// Create a program that uses a closure to generate a sequence of numbers and returns the result as a vector.

fn main() {
    let numbers: Vec<u32> = vec![2, 1, 7, 6, 4, 9, 3];
    println!("Numbers: {:?}", numbers);

    let squares: Vec<u32> = numbers.iter()
        .map(|num| { u32::pow(*num, 2) })
        .collect();

    println!("Squares of numbers: {:?}", squares);
}
