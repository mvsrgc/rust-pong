use ggez::input::mouse::MouseButton;

use ggez::event::{self, EventHandler};
use ggez::nalgebra::Point2;
use ggez::{conf, Context, GameResult};
use ggez::{graphics, timer};

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

        utils::print_fps("draw", ctx);

        graphics::clear(ctx, graphics::WHITE);

        graphics::draw(ctx, &fps_display, (Point2::new(0.0, 0.0), graphics::BLACK))?;

        graphics::draw(
            ctx,
            &clicks_display,
            (Point2::new(0.0, 20.0), graphics::BLACK),
        )?;

        graphics::present(ctx)
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        match button {
            MouseButton::Left => self.clicks += 1,
            default => (),
        }
    }
}

struct State {
    clicks: usize,
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("name", "author")
        .window_mode(conf::WindowMode::default().dimensions(1920.0, 1080.0));

    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut State { clicks: 0 };

    event::run(ctx, event_loop, state)
}
