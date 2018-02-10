use utils::ord_nan::OrdNan;

pub trait Complex {
    fn x(&self) -> f64;
    fn y(&self) -> f64;

    fn distance_between(&self, p2: &Self) -> f64 {
        let (dx, dy) = (self.x() - p2.x(), self.y() - p2.y());
        let distances_sqr = dx.powi(2) + dy.powi(2);
        distances_sqr.sqrt()
    }

    fn find_closest<'a>(&'a self, points: &'a [Self]) -> &'a Self where Self: Sized {
        let min = points.iter().min_by_key(|p| {
            let candidate_point = *p;
            OrdNan::new(self.distance_between(candidate_point))
        });
        min.unwrap()
    }

    fn find_farthest<'a>(&'a self, points: &'a [Self]) -> &'a Self where Self: Sized {
        let min = points.iter().max_by_key(|p| {
            let candidate_point = *p;
            OrdNan::new(self.distance_between(candidate_point))
        });
        min.unwrap()
    }

    //@TODO create a orientation enum so we can match instead of doing ifs

    fn is_above(&self, other: &Self) -> bool {        self.y() > other.y()    }
    fn is_below(&self, other: &Self) -> bool {        self.y() > other.y()    }
    fn is_vertically_aligned(&self, other: &Self) -> bool {        self.y() == other.y()    }

    fn is_at_right(&self, other: &Self) -> bool {        self.x() > other.x()    }
    fn is_at_left(&self, other: &Self) -> bool {        self.x() < other.x()    }
    fn is_horizontally_aligned(&self, other: &Self) -> bool {        self.x() == other.x()    }
}


