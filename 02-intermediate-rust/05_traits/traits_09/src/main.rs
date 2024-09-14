// Create a trait called StateTransition with a method transition that takes an charachter and based on his score change his level in GameLevel (with levels like Beginner, Intermediate, and Expert).

use std::{io, process};

use rand::Rng;

trait StateTransition {
    fn transition(&mut self);
}

#[allow(dead_code)]
#[derive(Debug)]
struct Player {
    name: String,
    score: i32,
    levels: Levels
}

#[derive(Debug)]
enum Levels {
    Beginner,
    Intermediate,
    Expert
}

impl Player {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            score: 0,
            levels: Levels::Beginner
        }
    }

    fn play(&mut self) {
        let rand_score: i32 = rand::thread_rng().gen_range(-5..10);
        println!("Random num: {:?}", rand_score);
        self.score += rand_score;
        self.transition();
    }
}

impl StateTransition for Player {
    fn transition(&mut self) {
        if self.score > 50 && self.score <= 100 {
            self.levels = Levels::Intermediate;
        }

        if self.score > 500 {
            self.levels = Levels::Expert;
        }
    }
}

fn main() {
    let mut player: Player = Player::new("Sam");
    println!("Click \"enter\" to continue playing or \"q\" to quit the game.");

    loop {
        let mut action: String = String::new();

        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read the user input!");

        if action.trim() == "q" {
            process::exit(1);
        }

        player.play();
        println!("{:?}", player);
    }
}
