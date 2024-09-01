// Create an enum to represent traffic light states and use pattern matching to display messages.

enum TrafficLight {
    Red,
    Yellow,
    Green
}

impl TrafficLight {
    fn display(&self) {
        match self {
            Self::Red => println!("STOP!"),
            Self::Yellow => println!("Slow Down!"),
            Self::Green => println!("Go!"),
        }
    }
}

fn main() {
    let light: TrafficLight = TrafficLight::Yellow;
    light.display();
    let light: TrafficLight = TrafficLight::Red;
    light.display();
    let light: TrafficLight = TrafficLight::Green;
    light.display();
}
