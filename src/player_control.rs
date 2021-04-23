use ggez::{
    event,
    input::keyboard::{KeyCode, KeyMods},
    input::mouse::MouseButton,
    Context,
};

use crate::pong::{
    Direction, GameState, DEFAULT_TIME_SCALE, LEFT_PADDLE_INDEX, RIGHT_PADDLE_INDEX,
};

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
            KeyCode::W => self.paddles[LEFT_PADDLE_INDEX].direction = Direction::Up,
            KeyCode::S => self.paddles[LEFT_PADDLE_INDEX].direction = Direction::Down,
            KeyCode::Up => self.paddles[RIGHT_PADDLE_INDEX].direction = Direction::Up,
            KeyCode::Down => self.paddles[RIGHT_PADDLE_INDEX].direction = Direction::Down,
            KeyCode::PageUp => self.time_scale *= 1.5,
            KeyCode::PageDown => self.time_scale /= 1.5,
            KeyCode::Home => self.time_scale = DEFAULT_TIME_SCALE,
            KeyCode::End => self.time_scale = 0.0,
            _default => (),
        }
    }
}
