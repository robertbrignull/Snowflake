extern crate clap;
extern crate anyhow;
extern crate image;
extern crate rand;

mod flake;
mod generate;
mod point;
mod quad_tree;
mod render;
mod test_utils;

use crate::flake::Flake;
use crate::generate::{generate, Symmetry};
use crate::render::render;

fn main() {
    match parse_args().subcommand() {
        ("generate", Some(args)) => {
            let flake_file = args
                .value_of("flake-file")
                .expect("flake-file not passed");
            let mut flake = Flake::new(&flake_file);

            let num_points = args
                .value_of("num-particles")
                .unwrap_or("0")
                .parse::<u32>()
                .expect("num-particles arg could not be parsed");
            let num_points = match num_points {
                0 => Option::None,
                x => Option::Some(x),
            };

            let rotational = args
                .value_of("rotational-symmetry")
                .unwrap_or("0")
                .parse::<u32>()
                .expect("rotational-symmetry arg could not be parsed");
            let reflectional = args
                .value_of("reflectional-symmetry")
                .unwrap_or("0")
                .parse::<u32>()
                .expect("reflectional-symmetry arg could not be parsed");
            let symmetry = Symmetry::from(rotational, reflectional);

            match generate(&mut flake, symmetry, num_points) {
                Err(err) => {
                    println!("Unable to generate flake");
                    for cause in err.chain() {
                        println!("{}", cause);
                    }
                    std::process::exit(1);
                },
                _ => {}
            }
        }
        ("render", Some(args)) => {
            let flake_file = args
                .value_of("flake-file")
                .expect("flake-file not passed");
            let flake = Flake::new(&flake_file);

            let output_file = args
                .value_of("output")
                .expect("output not passed");
            
            match render(&flake, &output_file) {
                Err(err) => {
                    println!("Unable to render flake");
                    for cause in err.chain() {
                        println!("{}", cause);
                    }
                    std::process::exit(1);
                },
                _ => {}
            }
        }
        (command, _)  => {
            println!("Unknown subcommand: {}", command);
            std::process::exit(1);
        }
    }
}

fn parse_args() -> clap::ArgMatches<'static> {
    return clap::App::new("snowflake-rs")
        .about("Generates snowflakes through random motion")
        .subcommand(clap::App::new("generate")
            .about("Generate a snowflake")
            .arg(clap::Arg::with_name("flake-file")
                    .short("f")
                    .long("flake-file")
                    .number_of_values(1)
                    .value_name("FILE")
                    .required(true)
                    .help("Location of file to store flake information to"))
                .arg(clap::Arg::with_name("num-particles")
                    .short("n")
                    .long("num-particles")
                    .number_of_values(1)
                    .value_name("NUM")
                    .help("The number of particles to add to the flake, or omit to add indefinitely until stopped"))
                .arg(clap::Arg::with_name("rotational-symmetry")
                    .long("rotational-symmetry")
                    .number_of_values(1)
                    .value_name("NUM")
                    .help("The level of rotational symmetry to use, or omit to have no rotational symmetry"))
                .arg(clap::Arg::with_name("reflectional-symmetry")
                    .long("reflectional-symmetry")
                    .number_of_values(1)
                    .value_name("NUM")
                    .help("The number of axis of reflectional symmetry to use, or omit to have no reflectional symmetry")
                    .conflicts_with("rotational-symmetry")))
        .subcommand(clap::App::new("render")
            .about("Render a flake file to an image")
            .arg(clap::Arg::with_name("flake-file")
                    .short("f")
                    .long("flake-file")
                    .number_of_values(1)
                    .value_name("FILE")
                    .required(true)
                    .help("Location of file to read flake information from"))
            .arg(clap::Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .number_of_values(1)
                    .value_name("FILE")
                    .required(true)
                    .help("Location of file to output image to")))
        .get_matches();
}
