use std::time::Duration;

use ggez::{
    audio,
    audio::SoundSource,
    event::EventHandler,
    graphics::Rect,
    input::keyboard::{KeyCode, KeyMods},
    input::mouse::MouseButton,
    timer, Context, GameResult,
};

use crate::{ball::Ball, paddle::Paddle};

pub const DEFAULT_TIME_SCALE: f64 = 1.0;

pub enum Side {
    Left,
    Right,
}

pub enum SoundType {
    Goal,
    Pad,
    Wall,
}

pub struct GameState {
    pub clicks: usize,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub dt: f64,
    pub time_scale: f64,
    pub debug_mode: bool,
    pub paddles: Vec<Paddle>,
    pub ball: Ball,
    pub game_width: f32,
    pub game_height: f32,
    pub goal_sound: audio::Source,
    pub pad_sound: audio::Source,
    pub wall_sound: audio::Source,
    pub play_sounds: bool,
    pub paused: Option<Duration>,
    pub player1_score: usize,
    pub player2_score: usize,
}

impl GameState {
    pub fn new(ctx: &mut Context, game_width: f32, game_height: f32) -> GameState {
        let time_scale: f64 = DEFAULT_TIME_SCALE;

        let left_paddle = Paddle::new(game_width, game_height, Side::Left);
        let right_paddle = Paddle::new(game_width, game_height, Side::Right);

        let paddles = vec![left_paddle, right_paddle];

        GameState {
            clicks: 0,
            mouse_x: 0.0,
            mouse_y: 0.0,
            time_scale,
            dt: (1.0 / 60.0) * time_scale,
            debug_mode: false,
            game_width,
            game_height,
            paddles,
            ball: Ball::new(game_width, game_height),
            goal_sound: audio::Source::new(ctx, "/goal.wav").unwrap(),
            pad_sound: audio::Source::new(ctx, "/pad.wav").unwrap(),
            wall_sound: audio::Source::new(ctx, "/wall.wav").unwrap(),
            play_sounds: false,
            paused: None,
            player1_score: 0,
            player2_score: 0,
        }
    }

    pub fn simulate(&mut self, ctx: &mut Context, time: f64) {
        match self.paused {
            Some(time_paused) => {
                self.paused = time_paused.checked_sub(timer::delta(ctx));

                return;
            }
            None => (),
        }

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

        // If ball collides with left or right wall
        if self.ball.x - self.ball.radius <= 0.0
            || self.ball.x + self.ball.radius >= self.game_width
        {
            // Left wall
            if self.ball.x - self.ball.radius <= 0.0 {
                self.ball.x = 0.0 + self.ball.radius;
                self.player2_score = self.player2_score + 1;
            }

            // Right wall
            if self.ball.x + self.ball.radius >= self.game_width {
                self.ball.x = self.game_width - self.ball.radius;
                self.player1_score = self.player1_score + 1;
            }

            self.ball.dx = -self.ball.dx;

            self.play_sound(SoundType::Goal);

            self.ball.x = self.game_width / 2.0;
            self.ball.y = self.game_height / 2.0;

            self.paused = Some(Duration::from_millis(1000));
        }

        // If ball collides with bottom or top wall
        if self.ball.y - self.ball.radius <= 0.0
            || self.ball.y + self.ball.radius >= self.game_height
        {
            // Top wall
            if self.ball.y - self.ball.radius <= 0.0 {
                self.ball.y = 0.0 + self.ball.radius;
            }

            // Bottom wall
            if self.ball.y + self.ball.radius >= self.game_height {
                self.ball.y = self.game_height - self.ball.radius;
            }

            self.ball.dy = -self.ball.dy;

            self.play_sound(SoundType::Wall);
        }

        // If ball collides with paddles
        for i in 0..self.paddles.len() {
            //@Cleanup add this to ball struct
            let ball_rect = Rect::new(
                self.ball.x - self.ball.radius,
                self.ball.y - self.ball.radius,
                self.ball.radius * 2.0,
                self.ball.radius * 2.0,
            );

            if ball_rect.overlaps(&self.paddles[i].rect) {
                match self.paddles[i].side {
                    Side::Left => {
                        self.ball.x =
                            self.paddles[i].rect.x + self.paddles[i].rect.w + self.ball.radius;
                    }
                    Side::Right => {
                        self.ball.x = self.paddles[i].rect.x - self.ball.radius;
                    }
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

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        self.key_up_event(ctx, keycode, keymods);
    }
}
