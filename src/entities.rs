use complex::*;
use ui::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Pool {
    pub cueball: Ball,
    pub balls: Vec<Ball>,
    pub pockets: Vec<Pocket>,
    pub mouse_pos: ScreenPoint2D,
    pub window_width: f64,
    pub window_height: f64,
}

impl Pool {
    pub fn update(&mut self) {

        if !self.cueball.is_stopped() {
            self.cueball.position.x += self.cueball.speed.x;
            self.cueball.position.y += self.cueball.speed.y;

            let ups = 120.0;
            let five_sec = 5.0 * ups;

            let speed_loss = (1.0 / five_sec);

            if self.cueball.speed.magnitude().abs() <= speed_loss {
                self.cueball.speed.x = 0.0;
                self.cueball.speed.y = 0.0;

                let time = SystemTime::now().duration_since(UNIX_EPOCH);

                match time {
                    Ok(n) => println!("stopped at {:?}", n.as_secs()),
                    Err(_) => println!("fail")
                }

            } else {
                self.cueball.speed = self.cueball.speed.multiply(1.0 - speed_loss);
            }
            if self.cueball.position.x <= 0.0 || self.cueball.position.x >= 1.0 {
                self.cueball.speed.x *= -1.0;
            }

            if self.cueball.position.y <= 0.0 || self.cueball.position.y >= 1.0 {
                self.cueball.speed.y *= -1.0;
            }
        }

    }

    pub fn set_mouse_pos(&mut self, mouse_pos: [f64; 2]) {
        self.mouse_pos = ScreenPoint2D::new(
            mouse_pos[0],
            mouse_pos[1]
        )
    }

    pub fn stun_shot(&mut self) {
        let ball_pos = &self
            .cueball.position;

        let mouse_pos = &self.mouse_pos.canonical(self.window_width, self.window_height);

        let vector = CueLine::get_shot_vector(mouse_pos, ball_pos);

        self.cueball.speed = vector.divide(100.0);

        let time = SystemTime::now().duration_since(UNIX_EPOCH);

        match time {
            Ok(n) => println!("Shot at {:?}", n.as_secs()),
            Err(_) => println!("fail")
        }
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