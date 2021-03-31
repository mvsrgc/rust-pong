const PADDLE_WIDTH: f32 = 12.0;
const PADDLE_HEIGHT: f32 = 96.0;
const PADDLE_SPEED: f32 = 350.0;

use crate::pong::Side;

pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub speed: f32,
    pub direction: f32,
}

impl Paddle {
    pub fn new(game_width: f32, game_height: f32, side: Side) -> Paddle {
        Paddle {
            x: match side {
                Side::Left => 0.0,
                Side::Right => game_width - PADDLE_WIDTH,
            },
            y: ((game_height - PADDLE_HEIGHT) / 2.0),
            w: PADDLE_WIDTH,
            h: PADDLE_HEIGHT,
            speed: PADDLE_SPEED,
            direction: 0.0,
        }
    }
}
