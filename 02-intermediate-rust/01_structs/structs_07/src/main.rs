// Create a struct to represent a car with fields for make, model, and year, and implement methods to start and stop the car.

struct Car {
    make: String,
    model: String,
    year: u16,
    running: bool
}

impl Car {
    fn start(&mut self) {
        self.running = true;
        println!("The {} {} from {} has started.", self.make, self.model, self.year);
    }

    fn stop(&mut self) {
        if self.running {
            println!("The {} {} from {} has stopped.", self.make, self.model, self.year);
            self.running = false;
        } else {
            println!("Really Bro 😏😏! You want to stop a car which is not running now!");
        }
    }
}

fn main() {
    let mut car: Car = Car {
        make: "Toyota".to_string(),
        model: "GR 86".to_string(),
        year: 2023,
        running: false
    };

    car.stop();
    car.start();
    println!("Car is running...");
    car.stop();
}
