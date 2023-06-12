#[allow(dead_code, unused_variables)]

mod ttt;
mod util;

fn main() {
    // let mut start: std::time::Instant;
    // let mut total_time: u128 = 0;
    // let game_count: u128 = 25000;
    
    // for _ in 0..game_count {
    //     let mut game = ttt::Game::new(
    //         ttt::Level2Bot::new(ttt::Mark::X),
    //         ttt::Level2Bot::new(ttt::Mark::O),
    //     );
    //     start = std::time::Instant::now();
    //     game.play();
    //     total_time += start.elapsed().as_micros();
    // }
    // println!("{} games took {} microseconds", game_count, total_time);
    // println!("Average time per game: {} microseconds", total_time / game_count);
    let mut game = ttt::Game::new(
        ttt::TranspositionTableBot::new(ttt::Mark::X),
        ttt::HumanPlayer::new(ttt::Mark::O),
    );
    let start: std::time::Instant = std::time::Instant::now();
    game.play();
    let time_taken = start.elapsed().as_micros();
    println!("Game took {} microseconds", time_taken);
}
