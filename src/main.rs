use ggez::event::{self};
use ggez::{conf, GameResult};

mod pong;

use crate::pong::{Paddle, Pong};

const GAME_WIDTH: f32 = 1920.0;
const GAME_HEIGHT: f32 = 1080.0;

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("name", "author")
        .window_setup(conf::WindowSetup::default().vsync(false))
        .window_mode(conf::WindowMode::default().dimensions(GAME_WIDTH, GAME_HEIGHT));

    let (ctx, event_loop) = &mut cb.build()?;

    let pong = &mut Pong {
        clicks: 0,
        mouse_x: 0.0,
        mouse_y: 0.0,
        velocity_x: 240.0,
        dt: 1.0f64 / 60.0f64,
        debug_mode: false,
        left_paddle: Paddle::new(GAME_WIDTH, GAME_HEIGHT, true),
        right_paddle: Paddle::new(GAME_WIDTH, GAME_HEIGHT, false),
    };

    event::run(ctx, event_loop, pong)
}
