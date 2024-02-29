mod ttt;

use ttt::board::*;
use ttt::random_player::*;
use ttt::*;

fn main() {
    let mut board = Board::new(None);

    let num_games = 100000;

    let mut draw_count = 0;
    let mut cross_count = 0;
    let mut naught_count = 0;

    let mut p1 = RandomPlayer::new(None);
    let mut p2 = RandomPlayer::new(None);

    for _ in 0..num_games {
        let result = game::play_game(&mut board, &mut p1, &mut p2, &mut |_| {});
        match result {
            GameResult::CrossWin => cross_count += 1,
            GameResult::NaughtWin => naught_count += 1,
            GameResult::Draw => draw_count += 1,
        }
    }

    println!("After {num_games} game we have: draws: {draw_count}, cross wins: {cross_count}, and naught wins: {naught_count}.");
    println!(
        "Which gives percentages as:\n    draws: {:2} ({:.2}%)\n    cross: {:2} ({:.2}%)\n   naught: {:2} ({:.2}%)",
        draw_count,
        draw_count as f32 / num_games as f32 * 100.0,
        naught_count,
        naught_count as f32 / num_games as f32 * 100.0,
        cross_count,
        cross_count as f32 / num_games as f32 * 100.0,
    );
}
