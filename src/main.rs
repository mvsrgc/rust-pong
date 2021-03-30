use ggez::event::{self};
use ggez::{conf, GameResult};

mod pong;

use crate::pong::Pong;

const GAME_WIDTH: f32 = 1920.0;
const GAME_HEIGHT: f32 = 1080.0;

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("name", "author")
        .window_setup(conf::WindowSetup::default().vsync(false))
        .window_mode(conf::WindowMode::default().dimensions(GAME_WIDTH as f32, GAME_HEIGHT as f32));

    let (ctx, event_loop) = &mut cb.build()?;

    let pong = &mut Pong::new(GAME_WIDTH, GAME_HEIGHT);

    event::run(ctx, event_loop, pong)
}
