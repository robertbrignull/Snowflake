use anyhow::{Context, Result};

use crate::flake::Flake;
use crate::point::Point;

const MAX_POINTS_NODE_SIZE: usize = 50;

pub struct QuadTree {
    root: QuadTreeNode,
    farthest_distance: f64,
}

enum QuadTreeNode {
    Points(QuadTreePointsNode),
    Split(QuadTreeSplitNode),
}

struct QuadTreePointsNode {
    center: Point,
    radius: f64,
    points: Vec<Point>,
}

struct QuadTreeSplitNode {
    center: Point,
    radius: f64,
    north_west: Box<QuadTreeNode>,
    north_east: Box<QuadTreeNode>,
    south_west: Box<QuadTreeNode>,
    south_east: Box<QuadTreeNode>,
}

impl QuadTreeSplitNode {
    fn quadrants(&self) -> [&Box<QuadTreeNode>; 4] {
        return [
            &self.north_west,
            &self.north_east,
            &self.south_west,
            &self.south_east,
        ];
    }
}

impl QuadTreeNode {
    fn center(&self) -> Point {
        return match self {
            QuadTreeNode::Points(node) => node.center,
            QuadTreeNode::Split(node) => node.center,
        };
    }

    fn radius(&self) -> f64 {
        return match self {
            QuadTreeNode::Points(node) => node.radius,
            QuadTreeNode::Split(node) => node.radius,
        };
    }

    // Returns if the given point is within the bounds of this node.
    fn point_is_in_bounds(&self, point: &Point) -> bool {
        let center = self.center();
        let radius = self.radius();
        return point.x >= center.x - radius
            && point.x < center.x + radius
            && point.y >= center.y - radius
            && point.y < center.y + radius;
    }

    // Make a new and empty points node
    fn empty_points_node(center: Point, radius: f64) -> QuadTreeNode {
        return QuadTreeNode::Points(QuadTreePointsNode {
            center,
            radius,
            points: Vec::new(),
        });
    }

    // Make a new and empty split node
    fn empty_split_node(center: Point, radius: f64) -> QuadTreeNode {
        let child_radius = radius / 2.0;
        return QuadTreeNode::Split(QuadTreeSplitNode {
            center,
            radius,
            north_west: Box::new(QuadTreeNode::empty_points_node(
                Point {
                    x: center.x - child_radius,
                    y: center.y + child_radius,
                },
                child_radius,
            )),
            north_east: Box::new(QuadTreeNode::empty_points_node(
                Point {
                    x: center.x + child_radius,
                    y: center.y + child_radius,
                },
                child_radius,
            )),
            south_west: Box::new(QuadTreeNode::empty_points_node(
                Point {
                    x: center.x - child_radius,
                    y: center.y - child_radius,
                },
                child_radius,
            )),
            south_east: Box::new(QuadTreeNode::empty_points_node(
                Point {
                    x: center.x + child_radius,
                    y: center.y - child_radius,
                },
                child_radius,
            )),
        });
    }

    fn is_empty(&self) -> bool {
        return match self {
            QuadTreeNode::Points(node) => node.points.len() == 0,
            QuadTreeNode::Split(node) => {
                node.north_east.is_empty()
                    && node.north_west.is_empty()
                    && node.south_east.is_empty()
                    && node.south_west.is_empty()
            }
        };
    }

