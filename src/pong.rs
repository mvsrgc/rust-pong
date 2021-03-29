use ggez::graphics::DrawMode;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::input::mouse::MouseButton;

use ggez::event::EventHandler;
use ggez::graphics::{self, DrawParam};
use ggez::nalgebra::Point2;
use ggez::timer;
use ggez::*;
use ggez::{Context, GameResult};

const PADDLE_WIDTH: f32 = 12.0;
const PADDLE_HEIGHT: f32 = 96.0;

pub struct Pong {
    pub clicks: usize,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub dt: f64,
    pub debug_mode: bool,
    pub left_paddle: Paddle,
    pub right_paddle: Paddle,
}

pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub speed: f32,
    pub direction: f32,
}

impl Paddle {
    pub fn new(game_width: f32, game_height: f32, left: bool) -> Paddle {
        Paddle {
            x: match left {
                true => 0.0,
                false => game_width - PADDLE_WIDTH,
            },
            y: ((game_height - PADDLE_HEIGHT) / 2.0),
            w: PADDLE_WIDTH,
            h: PADDLE_HEIGHT,
            speed: 350.0,
            direction: 0.0,
        }
    }
}

impl Pong {
    pub fn simulate(&mut self, time: f64) {
        if self.left_paddle.direction != 0.0 {
            let distance = self.left_paddle.speed as f64 * time;
            self.left_paddle.y =
                self.left_paddle.y - (distance as f32 * self.left_paddle.direction);
        }

        if self.right_paddle.direction != 0.0 {
            let distance = self.right_paddle.speed as f64 * time;
            self.right_paddle.y =
                self.right_paddle.y - (distance as f32 * self.right_paddle.direction)
        }
    }
}

fn build_rectangle(
    ctx: &mut Context,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> GameResult<graphics::Mesh> {
    let mb = &mut graphics::MeshBuilder::new();

    let rect = graphics::Rect::new(x, y, w, h);

    mb.rectangle(DrawMode::fill(), rect, graphics::WHITE);

    mb.build(ctx)
}

impl EventHandler for Pong {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut frame_time = timer::delta(ctx).as_secs_f64();
        while frame_time > 0.0 {
            let cmp = frame_time.partial_cmp(&self.dt).expect("float NaN error");
            let delta_time: f64 = if let std::cmp::Ordering::Less = cmp {
                frame_time
            } else {
                self.dt
            };
            self.simulate(delta_time);
            frame_time -= delta_time;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let fps_display = graphics::Text::new(format!("FPS: {}", timer::fps(ctx)));
        let mouse_display =
            graphics::Text::new(format!("Mouse: ({}, {})", self.mouse_x, self.mouse_y));

        graphics::clear(ctx, graphics::BLACK);

        if self.debug_mode {
            graphics::draw(ctx, &fps_display, (Point2::new(0.0, 0.0), graphics::WHITE))?;
            graphics::draw(
                ctx,
                &mouse_display,
                (Point2::new(0.0, 20.0), graphics::WHITE),
            )?;
        }

        let left_rectangle = build_rectangle(
            ctx,
            self.left_paddle.x,
            self.left_paddle.y,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        )?;

        let right_rectangle = build_rectangle(
            ctx,
            self.right_paddle.x,
            self.right_paddle.y,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        )?;

        graphics::draw(ctx, &left_rectangle, DrawParam::default())?;
        graphics::draw(ctx, &right_rectangle, DrawParam::default())?;

        graphics::present(ctx)
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        match button {
            MouseButton::Left => self.clicks += 1,
            default => (),
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) {
        self.mouse_x = _x;
        self.mouse_y = _y;
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
    ) {
        match keycode {
            KeyCode::F1 => self.debug_mode = !self.debug_mode,
            KeyCode::Escape => event::quit(ctx),
            KeyCode::W => self.left_paddle.direction = 1.0,
            KeyCode::S => self.left_paddle.direction = -1.0,
            KeyCode::Up => self.right_paddle.direction = 1.0,
            KeyCode::Down => self.right_paddle.direction = -1.0,
            default => (),
        }
    }
}
