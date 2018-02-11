use complex::complex::*;
use complex::Point2D;

/// A point in the screen
#[derive(Debug)]
pub struct ScreenPoint2D {
    pub x: f64,
    pub y: f64
}

impl ScreenPoint2D {
    pub fn new(x: f64, y: f64) -> ScreenPoint2D { ScreenPoint2D { x, y } }
    pub fn to_point2d(&self, width: f64, height: f64) -> Point2D {
        Point2D::new(self.x / width, self.y / height)
    }
}

impl Complex for ScreenPoint2D {
    fn x(&self) -> f64 { self.x }
    fn y(&self) -> f64 { self.y }
}

