use anyhow::{Context, Result};

use crate::flake::Flake;
use crate::point::Point;

pub struct CoverTree {
    points: Vec<Point>,
    farthest_distance: f64,
}

impl CoverTree {
    pub fn from_flake(flake: &Flake) -> Result<CoverTree> {
        let mut points = flake.get_points().context("Unable to get flake points")?;
        if points.len() == 0 {
            points.push(Point::ZERO);
        }

        let mut farthest_distance = 0.0;
        for point in &points {
            let d = point.distance(Point::ZERO);
            if d > farthest_distance {
                farthest_distance = d;
            }
        }

        return Result::Ok(CoverTree {
            points,
            farthest_distance,
        });
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);

        let d = point.distance(Point::ZERO);
        if d > self.farthest_distance {
            self.farthest_distance = d;
        }
    }

    pub fn get_nearest_point(&self, point: Point) -> Point {
        let mut nearest_point = self.points[0];
        let mut min_distance_2 = nearest_point.distance_2(point);
        for point_2 in &self.points[1..] {
            let d = point_2.distance_2(point);
            if d < min_distance_2 {
                nearest_point = *point_2;
                min_distance_2 = d;
            }
        }
        return nearest_point;
    }

    pub fn get_farthest_distance(&self) -> f64 {
        return self.farthest_distance;
    }
}
