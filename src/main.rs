mod ttt;

use std::io::Write;
use ttt::game::*;
use ttt::human_player::*;
use ttt::tq_player::*;

fn get_move() -> usize {
    print!("Your move> ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input.trim().parse::<usize>().unwrap()
}

fn main() {
    let mut p1 = TQPlayer::new(None, None, None);
    let mut p2 = TQPlayer::new(None, None, None);

    let mut callback =
        |x: (usize, usize, usize, usize)| println!("{},{},{},{}", x.0, x.1, x.2, x.3);

    evaluate_players(&mut p1, &mut p2, 100, 100, true, &mut callback);

    let mut p1 = HumanPlayer::new(get_move);
    let mut callback = |gs: GameState| println!("{gs}");
    let _ = battle(0, &mut p1, &mut p2, 20, true, &mut callback);
}
