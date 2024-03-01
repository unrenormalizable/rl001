use super::board::*;
use super::player::*;
use std::option::*;

pub struct HumanPlayer {
    desc: String,
    side: Option<PlayerId>,
    get_move: fn() -> usize,
}

impl HumanPlayer {
    pub fn new(get_move: fn() -> usize) -> Self {
        Self {
            desc: "Human".to_string(),
            side: None,
            get_move,
        }
    }
}

impl Player for HumanPlayer {
    fn desc(&self) -> String {
        self.desc.clone()
    }

    fn new_game(&mut self, side: PlayerId) {
        self.side = Some(side);
    }

    fn make_move(&mut self, board: &mut Board) -> Option<GameResult> {
        let m = (self.get_move)();

        let (_, res) = board.make_move(m, self.side.unwrap());

        res
    }

    fn final_result(&mut self, _result: GameResult) {}
}
