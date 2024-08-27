// Create a mutable variable and change its value inside a loop.

fn main() {
    let mut mutted_times: u8 = 0;

    loop {
        mutted_times += 1;

        if mutted_times > 23 {
            break;
        }
    }

    println!("Final value is {}.", mutted_times);
}
