mod game;

use game::board::*;
use game::player::*;

fn play_game(
    board: &mut Board,
    player1: &mut dyn Player,
    player2: &mut dyn Player,
    callback: impl Fn(&String),
) -> GameResult {
    player1.new_game(CROSS);
    player2.new_game(NAUGHT);
    board.reset();

    let final_result: GameResult;
    loop {
        let (result, finished) = player1.make_move(board);
        callback(&board.to_string());

        if finished {
            if result == GameResult::Draw {
                final_result = GameResult::Draw;
            } else {
                final_result = GameResult::CrossWin;
            }

            break;
        } else {
            let (result, finished) = player2.make_move(board);
            callback(&board.to_string());
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

fn main() {
    let mut board = Board::new(None);
    //let mut p1 = game::random_player::RandomPlayer::new();
    //let mut p2 = game::random_player::RandomPlayer::new();

    //let callback = |b: &String| {
    //    println!("{}", b);
    //};

    //let result = play_game(&mut board, &mut p1, &mut p2, callback);
    //match result {
    //    GameResult::NotFinished => panic!("Should not be here!"),
    //    GameResult::CrossWin => println!("Cross won!"),
    //    GameResult::NaughtWin => println!("Naught won!"),
    //    GameResult::Draw => println!("Game is a draw!"),
    //}

    let num_games = 100000;

    let mut draw_count = 0;
    let mut cross_count = 0;
    let mut naught_count = 0;

    let mut p1 = game::random_player::RandomPlayer::new();
    let mut p2 = game::random_player::RandomPlayer::new();

    for _ in 0..num_games {
        let result = play_game(&mut board, &mut p1, &mut p2, |_| {});
        match result {
            GameResult::NotFinished => panic!("Should not be here!"),
            GameResult::CrossWin => cross_count += 1,
            GameResult::NaughtWin => naught_count += 1,
            GameResult::Draw => draw_count += 1,
        }
    }

    println!("After {num_games} game we have draws: {draw_count}, cross wins: {cross_count}, and naught wins: {naught_count}.");
    println!(
        "Which gives percentages of draws : cross : naught of about {:.2} : {:.2} : {:.2}",
        draw_count as f32 / num_games as f32 * 100.0,
        cross_count as f32 / num_games as f32 * 100.0,
        naught_count as f32 / num_games as f32 * 100.0
    );
}
