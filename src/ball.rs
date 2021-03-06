use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

const BALL_RADIUS: f32 = 8.0;
const BALL_SPEED: f32 = 500.0;
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub radius: f32,
}

impl Ball {
    pub fn new(game_width: f32, game_height: f32) -> Ball {
        let mut rng = thread_rng();

        let x = game_width / 2.0;
        let y = game_height / 2.0;

        let directions = vec![1, -1];

        let direction_die = Uniform::new_inclusive(0, directions.len() - 1);

        let dx = BALL_SPEED * directions[direction_die.sample(&mut rng)] as f32;
        let dy = BALL_SPEED * directions[direction_die.sample(&mut rng)] as f32;

        Ball {
            x,
            y,
            dx,
            dy,
            radius: BALL_RADIUS,
        }
    }
}
