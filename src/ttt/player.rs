use super::board::Board;


pub trait Player: Sized {
    /// Returns the next move for the player given the current state of the board.
    fn next_move(&mut self, board: &Board) -> usize;

    fn time_taken(&self) -> u128;
}
