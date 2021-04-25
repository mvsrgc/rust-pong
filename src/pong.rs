use std::time::Duration;

use ggez::{
    audio::SoundSource,
    event::EventHandler,
    graphics::Rect,
    input::keyboard::{KeyCode, KeyMods},
    timer, Context, GameResult,
};

use crate::{ball::Ball, game_state::GameState, paddle::Paddle};

#[derive(Clone, Copy)]
pub enum Side {
    Left,
    Right,
    Top,
    Bottom,
}

pub enum SoundType {
    Goal,
    Pad,
    Wall,
}

pub struct Wall {
    rect: Rect,
    side: Side,
}

impl Wall {
    pub fn new(rect: Rect, side: Side) -> Wall {
        Wall { rect, side }
    }
}

impl GameState {
    pub fn simulate(&mut self, ctx: &mut Context, time: f64) {
        // If the game is paused, then we don't want to simulate.
        match self.paused {
            Some(time_paused) => {
                self.paused = time_paused.checked_sub(timer::delta(ctx));

                return;
            }
            None => (),
        }

        // Update poaddle positions and check paddle collisions.
        for i in 0..self.paddles.len() {
            let distance = self.paddles[i].dy as f64 * time;

            let direction_value;
            if self.paddles[i].is_up_holding && self.paddles[i].is_down_holding {
                direction_value = 0.0;
            } else if self.paddles[i].is_up_holding {
                direction_value = 1.0;
            } else if self.paddles[i].is_down_holding {
                direction_value = -1.0;
            } else {
                direction_value = 0.0;
            }

            // Update paddle position
            self.paddles[i].rect.y = self.paddles[i].rect.y - (distance as f32 * direction_value);

            // Paddle collides with top wall
            if self.paddles[i].rect.y <= 0.0 {
                self.paddles[i].rect.y = 0.0
            }

            // Paddle collides with bottom wall
            if self.paddles[i].rect.y + self.paddles[i].rect.h >= self.game_height {
                self.paddles[i].rect.y = self.game_height - self.paddles[i].rect.h;
            }
        }

        // Update ball position
        self.ball.x = (self.ball.x as f64 + (self.ball.dx as f64 * time)) as f32;
        self.ball.y = (self.ball.y as f64 + (self.ball.dy as f64 * time)) as f32;

        let ball_rect = Rect::new(
            self.ball.x - self.ball.radius,
            self.ball.y - self.ball.radius,
            self.ball.radius * 2.0,
            self.ball.radius * 2.0,
        );

        // Check if ball collides with any walls
        for i in 0..self.walls.len() {
            if !ball_rect.overlaps(&self.walls[i].rect) {
                continue;
            }

            match self.walls[i].side {
                Side::Left => {
                    // Left wall
                    self.ball.x = 0.0 + self.ball.radius;
                    self.player2_score = self.player2_score + 1;

                    self.ball.dx = -self.ball.dx;

                    self.play_sound(SoundType::Goal);

                    self.reset_game(false);

                    self.paused = Some(Duration::from_millis(1200));
                }
                Side::Right => {
                    // Right wall
                    self.ball.x = self.game_width - self.ball.radius;
                    self.player1_score = self.player1_score + 1;

                    self.ball.dx = -self.ball.dx;

                    self.play_sound(SoundType::Goal);

                    self.reset_game(false);

                    self.paused = Some(Duration::from_millis(1200));
                }
                Side::Top => {
                    // Top wall
                    self.ball.y = 0.0 + self.ball.radius;

                    self.ball.dy = -self.ball.dy;

                    self.play_sound(SoundType::Wall)
                }
                Side::Bottom => {
                    // Bottom wall
                    self.ball.y = self.game_height - self.ball.radius;

                    self.ball.dy = -self.ball.dy;

                    self.play_sound(SoundType::Wall);
                }
            }
        }

        // If ball collides with paddles
        for i in 0..self.paddles.len() {
            if ball_rect.overlaps(&self.paddles[i].rect) {
                match self.paddles[i].side {
                    Side::Left => {
                        self.ball.x =
                            self.paddles[i].rect.x + self.paddles[i].rect.w + self.ball.radius;
                    }
                    Side::Right => {
                        self.ball.x = self.paddles[i].rect.x - self.ball.radius;
                    }
                    _ => {}
                }

                self.ball.dx = -self.ball.dx;

                self.play_sound(SoundType::Pad);
            }
        }
    }

    pub fn play_sound(&mut self, sound_type: SoundType) {
        if self.play_sounds {
            match sound_type {
                SoundType::Goal => self.goal_sound.play_detached().unwrap(),
                SoundType::Pad => self.pad_sound.play_detached().unwrap(),
                SoundType::Wall => self.wall_sound.play_detached().unwrap(),
            }
        }
    }

    pub fn reset_game(&mut self, reset_score: bool) {
        self.ball = Ball::new(self.game_width, self.game_height);

        for i in 0..self.paddles.len() {
            self.paddles[i] = Paddle::new(self.game_width, self.game_height, self.paddles[i].side);
        }

        if reset_score {
            self.player1_score = 0;
            self.player2_score = 0;
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut frame_time = timer::delta(ctx).as_secs_f64();
        while frame_time > 0.0 {
            let cmp = frame_time.partial_cmp(&self.dt).expect("float NaN error");

            let delta_time: f64 = if let std::cmp::Ordering::Less = cmp {
                frame_time
            } else {
                self.dt
            };

            self.simulate(ctx, delta_time * self.time_scale);

            frame_time -= delta_time;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.draw(ctx)
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

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        self.key_up_event(ctx, keycode, keymods);
    }
}
