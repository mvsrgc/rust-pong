use std::time::Duration;

use std::collections::HashMap;

use crate::{
    ball::Ball,
    paddle::Paddle,
    particle::{Particle, ParticleType},
    pong::Wall,
};

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
    pub particle_images: HashMap<ParticleType, Image>,
    pub particles: Vec<Particle>,
    pub menu: Menu,
}

impl GameState {
    pub fn new(ctx: &mut Context, game_width: f32, game_height: f32) -> GameState {
        let dt = 1.0 / 60.0;

        // Create the paddles
        let left_paddle = Paddle::new(game_width, game_height, Side::Left);
        let right_paddle = Paddle::new(game_width, game_height, Side::Right);

        // Vector of paddles to easily iterate over each paddle and check collisions with one loop.
        let paddles = vec![left_paddle, right_paddle];

        // Vec of walls for the same reason.
        let walls = vec![
            Wall::new(Rect::new(0.0, 0.0, game_width, 0.0), Side::Top),
            Wall::new(Rect::new(0.0, 0.0, 0.0, game_height), Side::Left),
            Wall::new(Rect::new(game_width, 0.0, 0.0, game_height), Side::Right),
            Wall::new(Rect::new(0.0, game_height, game_width, 0.0), Side::Bottom),
        ];

        // Create the ball.
        let ball = Ball::new(game_width, game_height);

        // Load sounds
        //  We should handle errors here instead of .unwrap()
        let goal_sound = audio::Source::new(ctx, "/goal.wav").unwrap();
        let pad_sound = audio::Source::new(ctx, "/pad.wav").unwrap();
        let wall_sound = audio::Source::new(ctx, "/wall.wav").unwrap();

        // Initialize the menu
        let menu = Menu::new(0);

        // Initialize particles
        let particle_images = Self::load_particle_images(ctx);

        let particles: Vec<Particle> =
            vec![Particle::new(ball.x, ball.y, &particle_images, false); 12];

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

    fn load_particle_images(ctx: &mut Context) -> HashMap<ParticleType, Image> {
        // We should handle errors here instead of .unwrap()

        let mut particle_images = HashMap::new();
        particle_images.insert(ParticleType::Green, Image::new(ctx, "/green.bmp").unwrap());
        particle_images.insert(ParticleType::Red, Image::new(ctx, "/red.bmp").unwrap());
        particle_images.insert(ParticleType::Blue, Image::new(ctx, "/blue.bmp").unwrap());
        particle_images.insert(
            ParticleType::Shimmer,
            Image::new(ctx, "/shimmer.bmp").unwrap(),
        );

        particle_images
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

        self.stop_particles();
    }

    pub fn stop_particles(&mut self) {
        for particle in self.particles.iter_mut() {
            (*particle).is_dead = true;
        }
    }
}