    fn add_point(&mut self, new_point: &Point) {
        if !self.point_is_in_bounds(new_point) {
            panic!(
                "Unable to add point {} that is outside of node boundaries {} / {}",
                new_point,
                self.center(),
                self.radius()
            );
        }

        match self {
            QuadTreeNode::Points(node) => {
                // Ignore the size of the points array here.
                // Due to the checks in the other branch this should never get too large.
                node.points.push(*new_point);
            }
            QuadTreeNode::Split(node) => {
                let quadrant: &mut Box<QuadTreeNode> =
                    if new_point.x < node.center.x && new_point.y >= node.center.y {
                        &mut node.north_west
                    } else if new_point.x >= node.center.x && new_point.y >= node.center.y {
                        &mut node.north_east
                    } else if new_point.x < node.center.x && new_point.y < node.center.y {
                        &mut node.south_west
                    } else {
                        &mut node.south_east
                    };
                match &**quadrant {
                    QuadTreeNode::Points(quadrant_node) => {
                        if quadrant_node.points.len() >= MAX_POINTS_NODE_SIZE {
                            let mut new_quadrant = QuadTreeNode::empty_split_node(
                                quadrant_node.center,
                                quadrant_node.radius,
                            );
                            for point in &quadrant_node.points {
                                new_quadrant.add_point(point);
                            }
                            new_quadrant.add_point(new_point);
                            **quadrant = new_quadrant;
                        } else {
                            quadrant.add_point(new_point);
                        }
                    }
                    QuadTreeNode::Split(_) => {
                        quadrant.add_point(new_point);
                    }
                };
            }
        }
    }

    fn get_nearest(&self, point: &Point) -> Option<(Point, f64)> {
        match self {
            QuadTreeNode::Points(node) => {
                if node.points.len() == 0 {
                    return Option::None;
                }
                let mut nearest = node.points[0];
                let mut nearest_distance_2 = point.distance_2(&nearest);
                for other_point in &node.points[1..] {
                    let d2 = point.distance_2(other_point);
                    if d2 < nearest_distance_2 {
                        nearest = *other_point;
                        nearest_distance_2 = d2;
                    }
                }
                return Option::Some((nearest, nearest_distance_2.sqrt()));
            }
            QuadTreeNode::Split(node) => {
                let mut result = Option::None;
                for quadrant in node.quadrants() {
                    if quadrant.point_is_in_bounds(point) {
                        result = quadrant.get_nearest(point);
                    }
                }

                for quadrant in node.quadrants() {
                    if !quadrant.point_is_in_bounds(point) {
                        let d = quadrant.distance(point);
                        if result.is_none() || d < result.unwrap().1 {
                            let r = quadrant.get_nearest(point);
                            if !r.is_none() && (result.is_none() || r.unwrap().1 < result.unwrap().1) {
                                result = r;
                            }
                        }
                    }
                }

                return result;
            }
        };
    }

    // Returns the distance from the given point to the bounds of this node.
    // Returns 0 if the point is within the bounds of this node.
    fn distance(&self, point: &Point) -> f64 {
        let center = self.center();
        let radius = self.radius();
        let dx = (center.x - point.x).abs();
        let dy = (center.y - point.y).abs();
        if dx <= radius && dy <= radius {
            return 0.0;
        }
        if dx <= radius {
            return dy - radius;
        }
        if dy <= radius {
            return dx - radius;
        }
        return ((dx - radius) * (dx - radius) + (dy - radius) * (dy - radius)).sqrt();
    }
}

impl QuadTree {
    pub fn from_flake(flake: &Flake) -> Result<QuadTree> {
        let points = flake.get_points().context("Unable to get flake points")?;

        let mut farthest_distance: f64 = 500.0;
        for point in &points {
            farthest_distance = farthest_distance.max(point.distance(&Point::ZERO));
        }

        let mut tree = QuadTree {
            root: QuadTreeNode::empty_split_node(Point::ZERO, farthest_distance * 2.0),
            farthest_distance: 0.0,
        };

        for point in &points {
            tree.add_point(point);
        }

        return Result::Ok(tree);
    }

    pub fn is_empty(&self) -> bool {
        return self.root.is_empty();
    }

    pub fn add_point(&mut self, point: &Point) {
        if !self.root.point_is_in_bounds(point) {
            panic!("Unable to add point {} that is outside of root node", point);
        }

        self.root.add_point(point);
        self.farthest_distance = self.farthest_distance.max(point.distance(&Point::ZERO));
    }

