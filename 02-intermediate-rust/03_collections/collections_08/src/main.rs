// Implement a program that uses a HashMap to group words by their first letter.

use std::collections::HashMap;

fn group_words(grouped_words: &mut HashMap<char, Vec<String>>, words: &[String]) {
    words.iter().for_each(|word| {
        let first_char: char = word.chars().next().unwrap();

        let words: &mut Vec<String> = grouped_words.entry(first_char).or_default();
        words.push(word.clone());
    });
}

fn main() {
    let words: Vec<String> = vec![
        "hello".to_string(),
        "World".to_string(),
        "Alice".to_string(),
        "Google".to_string(),
        "Amazon".to_string(),
        "Netflix".to_string(),
        "Godzila".to_string()
    ];

    let mut grouped_words: HashMap<char, Vec<String>> = HashMap::new();

    group_words(&mut grouped_words, &words);

    println!("{:?}", grouped_words);
}
