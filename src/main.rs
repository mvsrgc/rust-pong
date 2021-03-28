use ggez::input::mouse::MouseButton;

use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam};
use ggez::nalgebra::Point2;
use ggez::timer;
use ggez::{conf, Context, GameResult};

mod utils;

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        utils::print_fps("update", ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let fps = timer::fps(ctx);
        let fps_display = graphics::Text::new(format!("FPS: {}", fps));
        let clicks_display = graphics::Text::new(format!("Clicks: {}", self.clicks));
        let mouse_display =
            graphics::Text::new(format!("Mouse: ({}, {})", self.mouse_x, self.mouse_y));

        utils::print_fps("draw", ctx);

        graphics::clear(ctx, graphics::WHITE);

        graphics::draw(ctx, &fps_display, (Point2::new(0.0, 0.0), graphics::BLACK))?;

        graphics::draw(
            ctx,
            &clicks_display,
            (Point2::new(0.0, 20.0), graphics::BLACK),
        )?;

        graphics::draw(
            ctx,
            &mouse_display,
            (Point2::new(0.0, 40.0), graphics::BLACK),
        )?;

        let rect = graphics::Rect::new(self.mouse_x, self.mouse_y, 50.0, 50.0);
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
}

struct State {
    clicks: usize,
    mouse_x: f32,
    mouse_y: f32,
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("name", "author")
        .window_setup(conf::WindowSetup::default().vsync(true));

    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut State {
        clicks: 0,
        mouse_x: 0.0,
        mouse_y: 0.0,
    };

    event::run(ctx, event_loop, state)
}