    pub fn get_nearest(&self, point: &Point) -> Option<(Point, f64)> {
        return self.root.get_nearest(point);
    }

    pub fn get_farthest_distance(&self) -> f64 {
        return self.farthest_distance;
    }
}

#[cfg(test)]
mod tests {
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    use super::QuadTree;
    use super::QuadTreeNode;
    use crate::flake::Flake;
    use crate::point::Point;
    use crate::test_utils::test::{time_func, with_test_dir};

    fn assert_close(dx: f64, dy: f64, actual: f64) {
        let expected = (dx * dx + dy * dy).sqrt();
        assert!(expected - 0.01 < actual);
        assert!(expected + 0.01 > actual);
    }

    impl QuadTreeNode {
        // Returns all points in the node.
        fn points(&self) -> Vec<Point> {
            match self {
                QuadTreeNode::Points(node) => {
                    return node.points.clone();
                }
                QuadTreeNode::Split(node) => {
                    let mut points = node.north_west.points();
                    points.append(&mut node.north_east.points());
                    points.append(&mut node.south_west.points());
                    points.append(&mut node.south_east.points());
                    return points;
                }
            };
        }

        fn print(&self, indent: usize) {
            match self {
                QuadTreeNode::Points(node) => {
                    println!(
                        "{}Points({}, {}) (",
                        " ".repeat(indent),
                        node.center,
                        node.radius
                    );
                    for point in &node.points {
                        println!("{}{}", " ".repeat(indent + 2), point);
                    }
                    println!("{})", " ".repeat(indent));
                }
                QuadTreeNode::Split(node) => {
                    println!(
                        "{}Split({}, {}) (",
                        " ".repeat(indent),
                        node.center,
                        node.radius
                    );
                    println!("{}NW (", " ".repeat(indent + 2));
                    node.north_west.print(indent + 4);
                    println!("{})", " ".repeat(indent + 2));
                    println!("{}NE (", " ".repeat(indent + 2));
                    node.north_east.print(indent + 4);
                    println!("{})", " ".repeat(indent + 2));
                    println!("{}SW (", " ".repeat(indent + 2));
                    node.south_west.print(indent + 4);
                    println!("{})", " ".repeat(indent + 2));
                    println!("{}SE (", " ".repeat(indent + 2));
                    node.south_east.print(indent + 4);
                    println!("{})", " ".repeat(indent + 2));
                    println!("{})", " ".repeat(indent));
                }
            };
        }
    }

    #[test]
    fn node_points_print() {
        let mut node = QuadTreeNode::empty_points_node(Point::ZERO, 10.0);
        node.add_point(&Point { x: -1.0, y: -1.0 });
        node.add_point(&Point { x: 1.0, y: -1.0 });
        node.add_point(&Point { x: 2.0, y: -2.0 });

        node.print(0);
    }

    #[test]
    fn node_split_print() {
        let mut node = QuadTreeNode::empty_split_node(Point::ZERO, 10.0);
        node.add_point(&Point { x: -1.0, y: -1.0 });
        node.add_point(&Point { x: 1.0, y: -1.0 });
        node.add_point(&Point { x: 2.0, y: -2.0 });
        node.add_point(&Point { x: 1.0, y: 1.0 });
        node.add_point(&Point { x: 2.0, y: 2.0 });
        node.add_point(&Point { x: 3.0, y: 3.0 });
        node.add_point(&Point { x: -1.0, y: 1.0 });
        node.add_point(&Point { x: -2.0, y: 2.0 });
        node.add_point(&Point { x: -3.0, y: 3.0 });
        node.add_point(&Point { x: -4.0, y: 4.0 });

        node.print(0);
    }

    #[test]
    fn node_points_is_empty() {
        let mut node = QuadTreeNode::empty_points_node(Point::ZERO, 10.0);
        assert_eq!(true, node.is_empty());

        node.add_point(&Point::ZERO);
        assert_eq!(false, node.is_empty());
    }

