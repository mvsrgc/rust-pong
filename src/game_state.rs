use std::time::Duration;

use crate::{ball::Ball, paddle::Paddle, particle::Particle, pong::Wall};

use ggez::{
    audio,
    graphics::{Image, Rect},
    Context,
};

use crate::pong::Side;

use crate::menu::Menu;

pub const DEFAULT_TIME_SCALE: f64 = 1.0;

pub enum GameMode {
    Menu,
    Game,
}

pub struct GameState {
    pub dt: f64,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub debug_mode: bool,
    pub play_sounds: bool,
    pub show_particles: bool,
    pub game_width: f32,
    pub game_height: f32,
    pub game_mode: GameMode,
    pub ball: Ball,
    pub walls: Vec<Wall>,
    pub paddles: Vec<Paddle>,
    pub paused: Option<Duration>,
    pub player1_score: usize,
    pub player2_score: usize,
    pub goal_sound: audio::Source,
    pub pad_sound: audio::Source,
    pub wall_sound: audio::Source,
    pub particle_images: Vec<Image>,
    pub particles: Vec<Particle>,
    pub menu: Menu,
}

impl GameState {
    pub fn new(ctx: &mut Context, game_width: f32, game_height: f32) -> GameState {
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

        let dt = 1.0 / 60.0;

        // @Error - We should handle errors here instead of .unwrap()
        let goal_sound = audio::Source::new(ctx, "/goal.wav").unwrap();
        let pad_sound = audio::Source::new(ctx, "/pad.wav").unwrap();
        let wall_sound = audio::Source::new(ctx, "/wall.wav").unwrap();

        let ball = Ball::new(ctx, game_width, game_height);

        let menu = Menu::new(0);

        // @Error - We should handle errors here instead of .unwrap()
        let mut particles = vec![];
        let particle_images = vec![
            Image::new(ctx, "/blue.bmp").unwrap(),
            Image::new(ctx, "/red.bmp").unwrap(),
            Image::new(ctx, "/green.bmp").unwrap(),
        ];
        for i in 0..10 {
            let particle = Particle::new(ball.x, ball.y, particle_images.clone());
            particles.push(particle);
        }

        // Initialize the state
        GameState {
            dt,
            mouse_x: 0.0,
            mouse_y: 0.0,
            debug_mode: false,
            play_sounds: true,
            show_particles: true,
            game_width,
            game_height,
            game_mode: GameMode::Game,
            ball,
            walls,
            paddles,
            paused: None,
            player1_score: 0,
            player2_score: 0,
            goal_sound,
            pad_sound,
            wall_sound,
            particle_images,
            particles,
            menu,
        }
    }

    pub fn toggle_menu(&mut self) {
        match self.game_mode {
            GameMode::Game => {
                self.menu = Menu::new(self.menu.current_menu_choice);
                self.game_mode = GameMode::Menu;
            }
            GameMode::Menu => {
                self.menu = Menu::new(self.menu.current_menu_choice);
                self.game_mode = GameMode::Game;
            }
        }
    }

    pub fn stop_particles(&mut self) {
        for i in 0..self.particles.len() {
            self.particles[i].is_dead = true;
        }
    }
}
