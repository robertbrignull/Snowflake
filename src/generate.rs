use anyhow::{Context, Result};
use rand::Rng;

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
    let _points = flake.get_points().context("Unable to get flake points")?;

    let num_points = num_points.unwrap_or(1000);
    let mut rng = rand::thread_rng();
    for _i in 0..num_points {
        let x: f64 = rng.gen_range(-100.0..100.0);
        let y: f64 = rng.gen_range(-100.0..100.0);
        flake
            .add_point(Point { x, y })
            .context("Unable to add points to flake")?;
    }

    flake.flush().context("Unable to flush flake")?;

    return Result::Ok(());
}
