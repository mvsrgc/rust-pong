use ggez::{
    audio::{self, SoundSource},
    graphics, Context, GameResult,
};

use crate::{particle::ParticleType, pong::SoundType};

pub struct Assets {
    pub goal_sound: audio::Source,
    pub wall_sound: audio::Source,
    pub pad_sound: audio::Source,
    pub blue_particle: graphics::Image,
    pub red_particle: graphics::Image,
    pub green_particle: graphics::Image,
    pub shimmer_particle: graphics::Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let goal_sound = audio::Source::new(ctx, "/goal.wav")?;
        let wall_sound = audio::Source::new(ctx, "/wall.wav")?;
        let pad_sound = audio::Source::new(ctx, "/pad.wav")?;

        let green_particle = graphics::Image::new(ctx, "/green.bmp")?;
        let red_particle = graphics::Image::new(ctx, "/red.bmp")?;
        let blue_particle = graphics::Image::new(ctx, "/blue.bmp")?;
        let shimmer_particle = graphics::Image::new(ctx, "/shimmer.bmp")?;

        Ok(Assets {
            goal_sound,
            wall_sound,
            pad_sound,
            green_particle,
            red_particle,
            blue_particle,
            shimmer_particle,
        })
    }

    pub fn play_sound(&mut self, play_sounds: bool, sound_type: SoundType) {
        if !play_sounds {
            return;
        };

        match sound_type {
            SoundType::Goal => self.goal_sound.play_detached().unwrap(),
            SoundType::Pad => self.pad_sound.play_detached().unwrap(),
            SoundType::Wall => self.wall_sound.play_detached().unwrap(),
        }
    }

    pub fn particle_image(&self, particle: ParticleType) -> &graphics::Image {
        match particle {
            ParticleType::Green => &self.green_particle,
            ParticleType::Red => &self.red_particle,
            ParticleType::Blue => &self.blue_particle,
            ParticleType::Shimmer => &self.shimmer_particle,
            _ => &self.green_particle,
        }
    }
}
