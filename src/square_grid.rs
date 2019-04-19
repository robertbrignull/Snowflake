extern crate rand;

use rand::Rng;
use std::f64::consts::PI;

use crate::data::Point;
use crate::grid::Grid;

const BORDER: f64 = 20.0;

pub struct SquareGrid {
    grid: Vec<Vec<bool>>,
    radius: f64,
}

impl SquareGrid {
    pub fn new() -> Self {
        let w = BORDER as usize * 2 + 100;
        let mut grid = vec![vec![false; w]; w];
        grid[w / 2][w / 2] = true;

        return SquareGrid {
            grid,
            radius: BORDER,
        };
    }

    /** The distance from the given point to the centre of the grid */
    fn distance_to_centre(&self, point: &Point) -> f64 {
        let c = (self.grid.len() / 2) as f64;
        let d = (point.x - c) * (point.x - c) +
            (point.y - c) * (point.y - c);
        return d.sqrt();
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
    fn get_start_position(&self) -> Point {
        let dir = rand::thread_rng().gen::<f64>() * PI * 2.0;
        // Round towards zero to keep inside the radius
        let dx = (self.radius * dir.sin()).trunc();
        let dy = (self.radius * dir.cos()).trunc();

        return Point {
            x: self.grid.len() as f64 / 2.0 + dx,
            y: self.grid.len() as f64 / 2.0 + dy,
        };
    }

    /** Walk the particle one step */
    fn get_next_position(&self, point: Point) -> Point {
        let dir = rand::thread_rng().gen::<u32>() % 4;
        let new_point = match dir {
            0 => Point { x: point.x,       y: point.y - 1.0 },
            1 => Point { x: point.x + 1.0, y: point.y       },
            2 => Point { x: point.x,       y: point.y + 1.0 },
            _ => Point { x: point.x - 1.0, y: point.y       },
        };
        if self.distance_to_centre(&new_point) < self.radius {
            return new_point;
        } else {
            return point;
        }
    }

    /** Is the given point in contact with the flake */
    fn has_hit_flake(&self, point: &Point) -> bool {
        let x = point.x as usize;
        let y = point.y as usize;
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
}

impl Grid for SquareGrid {
    fn add_point(&mut self) {
        let mut point = self.get_start_position();
        while !self.has_hit_flake(&point) {
            point = self.get_next_position(point);
        }
        self.grid[point.x as usize][point.y as usize] = true;
        self.radius = self.radius.max(self.distance_to_centre(&point) + 10.0);

        // If we're even getting close to the grid then increase the size
        if self.radius >= self.grid.len() as f64 / 2.0 - 10.0 {
            self.increase_grid_size();
        }
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
