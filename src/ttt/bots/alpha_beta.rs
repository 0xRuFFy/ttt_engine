
use std::collections::HashMap;

use crate::ttt;

pub struct AlphaBetaBot {
    mark: ttt::Mark,
    last_time_taken: u128,
    seen_boards: HashMap<ttt::Board, (usize, f64)>,
    catches: u128,
}

impl AlphaBetaBot {
    pub fn new(mark: ttt::Mark) -> Self {
        AlphaBetaBot {
            mark,
            last_time_taken: 0,
            seen_boards: HashMap::new(),
            catches: 0,
        }
    }

    fn alpha_beta(&mut self, board: &ttt::Board, maximizing: bool, low: f64, high: f64) -> (usize, f64) {
        if let Some((pos, score)) = self.seen_boards.get(board) {
            self.catches += 1;
            return (*pos, *score);
        }

        
        let mut moves: HashMap<usize, f64> = HashMap::new();
        let mut alpha = low;
        let mut beta = high;

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
            let score = self.alpha_beta(&new_board, !maximizing, alpha, beta);
            moves.insert(pos, score.1);
            if maximizing {
                alpha = alpha.max(score.1);
            } else {
                beta = beta.min(score.1);
            }
            if alpha >= beta {
                break;
            }
        }

        let best_move = moves.iter().max_by(|a, b| {
            if maximizing {
                a.1.partial_cmp(b.1).unwrap()
            } else {
                b.1.partial_cmp(a.1).unwrap()
            }
        }).unwrap();

        self.seen_boards.insert(board.clone(), (*best_move.0, *best_move.1));

        (*best_move.0, *best_move.1)
    }
}


impl ttt::Player for AlphaBetaBot {
    fn next_move(&mut self, board: &ttt::Board) -> usize {
        let start = std::time::Instant::now();
        let (pos, _) = self.alpha_beta(board, true, std::f64::NEG_INFINITY, std::f64::INFINITY);
        self.last_time_taken = start.elapsed().as_micros();
        pos
    }

    fn time_taken(&self) -> u128 {
        // print seen boards length
        println!("seen boards: {}", self.seen_boards.len());
        println!("catches: {}", self.catches);
        self.last_time_taken
    }
}


