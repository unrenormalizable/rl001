use super::board::*;
use super::player::*;
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
        let pos = board.random_empty_spot().unwrap();
        let (_, res, finished) = board.make_move(pos, self.side.unwrap());
        (res, finished)
    }

    fn final_result(&self, _result: GameResult) {}
}