    #[test]
    fn node_split_is_empty() {
        let mut node = QuadTreeNode::empty_split_node(Point::ZERO, 10.0);
        assert_eq!(true, node.is_empty());

        node.add_point(&Point::ZERO);
        assert_eq!(false, node.is_empty());
    }

    #[test]
    fn node_point_is_in_bounds() {
        let node = QuadTreeNode::empty_points_node(Point::ZERO, 10.0);

        assert_eq!(true, node.point_is_in_bounds(&Point::ZERO));
        assert_eq!(true, node.point_is_in_bounds(&Point { x: 5.0, y: 5.0 }));

        assert_eq!(false, node.point_is_in_bounds(&Point { x: 100.0, y: 0.0 }));
        assert_eq!(false, node.point_is_in_bounds(&Point { x: 0.0, y: 50.0 }));
        assert_eq!(false, node.point_is_in_bounds(&Point { x: 20.0, y: 20.0 }));

        assert_eq!(false, node.point_is_in_bounds(&Point { x: 10.0, y: 0.0 }));
        assert_eq!(true, node.point_is_in_bounds(&Point { x: -10.0, y: 0.0 }));
        assert_eq!(false, node.point_is_in_bounds(&Point { x: 0.0, y: 10.0 }));
        assert_eq!(true, node.point_is_in_bounds(&Point { x: 0.0, y: -10.0 }));
    }

    #[test]
    fn node_distance() {
        let node = QuadTreeNode::empty_points_node(Point::ZERO, 10.0);

        assert_eq!(0.0, node.distance(&Point::ZERO));
        assert_eq!(0.0, node.distance(&Point { x: 10.0, y: 0.0 }));
        assert_eq!(0.0, node.distance(&Point { x: 10.0, y: 10.0 }));

        assert_eq!(10.0, node.distance(&Point { x: 20.0, y: 0.0 }));
        assert_eq!(10.0, node.distance(&Point { x: 20.0, y: 10.0 }));

        assert_close(10.0, 10.0, node.distance(&Point { x: 20.0, y: 20.0 }));
    }

    #[test]
    fn node_points_points() {
        let mut node = QuadTreeNode::empty_points_node(Point::ZERO, 10.0);
        assert_eq!(true, node.points().is_empty());

        for i in 0..8 {
            node.add_point(&Point {
                x: i as f64,
                y: i as f64,
            });
        }
        let points = node.points();
        assert_eq!(8, points.len());
    }

    #[test]
    fn node_split_points() {
        let mut node = QuadTreeNode::empty_split_node(Point::ZERO, 10.0);
        node.add_point(&Point { x: -1.0, y: -1.0 });
        node.add_point(&Point { x: 1.0, y: -1.0 });
        node.add_point(&Point { x: 2.0, y: -2.0 });
        node.add_point(&Point { x: 1.0, y: 1.0 });
        node.add_point(&Point { x: 2.0, y: 2.0 });
        node.add_point(&Point { x: 3.0, y: 3.0 });
        node.add_point(&Point { x: -1.0, y: 1.0 });
        node.add_point(&Point { x: -2.0, y: 2.0 });
        node.add_point(&Point { x: -3.0, y: 3.0 });
        node.add_point(&Point { x: -4.0, y: 4.0 });
        let points = node.points();
        assert_eq!(10, points.len());
    }

