use super::board::*;
use super::player::*;
use serde::*;

#[derive(Serialize)]
pub struct GameState(PlayerId, Option<GameResult>, [Option<PlayerId>; BOARD_SIZE]);

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
        callback(GameState(PlayerId::Naught, result, board.get_state()));

        match result {
            None => {}
            Some(r) => {
                final_result = r;
                break;
            }
        }

        let result = player2.make_move(board);
        callback(GameState(PlayerId::Cross, result, board.get_state()));
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

#[cfg(test)]
mod tests {
    use super::super::random_player::*;
    use super::*;

    #[test]
    fn test_games() {
        let mut board = Board::new(None);

        let mut p1 = RandomPlayer::new(Some(2718));
        let mut p2 = RandomPlayer::new(Some(2718));

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
