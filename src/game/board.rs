use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::option::*;

// TODO: get rid of all (as i32) and (as usize)

// TODO: Board should not need to be mutable for the users.

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameResult {
    NotFinished,
    NaughtWin,
    CrossWin,
    Draw,
}

// TODO: This should be enum.
const EMPTY: i32 = 0;
pub const NAUGHT: i32 = 1;
pub const CROSS: i32 = 2;

const BOARD_DIM: usize = 3;
const BOARD_SIZE: usize = BOARD_DIM * BOARD_DIM;

#[derive(Debug)]
pub struct Board {
    state: [i32; BOARD_SIZE],
    win_check_dirs: HashMap<i32, Vec<(i32, i32)>>,
}

#[allow(dead_code)] // TODO: Remove this.
impl Board {
    pub fn new(board: Option<Board>) -> Self {
        let state = board.map_or([0; BOARD_SIZE], |b| b.state);
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

    fn hash_value(&self) -> i32 {
        let mut res = 0;

        for i in 0..self.state.len() {
            res *= 3;
            res += self.state[i];
        }

        res
    }

    pub fn other_side(side: i32) -> i32 {
        if side == EMPTY {
            panic!("EMPTY has no 'other side'")
        }

        if side == CROSS {
            return NAUGHT;
        }

        if side == NAUGHT {
            return CROSS;
        }

        panic!("{side} is not a valid side")
    }

    fn coord_to_pos(&self, coord: (i32, i32)) -> i32 {
        coord.0 * BOARD_DIM as i32 + coord.1
    }

    fn pos_to_coord(self, pos: i32) -> (i32, i32) {
        (pos / BOARD_DIM as i32, pos % BOARD_DIM as i32)
    }

    // TODO: Remove this.
    pub fn reset(&mut self) {
        self.state = [0; BOARD_SIZE];
    }

    fn num_empty(&self) -> i32 {
        self.state.iter().filter(|&&x| x == EMPTY).count() as i32
    }

    pub fn random_empty_spot(&self) -> Option<i32> {
        if self.num_empty() == 0 {
            return None;
        }

        let empty_cells: Vec<usize> = self
            .state
            .iter()
            .enumerate()
            .filter_map(|(i, &val)| if val == EMPTY { Some(i) } else { None })
            .collect();

        let index = thread_rng().gen_range(0..empty_cells.len());

        Some(empty_cells[index] as i32)
    }

    fn is_legal(&self, pos: i32) -> bool {
        (0 <= pos && pos < BOARD_SIZE as i32) && (self.state[pos as usize] == EMPTY)
    }

    pub fn make_move(&mut self, position: i32, side: i32) -> ([i32; BOARD_SIZE], GameResult, bool) {
        if self.state[position as usize] != EMPTY {
            panic!("Invalid move")
        }

        self.state[position as usize] = side;

        if self.check_win() {
            let winner = if side == CROSS {
                GameResult::CrossWin
            } else {
                GameResult::NaughtWin
            };
            return (self.state, winner, true);
        }

        if self.num_empty() == 0 {
            return (self.state, GameResult::Draw, true);
        }

        (self.state, GameResult::NotFinished, false)
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
        if c == EMPTY {
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

    fn who_won(&self) -> i32 {
        for start_pos in &self.win_check_dirs {
            if self.state[*start_pos.0 as usize] != EMPTY {
                for direction in start_pos.1 {
                    let res = self.check_win_in_dir(*start_pos.0, *direction);
                    if res {
                        return self.state[*start_pos.0 as usize];
                    }
                }
            }
        }

        EMPTY
    }

    fn check_win(&self) -> bool {
        for start_pos in &self.win_check_dirs {
            if self.state[*start_pos.0 as usize] != EMPTY {
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
        if (self.state[pos as usize]) == EMPTY {
            return ' '.to_string();
        }

        if (self.state[pos as usize]) == NAUGHT {
            return 'o'.to_string();
        }

        'x'.to_string()
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
