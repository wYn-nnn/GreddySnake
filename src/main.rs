// main.rs - 无需修改
mod game;

use ggez::{event, graphics, Context, ContextBuilder, GameResult};
use game::Game;

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("snake_game", "me")
        .window_setup(ggez::conf::WindowSetup::default().title("Snake Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(640.0, 480.0))
        //.window_mode(ggez::conf::WindowMode::default().dimensions(1280.0, 960.0))
        .build()?;
    
    let mut game = Game::new(&mut ctx)?;
    event::run(ctx, event_loop, game)
}