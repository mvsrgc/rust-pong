use ggez::event::{self};
use ggez::{conf, GameResult};

mod pong;

use crate::pong::Pong;

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("name", "author")
        .window_setup(conf::WindowSetup::default().vsync(false))
        .window_mode(conf::WindowMode::default().dimensions(1920.0, 1080.0));

    let (ctx, event_loop) = &mut cb.build()?;

    let pong = &mut Pong {
        clicks: 0,
        mouse_x: 0.0,
        mouse_y: 0.0,
        rect_x: (1920 / 2) as f32,
        rect_y: (1080 / 2) as f32,
        velocity_x: 240.0,
        dt: 1.0f64 / 60.0f64,
        debug_mode: false,
    };

    event::run(ctx, event_loop, pong)
}
