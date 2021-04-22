const PADDLE_WIDTH: f32 = 12.0;
const PADDLE_HEIGHT: f32 = 75.0;
const PADDLE_SPEED: f32 = 350.0;

use ggez::graphics::Rect;

use crate::pong::Side;

pub struct Paddle {
    pub rect: Rect,
    pub speed: f32,
    pub direction: f32,
}

impl Paddle {
    pub fn new(game_width: f32, game_height: f32, side: Side) -> Paddle {
        let x = match side {
            Side::Left => 0.0,
            Side::Right => game_width - PADDLE_WIDTH,
        };

        let y = (game_height - PADDLE_HEIGHT) / 2.0;
        let w = PADDLE_WIDTH;
        let h = PADDLE_HEIGHT;

        Paddle {
            speed: PADDLE_SPEED,
            direction: 0.0,
            rect: Rect::new(x, y, w, h),
        }
    }
}
