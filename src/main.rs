mod ttt;

use ttt::board::*;
use ttt::random_player::*;
use ttt::*;

fn main() {
    let mut board = Board::new(None, None);
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

    let mut p1 = RandomPlayer::new();
    let mut p2 = RandomPlayer::new();

    for _ in 0..num_games {
        let result = game::play_game(&mut board, &mut p1, &mut p2, &mut |_| {});
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
