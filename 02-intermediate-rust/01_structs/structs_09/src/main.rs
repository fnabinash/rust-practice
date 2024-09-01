// Write a program that creates a struct to represent a person with fields for name, age, and address, and implement methods to update the address.


#[allow(dead_code)]
struct Person {
    name: String,
    age: u8,
    address: Address
}

#[allow(dead_code)]
#[derive(Debug)]
struct Address {
    home_no: u8,
    street: String,
    city: String,
    pincode: u32,
}

impl Person {
    fn change_address(&mut self, new_address: Address) {
        self.address = new_address;
        println!("You have successfully changed your address to {:?}.", self.address);
    }
}

fn main() {
    let address: Address = Address {
        home_no: 89,
        street: "Jasmine".to_string(),
        city: "Deskodia".to_string(),
        pincode: 459234
    };
    let mut person: Person = Person {
        name: "Steve".to_string(),
        age: 34,
        address
    };

    let new_address: Address = Address {
        home_no: 23,
        street: "Coriander".to_string(),
        city: "Coleg".to_string(),
        pincode: 190220
    };

    person.change_address(new_address);
}
