use super::board::*;
use super::player::*;
use std::collections::HashMap;
use std::option::*;

pub struct TQPlayer {
    desc: String,
    side: Option<PlayerId>,
    q: HashMap<usize, [f32; BOARD_SIZE]>,
    move_history: Vec<(usize, usize)>,
    learning_rate: f32,
    value_discount: f32,
    q_init: f32,
}

impl TQPlayer {
    pub fn new(
        learning_rate: Option<f32>,
        value_discount: Option<f32>,
        q_init: Option<f32>,
    ) -> Self {
        Self {
            desc: "TQ".to_string(),
            side: None,
            q: HashMap::<usize, [f32; BOARD_SIZE]>::new(),
            move_history: Vec::<(usize, usize)>::new(),
            learning_rate: learning_rate.unwrap_or(0.9),
            value_discount: value_discount.unwrap_or(0.95),
            q_init: q_init.unwrap_or(0.6),
        }
    }

    fn get_q(&mut self, board_hash: usize) -> &mut [f32; BOARD_SIZE] {
        self.q
            .entry(board_hash)
            .or_insert([self.q_init; BOARD_SIZE])
    }

    // TODO: remove the blind loop here.
    fn get_move(&mut self, board: &Board) -> usize {
        let board_hash = board.hash_value();
        let qvals = self.get_q(board_hash);
        loop {
            let m = Self::get_f32_max(qvals);
            if board.is_legal(m.0) {
                return m.0;
            }

            qvals[m.0] = -1.0;
        }
    }

    fn get_f32_max(xs: &[f32; BOARD_SIZE]) -> (usize, &f32) {
        let x = xs
            .iter()
            .rev()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .unwrap();

        // TODO: Add tests.
        (xs.len() - 1 - x.0, x.1)
    }
}

const WIN_VALUE: f32 = 1.0;
const DRAW_VALUE: f32 = 0.5;
const LOSS_VALUE: f32 = 0.0;

impl Player for TQPlayer {
    fn desc(&self) -> String {
        self.desc.clone()
    }

    fn new_game(&mut self, side: PlayerId) {
        self.side = Some(side);
        self.move_history.clear();
    }

    fn make_move(&mut self, board: &mut Board) -> Option<GameResult> {
        let m = self.get_move(board);
        self.move_history.push((board.hash_value(), m));
        let (_, res) = board.make_move(m, self.side.unwrap());

        res
    }

    fn final_result(&mut self, result: GameResult) {
        let final_value = match (result, self.side) {
            (GameResult::NaughtWin, Some(PlayerId::Naught)) => WIN_VALUE,
            (GameResult::CrossWin, Some(PlayerId::Cross)) => WIN_VALUE,
            (GameResult::NaughtWin, Some(PlayerId::Cross)) => LOSS_VALUE,
            (GameResult::CrossWin, Some(PlayerId::Naught)) => LOSS_VALUE,
            (GameResult::Draw, Some(_)) => DRAW_VALUE,
            _ => panic!("Unexpted game result: {:?}, side {:?}", result, self.side),
        };

        self.move_history.reverse();
        let mut next_max = -1.0;

        let lr = self.learning_rate;
        let vd = self.value_discount;
        for h in self.move_history.clone().iter() {
            let qvals = self.get_q(h.0);
            if next_max < 0.0 {
                qvals[h.1] = final_value;
            } else {
                qvals[h.1] = qvals[h.1] * (1.0 - lr) + lr * vd * next_max;
            }

            next_max = *Self::get_f32_max(qvals).1;
        }
    }
}
