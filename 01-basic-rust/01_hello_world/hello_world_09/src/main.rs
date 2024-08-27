// Print a countdown from 10 to 1.

fn main() {
    println!("Countdown Starts...");

    for i in (1..=10).rev() {
        println!("Tik Tik {}", i);
    }

    println!("Countdown Finished...");
}
