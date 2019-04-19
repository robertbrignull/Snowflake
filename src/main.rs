extern crate clap;
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

    let args = parse_args();

    let num_points = args
        .value_of("num-particles")
        .unwrap_or("10000")
        .parse::<u32>()
        .expect("num-particles arg could not be parsed");

    grid.add_points(num_points);

    save_png(grid.list_points());
}

fn parse_args() -> clap::ArgMatches<'static> {
    return clap::App::new("snowflake-rs")
        .about("Generates snowflakes through random motion")
        .arg(clap::Arg::with_name("num-particles")
            .short("n")
            .long("num-particles")
            .value_name("NUM")
            .help("The number of particles to add to the flake"))
        .get_matches();
}
