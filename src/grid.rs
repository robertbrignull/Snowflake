use crate::data::Point;

pub trait Grid {
    fn add_points(&mut self, num_points: i32) {
        for _i in 0..num_points {
            self.add_point();
        }
    }

    fn add_point(&mut self);

    fn list_points(&self) -> Vec<Point>;
}
