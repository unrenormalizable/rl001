use crate::game::board::*;

pub trait Player {
    // TODO: Remove mut.
    fn new_game(&mut self, side: i32);

    fn make_move(&self, board: &mut Board) -> (GameResult, bool);

    fn final_result(&self, result: GameResult);
}
