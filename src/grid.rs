use std::time::Instant;

use crate::data::Point;

pub trait Grid {
    /** Add multiple points to the flake */
    fn add_points(&mut self, num_points: u32) {
        let mut t = Instant::now();
        let start_num_points = self.get_num_points();
        while self.get_num_points() < start_num_points + num_points {
            self.add_point();

            let now = Instant::now();
            if now.duration_since(t).as_millis() > 500 {
                t = now;
                println!("Num points = {}", self.get_num_points());
            }
        }
    }

    /** Add a single point to the flake */
    fn add_point(&mut self);

    /** Return the number of points currently in the flake */
    fn get_num_points(&self) -> u32;

    /** List all the points in the flake */
    fn list_points(&self) -> Vec<Point>;
}
