use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::input::mouse::MouseButton;

use ggez::event::EventHandler;
use ggez::graphics::{self, DrawParam};
use ggez::nalgebra::Point2;
use ggez::timer;
use ggez::*;
use ggez::{Context, GameResult};

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

        graphics::clear(ctx, graphics::WHITE);

        if self.debug_mode {
            graphics::draw(ctx, &fps_display, (Point2::new(0.0, 0.0), graphics::BLACK))?;
            graphics::draw(
                ctx,
                &mouse_display,
                (Point2::new(0.0, 20.0), graphics::BLACK),
            )?;
        }

        let rect = graphics::Rect::new(self.rect_x, self.rect_y, 50.0, 50.0);
        let rect_mesh =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::BLACK)?;

        graphics::draw(ctx, &rect_mesh, DrawParam::default())?;

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
            default => (),
        }
    }
}

pub struct Pong {
    pub clicks: usize,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub rect_x: f32,
    pub rect_y: f32,
    pub velocity_x: f32,
    pub dt: f64,
    pub debug_mode: bool,
}

impl Pong {
    pub fn simulate(&mut self, time: f64) {
        let distance = self.velocity_x as f64 * time;
        self.rect_x = self.rect_x % 1920.0 + distance as f32;
    }
}
