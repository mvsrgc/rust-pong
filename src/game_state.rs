use std::time::Duration;

use ggez::{audio, graphics::Rect, Context};

use crate::pong::Side;
use crate::{ball::Ball, paddle::Paddle, pong::Wall};

pub const DEFAULT_TIME_SCALE: f64 = 1.0;

pub struct GameState {
    pub dt: f64,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub debug_mode: bool,
    pub play_sounds: bool,
    pub time_scale: f64,
    pub game_width: f32,
    pub game_height: f32,
    pub ball: Ball,
    pub walls: Vec<Wall>,
    pub paddles: Vec<Paddle>,
    pub paused: Option<Duration>,
    pub player1_score: usize,
    pub player2_score: usize,
    pub goal_sound: audio::Source,
    pub pad_sound: audio::Source,
    pub wall_sound: audio::Source,
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

        let dt = (1.0 / 60.0) * time_scale;

        let ball = Ball::new(game_width, game_height);

        let goal_sound = audio::Source::new(ctx, "/goal.wav").unwrap();
        let pad_sound = audio::Source::new(ctx, "/pad.wav").unwrap();
        let wall_sound = audio::Source::new(ctx, "/wall.wav").unwrap();

        // Initialize the state
        GameState {
            dt,
            mouse_x: 0.0,
            mouse_y: 0.0,
            debug_mode: false,
            play_sounds: true,
            time_scale,
            game_width,
            game_height,
            ball,
            walls,
            paddles,
            paused: None,
            player1_score: 0,
            player2_score: 0,
            goal_sound,
            pad_sound,
            wall_sound,
        }
    }
}
