// Write a function that checks if a number is prime.

fn main() {
    let num: u32 = 27;
    is_prime(num);
}

fn is_prime(num: u32) {
    for i in 2..=num / 2 {
        if num % i == 0 {
            println!("{} is not prime.", num);
            return;
        }
    }

    println!("{} is prime.", num);
}