    #[test]
    fn node_points_get_nearest() {
        let mut node = QuadTreeNode::empty_points_node(Point::ZERO, 10.0);
        assert_eq!(true, node.get_nearest(&Point::ZERO).is_none());

        node.add_point(&Point::ZERO);

        assert_eq!(0.0, node.get_nearest(&Point::ZERO).unwrap().1);
        assert_eq!(1.0, node.get_nearest(&Point { x: 1.0, y: 0.0 }).unwrap().1);

        node.add_point(&Point { x: 8.0, y: 0.0 });

        assert_eq!(1.0, node.get_nearest(&Point { x: 1.0, y: 0.0 }).unwrap().1);
        assert_eq!(2.0, node.get_nearest(&Point { x: 6.0, y: 0.0 }).unwrap().1);
        assert_eq!(2.0, node.get_nearest(&Point { x: 10.0, y: 0.0 }).unwrap().1);
    }

    #[test]
    fn node_split_get_nearest_one_zone() {
        let mut node = QuadTreeNode::empty_split_node(Point::ZERO, 10.0);
        node.add_point(&Point { x: 1.0, y: 0.0 });

        assert_eq!(1.0, node.get_nearest(&Point { x: 1.0, y: 1.0 }).unwrap().1);
        assert_eq!(3.0, node.get_nearest(&Point { x: 4.0, y: 0.0 }).unwrap().1);
        assert_eq!(5.0, node.get_nearest(&Point { x: -4.0, y: 0.0 }).unwrap().1);
    }

    #[test]
    fn node_split_get_nearest_two_zones() {
        let mut node = QuadTreeNode::empty_split_node(Point::ZERO, 10.0);
        node.add_point(&Point { x: 4.0, y: 0.0 });
        node.add_point(&Point { x: -8.0, y: 0.0 });

        assert_eq!(1.0, node.get_nearest(&Point { x: 5.0, y: 0.0 }).unwrap().1);
        assert_eq!(2.0, node.get_nearest(&Point { x: 2.0, y: 0.0 }).unwrap().1);
        assert_eq!(5.0, node.get_nearest(&Point { x: -1.0, y: 0.0 }).unwrap().1);
        assert_eq!(3.0, node.get_nearest(&Point { x: -5.0, y: 0.0 }).unwrap().1);
        assert_eq!(6.0, node.get_nearest(&Point { x: 4.0, y: 6.0 }).unwrap().1);
        assert_eq!(6.0, node.get_nearest(&Point { x: 4.0, y: -6.0 }).unwrap().1);
    }

    #[test]
    fn node_split_get_nearest_all_zones() {
        let mut node = QuadTreeNode::empty_split_node(Point::ZERO, 10.0);
        node.add_point(&Point { x: 5.0, y: 5.0 });
        node.add_point(&Point { x: 5.0, y: -5.0 });
        node.add_point(&Point { x: -5.0, y: -5.0 });
        node.add_point(&Point { x: -5.0, y: 5.0 });

        assert_eq!(1.0, node.get_nearest(&Point { x: 6.0, y: 5.0 }).unwrap().1);
        assert_eq!(1.0, node.get_nearest(&Point { x: 6.0, y: -5.0 }).unwrap().1);
        assert_eq!(
            1.0,
            node.get_nearest(&Point { x: -6.0, y: -5.0 }).unwrap().1
        );
        assert_eq!(1.0, node.get_nearest(&Point { x: -6.0, y: 5.0 }).unwrap().1);

        assert_close(
            5.0,
            5.0,
            node.get_nearest(&Point { x: 0.0, y: 0.0 }).unwrap().1,
        );
        assert_close(
            4.0,
            4.0,
            node.get_nearest(&Point { x: 1.0, y: 1.0 }).unwrap().1,
        );
        assert_close(
            4.0,
            4.0,
            node.get_nearest(&Point { x: 1.0, y: -1.0 }).unwrap().1,
        );
        assert_close(
            4.0,
            4.0,
            node.get_nearest(&Point { x: -1.0, y: -1.0 }).unwrap().1,
        );
        assert_close(
            4.0,
            4.0,
            node.get_nearest(&Point { x: -1.0, y: 1.0 }).unwrap().1,
        );
    }

