use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

// Has a lot in common with Ball...
const BALL_RADIUS: f32 = 8.0;
const BALL_SPEED: f32 = 450.0;
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub dx: f32,
    pub dy: f32,
}

impl Ball {
    pub fn new(game_width: f32, game_height: f32) -> Ball {
        let mut rng = thread_rng();

        let directions = vec![1, -1];

        let direction_die = Uniform::new_inclusive(0, directions.len() - 1);

        Ball {
            x: game_width / 2.0 - 1.0,
            y: game_height / 2.0,
            radius: BALL_RADIUS,
            dx: BALL_SPEED * directions[direction_die.sample(&mut rng)] as f32,
            dy: BALL_SPEED * directions[direction_die.sample(&mut rng)] as f32,
        }
    }
}
