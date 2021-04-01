use ggez::graphics::DrawMode;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::input::mouse::MouseButton;

use ggez::event::EventHandler;
use ggez::graphics::{self, DrawParam};
use ggez::nalgebra::Point2;
use ggez::timer;
use ggez::*;
use ggez::{Context, GameResult};

use crate::paddle::Paddle;

pub const DEFAULT_TIME_SCALE: f64 = 1.0;

pub const DIRECTION_UP: f32 = 1.0;
pub const DIRECTION_DOWN: f32 = -1.0;

pub enum Side {
    Left,
    Right,
}

pub enum Direction {
    Up,
    Down,
}

pub struct GameState {
    pub clicks: usize,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub dt: f64,
    pub time_scale: f64,
    pub debug_mode: bool,
    pub left_paddle: Paddle,
    pub right_paddle: Paddle,
    pub game_width: f32,
    pub game_height: f32,
}

impl GameState {
    pub fn new(game_width: f32, game_height: f32) -> GameState {
        let time_scale: f64 = DEFAULT_TIME_SCALE;
        GameState {
            clicks: 0,
            mouse_x: 0.0,
            mouse_y: 0.0,
            time_scale,
            dt: (1.0f64 / 60.0f64) * time_scale,
            debug_mode: false,
            game_width,
            game_height,
            left_paddle: Paddle::new(game_width, game_height, Side::Left),
            right_paddle: Paddle::new(game_width, game_height, Side::Right),
        }
    }

    pub fn simulate(&mut self, time: f64) {
        if self.left_paddle.direction != 0.0 {
            let distance = self.left_paddle.speed as f64 * time;
            self.left_paddle.y =
                self.left_paddle.y - (distance as f32 * self.left_paddle.direction);

            if self.left_paddle.direction == DIRECTION_UP && self.left_paddle.y < 0.0 {
                self.left_paddle.y = 0.0;
            }

            if self.left_paddle.direction == DIRECTION_DOWN
                && self.left_paddle.y + self.left_paddle.h > self.game_height
            {
                self.left_paddle.y = self.game_height - self.left_paddle.h;
            }
        }

        if self.right_paddle.direction != 0.0 {
            let distance = self.right_paddle.speed as f64 * time;
            self.right_paddle.y =
                self.right_paddle.y - (distance as f32 * self.right_paddle.direction);

            if self.right_paddle.direction == DIRECTION_UP && self.right_paddle.y < 0.0 {
                self.right_paddle.y = 0.0;
            }

            if self.right_paddle.direction == DIRECTION_DOWN
                && self.right_paddle.y + self.right_paddle.h > self.game_height
            {
                self.right_paddle.y = self.game_height - self.right_paddle.h;
            }
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut frame_time = timer::delta(ctx).as_secs_f64();
        while frame_time > 0.0 {
            let cmp = frame_time.partial_cmp(&self.dt).expect("float NaN error");

            let mut delta_time: f64 = if let std::cmp::Ordering::Less = cmp {
                frame_time
            } else {
                self.dt
            };

            self.simulate(delta_time * self.time_scale);

            frame_time -= delta_time;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.draw(ctx)
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.mouse_button_up_event(ctx, button, x, y);
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) {
        self.mouse_motion_event(ctx, x, y, dx, dy);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
    ) {
        self.key_down_event(ctx, keycode, keymod, repeat);
    }
}
