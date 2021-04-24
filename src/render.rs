use ggez::{
    graphics::{self, DrawMode, DrawParam, Scale},
    graphics::{Font, Text},
    nalgebra::Point2,
    timer, Context, GameResult,
};

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

fn build_net_line(
    ctx: &mut Context,
    game_width: f32,
    game_height: f32,
) -> GameResult<graphics::Mesh> {
    build_rectangle(ctx, game_width / 2.0 - (1.0 / 2.0), 0.0, 1.0, game_height)
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

        let mut paddle_rectangles = vec![];
        for i in 0..self.paddles.len() {
            let paddle_rect = build_rectangle(
                ctx,
                self.paddles[i].rect.x,
                self.paddles[i].rect.y,
                self.paddles[i].rect.w,
                self.paddles[i].rect.h,
            )?;

            paddle_rectangles.push(paddle_rect);
        }

        let net_line = build_net_line(ctx, self.game_width, self.game_height)?;

        // @Cleanup Maybe have a vec that holds all the items in the game and then loop
        // on that vec and call draw() on everything ?
        let ball = build_circle(ctx, self.ball.x, self.ball.y, self.ball.radius)?;

        for i in 0..self.paddles.len() {
            graphics::draw(ctx, &paddle_rectangles[i], DrawParam::default())?;
        }
        graphics::draw(ctx, &net_line, DrawParam::default())?;
        graphics::draw(ctx, &ball, DrawParam::default())?;

        let fancy_font = Font::new(ctx, "/joystix_mono.ttf")?;

        let mut text = Text::new("PONG");
        text.set_font(fancy_font.clone(), Scale::uniform(80.0))
            .set_bounds(
                Point2::new(self.game_width, f32::INFINITY),
                graphics::Align::Center,
            );

        graphics::draw(ctx, &text, DrawParam::default())?;

        graphics::present(ctx)
    }
}
