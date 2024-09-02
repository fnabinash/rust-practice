// Create a program that uses a HashMap to implement a basic phone book with names and phone numbers.

use std::collections::HashMap;

fn entry_number(phone_book: &mut HashMap<String, u64>, name: &str, no: u64) {
    phone_book.entry(name.to_string()).or_insert(no);
}

fn update_number(phone_book: &mut HashMap<String, u64>, name: &str, new_no: u64) {
    let no: &mut u64 = phone_book.get_mut(name).unwrap();
    *no = new_no;
}

fn update_name(phone_book: &mut HashMap<String, u64>, old_name: &str, new_name: &str) {
    let no: u64 = phone_book.get(old_name).unwrap().to_owned();
    phone_book.remove(old_name).unwrap();

    phone_book.entry(new_name.to_string()).or_insert(no);
}

fn get_number(phone_book: &HashMap<String, u64>, name: &str) {
    let no: &u64 = phone_book.get(name).unwrap();
    println!("Phone no. of {:?} is {}", name, no);
}

fn main() {
    let mut phone_book: HashMap<String, u64> = HashMap::new();
    let name: String = String::from("Haleb");
    let no: u64 = 6748314092;

    entry_number(&mut phone_book, &name, no);
    get_number(&phone_book, &name);

    let new_no: u64 = 2348924325;
    update_number(&mut phone_book, &name, new_no);
    get_number(&phone_book, &name);

    let new_name: String = "Helio".to_string();
    update_name(&mut phone_book, &name, &new_name);
    get_number(&phone_book, &new_name);
}
