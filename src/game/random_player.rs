use crate::game::board::*;
use crate::game::player::*;
use std::option::*;

pub struct RandomPlayer {
    side: Option<i32>,
}

impl RandomPlayer {
    pub fn new() -> Self {
        Self { side: None }
    }
}

impl Player for RandomPlayer {
    fn new_game(&mut self, side: i32) {
        self.side = Some(side);
    }

    fn make_move(&self, board: &mut Board) -> (GameResult, bool) {
        let (_, res, finished) =
            board.make_move(board.random_empty_spot().unwrap(), self.side.unwrap());
        (res, finished)
    }

    fn final_result(&self, _result: GameResult) {}
}
