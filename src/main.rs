extern crate image;
extern crate rand;

mod data;
mod grid;
mod image_output;
mod square_grid;

use crate::grid::Grid;
use crate::image_output::save_png;
use crate::square_grid::SquareGrid;

fn main() {
    let mut grid: Box<Grid> = Box::new(SquareGrid::new());

    grid.add_points(10000);

    save_png(grid.list_points());
}
