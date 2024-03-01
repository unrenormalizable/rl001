use crate::ttt::board::*;

pub trait Player {
    fn desc(&self) -> String;

    fn new_game(&mut self, side: PlayerId);

    fn make_move(&mut self, board: &mut Board) -> Option<GameResult>;

    fn final_result(&mut self, result: GameResult);
}
