use ggez::{
    event,
    input::keyboard::{KeyCode, KeyMods},
    input::mouse::MouseButton,
    Context,
};

use crate::pong::{Direction, GameState, Side, DEFAULT_TIME_SCALE};

impl GameState {
    pub fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        match button {
            MouseButton::Left => self.clicks += 1,
            _default => (),
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
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::F1 => self.debug_mode = !self.debug_mode,
            KeyCode::F2 => self.play_sounds = !self.play_sounds,
            KeyCode::Escape => event::quit(ctx),
            KeyCode::W => {
                for i in 0..self.paddles.len() {
                    match self.paddles[i].side {
                        Side::Left => self.paddles[i].direction = Direction::Up,
                        Side::Right => {}
                    }
                }
            }
            KeyCode::S => {
                for i in 0..self.paddles.len() {
                    match self.paddles[i].side {
                        Side::Left => self.paddles[i].direction = Direction::Down,
                        Side::Right => {}
                    }
                }
            }
            KeyCode::Up => {
                for i in 0..self.paddles.len() {
                    match self.paddles[i].side {
                        Side::Left => {}
                        Side::Right => self.paddles[i].direction = Direction::Up,
                    }
                }
            }
            KeyCode::Down => {
                for i in 0..self.paddles.len() {
                    match self.paddles[i].side {
                        Side::Left => {}
                        Side::Right => self.paddles[i].direction = Direction::Down,
                    }
                }
            }
            KeyCode::PageUp => self.time_scale *= 1.5,
            KeyCode::PageDown => self.time_scale /= 1.5,
            KeyCode::Home => self.time_scale = DEFAULT_TIME_SCALE,
            KeyCode::End => self.time_scale = 0.0,
            _default => (),
        }
    }
}
