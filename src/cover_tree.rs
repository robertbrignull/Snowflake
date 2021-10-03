use anyhow::{Context, Result};

use crate::flake::Flake;
use crate::point::Point;

pub struct CoverTree {
    points: Vec<Point>,
    farthest_distance: f64,
}

impl CoverTree {
    pub fn from_flake(flake: &Flake) -> Result<CoverTree> {
        let points = flake.get_points().context("Unable to get flake points")?;

        let mut farthest_distance: f64 = 0.0;
        for point in &points {
            farthest_distance = farthest_distance.max(point.distance(Point::ZERO));
        }

        return Result::Ok(CoverTree {
            points,
            farthest_distance,
        });
    }

    pub fn is_empty(&self) -> bool {
        return self.points.len() == 0;
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
        self.farthest_distance = self.farthest_distance.max(point.distance(Point::ZERO));
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
