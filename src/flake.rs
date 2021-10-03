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
        let mut buf: [u8; 16 * MAX_BUFFERED_POINTS] = [0; 16 * MAX_BUFFERED_POINTS];
        let mut f64_buf: [u8; 8] = [0; 8];
        loop {
            let bytes_read = f.read(&mut buf).context("Unable to read from flake file")?;
            if bytes_read == 0 {
                return Result::Ok(points);
            }

            for i in (0..bytes_read).step_by(16) {
                f64_buf.clone_from_slice(&buf[i..i + 8]);
                let x: f64 = f64::from_be_bytes(f64_buf);
                f64_buf.clone_from_slice(&buf[i + 8..i + 16]);
                let y: f64 = f64::from_be_bytes(f64_buf);
                points.push(Point { x, y });
            }
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

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::Flake;
    use crate::point::Point;
    use crate::test_utils::test::{time_func, with_test_dir};

    #[test]
    fn get_points_for_empty_file() {
        with_test_dir(|test_dir: &str| {
            let flake_file = format!("{}/test.flake", test_dir);
            let flake = Flake::new(&flake_file);
            assert_eq!(0, flake.get_points().expect("Unable to get points").len());
        });
    }

    #[test]
    fn write_points_to_new_file() {
        with_test_dir(|test_dir: &str| {
            let flake_file = format!("{}/test.flake", test_dir);
            let mut flake = Flake::new(&flake_file);
            flake
                .add_point(Point { x: 0.0, y: 0.0 })
                .expect("Unable to add point");
            flake.flush().expect("Unable to flush");
        });
    }

    #[test]
    fn write_points_to_existing_file() {
        with_test_dir(|test_dir: &str| {
            let flake_file = format!("{}/test.flake", test_dir);

            let mut flake = Flake::new(&flake_file);
            flake
                .add_point(Point { x: 0.0, y: 0.0 })
                .expect("Unable to add point");
            flake.flush().expect("Unable to flush");

            // Do the same thing but with a new flake object
            let mut flake = Flake::new(&flake_file);
            flake
                .add_point(Point { x: 1.0, y: 1.0 })
                .expect("Unable to add point");
            flake.flush().expect("Unable to flush");
        });
    }

    #[test]
    fn roundtrip() {
        with_test_dir(|test_dir: &str| {
            let flake_file = format!("{}/test.flake", test_dir);
            let mut flake = Flake::new(&flake_file);

            let num_points = 1500;
            for i in 0..num_points {
                flake
                    .add_point(Point {
                        x: i as f64,
                        y: i as f64,
                    })
                    .expect("Unable to add point");
            }
            flake.flush().expect("Unable to flush");

            let points = flake.get_points().expect("Unable to get points");
            assert_eq!(num_points, points.len());
            for i in 0..num_points {
                assert_eq!(i as f64, points[i].x);
                assert_eq!(i as f64, points[i].y);
            }
        });
    }

    /*
     * Last recorded performance:
     *
     * Time to write 10000000 points: 195.548667ms
     * Time per point: 19ns
     *
     * Time to read 10000000 points: 97.262104ms
     * Time per point: 9ns
     */
    #[test]
    #[ignore]
    fn roundtrip_perf() {
        with_test_dir(|test_dir: &str| {
            let flake_file = format!("{}/test.flake", test_dir);
            let mut flake = Flake::new(&flake_file);

            let num_points = 10000000;
            let mut points: Vec<Point> = Vec::new();
            let mut rng = rand::thread_rng();
            for _i in 0..num_points {
                points.push(Point {
                    x: rng.gen_range(-1000.0..1000.0),
                    y: rng.gen_range(-1000.0..1000.0),
                });
            }

            let write_time = time_func(|| {
                for point in points {
                    flake.add_point(point).expect("Unable to add point");
                }
                flake.flush().expect("Unable to flush");
            });
            println!("Time to write {} points: {:?}", num_points, write_time);
            println!("Time per point: {:?}", write_time / num_points);
            println!();

            let read_time = time_func(|| {
                assert_eq!(
                    num_points as usize,
                    flake.get_points().expect("Unable to get points").len()
                );
            });
            println!("Time to read {} points: {:?}", num_points, read_time);
            println!("Time per point: {:?}", read_time / num_points);
            println!();
        });
    }
}
