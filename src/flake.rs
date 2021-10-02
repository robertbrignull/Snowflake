use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use anyhow::{Context, Result};

use crate::point::Point;

const MAX_BUFFERED_POINTS: usize = 1000;

pub struct Flake {
    flake_file: String,
    buffered_points: Vec<Point>,
}

impl Flake {
    // Construct a new flake, using the given file as storage.
    pub fn new(flake_file: &str) -> Flake {
        return Flake {
            flake_file: flake_file.to_string(),
            buffered_points: Vec::new(),
        };
    }

    // Read any existing points from the flake file.
    pub fn get_points(&self) -> Result<Vec<Point>> {
        let mut points = Vec::new();
        if !Path::new(&self.flake_file).exists() {
            return Result::Ok(points);
        }

        let mut f = File::open(&self.flake_file)?;
        let mut x_buf: [u8; 8] = [0; 8];
        let mut y_buf: [u8; 8] = [0; 8];
        loop {
            if f.read(&mut x_buf)? != 8 || f.read(&mut y_buf)? != 8 {
                return Result::Ok(points);
            }

            let x: f64 = f64::from_be_bytes(x_buf);
            let y: f64 = f64::from_be_bytes(y_buf);
            points.push(Point { x, y });
        }
    }

    // Add a point to the flake. This may write the data out to the flake
    // file or may buffer internally. See also the flush method.
    pub fn add_point(&mut self, point: Point) -> Result<()> {
        self.buffered_points.push(point);
        if self.buffered_points.len() >= MAX_BUFFERED_POINTS {
            self.flush().context("Unable to flush")?;
        }
        return Result::Ok(());
    }

    // Write to the flake file any points that have been buffered in memory.
    pub fn flush(&mut self) -> Result<()> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&self.flake_file)
            .context(format!("Unable to open flake file: {}", self.flake_file))?;

        let mut buf = vec![0; self.buffered_points.len() * 16];
        for (i, point) in self.buffered_points.iter().enumerate() {
            let index = i * 16;
            buf[index..index + 8].clone_from_slice(&point.x.to_be_bytes());
            buf[index + 8..index + 16].clone_from_slice(&point.y.to_be_bytes());
        }
        f.write(&buf).context("Unable to write buf to flake file")?;

        self.buffered_points.truncate(0);
        return Result::Ok(());
    }
}
