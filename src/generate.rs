use std::f64::consts::PI;

use anyhow::{Context, Result};
use rand::{Rng, RngCore};

use crate::bsp::BSP;
use crate::flake::Flake;
use crate::point::Point;

pub enum Symmetry {
    None,
    Rotational(u32),
    Reflectional(u32),
}

impl Symmetry {
    pub fn from(rotational: u32, reflectional: u32) -> Symmetry {
        if rotational > 0 {
            return Symmetry::Rotational(rotational);
        }
        if reflectional > 0 {
            return Symmetry::Reflectional(reflectional);
        }
        return Symmetry::None;
    }
}

pub fn generate(flake: &mut Flake, _symmetry: Symmetry, num_points: Option<u32>) -> Result<()> {
    let mut bsp = BSP::from_flake(flake)?;

    let num_points = num_points.unwrap_or(1000);
    let mut rng = rand::thread_rng();

    for i in 0..num_points {
        let construction_radius = bsp.get_farthest_distance() + 5.0;
        let destruction_radius = construction_radius * 2.0;

        let mut point = new_point(construction_radius, &mut rng);
        let mut distance_to_flake = bsp.get_nearest_point(point).distance(point);

        while distance_to_flake > 2.0 {
            let r = rng.gen_range(0.0..PI * 2.0);
            point.x += r.sin() * distance_to_flake;
            point.y += r.cos() * distance_to_flake;

            if point.distance(Point::ZERO) > destruction_radius {
                point = new_point(construction_radius, &mut rng);
            }

            distance_to_flake = bsp.get_nearest_point(point).distance(point);
        }

        println!("Adding point {}/{} : {}", i, num_points, point);
        bsp.add_point(point);
        flake
            .add_point(point)
            .context("Unable to add points to flake")?;
    }

    flake.flush().context("Unable to flush flake")?;

    return Result::Ok(());
}

fn new_point(distance_to_center: f64, rng: &mut dyn RngCore) -> Point {
    let r = rng.gen_range(0.0..PI * 2.0);
    let x = r.sin() * distance_to_center;
    let y = r.cos() * distance_to_center;
    return Point { x, y };
}
