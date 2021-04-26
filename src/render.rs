use ggez::{
    graphics::Font,
    graphics::{self, DrawMode, DrawParam, Image, Scale},
    nalgebra::Point2,
    timer, Context, GameResult,
};
use graphics::Color;
use std::{collections::HashMap, time::Duration};

use crate::{assets::Assets, particle::Particle};
use crate::{
    game_state::{GameMode, GameState},
    particle::ParticleType,
};

fn get_text_width(ctx: &mut Context, text: &str, font: Font, scale: f32) -> u32 {
    let mut text = graphics::Text::new(text);
    text.set_font(font, Scale::uniform(scale));

    text.width(ctx)
}

fn get_text_height(ctx: &mut Context, text: &str, font: Font, scale: f32) -> u32 {
    let mut text = graphics::Text::new(text);
    text.set_font(font, Scale::uniform(scale));

    text.height(ctx)
}

fn draw_text(
    ctx: &mut Context,
    text: &str,
    pos: Point2<f32>,
    font: Font,
    scale: f32,
    color: Color,
) -> GameResult<()> {
    let mut text = graphics::Text::new(text);
    text.set_font(font, graphics::Scale::uniform(scale));

    let params = graphics::DrawParam::default().dest(pos).color(color);

    graphics::draw(ctx, &text, params)?;

    Ok(())
}

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

fn draw_particles(
    ctx: &mut Context,
    x: f32,
    y: f32,
    particles: &mut Vec<Particle>,
    assets: &mut Assets,
) -> GameResult<()> {
    for particle in particles.iter_mut() {
        if particle.is_dead {
            *particle = Particle::new(x, y, false);
        }

        if particle.frame % 2 == 0 {
            particle.shimmer = true;
        }

        particle.frame = particle.frame + 1;

        if particle.frame > 12 {
            particle.is_dead = true;
        }

        let image = assets.particle_image(particle.particle_type);

        graphics::draw(
            ctx,
            image,
            DrawParam::new().dest(Point2::new(particle.x, particle.y)),
        )?;

        if particle.shimmer {
            let shimmer_image = assets.particle_image(ParticleType::Shimmer);

            graphics::draw(
                ctx,
                shimmer_image,
                DrawParam::new()
                    .dest(Point2::new(particle.x, particle.y))
                    .color(Color::from_rgba(255, 255, 255, 50)),
            )?;

            particle.shimmer = false;
        }
    }

    Ok(())
}

impl GameState {
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        match self.game_mode {
            GameMode::Menu => {
                self.draw_menu(ctx)?;
            }
            GameMode::Game => {
                self.draw_game(ctx)?;
            }
        }

        graphics::present(ctx)
    }

    fn draw_game(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Draw debug mode information like FPS, mouse coordinates, time scale.
        if self.debug_mode {
            draw_text(
                ctx,
                &format!("FPS: {}", timer::fps(ctx)),
                Point2::new(0.0, 0.0),
                Font::default(),
                20.0,
                graphics::WHITE,
            )?;
        }

        // Draw the paddles
        for paddle in self.paddles.iter_mut() {
            let paddle_rect = build_rectangle(
                ctx,
                (*paddle).rect.x,
                (*paddle).rect.y,
                (*paddle).rect.w,
                (*paddle).rect.h,
            )?;

            graphics::draw(ctx, &paddle_rect, DrawParam::default())?;
        }

        // Draw the ball
        let ball = build_circle(ctx, self.ball.x, self.ball.y, self.ball.radius)?;
        graphics::draw(ctx, &ball, DrawParam::default())?;

        // Draw ball particles
        if self.show_particles {
            draw_particles(
                ctx,
                self.ball.x,
                self.ball.y,
                &mut self.particles,
                &mut self.assets,
            )?;
        }

        // Draw UI text
        let fancy_font = Font::new(ctx, "/joystix_mono.ttf")?;

        // Game title
        let width = get_text_width(ctx, "PONG", fancy_font.clone(), 80.0);
        draw_text(
            ctx,
            "PONG",
            Point2::new(self.game_width / 2.0 - width as f32 / 2.0, 10.0),
            fancy_font.clone(),
            80.0,
            graphics::WHITE,
        )?;

        // Scores
        let score_text = &format!("{} \t {}", self.player1_score, self.player2_score);
        let width = get_text_width(ctx, score_text, fancy_font.clone(), 80.0);
        let height = get_text_height(ctx, score_text, fancy_font.clone(), 80.0);
        draw_text(
            ctx,
            score_text,
            Point2::new(
                self.game_width / 2.0 - width as f32 / 2.0,
                self.game_height / 2.0 - height as f32 / 2.0,
            ),
            fancy_font.clone(),
            80.0,
            Color::from_rgba(255, 255, 255, 25),
        )?;

        // Draw READY then draw START! when the game is reset
        if let Some(pause_time) = self.paused {
            let mut status_text_string = "READY";

            if pause_time <= Duration::from_millis(500) {
                status_text_string = "START!";
            }

            let width = get_text_width(ctx, status_text_string, fancy_font.clone(), 25.0);
            let height = get_text_height(ctx, status_text_string, fancy_font.clone(), 25.0);
            draw_text(
                ctx,
                status_text_string,
                Point2::new(
                    self.game_width / 2.0 - width as f32 / 2.0,
                    (self.game_height / 2.0 - height as f32 / 2.0) + height as f32 * 1.7,
                ),
                fancy_font.clone(),
                25.0,
                Color::from_rgba(255, 255, 255, 25),
            )?;
        }

        Ok(())
    }

    fn draw_menu(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Draw UI text
        let fancy_font = Font::new(ctx, "/joystix_mono.ttf")?;

        // Game title
        let width = get_text_width(ctx, "PONG", fancy_font.clone(), 80.0);
        draw_text(
            ctx,
            "PONG",
            Point2::new(self.game_width / 2.0 - width as f32 / 2.0, 10.0),
            fancy_font.clone(),
            80.0,
            graphics::WHITE,
        )?;

        let sound_toggle_text = match self.play_sounds {
            true => "Sounds ON",
            false => "Sounds OFF",
        };

        let particles_toggle_text = match self.show_particles {
            true => "Particles ON",
            false => "Particles OFF",
        };

        let menu_items = vec![
            "Resume",
            sound_toggle_text,
            particles_toggle_text,
            "Restart",
            "Quit",
        ];

        for i in 0..menu_items.len() {
            let mut color = Color::from_rgba(255, 255, 255, 25);

            if self.menu.current_menu_choice == i as isize {
                color = Color::from_rgba(0, 51, 102, 255);
            }

            let width = get_text_width(ctx, menu_items[i], fancy_font.clone(), 60.0);
            let height = get_text_height(ctx, menu_items[i], fancy_font.clone(), 60.0);

            draw_text(
                ctx,
                menu_items[i],
                Point2::new(
                    self.game_width / 2.0 - width as f32 / 2.0,
                    self.game_height / 3.0 + ((height + 10) * i as u32) as f32,
                ),
                fancy_font.clone(),
                60.0,
                color,
            )?;
        }

        Ok(())
    }
}
