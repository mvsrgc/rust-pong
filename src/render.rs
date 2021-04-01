use ggez::graphics::DrawMode;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::input::mouse::MouseButton;

use ggez::event::EventHandler;
use ggez::graphics::{self, DrawParam};
use ggez::nalgebra::Point2;
use ggez::timer;
use ggez::*;
use ggez::{Context, GameResult};

use crate::pong::GameState;

fn build_rectangle(
    ctx: &mut Context,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
) -> GameResult<graphics::Mesh> {
    let mb = &mut graphics::MeshBuilder::new();

    let rect = graphics::Rect::new(x as f32, y as f32, w as f32, h as f32);

    mb.rectangle(DrawMode::fill(), rect, graphics::WHITE);

    mb.build(ctx)
}

fn build_net_line(
    ctx: &mut Context,
    game_width: i32,
    game_height: i32,
) -> GameResult<graphics::Mesh> {
    let mb = &mut graphics::MeshBuilder::new();

    let rect = graphics::Rect::new(
        ((game_width as f32 - (5.0 / 2.0)) / 2.0) as f32,
        0.0,
        5.0,
        game_height as f32,
    );
    mb.rectangle(DrawMode::fill(), rect, graphics::WHITE);

    mb.build(ctx)
}

impl GameState {
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let fps_display = graphics::Text::new(format!("FPS: {}", timer::fps(ctx)));
        let mouse_display =
            graphics::Text::new(format!("Mouse: ({}, {})", self.mouse_x, self.mouse_y));
        let dt_display = graphics::Text::new(format!(
            "Dt: {} - Scale: {}",
            self.dt * self.time_scale,
            self.time_scale,
        ));

        graphics::clear(ctx, graphics::BLACK);

        if self.debug_mode {
            graphics::draw(ctx, &fps_display, (Point2::new(0.0, 0.0), graphics::WHITE))?;
            graphics::draw(
                ctx,
                &mouse_display,
                (Point2::new(0.0, 20.0), graphics::WHITE),
            )?;
            graphics::draw(ctx, &dt_display, (Point2::new(0.0, 40.0), graphics::WHITE))?;
        }

        let left_rectangle = build_rectangle(
            ctx,
            self.left_paddle.x as i32,
            self.left_paddle.y as i32,
            self.left_paddle.w as i32,
            self.left_paddle.h as i32,
        )?;

        let right_rectangle = build_rectangle(
            ctx,
            self.right_paddle.x as i32,
            self.right_paddle.y as i32,
            self.right_paddle.w as i32,
            self.right_paddle.h as i32,
        )?;

        let middle_line = build_net_line(ctx, self.game_width as i32, self.game_height as i32)?;

        graphics::draw(ctx, &left_rectangle, DrawParam::default())?;
        graphics::draw(ctx, &right_rectangle, DrawParam::default())?;
        graphics::draw(ctx, &middle_line, DrawParam::default())?;

        graphics::present(ctx)
    }
}
