use super::board::*;
use super::player::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::option::*;

pub struct RandomPlayer {
    desc: String,
    side: Option<PlayerId>,
    rng: StdRng,
}

#[allow(dead_code)] // TODO: Remove this.
impl RandomPlayer {
    pub fn new(seed: Option<u64>) -> Self {
        let rng = match seed {
            Some(s) => StdRng::seed_from_u64(s),
            None => StdRng::from_rng(rand::thread_rng()).unwrap(),
        };

        Self {
            desc: "Random".to_string(),
            side: None,
            rng,
        }
    }
}

impl Player for RandomPlayer {
    fn desc(&self) -> String {
        self.desc.clone()
    }

    fn new_game(&mut self, side: PlayerId) {
        self.side = Some(side);
    }

    fn make_move(&mut self, board: &mut Board) -> Option<GameResult> {
        let moves = board.get_possible_next_moves();
        let pos = self.rng.gen_range(0..moves.len());

        let (_, res) = board.make_move(moves[pos], self.side.unwrap());

        res
    }

    fn final_result(&mut self, _result: GameResult) {}
}
