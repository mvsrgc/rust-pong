use ggez::event;
use ggez::{conf, GameResult};

pub mod paddle;
pub mod player_control;
pub mod pong;
pub mod render;

use crate::pong::GameState;

const GAME_WIDTH: f32 = 1920.0;
const GAME_HEIGHT: f32 = 1080.0;

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("name", "author")
        .window_setup(conf::WindowSetup::default().vsync(false))
        .window_mode(conf::WindowMode::default().dimensions(GAME_WIDTH as f32, GAME_HEIGHT as f32));

    let (ctx, event_loop) = &mut cb.build()?;

    let game_state = &mut GameState::new(GAME_WIDTH, GAME_HEIGHT);

    event::run(ctx, event_loop, game_state)
}
