// Create an enum to represent different weather conditions (Sunny, Rainy, Cloudy) and use pattern matching to display appropriate messages.

enum Weather {
    Sunny,
    Rainy,
    Cloudy
}

impl Weather {
    fn get(&self) {
        match self {
            Self::Sunny => println!("It's Sunny."),
            Self::Rainy => println!("It's Rainy."),
            Self::Cloudy => println!("It's Cloudy"),
        }
    }
}

fn main() {
    let weather: Weather = Weather::Sunny;
    weather.get();

    let weather: Weather = Weather::Rainy;
    weather.get();

    let weather: Weather = Weather::Cloudy;
    weather.get();
}
