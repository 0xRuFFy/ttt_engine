use std::{fmt::Display, hash::Hash};

use super::mark::Mark;

const WINNING_LINES: [[usize; 3]; 8] = [
    // rows
    [0, 1, 2], [3, 4, 5], [6, 7, 8],
    // columns
    [0, 3, 6], [1, 4, 7], [2, 5, 8],
    // diagonals
    [0, 4, 8], [2, 4, 6],
];

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Board([Option<Mark>; 9]);

impl Board {
    pub fn new() -> Board {
        Board([None; 9])
    }

    pub fn is_free(&self, position: usize) -> bool {
        self.0[position].is_none()
    }

    pub fn is_full(&self) -> bool {
        self.0.iter().all(|mark| mark.is_some())
    }

    pub fn winner(&self) -> Option<Mark> {
        for line in WINNING_LINES.iter() {
            let [a, b, c] = *line;
            if self.0[a] == self.0[b] && self.0[b] == self.0[c] {
                return self.0[a];
            }
        }
    
        None
    }

    pub fn place_mark(&mut self, mark: Mark, position: usize) -> Result<(), String> {
        if position > 8 {
            return Err(format!("Position {} is out of bounds", position));
        }
        if self.0[position].is_some() {
            return Err(format!("Position {} is already taken", position));
        }
    
        self.0[position] = Some(mark);
        Ok(())
    }

    pub fn next_mark(&self) -> Option<Mark> {
        let mut x_count = 0;
        let mut o_count = 0;
    
        for mark in self.0.iter() {
            match mark {
                Some(Mark::X) => x_count += 1,
                Some(Mark::O) => o_count += 1,
                None => (),
            }
        }
    
        if x_count > o_count {
            Some(Mark::O)
        } else {
            Some(Mark::X)
        }
    }

    pub fn place_next_mark(&mut self, position: usize) -> Result<(), String> {
        let mark = self.next_mark().ok_or("Board is full")?;
        self.place_mark(mark, position)
    }
    
}

impl Hash for Board {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for mark in self.0.iter() {
            match mark {
                Some(Mark::X) => 0.hash(state),
                Some(Mark::O) => 1.hash(state),
                None => 2.hash(state),
            }
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();

        for (i, mark) in self.0.iter().enumerate() {
            let mark = match mark {
                Some(Mark::X) => " \x1b[94mX\x1b[0m ",
                Some(Mark::O) => " \x1b[33mO\x1b[0m ",
                None => "",
            };
    
            if mark.is_empty() {
                board.push_str(format!(" \x1b[2m{}\x1b[0m ", i).as_str());
            } else {
                board.push_str(mark);
            }
    
            if i % 3 == 2 {
                board.push('\n');
                if i < 8 {
                    board.push_str("\x1b[1m---+---+---\x1b[0m\n");
                }
            } else {
                board.push_str("\x1b[1m|\x1b[0m");
            }
        }
    
        write!(f, "{}", board)
    }
}