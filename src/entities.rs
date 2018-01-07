use piston::input::RenderArgs;
use utils::ord_nan::OrdNan;

pub struct Pool {
    pub cueball: Ball,
    pub balls: Vec<Ball>,
    pub pockets: Vec<Pocket>,
    pub mouse_pos: ScreenPoint2D
}

impl Pool {
    pub fn set_mouse_pos(&mut self, mouse_pos: [f64; 2]) {
        self.mouse_pos = ScreenPoint2D {
            x: mouse_pos[0],
            y: mouse_pos[1]
        }
    }
}

pub struct Ball {
    pub position: Point2D,
    pub speed: Vector2D,
    pub number: i32,
    pub radius: f64
}

#[derive(Debug)]
pub struct Complex {
    pub x: f64,
    pub y: f64
}

pub type Point2D = Complex;
pub type Vector2D = Complex;


pub struct Circle {
    pub position: Point2D,
    pub radius: f64
}

pub type Pocket = Circle;

#[derive(Debug)]
pub struct ScreenPoint2D {
    pub x: f64,
    pub y: f64
}

impl Complex {
    pub fn scale(&self, width: f64, height: f64) -> ScreenPoint2D {
        ScreenPoint2D {
            x: self.x * width,
            y: self.y * height
        }
    }

    pub fn scale_to_args(&self, args: &RenderArgs) -> ScreenPoint2D {
        self.scale(args.width as f64, args.height as f64)
    }

    pub fn distance_between(p1: &Complex, p2: &Complex) -> f64 {
        let (dx, dy) = (p1.x - p2.x, p1.y - p2.y);

        let distances_sqr = dx.powi(2) + dy.powi(2);

        distances_sqr.sqrt()
    }

    pub fn distance_to_point(&self, other: &Complex) -> f64 {
        Complex::distance_between(self, other)
    }

    pub fn find_closest<'a>(&self, points: &'a [Complex]) -> &'a Complex {
        let min = points.iter().min_by_key(|point| OrdNan::new(point.distance_to_point(self)));
        min.unwrap()
    }

    pub fn find_farthest<'a>(&self, points: &'a [Complex]) -> &'a Complex {
        let max = points.iter().max_by_key(|point| OrdNan::new(point.distance_to_point(self)));
        max.unwrap()
    }
}

impl ScreenPoint2D {
    pub fn as_complex(&self) -> Complex {
        Complex { x: self.x, y: self.y }
    }
}