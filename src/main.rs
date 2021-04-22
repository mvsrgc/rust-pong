use ggez::event;
use ggez::{conf, GameResult};

pub mod ball;
pub mod paddle;
pub mod player_control;
pub mod pong;
pub mod render;

use crate::pong::GameState;

const GAME_WIDTH: f32 = 640.0;
const GAME_HEIGHT: f32 = 480.0;

fn main() -> GameResult {
    let resource_dir = std::path::PathBuf::from("./resources");

    let cb = ggez::ContextBuilder::new("name", "author")
        .window_setup(conf::WindowSetup::default().vsync(true))
        .window_mode(conf::WindowMode::default().dimensions(GAME_WIDTH as f32, GAME_HEIGHT as f32))
        .add_resource_path(resource_dir);

    let (ctx, event_loop) = &mut cb.build()?;

    let game_state = &mut GameState::new(ctx, GAME_WIDTH, GAME_HEIGHT);

    event::run(ctx, event_loop, game_state)
}
