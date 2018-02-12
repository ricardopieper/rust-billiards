use complex::*;
use ui::*;
use std::time::{SystemTime, UNIX_EPOCH};
use geometry::*;

pub const MIN_SPEED: f64 = 0.00018;
pub const DECELERATION: f64 = 0.992;
pub const DECELERATION_IMPACT: f64 = 0.95;
pub const SPEED_RATIO: f64 = 30.0;

pub struct Pool {
    pub cueball: Ball,
    pub balls: Vec<Ball>,
    pub pockets: Vec<Pocket>,
    pub mouse_pos: ScreenPoint2D,
    pub play_area: ScreenRectangle
}

impl Pool {
    pub fn update(&mut self) {
        if !self.cueball.is_stopped() {
            self.cueball.position.x += self.cueball.speed.x;
            self.cueball.position.y += self.cueball.speed.y;

            if self.cueball.speed.magnitude().abs() <= MIN_SPEED {
                self.cueball.speed.x = 0.0;
                self.cueball.speed.y = 0.0;
            } else {
                self.cueball.speed = self.cueball.speed.multiply(DECELERATION);
            }

            if self.cueball.position.x <= 0.0 {
                self.cueball.position.x = 0.0;
                self.cueball.speed.x *= -1.0;
            }

            if self.cueball.position.x >= 1.0 {
                self.cueball.position.x = 1.0;
                self.cueball.speed.x *= -1.0;
            }

            if self.cueball.position.y <= 0.0 {
                self.cueball.position.y = 0.0;
                self.cueball.speed.y *= -1.0;
            }

            //only half of the space is usable
            if self.cueball.position.y >= 0.5 {
                self.cueball.position.y = 0.5;
                self.cueball.speed.y *= -1.0;
            }

            if self.cueball.position.y <= 0.0 || self.cueball.position.y >= 0.5 ||
                self.cueball.position.x <= 0.0 || self.cueball.position.x >= 1.0 {
                self.cueball.speed = self.cueball.speed.multiply(DECELERATION_IMPACT);
            }
        }
    }

    pub fn mouse_table_position(&self) -> Point2D {
        let mouse_pos_relative = ScreenPoint2D {
            x: self.mouse_pos.x - self.play_area.origin.x,
            y: (self.mouse_pos.y - self.play_area.origin.y) / 2.0,
        };
        mouse_pos_relative.to_point2d(self.play_area.width, self.play_area.height)
    }

    pub fn set_mouse_pos(&mut self, mouse_pos: [f64; 2]) {
        self.mouse_pos = ScreenPoint2D::new(
            mouse_pos[0],
            mouse_pos[1]
        )
    }

    pub fn stun_shot(&mut self) {
        let ball_pos = &self.cueball.position;

        let mouse_pos_2d = self.mouse_table_position();

        let vector = CueLine::get_shot_vector(&mouse_pos_2d, ball_pos);

        self.cueball.speed = vector.divide(SPEED_RATIO);
    }
}

pub struct Ball {
    pub position: Point2D,
    pub speed: Vector2D,
    pub number: i32,
    pub radius: f64
}

impl Ball {
    pub fn is_stopped(&self) -> bool {
        self.speed.is_zero()
    }
}

pub struct Circle<T> where T: Complex + Sized {
    pub position: T,
    pub radius: f64
}

pub type Pocket = Circle<Point2D>;

pub struct Line {
    pub slope: f64,
    pub y_intercept: f64
}

impl Line {
    pub fn y(&self, x: f64) -> f64 {
        (self.slope * x) + self.y_intercept
    }
}