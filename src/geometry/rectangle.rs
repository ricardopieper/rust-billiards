use complex::Point2D;

pub struct Rectangle {
    pub origin: Point2D,
    pub vertical_size: f64,
    pub horizontal_size: f64
}

impl Rectangle {
    pub fn is_inside(&self, point: &Point2D) -> bool {
        point.y >= self.up()
            && point.y <= self.down()
            && point.x >= self.left()
            && point.x <= self.right()
    }

    pub fn left(&self) -> f64 {
        self.origin.x
    }
    pub fn right(&self) -> f64 {
        self.origin.x + self.horizontal_size
    }
    pub fn up(&self) -> f64 {
        self.origin.y
    }
    pub fn down(&self) -> f64 {
        self.origin.y + self.vertical_size
    }
}