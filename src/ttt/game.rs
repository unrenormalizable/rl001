use super::board::*;
use super::player::*;
use serde::*;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct GameState(
    PlayerId,
    String,
    Option<GameResult>,
    [Option<PlayerId>; BOARD_SIZE],
);

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Move by: {:?} ({})", self.0, self.1)?;
        writeln!(
            f,
            "Result: {}",
            self.2
                .map_or_else(|| "In progress...".to_string(), |x| format!("{:?}", x))
        )?;
        for x in self.3.iter().enumerate() {
            write!(f, "{}", &x.1.map_or("-".to_string(), |x| x.to_string()))?;
            if x.0 == 2 || x.0 == 5 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

pub fn play_game(
    board: &mut Board,
    player1: &mut dyn Player,
    player2: &mut dyn Player,
    callback: &mut impl FnMut(GameState),
) -> GameResult {
    player1.new_game(PlayerId::Naught);
    player2.new_game(PlayerId::Cross);
    board.reset();

    let final_result: GameResult;
    loop {
        let result = player1.make_move(board);
        callback(GameState(
            PlayerId::Naught,
            player1.desc(),
            result,
            board.get_state(),
        ));

        match result {
            None => {}
            Some(r) => {
                final_result = r;
                break;
            }
        }

        let result = player2.make_move(board);
        callback(GameState(
            PlayerId::Cross,
            player2.desc(),
            result,
            board.get_state(),
        ));
        match result {
            None => {}
            Some(r) => {
                final_result = r;
                break;
            }
        }
    }

    let _ = &player1.final_result(final_result);
    let _ = &player2.final_result(final_result);

    final_result
}

// TODO: convert return value into struct
pub fn battle(
    id: usize,
    player1: &mut dyn Player,
    player2: &mut dyn Player,
    num_games: usize,
    silent: bool,
    callback: &mut impl FnMut(GameState),
) -> (usize, usize, usize) {
    let mut board = Board::new(None);
    let mut draw_count = 0_usize;
    let mut cross_count = 0_usize;
    let mut naught_count = 0_usize;
    for _ in 0..num_games {
        match play_game(&mut board, player1, player2, callback) {
            GameResult::CrossWin => cross_count += 1,
            GameResult::NaughtWin => naught_count += 1,
            GameResult::Draw => draw_count += 1,
        }
    }

    if !silent {
        println!(
            "Battle #{id} results:\n    draws: {:2} ({:.2}%)\n   naught: {:2} ({:.2}%)\n    cross: {:2} ({:.2}%)",
            draw_count,
            draw_count as f32 / num_games as f32 * 100.0,
            naught_count,
            naught_count as f32 / num_games as f32 * 100.0,
            cross_count,
            cross_count as f32 / num_games as f32 * 100.0,
        );
    }

    (naught_count, cross_count, draw_count)
}

pub fn evaluate_players(
    p1: &mut dyn Player,
    p2: &mut dyn Player,
    games_per_battle: usize,
    num_battles: usize,
    silent: bool,
    callback: &mut impl FnMut((usize, usize, usize, usize)),
) {
    for i in 0..num_battles {
        let (p1win, p2win, draw) = battle(i, p1, p2, games_per_battle, silent, &mut |_| {});
        callback((i, p1win, p2win, draw));
    }
}

#[cfg(test)]
mod tests {
    use super::super::random_player::*;
    use super::*;

    #[test]
    fn test_games() {
        let mut p1 = RandomPlayer::new(Some(2718));
        let mut p2 = RandomPlayer::new(Some(2718));

        let mut game_output = Vec::<GameState>::new();
        let mut callback = |gr| {
            game_output.push(gr);
        };

        let (naught_count, cross_count, draw_count) =
            battle(0, &mut p1, &mut p2, 20, true, &mut callback);

        assert!(draw_count != 0);
        assert!(cross_count != 0);
        assert!(naught_count != 0);

        insta::assert_yaml_snapshot!(game_output);
    }
}
