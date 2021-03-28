use ggez::input::mouse::MouseButton;

use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam};
use ggez::nalgebra::Point2;
use ggez::timer;
use ggez::{conf, Context, GameResult};

mod utils;

impl EventHandler for State {
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
}

struct State {
    clicks: usize,
    mouse_x: f32,
    mouse_y: f32,
    rect_x: f32,
    rect_y: f32,
    velocity_x: f32,
    dt: f64,
}

impl State {
    pub fn simulate(&mut self, time: f64) {
        let distance = self.velocity_x as f64 * time;
        self.rect_x = self.rect_x % 1920.0 + distance as f32;
        println!("[update] distance {}", distance);
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("name", "author")
        .window_setup(conf::WindowSetup::default().vsync(false))
        .window_mode(conf::WindowMode::default().dimensions(1920.0, 1080.0));

    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut State {
        clicks: 0,
        mouse_x: 0.0,
        mouse_y: 0.0,
        rect_x: (1920 / 2) as f32,
        rect_y: (1080 / 2) as f32,
        velocity_x: 60.0,
        dt: 1.0f64 / 60.0f64,
    };

    event::run(ctx, event_loop, state)
}
