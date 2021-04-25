use std::time::Duration;

use ggez::{audio, graphics::Rect, Context};

use crate::pong::{Side, DEFAULT_TIME_SCALE};
use crate::{ball::Ball, paddle::Paddle, pong::Wall};

pub struct GameState {
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub dt: f64,
    pub time_scale: f64,
    pub debug_mode: bool,
    pub paddles: Vec<Paddle>,
    pub ball: Ball,
    pub game_width: f32,
    pub game_height: f32,
    pub goal_sound: audio::Source,
    pub pad_sound: audio::Source,
    pub wall_sound: audio::Source,
    pub play_sounds: bool,
    pub paused: Option<Duration>,
    pub player1_score: usize,
    pub player2_score: usize,
    pub walls: Vec<Wall>,
}

impl GameState {
    pub fn new(ctx: &mut Context, game_width: f32, game_height: f32) -> GameState {
        let time_scale: f64 = DEFAULT_TIME_SCALE; // You can slow down or speed up time with PgUP, PgDown

        // Create the paddles
        let left_paddle = Paddle::new(game_width, game_height, Side::Left);
        let right_paddle = Paddle::new(game_width, game_height, Side::Right);

        let paddles = vec![left_paddle, right_paddle];

        let walls = vec![
            Wall::new(Rect::new(0.0, 0.0, game_width, 0.0), Side::Top),
            Wall::new(Rect::new(0.0, 0.0, 0.0, game_height), Side::Left),
            Wall::new(Rect::new(game_width, 0.0, 0.0, game_height), Side::Right),
            Wall::new(Rect::new(0.0, game_height, game_width, 0.0), Side::Bottom),
        ];

        // Initialize the state
        GameState {
            mouse_x: 0.0,
            mouse_y: 0.0,
            time_scale,
            dt: (1.0 / 60.0) * time_scale,
            debug_mode: false,
            game_width,
            game_height,
            paddles,
            ball: Ball::new(game_width, game_height),
            goal_sound: audio::Source::new(ctx, "/goal.wav").unwrap(),
            pad_sound: audio::Source::new(ctx, "/pad.wav").unwrap(),
            wall_sound: audio::Source::new(ctx, "/wall.wav").unwrap(),
            play_sounds: true,
            paused: None,
            player1_score: 0,
            player2_score: 0,
            walls,
        }
    }
}
