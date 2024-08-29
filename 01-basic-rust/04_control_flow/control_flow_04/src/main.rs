// Implement a program that checks whether a character is a vowel or consonant

fn main() {
    let ch: char = 'w';

    match ch {
        'a' | 'e' | 'i' | 'o' | 'u' => println!("{:?} is vowel.", ch),
        _ => println!("{:?} is consonant.", ch),
    }
}
