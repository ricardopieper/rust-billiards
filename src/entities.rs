use complex::*;
use ui::*;
use std::time::*;
use geometry::*;
use physics::impact_event::*;
use std::cell::Cell;

pub const MIN_SPEED: f64 = 0.00017;
pub const DECELERATION: f64 = 0.992;
pub const DECELERATION_IMPACT_WALL: f64 = 0.95;
pub const SPEED_RATIO: f64 = 30.0;

pub struct Pool {
    pub ball_positions: BallPositions,
    pub pockets: Vec<Pocket>,
    pub mouse_pos: ScreenPoint2D,
    pub play_area: ScreenRectangle
}

pub struct BallPositions {
    pub cueball: Ball,
    pub balls: Vec<Ball>
}

pub enum WallImpact {
    None,
    Top,
    Left,
    Right,
    Bottom
}

impl BallPositions {
    pub fn tick(&self) -> BallPositions {
        BallPositions {
            cueball: BallPositions::calculate_ball(&self.cueball),
            balls: BallPositions::calculate_every_ball(&self.balls)
        }
    }

    pub fn calculate_new_position(position: &Vector2D, speed: &Vector2D) -> Vector2D {
        if !speed.is_zero() {
            Vector2D {
                x: position.x + speed.x,
                y: position.y + speed.y
            }
        } else {
            *position
        }
    }

    pub fn calculate_new_speed(speed: &Vector2D) -> Vector2D {
        if speed.magnitude().abs() <= MIN_SPEED {
            Vector2D::new(0.0, 0.0)
        } else {
            speed.multiply(DECELERATION)
        }
    }

    pub fn calculate_ball(ball: &Ball) -> Ball {
        let mut new_ball = Ball {
            position: BallPositions::calculate_new_position(&ball.position, &ball.speed),
            speed: BallPositions::calculate_new_speed(&ball.speed),
            number: ball.number,
            radius: ball.radius
        };

        let impact = BallPositions::check_impact_wall(&new_ball);

        match impact {
            WallImpact::Left | WallImpact::Right => {
                new_ball.speed.x *= -1.0;
            }
            WallImpact::Top | WallImpact::Bottom => {
                new_ball.speed.y *= -1.0;
            }
            WallImpact::None => {}
        };

        let radius = new_ball.radius;

        match impact {
            WallImpact::Left => {
                new_ball.position.x = radius;
            }
            WallImpact::Right => {
                new_ball.position.x = 1.0 - radius;
            }
            WallImpact::Top => {
                new_ball.position.y = radius;
            }
            WallImpact::Bottom => {
                new_ball.position.y = 0.5 - radius;
            }
            WallImpact::None => {}
        };
        new_ball
    }

    pub fn calculate_every_ball(balls: &[Ball]) -> Vec<Ball> {
        balls.iter().map(|b|
            BallPositions::calculate_ball(b)
        ).collect::<Vec<Ball>>()
    }

    pub fn check_impact_wall(ball: &Ball) -> WallImpact {
        if !ball.is_stopped() {
            let radius = ball.radius;
            if ball.position.x - radius <= 0.0 {
                WallImpact::Left
            } else if ball.position.x + radius >= 1.0 {
                WallImpact::Right
            } else if ball.position.y - radius <= 0.0 {
                WallImpact::Top
            } else if ball.position.y + radius >= 0.5 {
                WallImpact::Bottom
            } else {
                WallImpact::None
            }
        } else {
            WallImpact::None
        }
    }

    fn balls_overlap(ball_a: &Ball, ball_b: &Ball) -> bool {
        let distance_between_centers = ball_a.position.distance_between(&ball_b.position);
        let sum_of_radius = ball_a.radius + ball_b.radius;
        sum_of_radius >= distance_between_centers
    }
}

impl Pool {
    pub fn update(&mut self) {
        let mut current_pos = self.ball_positions.tick();
        self.ball_positions = current_pos;
        self.check_cueball_impact();
    }