    #[test]
    fn flake_is_empty() {
        with_test_dir(|test_dir: &str| {
            let flake = Flake::new(&format!("{}/test.flake", test_dir));
            let mut tree = QuadTree::from_flake(&flake).expect("Unable to make flake");
            assert_eq!(true, tree.is_empty());

            tree.add_point(&Point::ZERO);
            assert_eq!(false, tree.is_empty());
        });
    }

    #[test]
    fn flake_get_nearest() {
        with_test_dir(|test_dir: &str| {
            let flake = Flake::new(&format!("{}/test.flake", test_dir));
            let mut tree = QuadTree::from_flake(&flake).expect("Unable to make flake");

            tree.add_point(&Point::ZERO);
            assert_eq!(0.0, tree.get_nearest(&Point::ZERO).unwrap().1);
            assert_eq!(1.0, tree.get_nearest(&Point { x: 1.0, y: 0.0 }).unwrap().1);
            assert_eq!(4.0, tree.get_nearest(&Point { x: 0.0, y: 4.0 }).unwrap().1);

            tree.add_point(&Point { x: 0.0, y: 1.0 });
            assert_eq!(0.0, tree.get_nearest(&Point::ZERO).unwrap().1);
            assert_eq!(5.0, tree.get_nearest(&Point { x: 5.0, y: 1.0 }).unwrap().1);
            assert_eq!(6.0, tree.get_nearest(&Point { x: 6.0, y: 0.0 }).unwrap().1);

            tree.add_point(&Point { x: 10.0, y: 0.0 });
            assert_eq!(0.0, tree.get_nearest(&Point::ZERO).unwrap().1);
            assert_eq!(5.0, tree.get_nearest(&Point { x: 5.0, y: 1.0 }).unwrap().1);
            assert_eq!(4.0, tree.get_nearest(&Point { x: 6.0, y: 0.0 }).unwrap().1);
        });
    }

    #[test]
    fn flake_get_nearest_grid() {
        with_test_dir(|test_dir: &str| {
            let flake = Flake::new(&format!("{}/test.flake", test_dir));
            let mut tree = QuadTree::from_flake(&flake).expect("Unable to make flake");

            for x in -100..100 {
                for y in -100..100 {
                    tree.add_point(&Point {
                        x: x as f64,
                        y: y as f64,
                    });
                }
            }

            tree.root.print(0);

            for x in -100..100 {
                for y in -100..100 {
                    let result = tree
                        .get_nearest(&Point {
                            x: x as f64 + 0.25,
                            y: y as f64 + 0.25,
                        })
                        .unwrap();

                    assert_eq!(x as f64, result.0.x);
                    assert_eq!(y as f64, result.0.y);
                    assert_close(0.25, 0.25, result.1);
                }
            }
        });
    }

    #[test]
    fn flake_get_farthest_distance() {
        with_test_dir(|test_dir: &str| {
            let flake = Flake::new(&format!("{}/test.flake", test_dir));
            let mut tree = QuadTree::from_flake(&flake).expect("Unable to make flake");

            tree.add_point(&Point::ZERO);
            assert_eq!(0.0, tree.get_farthest_distance());

            tree.add_point(&Point { x: 0.0, y: 1.0 });
            assert_eq!(1.0, tree.get_farthest_distance());

            tree.add_point(&Point { x: 10.0, y: 0.0 });
            assert_eq!(10.0, tree.get_farthest_distance());
        });
    }

    /*
     * Last recorded performance:
     *
     * Time to add 100000 points: 17.675888ms
     * Time per point: 176ns
     *
     * Time to query 100000 points: 87.696022ms
     * Time per point: 876ns
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
                for point in &points {
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
                for query_point in &query_points {
                    assert!(tree.get_nearest(query_point).unwrap().1 > 0.0);
                }
            });
            println!(
                "Time to query {} points: {:?}",
                num_query_points, querying_time
            );
            println!("Time per point: {:?}", querying_time / num_query_points);
            println!();
        });
    }
}
