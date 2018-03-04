use complex::*;
use ui::*;
use std::time::*;
use geometry::*;

pub const MIN_SPEED: f64 = 0.00018;
pub const DECELERATION: f64 = 0.992;
pub const DECELERATION_IMPACT_WALL: f64 = 0.95;
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
        println!("cueball position: {:?}", self.cueball.position);
        println!("cueball stopped: {:?}", self.cueball.is_stopped());

        let balls_impact_check = self.balls.clone();

        Pool::move_ball(&mut self.cueball);
        Pool::impact_against_wall(&mut self.cueball);
        Pool::impact_against_other_balls(&mut self.cueball, balls_impact_check.as_slice());

        for ball in &mut self.balls {
            let all_except_self = balls_impact_check.iter()
                .filter(|b| (*b).number != ball.number)
                .map(|b| *b)
                .collect::<Vec<Ball>>();

            Pool::move_ball(ball);
            Pool::impact_against_wall(ball);
            Pool::impact_against_other_balls(ball, all_except_self.as_slice());
        }
    }

    pub fn move_ball(ball: &mut Ball) {
        if !ball.is_stopped() {
            ball.position.x += ball.speed.x;
            ball.position.y += ball.speed.y;
            if ball.speed.magnitude().abs() <= MIN_SPEED {
                ball.speed.x = 0.0;
                ball.speed.y = 0.0;
            } else {
                ball.speed = ball.speed.multiply(DECELERATION);
            }
        }
    }

    pub fn impact_against_wall(ball: &mut Ball) {
        if !ball.is_stopped() {
            let mut impact_happened = false;
            let radius = ball.radius;
            if ball.position.x - radius <= 0.0 {
                ball.position.x = radius;
                ball.speed.x *= -1.0;
                impact_happened = true;
            }
            if ball.position.x + radius >= 1.0 {
                ball.position.x = 1.0 - radius;
                ball.speed.x *= -1.0;
                impact_happened = true;
            }
            if ball.position.y - radius <= 0.0 {
                ball.position.y = radius;
                ball.speed.y *= -1.0;
                impact_happened = true;
            }
            if ball.position.y + radius >= 0.5 {
                ball.position.y = 0.5 - radius;
                ball.speed.y *= -1.0;
                impact_happened = true;
            }

            if impact_happened {
                ball.speed = ball.speed.multiply(DECELERATION_IMPACT_WALL);
            }
        }
    }

    pub fn impact_against_other_balls(ball: &mut Ball, other_balls: &[Ball]) {}
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

#[derive(Copy, Clone)]
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