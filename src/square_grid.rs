extern crate rand;

use rand::Rng;

use crate::data::Point;
use crate::grid::Grid;

pub struct SquareGrid {
    grid: Vec<Vec<bool>>,
}

impl SquareGrid {
    pub fn new() -> Self {
        let w = 40;
        let mut grid = vec![vec![false; w]; w];
        grid[w / 2][w / 2] = true;

        return SquareGrid {
            grid,
        };
    }

    /** Is the given point nearly at the edge of the grid */
    fn should_increase_grid_size(&self, p: Point) -> bool {
        let lb = self.grid.len() as f64 / 6.0;
        let ub = self.grid.len() as f64 - lb;
        return p.x < lb ||
            p.x > ub ||
            p.y < lb ||
            p.y > ub;
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
        let mut rng = rand::thread_rng();
        let dir = rng.gen::<u32>() % 4;
        return match dir {
            // up
            0 => Point {
                x: rng.gen::<usize>().min(self.grid.len() - 1) as f64,
                y: 0.0,
            },
            // right
            1 => Point {
                x: (self.grid.len() - 1) as f64,
                y: rng.gen::<usize>().min(self.grid.len() - 1) as f64,
            },
            // down
            2 => Point {
                x: rng.gen::<usize>().min(self.grid.len() - 1) as f64,
                y: (self.grid.len() - 1) as f64,
            },
            // left
            _ => Point {
                x: 0.0,
                y: rng.gen::<usize>().min(self.grid.len() - 1) as f64,
            },
        };
    }

    /** Walk the particle one step */
    fn get_next_position(&self, point: &Point) -> Point {
        let dir = rand::thread_rng().gen::<u32>() % 4;
        return match dir {
            // up
            0 => Point {
                x: point.x,
                y: (point.y - 1.0).max(0.0),
            },
            // right
            1 => Point {
                x: (point.x + 1.0).min((self.grid.len() - 1) as f64),
                y: point.y,
            },
            // down
            2 => Point {
                x: point.x,
                y: (point.y + 1.0).min((self.grid.len() - 1) as f64),
            },
            // left
            _ => Point {
                x: (point.x - 1.0).max(0.0),
                y: point.y,
            },
        };
    }

    /** Is the given point in contact with the flake */
    fn has_hit_flake(&self, point: &Point) -> bool {
        let x = point.x as usize;
        let y = point.y as usize;
        // It shouldn't happen but handle the case where it
        // is already overlapping with the flake.
        return self.grid[x][y] ||
            (y > 0 && self.grid[x][y - 1]) ||
            (x < self.grid.len() - 1 && self.grid[x + 1][y]) ||
            (y < self.grid.len() - 1 && self.grid[x][y + 1]) ||
            (x > 0 && self.grid[x - 1][y]);
    }
}

impl Grid for SquareGrid {
    fn add_point(&mut self) {
        let mut point = self.get_start_position();
        while !self.has_hit_flake(&point) {
            point = self.get_next_position(&point);
        }
        self.grid[point.x as usize][point.y as usize] = true;

        if self.should_increase_grid_size(point) {
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
