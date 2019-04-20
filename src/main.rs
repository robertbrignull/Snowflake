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
    let args = parse_args();

    let grid = args
        .value_of("grid-type").unwrap();

    let num_points = args
        .value_of("num-particles").unwrap()
        .parse::<u32>().expect("num-particles arg could not be parsed");

    let rotational = args
        .value_of("rotational-symmetry").unwrap_or("0")
        .parse::<u32>().expect("rotational-symmetry arg could not be parsed");

    let reflectional = args
        .value_of("reflectional-symmetry").unwrap_or("0")
        .parse::<u32>().expect("reflectional-symmetry arg could not be parsed");

    let output = args.value_of("output").unwrap();

    let mut grid: Box<Grid> = match grid {
        "square" => Box::new(SquareGrid::new(
            rotational,
            reflectional,
        )),
        _ => panic!("Unknown grid type: {}", grid)
    };

    grid.add_points(num_points);

    save_png(grid.list_points(), output);
}

fn parse_args() -> clap::ArgMatches<'static> {
    return clap::App::new("snowflake-rs")
        .about("Generates snowflakes through random motion")
        .arg(clap::Arg::with_name("grid-type")
            .short("g")
            .long("grid")
            .number_of_values(1)
            .value_name("TYPE")
            .possible_values(&["square"])
            .required(true)
            .help("The type of grid to use"))
        .arg(clap::Arg::with_name("num-particles")
            .short("n")
            .long("num-particles")
            .number_of_values(1)
            .value_name("NUM")
            .help("The number of particles to add to the flake")
            .default_value("10000"))
        .arg(clap::Arg::with_name("rotational-symmetry")
            .long("rotational-symmetry")
            .number_of_values(1)
            .value_name("NUM")
            .help("The level of rotational symmetry to use"))
        .arg(clap::Arg::with_name("reflectional-symmetry")
            .long("reflectional-symmetry")
            .number_of_values(1)
            .value_name("NUM")
            .help("The number of axis of reflectional symmetry to use")
            .conflicts_with("rotational-symmetry"))
        .arg(clap::Arg::with_name("output")
            .short("o")
            .long("output")
            .number_of_values(1)
            .value_name("FILE")
            .help("Location of output image")
            .default_value("output.png"))
        .get_matches();
}
