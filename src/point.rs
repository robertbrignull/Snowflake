#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub const ZERO: Point = Point { x: 0.0, y: 0.0 };

    pub fn distance(&self, other_point: Point) -> f64 {
        return self.distance_2(other_point).sqrt();
    }

    pub fn distance_2(&self, other_point: Point) -> f64 {
        return (self.x - other_point.x).powf(2.0) + (self.y - other_point.y).powf(2.0);
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        return write!(f, "({}, {})", self.x, self.y);
    }
}
