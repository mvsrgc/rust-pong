use ggez::{conf, event, GameResult};

pub mod assets;
pub mod ball;
pub mod game_state;
pub mod menu;
pub mod paddle;
pub mod particle;
pub mod player_control;
pub mod pong;
pub mod render;

use crate::game_state::GameState;

const GAME_WIDTH: f32 = 1280.0;
const GAME_HEIGHT: f32 = 720.0;

fn main() -> GameResult {
    let resource_dir = std::path::PathBuf::from("./resources");

    let cb = ggez::ContextBuilder::new("name", "author")
        .window_setup(conf::WindowSetup::default().vsync(true))
        .window_mode(conf::WindowMode::default().dimensions(GAME_WIDTH as f32, GAME_HEIGHT as f32))
        .add_resource_path(resource_dir);

    let (mut ctx, mut events_loop) = cb.build()?;

    let mut game = GameState::new(&mut ctx)?;

    event::run(&mut ctx, &mut events_loop, &mut game)
}
