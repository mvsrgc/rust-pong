use ggez::{
    event,
    input::keyboard::{KeyCode, KeyMods},
    Context,
};

use crate::game_state::{GameMode, GameState};

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
        match self.game_mode {
            GameMode::Game => match keycode {
                KeyCode::F1 => self.debug_mode = !self.debug_mode,
                KeyCode::F2 => self.play_sounds = !self.play_sounds,
                KeyCode::Escape => self.toggle_menu(),
                KeyCode::W => self.paddles[LEFT_PADDLE_INDEX].is_up_holding = true,
                KeyCode::S => self.paddles[LEFT_PADDLE_INDEX].is_down_holding = true,
                KeyCode::Up => self.paddles[RIGHT_PADDLE_INDEX].is_up_holding = true,
                KeyCode::Down => self.paddles[RIGHT_PADDLE_INDEX].is_down_holding = true,
                _ => (),
            },

            GameMode::Menu => match keycode {
                KeyCode::Up => self.menu.advance_menu_choice(1),
                KeyCode::Down => self.menu.advance_menu_choice(-1),
                KeyCode::Return => self.handle_menu_return(ctx, self.menu.current_menu_choice),
                KeyCode::Escape => self.toggle_menu(),
                _ => (),
            },
        }
    }

    pub fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        match self.game_mode {
            GameMode::Game => match keycode {
                KeyCode::W => self.paddles[LEFT_PADDLE_INDEX].is_up_holding = false,
                KeyCode::S => self.paddles[LEFT_PADDLE_INDEX].is_down_holding = false,
                KeyCode::Up => self.paddles[RIGHT_PADDLE_INDEX].is_up_holding = false,
                KeyCode::Down => self.paddles[RIGHT_PADDLE_INDEX].is_down_holding = false,
                _ => (),
            },
            _ => (),
        }
    }

    fn handle_menu_return(&mut self, ctx: &mut Context, current_menu_choice: isize) {
        match current_menu_choice {
            0 => self.toggle_menu(),
            1 => self.play_sounds = !self.play_sounds,
            2 => {
                if self.particle_images.len() > 1 {
                    self.show_particles = !self.show_particles;
                }
                self.stop_particles();
            }
            3 => {
                self.reset_game(true);
                self.toggle_menu()
            }
            4 => event::quit(ctx),
            _ => (),
        };
    }
}
