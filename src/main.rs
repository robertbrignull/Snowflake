extern crate rand;

mod data;
mod grid;
mod square_grid;

use crate::grid::Grid;
use crate::square_grid::SquareGrid;

fn main() {
    let mut grid: Box<Grid> = Box::new(SquareGrid::new());

    grid.add_points(500);

    for p in grid.list_points() {
        println!("{}", p);
    }
}
