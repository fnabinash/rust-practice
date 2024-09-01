// Create an enum to represent different types of users (Admin, Guest, Registered) and use pattern matching to display messages based on user type.

enum Users {
    Admin,
    Registered,
    Guest
}

impl Users {
    fn greet(&self) {
        match self {
            Self::Admin => println!("Welcome, Admin!"),
            Self::Registered => println!("Welcome, User!"),
            Self::Guest => println!("You are using as guest. Register Now!"),
        }
    }
}

fn main() {
    let admin: Users = Users::Admin;
    admin.greet();

    let registered_user: Users = Users::Registered;
    registered_user.greet();

    let guest: Users = Users::Guest;
    guest.greet();
}
