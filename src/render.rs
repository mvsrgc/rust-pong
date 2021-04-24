use ggez::{
    graphics::Font,
    graphics::{self, DrawMode, DrawParam},
    nalgebra::Point2,
    timer, Context, GameResult,
};
use graphics::Color;
use std::time::Duration;

use crate::pong::GameState;

fn build_rectangle(
    ctx: &mut Context,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> GameResult<graphics::Mesh> {
    let mb = &mut graphics::MeshBuilder::new();

    let rect = graphics::Rect::new(x as f32, y as f32, w as f32, h as f32);

    mb.rectangle(DrawMode::fill(), rect, graphics::WHITE);

    mb.build(ctx)
}

fn build_circle(ctx: &mut Context, x: f32, y: f32, r: f32) -> GameResult<graphics::Mesh> {
    let mb = &mut graphics::MeshBuilder::new();

    mb.circle(
        DrawMode::fill(),
        Point2::new(x, y),
        r,
        0.01,
        graphics::WHITE,
    );

    mb.build(ctx)
}

impl GameState {
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        // Draw debug mode information like FPS, mouse coordinates, time scale.
        if self.debug_mode {
            let fps_display = graphics::Text::new(format!("FPS: {}", timer::fps(ctx)));
            let dt_display = graphics::Text::new(format!(
                "Dt: {} - Scale: {}",
                self.dt * self.time_scale,
                self.time_scale,
            ));

            graphics::draw(ctx, &fps_display, (Point2::new(0.0, 0.0), graphics::WHITE))?;
            graphics::draw(ctx, &dt_display, (Point2::new(0.0, 40.0), graphics::WHITE))?;
        }

        // Draw the paddles
        for i in 0..self.paddles.len() {
            let paddle_rect = build_rectangle(
                ctx,
                self.paddles[i].rect.x,
                self.paddles[i].rect.y,
                self.paddles[i].rect.w,
                self.paddles[i].rect.h,
            )?;

            graphics::draw(ctx, &paddle_rect, DrawParam::default())?;
        }

        // Draw the ball
        let ball = build_circle(ctx, self.ball.x, self.ball.y, self.ball.radius)?;
        graphics::draw(ctx, &ball, DrawParam::default())?;

        // Draw UI text
        let fancy_font = Font::new(ctx, "/joystix_mono.ttf")?;

        // Game title
        let mut game_title_text = graphics::Text::new("PONG");
        game_title_text.set_font(fancy_font.clone(), graphics::Scale::uniform(80.0));

        let coords = [
            self.game_width / 2.0 - game_title_text.width(ctx) as f32 / 2.0,
            10.0,
        ];

        let params = graphics::DrawParam::default().dest(coords);
        graphics::draw(ctx, &game_title_text, params)?;

        // Scores
        let mut scoreboard_text =
            graphics::Text::new(format!("{} \t {}", self.player1_score, self.player2_score));
        scoreboard_text.set_font(fancy_font.clone(), graphics::Scale::uniform(80.0));

        let coords = [
            self.game_width / 2.0 - scoreboard_text.width(ctx) as f32 / 2.0,
            self.game_height / 2.0 - scoreboard_text.height(ctx) as f32 / 2.0,
        ];

        let params = graphics::DrawParam::default()
            .dest(coords)
            .color(Color::from_rgba(255, 255, 255, 25));
        graphics::draw(ctx, &scoreboard_text, params)?;

        // Draw READY then draw START! when the game is reset
        match self.paused {
            Some(time_paused) => {
                let mut status_text_string = "READY";

                if time_paused <= Duration::from_millis(500) {
                    status_text_string = "START!";
                }

                let mut status_text = graphics::Text::new(status_text_string);
                status_text.set_font(fancy_font.clone(), graphics::Scale::uniform(25.0));

                let coords = [
                    self.game_width / 2.0 - status_text.width(ctx) as f32 / 2.0,
                    (self.game_height / 2.0 - status_text.height(ctx) as f32 / 2.0)
                        + status_text.height(ctx) as f32 * 1.7 as f32,
                ];

                let params = graphics::DrawParam::default()
                    .dest(coords)
                    .color(Color::from_rgba(255, 255, 255, 25));
                graphics::draw(ctx, &status_text, params)?;
            }
            None => (),
        }

        graphics::present(ctx)
    }
}
