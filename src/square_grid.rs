extern crate rand;

use rand::Rng;
use std::f64::consts::PI;

use crate::data::Point;
use crate::grid::Grid;

const BORDER: f64 = 20.0;

pub struct SquareGrid {
    grid: Vec<Vec<bool>>,
    num_points: u32,
    radius: f64,
    rotational: u32,
    reflectional: u32,
}

/** Defines a point on the square grid, where (0, 0) is the centre */
#[derive(Clone, Copy)]
struct GridPoint {
    x: i32,
    y: i32,
}

impl SquareGrid {
    pub fn new(rotational: u32, reflectional: u32) -> Self {

        if !SquareGrid::is_valid_symmetry_level(rotational) {
            panic!("Unsupported symmetry level ({}) for square grid", rotational);
        }
        if !SquareGrid::is_valid_symmetry_level(reflectional) {
            panic!("Unsupported symmetry level ({}) for square grid", rotational);
        }
        if rotational > 0 && reflectional > 0 {
            panic!("Cannot specify both rotational and reflectional symmetry");
        }

        let w = BORDER as usize * 2 + 100;
        let mut grid = vec![vec![false; w]; w];
        grid[w / 2][w / 2] = true;

        return SquareGrid {
            grid,
            num_points: 0,
            radius: BORDER,
            rotational,
            reflectional,
        };
    }

    fn is_valid_symmetry_level(num: u32) -> bool {
        return num == 0 || num == 1 || num == 2 || num == 4;
    }

    /** The distance from the given point to the centre of the grid */
    fn distance_to_centre(&self, point: GridPoint) -> f64 {
        let d = point.x * point.x + point.y * point.y;
        return (d as f64).sqrt();
    }

    /** Double the grid size */
    fn increase_grid_size(&mut self) {
        let new_w = self.grid.len() * 2;
        let mut new_grid = Vec::new();

        // Rows above the flake
        for _i in 0..self.grid.len() / 2 {
            new_grid.push(vec![false; new_w]);
        }

        // Rows containing the flake
        for i in 0..self.grid.len() {
            let mut row = vec![false; new_w];
            row[self.grid.len() / 2 .. self.grid.len() / 2 * 3].clone_from_slice(self.grid[i].as_slice());
            new_grid.push(row);
        }

        // Rows below the flake
        for _i in 0..self.grid.len() / 2 {
            new_grid.push(vec![false; new_w]);
        }

        self.grid = new_grid;
    }

    /** Get a random position of the border of the grid */
    fn get_start_position(&self) -> GridPoint {
        let dir = rand::thread_rng().gen::<f64>() * PI * 2.0;
        // Round towards zero to keep inside the radius
        return GridPoint {
            x: (self.radius * dir.sin()).trunc() as i32,
            y: (self.radius * dir.cos()).trunc() as i32,
        };
    }

    /** Walk the particle one step */
    fn get_next_position(&self, point: GridPoint) -> GridPoint {
        let dir = rand::thread_rng().gen::<u32>() % 4;
        let new_point = match dir {
            0 => GridPoint { x: point.x,     y: point.y - 1 },
            1 => GridPoint { x: point.x + 1, y: point.y     },
            2 => GridPoint { x: point.x,     y: point.y + 1 },
            _ => GridPoint { x: point.x - 1, y: point.y     },
        };
        if self.distance_to_centre(new_point) < self.radius {
            return new_point;
        } else {
            return point;
        }
    }

    /** Is the given point in contact with the flake */
    fn has_hit_flake(&self, point: GridPoint) -> bool {
        let c = self.grid.len() as i32 / 2;
        let x = (point.x + c) as usize;
        let y = (point.y + c) as usize;
        // It shouldn't happen but handle the case where it
        // is already overlapping with the flake.
        return self.grid[x][y] ||
            // Because of the boundary circle we don't need
            // to worry about going off the edge of the grid.
            self.grid[x][y - 1] ||
            self.grid[x + 1][y] ||
            self.grid[x][y + 1] ||
            self.grid[x - 1][y];
    }

    fn add(&mut self, point: GridPoint) {
        let c = (self.grid.len() / 2) as i32;
        let x = (point.x + c) as usize;
        let y = (point.y + c) as usize;
        self.grid[x][y] = true;
    }

    /** Add any extra points to maintain the symmetry */
    fn add_symmetry_points(&mut self, point: GridPoint) {
        let c = self.grid.len() as i32 / 2;

        if self.rotational >= 2 {
            self.add(GridPoint { x: c - point.x, y: c - point.y });
            self.num_points += 1;
        }
        if self.rotational == 4 {
            self.add(GridPoint { x: c - point.y, y: c + point.x });
            self.add(GridPoint { x: c + point.y, y: c - point.x });
            self.num_points += 2;
        }

        if self.reflectional >= 1 {
            self.add(GridPoint { x: c - point.x, y: c + point.y });
            self.num_points += 1;
        }
        if self.reflectional >= 2 {
            self.add(GridPoint { x: c + point.x, y: c - point.y });
            self.add(GridPoint { x: c - point.x, y: c - point.y });
            self.num_points += 2;
        }
        if self.reflectional == 4 {
            self.add(GridPoint { x: c + point.y, y: c + point.x });
            self.add(GridPoint { x: c + point.y, y: c - point.x });
            self.add(GridPoint { x: c - point.y, y: c + point.x });
            self.add(GridPoint { x: c - point.y, y: c - point.x });
            self.num_points += 4;
        }
    }
}

impl Grid for SquareGrid {
    fn add_point(&mut self) {
        let mut point = self.get_start_position();
        while !self.has_hit_flake(point) {
            point = self.get_next_position(point);
        }
        self.add(point);
        self.num_points += 1;
        self.radius = self.radius.max(self.distance_to_centre(point) + BORDER);

        self.add_symmetry_points(point);

        // If we're even getting close to the grid then increase the size
        if self.radius >= self.grid.len() as f64 / 2.0 - 10.0 {
            self.increase_grid_size();
        }
    }

    fn get_num_points(&self) -> u32 {
        return self.num_points;
    }

    fn list_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        for y in 0..self.grid.len() {
            for x in 0..self.grid.len() {
                if self.grid[x][y] {
                    points.push(Point {
                        x: x as f64,
                        y: y as f64,
                    });
                }
            }
        }
        return points;
    }
}
