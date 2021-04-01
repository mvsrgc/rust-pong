use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::input::mouse::MouseButton;

use ggez::Context;
use ggez::*;

use crate::pong::GameState;
use crate::pong::{DEFAULT_TIME_SCALE, DIRECTION_DOWN, DIRECTION_UP};

impl GameState {
    pub fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        match button {
            MouseButton::Left => self.clicks += 1,
            default => (),
        }
    }

    pub fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) {
        self.mouse_x = _x;
        self.mouse_y = _y;
    }

    pub fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
    ) {
        match keycode {
            KeyCode::F1 => self.debug_mode = !self.debug_mode,
            KeyCode::Escape => event::quit(ctx),
            KeyCode::W => self.left_paddle.direction = DIRECTION_UP,
            KeyCode::S => self.left_paddle.direction = DIRECTION_DOWN,
            KeyCode::Up => self.right_paddle.direction = DIRECTION_UP,
            KeyCode::Down => self.right_paddle.direction = DIRECTION_DOWN,
            KeyCode::PageUp => self.time_scale *= 1.5,
            KeyCode::PageDown => self.time_scale /= 1.5,
            KeyCode::Home => self.time_scale = DEFAULT_TIME_SCALE,
            KeyCode::End => self.time_scale = 0.0,
            default => (),
        }
    }
}