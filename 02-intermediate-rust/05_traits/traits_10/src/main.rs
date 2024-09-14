// Create a trait called Loggable with a method log that takes an object and logs its state.

use std::{io, process};

use chrono::{Local, NaiveDateTime};

trait Loggable {
    fn log(&self);
}

struct User {
    name: String,
    password: String,
    last_login: NaiveDateTime
}

impl User {
    fn new(name: &str, password: &str) -> Self {
        Self {
            name: name.to_string(),
            password: password.to_string(),
            last_login: Local::now().naive_local()
        }
    }

    fn login(&mut self) {
        let mut password: String = String::new();
        println!("Enter your password: ");
        
        io::stdin()
            .read_line(&mut password)
            .expect("Failed to read!");

        if self.password == password.trim() {
            self.last_login = Local::now().naive_local();
        } else {
            eprintln!("Wrong password!");
            process::exit(1);
        }
    }
}

impl Loggable for User {
    fn log(&self) {
        let last_login_formatted: String = format!("{}", self.last_login.format("%Y-%m-%d %H:%M:%S"));
        println!("Username: {:?}, Last login: {:?}", self.name, last_login_formatted);
    }
}

fn main() {
    let mut user: User = User::new("Sam", "Sam@890");
    user.log();

    user.login();
    user.log();
}
