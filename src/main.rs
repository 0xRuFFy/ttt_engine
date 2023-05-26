#[allow(dead_code, unused_variables)]

mod ttt;
mod util;

fn main() {
    println!("Hello, world!");
    let mut game = ttt::Game::new(
        ttt::Level2Bot::new(ttt::Mark::X),
        ttt::Level1Bot::new(ttt::Mark::O),
    );
    game.play();
}
