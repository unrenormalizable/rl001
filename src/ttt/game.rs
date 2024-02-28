use super::board::*;
use super::player::*;
use serde::*;

// TODO: collapse finished and gameresult

#[derive(Serialize)]
pub struct GameState(i32, GameResult, bool, [i32; BOARD_SIZE]);

pub fn play_game(
    board: &mut Board,
    player1: &mut dyn Player,
    player2: &mut dyn Player,
    callback: &mut impl FnMut(GameState),
) -> GameResult {
    player1.new_game(CROSS);
    player2.new_game(NAUGHT);
    board.reset();

    let final_result: GameResult;
    loop {
        let (result, finished) = player1.make_move(board);
        callback(GameState(CROSS, result, finished, board.get_state()));

        if finished {
            if result == GameResult::Draw {
                final_result = GameResult::Draw;
            } else {
                final_result = GameResult::CrossWin;
            }

            break;
        } else {
            let (result, finished) = player2.make_move(board);
            callback(GameState(NAUGHT, result, finished, board.get_state()));
            if finished {
                if result == GameResult::Draw {
                    final_result = GameResult::Draw;
                } else {
                    final_result = GameResult::NaughtWin;
                }
                break;
            }
        }
    }

    let _ = &player1.final_result(final_result);
    let _ = &player2.final_result(final_result);

    final_result
}

#[cfg(test)]
mod tests {
    use super::super::random_player::*;
    use super::*;

    #[test]
    fn test_games() {
        let mut board = Board::new(None, Some(2718));
        let mut p1 = RandomPlayer::new();
        let mut p2 = RandomPlayer::new();

        let mut draw_count = 0;
        let mut cross_count = 0;
        let mut naught_count = 0;

        let mut game_output = Vec::<GameState>::new();
        let mut callback = |gr| {
            game_output.push(gr);
        };

        for _ in 0..20 {
            let result = play_game(&mut board, &mut p1, &mut p2, &mut callback);
            match result {
                GameResult::NotFinished => panic!("Should not be here!"),
                GameResult::CrossWin => cross_count += 1,
                GameResult::NaughtWin => naught_count += 1,
                GameResult::Draw => draw_count += 1,
            }
        }

        assert!(draw_count != 0);
        assert!(cross_count != 0);
        assert!(naught_count != 0);

        insta::assert_yaml_snapshot!(game_output);
    }
}
