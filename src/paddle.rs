use ggez::graphics::Rect;

use crate::pong::Side;

const PADDLE_WIDTH: f32 = 12.0;
const PADDLE_HEIGHT: f32 = 75.0;
const PADDLE_SPEED: f32 = 450.0;

pub struct Paddle {
    pub rect: Rect,
    pub side: Side,
    pub dy: f32,
    pub is_up_holding: bool,
    pub is_down_holding: bool,
}

impl Paddle {
    pub fn new(game_width: f32, game_height: f32, side: Side) -> Paddle {
        let x = match side {
            Side::Left => 0.0,
            Side::Right => game_width - PADDLE_WIDTH,
            _ => panic!("Paddles go on the left or right walls."),
        };

        let y = (game_height - PADDLE_HEIGHT) / 2.0;
        let w = PADDLE_WIDTH;
        let h = PADDLE_HEIGHT;

        Paddle {
            rect: Rect::new(x, y, w, h),
            side,
            dy: PADDLE_SPEED,
            is_up_holding: false,
            is_down_holding: false,
        }
    }
}
