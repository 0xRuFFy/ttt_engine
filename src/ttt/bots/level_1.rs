use std::collections::HashMap;

use crate::ttt;

pub struct Level1Bot {
    mark: ttt::Mark,
    last_time_taken: u128,
}

impl Level1Bot {
    pub fn new(mark: ttt::Mark) -> Self {
        Level1Bot {
            mark,
            last_time_taken: 0,
        }
    }

    pub fn evaluate(&self, board: &ttt::Board) -> HashMap<usize, f64> {
        let maximizing = self.mark == board.next_mark().unwrap();
        let mut moves: HashMap<usize, f64> = HashMap::new();
        for pos in 0..9 {
            if !board.is_free(pos) {
                continue;
            }
            let mut new_board = board.clone();
            new_board.place_next_mark(pos).unwrap();
            if new_board.winner().is_some() {
                if maximizing {
                    moves.insert(pos, 1.0);
                } else {
                    moves.insert(pos, -1.0);
                }
                continue;
            } else if new_board.is_full() {
                moves.insert(pos, 0.0);
                continue;
            }
            let score = self.minimax(&new_board, !maximizing).1;
            moves.insert(pos, score);
        }
        moves
    }

    fn minimax(&self, board: &ttt::Board, maximizing: bool) -> (usize, f64) {
        let start = std::time::Instant::now();
        let mut moves: HashMap<usize, f64> = HashMap::new();

        for pos in 0..9 {
            if !board.is_free(pos) {
                continue;
            }
            let mut new_board = board.clone();
            new_board.place_next_mark(pos).unwrap();
            if new_board.winner().is_some() {
                if maximizing {
                    moves.insert(pos, 1.0);
                } else {
                    moves.insert(pos, -1.0);
                }
                continue;
            } else if new_board.is_full() {
                moves.insert(pos, 0.0);
                continue;
            }
            let score = self.minimax(&new_board, !maximizing);
            moves.insert(pos, score.1);
        }
        
        let best_move = moves.iter().max_by(|a, b| {
            if maximizing {
                a.1.partial_cmp(b.1).unwrap()
            } else {
                b.1.partial_cmp(a.1).unwrap()
            }
        }).unwrap();
        (*best_move.0, *best_move.1)
    }
}

impl ttt::Player for Level1Bot {
    fn next_move(&mut self, board: &ttt::Board) -> usize {
        self.evaluate(board);
        let start = std::time::Instant::now();
        let (pos, _) = self.minimax(board, true);
        self.last_time_taken = start.elapsed().as_nanos();
        pos
    }

    fn time_taken(&self) -> u128 {
        self.last_time_taken
    }
}