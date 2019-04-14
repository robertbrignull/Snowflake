extern crate rand;

use rand::Rng;

use crate::data::Point;
use crate::grid::Grid;

const WIDTH: usize = 500;

pub struct SquareGrid {
    grid: [[bool; WIDTH]; WIDTH],
}

impl SquareGrid {
    pub fn new() -> Self {
        let mut grid = [[false; WIDTH]; WIDTH];
        grid[WIDTH / 2][WIDTH / 2] = true;

        return SquareGrid {
            grid,
        };
    }

    fn get_start_position() -> Point {
        let mut rng = rand::thread_rng();
        let dir = rng.gen::<u32>() % 4;
        return match dir {
            // up
            0 => Point {
                x: rng.gen::<usize>().min(WIDTH - 1) as f64,
                y: 0.0,
            },
            // right
            1 => Point {
                x: (WIDTH - 1) as f64,
                y: rng.gen::<usize>().min(WIDTH - 1) as f64,
            },
            // down
            2 => Point {
                x: rng.gen::<usize>().min(WIDTH - 1) as f64,
                y: (WIDTH - 1) as f64,
            },
            // left
            _ => Point {
                x: 0.0,
                y: rng.gen::<usize>().min(WIDTH - 1) as f64,
            },
        };
    }

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
                x: (point.x + 1.0).min((WIDTH - 1) as f64),
                y: point.y,
            },
            // down
            2 => Point {
                x: point.x,
                y: (point.y + 1.0).min((WIDTH - 1) as f64),
            },
            // left
            _ => Point {
                x: (point.x - 1.0).max(0.0),
                y: point.y,
            },
        };
    }

    fn has_hit_flake(&self, point: &Point) -> bool {
        let x = point.x as usize;
        let y = point.y as usize;
        return (y > 0 && self.grid[x][y - 1]) ||
            (x < WIDTH - 1 && self.grid[x + 1][y]) ||
            (y < WIDTH - 1 && self.grid[x][y + 1]) ||
            (x > 0 && self.grid[x - 1][y]);
    }
}

impl Grid for SquareGrid {
    fn add_point(&mut self) {
        let mut point = SquareGrid::get_start_position();
        while !self.has_hit_flake(&point) {
            point = self.get_next_position(&point);
        }
        self.grid[point.x as usize][point.y as usize] = true;
    }

    fn list_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        for y in 0..WIDTH {
            for x in 0..WIDTH {
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
