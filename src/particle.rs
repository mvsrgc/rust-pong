use ggez::{graphics::Image, Context, GameResult};
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

pub enum ParticleType {
    Green,
    Red,
    Blue,
    None,
}
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub frame: usize,
    pub is_dead: bool,
    pub particle_type: ParticleType,
    pub surface: Option<Image>,
}

impl Particle {
    pub fn new(x: f32, y: f32, images: Vec<Image>) -> Particle {
        let mut rng = rand::thread_rng();

        let die = Uniform::new_inclusive(0, 24);
        let x = x - 16.0 + die.sample(&mut rng) as f32;
        let y = y - 16.0 + die.sample(&mut rng) as f32;

        let die = Uniform::new_inclusive(0, 4);
        let frame = die.sample(&mut rng) as usize;

        let particle_types = vec![0, 1, 2];
        let die = Uniform::new_inclusive(0, particle_types.len() - 1);

        let particle_type;
        let surface;
        match particle_types[die.sample(&mut rng)] {
            0 => {
                particle_type = ParticleType::Blue;
                surface = Some(images[0].clone());
            }
            1 => {
                particle_type = ParticleType::Red;
                surface = Some(images[1].clone());
            }
            2 => {
                particle_type = ParticleType::Green;
                surface = Some(images[2].clone());
            }
            _ => {
                particle_type = ParticleType::None;
                surface = None;
            }
        };

        Particle {
            x,
            y,
            frame,
            is_dead: false,
            particle_type,
            surface,
        }
    }
}
