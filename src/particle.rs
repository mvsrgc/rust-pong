use std::collections::HashMap;

use ggez::graphics::Image;
use rand::distributions::{Distribution, Uniform};

use crate::assets::Assets;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum ParticleType {
    Green,
    Red,
    Blue,
    Shimmer,
    None,
}

#[derive(Clone)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub frame: usize,
    pub is_dead: bool,
    pub particle_type: ParticleType,
    pub shimmer: bool,
}

impl Particle {
    pub fn new(x: f32, y: f32, assets: &mut Assets, shimmer: bool) -> Particle {
        let mut rng = rand::thread_rng();

        let x = x - 16.0 + Uniform::new_inclusive(0, 24).sample(&mut rng) as f32;
        let y = y - 16.0 + Uniform::new_inclusive(0, 24).sample(&mut rng) as f32;

        let frame = Uniform::new_inclusive(0, 4).sample(&mut rng) as usize;

        let particle_type;
        match Uniform::from(0..3).sample(&mut rng) {
            0 => particle_type = ParticleType::Blue,
            1 => particle_type = ParticleType::Red,
            2 => particle_type = ParticleType::Green,
            _ => particle_type = ParticleType::None,
        }

        Particle {
            x,
            y,
            frame,
            is_dead: false,
            particle_type,
            shimmer,
        }
    }
}
