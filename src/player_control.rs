use ggez::{
    event,
    input::keyboard::{KeyCode, KeyMods},
    Context,
};

use crate::game_state::{GameMode, GameState, DEFAULT_TIME_SCALE};

// @Refactor
const LEFT_PADDLE_INDEX: usize = 0;
const RIGHT_PADDLE_INDEX: usize = 1;

impl GameState {
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
            KeyCode::Escape => self.toggle_menu(),
            KeyCode::W => self.paddles[LEFT_PADDLE_INDEX].is_up_holding = true,
            KeyCode::S => self.paddles[LEFT_PADDLE_INDEX].is_down_holding = true,
            KeyCode::Up => match self.game_mode {
                GameMode::Game => {
                    self.paddles[RIGHT_PADDLE_INDEX].is_up_holding = true;
                }
                GameMode::Menu => {
                    self.menu.advance_menu_choice(1);
                }
            },
            KeyCode::Down => match self.game_mode {
                GameMode::Game => self.paddles[RIGHT_PADDLE_INDEX].is_down_holding = true,
                GameMode::Menu => {
                    self.menu.advance_menu_choice(-1);
                }
            },
            KeyCode::Return => match self.game_mode {
                GameMode::Game => (),
                GameMode::Menu => match self.menu.current_menu_choice {
                    0 => self.toggle_menu(),
                    1 => self.play_sounds = !self.play_sounds,
                    2 => {
                        self.reset_game(true);
                        self.toggle_menu()
                    }
                    3 => event::quit(ctx),
                    _ => (),
                },
            },
            KeyCode::PageUp => self.time_scale *= 1.5,
            KeyCode::PageDown => self.time_scale /= 1.5,
            KeyCode::Home => self.time_scale = DEFAULT_TIME_SCALE,
            KeyCode::End => self.time_scale = 0.0,
            _ => (),
        }
    }

    pub fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        match keycode {
            KeyCode::W => self.paddles[LEFT_PADDLE_INDEX].is_up_holding = false,
            KeyCode::S => self.paddles[LEFT_PADDLE_INDEX].is_down_holding = false,
            KeyCode::Up => self.paddles[RIGHT_PADDLE_INDEX].is_up_holding = false,
            KeyCode::Down => self.paddles[RIGHT_PADDLE_INDEX].is_down_holding = false,
            _ => (),
        }
    }
}
