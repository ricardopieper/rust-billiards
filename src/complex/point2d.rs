use complex::*;

/// A type that represents a space between (0,0) and (1,1)
#[derive(Debug)]
pub struct Point2D {
    pub x: f64,
    pub y: f64
}

pub type Vector2D = Point2D;

impl Point2D {
    pub fn new(x: f64, y: f64) -> Point2D { Point2D { x, y } }

    pub fn to_screen_point(&self, width: f64, height: f64) -> ScreenPoint2D {
        let point = Point2D::new(self.x * width, self.y * height);
        point.as_screen_point()
    }

    pub fn as_screen_point(&self) -> ScreenPoint2D { ScreenPoint2D::new(self.x, self.y) }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Vector2D {
        let m = self.magnitude();

        Vector2D::new(self.x / m, self.y / m)
    }

    pub fn divide(&self, div: f64) -> Vector2D {
        Vector2D::new(self.x / div, self.y / div)
    }

    pub fn multiply(&self, mult: f64) -> Vector2D {
        Vector2D::new(self.x * mult, self.y * mult)
    }

    pub fn is_zero(&self) -> bool {
        self.x() == 0.0 && self.y() == 0.0
    }
}

impl Complex for Point2D {
    fn x(&self) -> f64 { self.x }
    fn y(&self) -> f64 { self.y }
}

