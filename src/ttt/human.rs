use std::io::{self, Write};

use super::{player::Player, mark::Mark};


pub struct HumanPlayer {
    mark: Mark,
    last_time_taken: u128,
}

impl HumanPlayer {
    pub fn new(mark: Mark) -> Self {
        HumanPlayer {
            mark,
            last_time_taken: 0,
        }
    }
}

impl Player for HumanPlayer {
    fn next_move(&mut self, board: &super::board::Board) -> usize {
        let start = std::time::Instant::now();
        let choice = loop {
            print!("Please enter a number between 0 and 8 to place your mark: ");
            io::stdout().flush().expect("Failed to flush stdout");
            let mut choice = String::new();
            std::io::stdin().read_line(&mut choice).expect("Failed to read line");
            let choice: usize = match choice.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number.");
                    continue;
                },
            };
            if choice > 8 {
                println!("Please enter a number between 0 and 8.");
                continue;
            }
            if !board.is_free(choice) {
                println!("That space is already taken.");
                continue;
            }
            break choice;
        };
        self.last_time_taken = start.elapsed().as_nanos();
        choice
    }

    fn time_taken(&self) -> u128 {
        self.last_time_taken
    }
}