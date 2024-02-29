use serde::*;
use std::collections::HashMap;
use std::option::*;

#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub enum GameResult {
    NaughtWin,
    CrossWin,
    Draw,
}

const BOARD_DIM: usize = 3;
pub const BOARD_SIZE: usize = BOARD_DIM * BOARD_DIM;

#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub enum PlayerId {
    Naught,
    Cross,
}

#[derive(Debug)]
pub struct Board {
    state: [Option<PlayerId>; BOARD_SIZE],
    win_check_dirs: HashMap<i32, Vec<(i32, i32)>>,
}

#[allow(dead_code)] // TODO: Remove this.
impl Board {
    pub fn new(board: Option<Board>) -> Self {
        let state = board.map_or([None; BOARD_SIZE], |b| b.state);
        let win_check_dirs = HashMap::from([
            (0, vec![(1, 1), (1, 0), (0, 1)]),
            (1, vec![(1, 0)]),
            (2, vec![(1, 0), (1, -1)]),
            (3, vec![(0, 1)]),
            (6, vec![(0, 1)]),
        ]);

        Self {
            state,
            win_check_dirs,
        }
    }

    pub fn get_state(&self) -> [Option<PlayerId>; BOARD_SIZE] {
        self.state
    }

    fn coord_to_pos(&self, coord: (i32, i32)) -> i32 {
        coord.0 * BOARD_DIM as i32 + coord.1
    }

    fn pos_to_coord(self, pos: i32) -> (i32, i32) {
        (pos / BOARD_DIM as i32, pos % BOARD_DIM as i32)
    }

    pub fn reset(&mut self) {
        self.state = [None; BOARD_SIZE];
    }

    fn num_empty(&self) -> i32 {
        self.state.iter().filter(|&&x| x.is_none()).count() as i32
    }

    pub fn get_possible_next_moves(&mut self) -> Vec<usize> {
        let empty_cells: Vec<usize> = self
            .state
            .iter()
            .enumerate()
            .filter_map(|(i, &val)| if val.is_none() { Some(i) } else { None })
            .collect();

        empty_cells
    }

    fn is_legal(&self, pos: i32) -> bool {
        (0 <= pos && pos < BOARD_SIZE as i32) && (self.state[pos as usize].is_none())
    }

    pub fn make_move(
        &mut self,
        position: usize,
        side: PlayerId,
    ) -> ([Option<PlayerId>; BOARD_SIZE], Option<GameResult>) {
        if self.state[position].is_some() {
            panic!("Invalid move")
        }

        self.state[position] = Some(side);

        if self.check_win() {
            let winner = if side == PlayerId::Naught {
                GameResult::NaughtWin
            } else {
                GameResult::CrossWin
            };

            return (self.state, Some(winner));
        }

        if self.num_empty() == 0 {
            return (self.state, Some(GameResult::Draw));
        }

        (self.state, None)
    }

    fn apply_dir(&self, pos: i32, direction: (i32, i32)) -> i32 {
        let mut row = pos / 3;
        let mut col = pos % 3;
        row += direction.0;

        if !(0..=2).contains(&row) {
            return -1;
        }

        col += direction.1;

        if !(0..=2).contains(&col) {
            return -1;
        }

        row * 3 + col
    }

    fn check_win_in_dir(&self, pos: i32, direction: (i32, i32)) -> bool {
        let c = self.state[pos as usize];
        if c.is_none() {
            return false;
        }

        let p1 = self.apply_dir(pos, direction);
        let p2 = self.apply_dir(p1, direction);

        if p1 == -1 || p2 == -1 {
            return false;
        }

        if c == self.state[p1 as usize] && c == self.state[p2 as usize] {
            return true;
        }

        false
    }

    fn who_won(&self) -> Option<PlayerId> {
        for start_pos in &self.win_check_dirs {
            if self.state[*start_pos.0 as usize].is_some() {
                for direction in start_pos.1 {
                    let res = self.check_win_in_dir(*start_pos.0, *direction);
                    if res {
                        return self.state[*start_pos.0 as usize];
                    }
                }
            }
        }

        None
    }

    fn check_win(&self) -> bool {
        for start_pos in &self.win_check_dirs {
            if self.state[*start_pos.0 as usize].is_some() {
                for direction in start_pos.1 {
                    let res = self.check_win_in_dir(*start_pos.0, *direction);
                    if res {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn state_to_char(&self, pos: i32) -> String {
        let ret = match self.state[pos as usize] {
            None => " ",
            Some(PlayerId::Naught) => "o",
            Some(PlayerId::Cross) => "x",
        };

        ret.to_string()
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut board_str = String::new();
        for i in 0..3 {
            board_str += &format!(
                "{}|{}|{}\n",
                self.state_to_char(i * 3),
                self.state_to_char(i * 3 + 1),
                self.state_to_char(i * 3 + 2)
            );
            if i != 2 {
                board_str += "-----\n"
            }
        }

        board_str += "\n";

        board_str
    }
}
