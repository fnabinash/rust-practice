// Create a program that prints a greeting based on the time of day.

use std::time::SystemTime;

const ONE_HOUR_IN_SEC: u64 = 3600;

fn main() {
    let mut epoch_time_at_12_am: u64 = 1724697000;
    let now: u64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    let mut difference: u64 = now - epoch_time_at_12_am;

    while difference >= 86400 {
        epoch_time_at_12_am += 86400;
        difference = now - epoch_time_at_12_am;
    }

    let hour_elapsed: u64 = difference / ONE_HOUR_IN_SEC;

    if hour_elapsed > 21 {
        println!("Good Night");
    } else if hour_elapsed >= 17 {
        println!("Good Evening");
    } else if hour_elapsed >= 12 {
        println!("Good Afternoon");
    } else if hour_elapsed >= 5 {
        println!("Good Morning");
    } else {
        println!("Good Night");
    }
}
