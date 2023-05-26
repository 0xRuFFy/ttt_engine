use super::{Board, Player, Mark};
use crate::util;


pub struct Game<T: Player, U: Player> {
    board: Board,
    player_1: T,
    player_2: U,
}

impl <T: Player, U: Player> Game<T, U> {
    pub fn new(player_1: T, player_2: U) -> Self {
        Game {
            board: Board::new(),
            player_1,
            player_2,
        }
    }

    fn print_winner(&self, winner: Mark) {
        util::clear();
        println!("{}", self.board);
        println!("{} has won!", winner);
    }

    pub fn play(&mut self) {
        loop {
            println!("{}", self.board);
            let position = self.player_1.next_move(&self.board);
            self.board.place_next_mark(position).expect("Failed to place mark");
            let winner = self.board.winner();
            if winner.is_some() {
                self.print_winner(winner.unwrap());
                break;
            }
            if self.board.is_full() {
                util::clear();
                println!("{}", self.board);
                println!("It's a draw!");
                break;
            }
            util::clear();
            println!("last move took {}ns", self.player_1.time_taken());
            println!("{}", self.board);
            let position = self.player_2.next_move(&self.board);
            self.board.place_next_mark(position).expect("Failed to place mark");
            let winner = self.board.winner();
            if winner.is_some() {
                self.print_winner(winner.unwrap());
                break;
            }
            if self.board.is_full() {
                println!("It's a draw!");
                break;
            }
            println!("last move took {}ns", self.player_2.time_taken());
            util::clear();
        }
    }
}