    pub fn check_cueball_impact(&mut self) {
        let cueball = self.ball_positions.cueball.clone();

        let impacts = self.ball_positions.balls.as_slice()
            .iter()
            .filter(|b| Pool::balls_overlap(&cueball, *b))
            .map(|b| ImpactEvent::new(&cueball, b))
            .collect::<Vec<ImpactEvent>>();

        for impact in impacts {
            println!("impact happened: {:?}", impact);
            let (ball_a, ball_b) = self.get_impact_balls(&impact);

            let new_vec2d_a = Pool::get_new_speed_a(&impact,
                                                    ball_a, ball_b);

            let new_vec2d_b = Pool::get_new_speed_b(&impact,
                                                    ball_a, ball_b);

            ball_a.speed = new_vec2d_a;
            ball_b.speed = new_vec2d_b;

            Pool::move_ball(ball_a);
            Pool::move_ball(ball_b);
            println!("ball b new position: {:?}", ball_b);
            println!("ball a new position: {:?}", ball_a);
        }
    }

    pub fn get_new_speed_a(impact: &ImpactEvent,
                           ball_a: &Ball, ball_b: &Ball) -> Vector2D {
        let with_lost_energy = &ball_a.speed.minus(
            &ball_a.speed.multiply(impact.energy_transfer_a_to_b)
        );

        let gain_magnitude = ball_b.speed.multiply(impact.energy_transfer_b_to_a)
            .magnitude();

        let vector_gain = &impact.ball_a_heading.multiply(
            gain_magnitude);

        let final_result = with_lost_energy.plus(vector_gain);

        final_result
    }

    pub fn get_new_speed_b(impact: &ImpactEvent,
                           ball_a: &Ball, ball_b: &Ball) -> Vector2D {
        let with_lost_energy = &ball_b.speed.minus(
            &ball_b.speed.multiply(impact.energy_transfer_b_to_a)
        );


        let gain_magnitude = ball_a.speed.multiply(impact.energy_transfer_a_to_b)
            .magnitude();
        let vector_gain = &impact.ball_b_heading.multiply(
            gain_magnitude);

        let final_result = with_lost_energy.plus(vector_gain);


        final_result
    }

    pub fn get_impact_balls(&mut self, impact: &ImpactEvent) -> (&mut Ball, &mut Ball) {
        let index_a_opt = self.ball_positions.balls.iter()
            .position(|b| b.number == impact.ball_a);

        let index_b_opt = self.ball_positions.balls.iter()
            .position(|b| b.number == impact.ball_b);

        match index_a_opt {
            Some(index_a) => match index_b_opt {
                Some(index_b) => self.get_impact_balls_index(index_a, index_b),
                None => (self.ball_positions.balls.get_mut(index_a).unwrap(), &mut self.ball_positions.cueball)
            },
            None => match index_b_opt {
                Some(index_b) => (&mut self.ball_positions.cueball, self.ball_positions.balls.get_mut(index_b).unwrap()),
                None => panic!("nothing was found")
            }
        }
    }

    pub fn get_impact_balls_index(&mut self, index_a: usize, index_b: usize) -> (&mut Ball, &mut Ball) {
        let (first, second) = self.ball_positions.balls.split_at_mut(index_a + 1);
        let ball_a: &mut Ball = first.last_mut().unwrap();
        let ball_b: &mut Ball = second.get_mut(index_b - index_a).unwrap();
        (ball_a, ball_b)
    }

    pub fn balls_overlap(ball_a: &Ball, ball_b: &Ball) -> bool {
        let distance_between_centers = ball_a.position.distance_between(&ball_b.position);
        let sum_of_radius = ball_a.radius + ball_b.radius;
        sum_of_radius >= distance_between_centers
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
        let ball_pos = &self.ball_positions.cueball.position;
        let mouse_pos_2d = self.mouse_table_position();
        let vector = CueLine::get_shot_vector(&mouse_pos_2d, ball_pos);
        self.ball_positions.cueball.speed = vector.divide(SPEED_RATIO);
    }

    pub fn move_ball(ball: &mut Ball) {
        ball.position.x += ball.speed.x;
        ball.position.y += ball.speed.y;
    }
}

#[derive(Copy, Debug, Clone)]
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