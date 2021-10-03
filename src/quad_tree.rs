use anyhow::{Context, Result};

use crate::flake::Flake;
use crate::point::Point;

pub struct QuadTree {
    points: Vec<Point>,
    farthest_distance: f64,
}

impl QuadTree {
    pub fn from_flake(flake: &Flake) -> Result<QuadTree> {
        let points = flake.get_points().context("Unable to get flake points")?;

        let mut farthest_distance: f64 = 0.0;
        for point in &points {
            farthest_distance = farthest_distance.max(point.distance(Point::ZERO));
        }

        return Result::Ok(QuadTree {
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

    pub fn get_nearest(&self, point: Point) -> (Point, f64) {
        let mut nearest_point = self.points[0];
        let mut min_distance_2 = nearest_point.distance_2(point);
        for point_2 in &self.points[1..] {
            let d = point_2.distance_2(point);
            if d < min_distance_2 {
                nearest_point = *point_2;
                min_distance_2 = d;
            }
        }
        return (nearest_point, min_distance_2.sqrt());
    }

    pub fn get_farthest_distance(&self) -> f64 {
        return self.farthest_distance;
    }
}

#[cfg(test)]
mod tests {
    use rand::{Rng,SeedableRng};
    use rand::rngs::StdRng;

    use super::QuadTree;
    use crate::flake::Flake;
    use crate::point::Point;
    use crate::test_utils::test::{time_func, with_test_dir};

    #[test]
    fn is_empty() {
        with_test_dir(|test_dir: &str| {
            let flake = Flake::new(&format!("{}/test.flake", test_dir));
            let mut tree = QuadTree::from_flake(&flake).expect("Unable to make flake");
            assert_eq!(true, tree.is_empty());

            tree.add_point(Point::ZERO);
            assert_eq!(false, tree.is_empty());
        });
    }

    #[test]
    fn get_nearest() {
        with_test_dir(|test_dir: &str| {
            let flake = Flake::new(&format!("{}/test.flake", test_dir));
            let mut tree = QuadTree::from_flake(&flake).expect("Unable to make flake");

            tree.add_point(Point::ZERO);
            assert_eq!(0.0, tree.get_nearest(Point::ZERO).1);
            assert_eq!(1.0, tree.get_nearest(Point { x: 1.0, y: 0.0 }).1);
            assert_eq!(4.0, tree.get_nearest(Point { x: 0.0, y: 4.0 }).1);

            tree.add_point(Point { x: 0.0, y: 1.0 });
            assert_eq!(0.0, tree.get_nearest(Point::ZERO).1);
            assert_eq!(5.0, tree.get_nearest(Point { x: 5.0, y: 1.0 }).1);
            assert_eq!(6.0, tree.get_nearest(Point { x: 6.0, y: 0.0 }).1);

            tree.add_point(Point { x: 10.0, y: 0.0 });
            assert_eq!(0.0, tree.get_nearest(Point::ZERO).1);
            assert_eq!(5.0, tree.get_nearest(Point { x: 5.0, y: 1.0 }).1);
            assert_eq!(4.0, tree.get_nearest(Point { x: 6.0, y: 0.0 }).1);
        });
    }

    #[test]
    fn get_farthest_distance() {
        with_test_dir(|test_dir: &str| {
            let flake = Flake::new(&format!("{}/test.flake", test_dir));
            let mut tree = QuadTree::from_flake(&flake).expect("Unable to make flake");

            tree.add_point(Point::ZERO);
            assert_eq!(0.0, tree.get_farthest_distance());

            tree.add_point(Point { x: 0.0, y: 1.0 });
            assert_eq!(1.0, tree.get_farthest_distance());

            tree.add_point(Point { x: 10.0, y: 0.0 });
            assert_eq!(10.0, tree.get_farthest_distance());
        });
    }

    /*
     * Last recorded performance:
     *
     * Time to add 100000 points: 962.388µs
     * Time per point: 9ns
     * 
     * Time to query 100000 points: 8.056774258s
     * Time per point: 80.567µs
     */
    #[test]
    #[ignore]
    fn insertion_and_query_100000_perf() {
        with_test_dir(|test_dir: &str| {
            let flake = Flake::new(&format!("{}/test.flake", test_dir));
            let mut tree = QuadTree::from_flake(&flake).expect("Unable to make flake");

            let mut rng = StdRng::seed_from_u64(17);

            let num_points = 100000;
            let mut points: Vec<Point> = Vec::new();
            for _i in 0..num_points {
                points.push(Point {
                    x: rng.gen_range(-1000.0..1000.0),
                    y: rng.gen_range(-1000.0..1000.0),
                });
            }
            
            let adding_time = time_func(|| {
                for point in points {
                    tree.add_point(point);
                }
            });
            println!("Time to add {} points: {:?}", num_points, adding_time);
            println!("Time per point: {:?}", adding_time / num_points);
            println!();

            let num_query_points = 100000;
            let mut query_points: Vec<Point> = Vec::new();
            for _i in 0..num_query_points {
                query_points.push(Point {
                    x: rng.gen_range(-1000.0..1000.0),
                    y: rng.gen_range(-1000.0..1000.0),
                });
            }

            let querying_time = time_func(|| {
                for query_point in query_points {
                    assert!(tree.get_nearest(query_point).1 > 0.0);
                }
            });
            println!("Time to query {} points: {:?}", num_query_points, querying_time);
            println!("Time per point: {:?}", querying_time / num_query_points);
            println!();
        });
    }
}
