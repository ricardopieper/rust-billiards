use utils::ord_nan::OrdNan;
use piston::input::RenderArgs;

pub trait Complex {
    fn x(&self) -> f64;
    fn y(&self) -> f64;

    fn find_closest<'a>(&'a self, points: &'a[Self]) -> &'a Self where Self: Sized {
        let min = points.iter().min_by_key(|p| {
            let candidate_point = *p;
            OrdNan::new(distance_between(self, candidate_point))
        });
        min.unwrap()
    }

    //@TODO create a orientation enum so we can match instead of doing ifs

    fn is_above(&self, other: &Self) -> bool {
        self.y() > other.y()
    }

    fn is_below(&self, other: &Self) -> bool {
        self.y() > other.y()
    }

    fn is_vertically_aligned(&self, other: &Self) -> bool {
        self.y() == other.y()
    }

    fn is_at_right(&self, other: &Self) -> bool {
        self.x() > other.x()
    }

    fn is_at_left(&self, other: &Self) -> bool {
        self.x() < other.x()
    }

    fn is_horizontally_aligned(&self, other: &Self) -> bool {
        self.x() == other.x()
    }

}

pub fn distance_between<T>(p1: &T, p2: &T) -> f64 where T: Complex {
    let (dx, dy) = (p1.x() - p2.x(), p1.y() - p2.y());
    let distances_sqr = dx.powi(2) + dy.powi(2);
    distances_sqr.sqrt()
}

//@TODO: Remove this if not useful
/*
pub fn find_closest<'a, T>(point: &'a T, points: &'a[T]) -> &'a T where T: Complex {
    let min = points.iter().min_by_key(|p| {
        let candidate_point = *p;
        OrdNan::new(distance_between(point, candidate_point))
    });
    min.unwrap()
}*/

#[derive(Debug)]
pub struct Point2D {
    pub x: f64,
    pub y: f64
}
pub type Vector2D = Point2D;

#[derive(Debug)]
pub struct ScreenPoint2D {
    pub x: f64,
    pub y: f64
}

impl Complex for Point2D {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }
}

impl Complex for ScreenPoint2D {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }
}

impl Point2D {
    pub fn scale(&self, width: f64, height: f64) -> Point2D {
        Point2D {
            x: self.x * width,
            y: self.y * height
        }
    }

    pub fn scale_to_args(&self, args: &RenderArgs) -> ScreenPoint2D {
        let point = self.scale(args.width as f64, args.height as f64);
        point.to_screen_point()
    }

    pub fn to_screen_point(&self) -> ScreenPoint2D {
        ScreenPoint2D {
            x: self.x,
            y: self.y
        }
    }
}