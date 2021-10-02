#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        return write!(f, "({}, {})", self.x, self.y);
    }
}