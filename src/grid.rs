use std::time::Instant;

use crate::data::Point;

pub trait Grid {
    /** Add multiple points to the flake */
    fn add_points(&mut self, num_points: u32) {
        let mut t = Instant::now();
        for i in 0..num_points {
            self.add_point();

            let now = Instant::now();
            if now.duration_since(t).as_millis() > 500 {
                t = now;
                println!("Added {} points", i);
            }
        }
    }

    /** Add a single point to the flake */
    fn add_point(&mut self);

    /** List all the points in the flake */
    fn list_points(&self) -> Vec<Point>;
}
