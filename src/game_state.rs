use std::time::Duration;

use crate::{assets::Assets, ball::Ball, paddle::Paddle, particle::Particle, pong::Wall};

use ggez::{
    audio,
    graphics::{draw, drawable_size, Rect},
    Context, GameResult,
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
    pub particles: Vec<Particle>,
    pub menu: Menu,
    pub assets: Assets,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let dt = 1.0 / 60.0;

        let (game_width, game_height) = drawable_size(ctx);

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

        // Initialize the menu
        let menu = Menu::new(0);

        // Initialize particles
        let mut assets = Assets::new(ctx)?;

        let particles: Vec<Particle> = vec![Particle::new(ball.x, ball.y, &mut assets, false); 12];

        // Initialize the state
        let s = GameState {
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
            particles,
            menu,
            assets,
        };

        Ok(s)
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
