// Write a program that uses a HashMap to store and retrieve configuration settings.

use std::collections::HashMap;

fn main() {
    let mut config_list: HashMap<String, String> = HashMap::new();
    config_list.entry("api_key".to_string()).or_insert("0abs423edcba87Dbajheiwa8122isetr".to_string());
    let api_key: &String = config_list.get("api_key").unwrap();
    println!("API key: {:?}", api_key);
}
