use std::f64::INFINITY;
use std::path::Path;

use anyhow::{Context, Result};

use crate::{flake::Flake, point::Point};

const BORDER: f64 = 10.0;

struct Window {
    left: f64,
    top: f64,
    width: usize,
    height: usize,
}

fn find_bounding_rect(points: Vec<Point>) -> Window {
    let mut left: f64 = INFINITY;
    let mut top: f64 = INFINITY;
    let mut right: f64 = -INFINITY;
    let mut bottom: f64 = -INFINITY;

    for point in &points {
        left = left.min(point.x);
        top = top.min(point.y);
        right = right.max(point.x);
        bottom = bottom.max(point.y);
    }
    
    let left = left - BORDER;
    let top = top - BORDER;
    let width = (right - left + BORDER + 1.0).ceil() as usize;
    let height = (bottom - top + BORDER + 1.0).ceil() as usize;

    return Window { left, top, width, height };
}

fn draw_point(point: &Point, window: &Window, buffer: &mut Vec<u8>) {
    let x = (point.x - window.left) as usize;
    let y = (point.y - window.top) as usize;
    let i = (x + y * window.width) * 3;
    buffer[i] = 255;
    buffer[i + 1] = 255;
    buffer[i + 2] = 255;
}

pub fn render(flake: &Flake, output_filename: &str) -> Result<()> {
    let points = flake.get_points().context("Unable to get points for flake")?;

    let window = find_bounding_rect(points.clone());

    let mut buffer: Vec<u8> = vec![0; window.width * window.height * 3];

    for point in &points {
        draw_point(point, &window, &mut buffer)
    }

    // Save the buffer as "image.png"
    image::save_buffer(
        &Path::new(output_filename),
        &buffer,
        window.width as u32,
        window.height as u32,
        image::ColorType::Rgb8,
    )
    .context("Unable to save image buffer to file")?;

    return Result::Ok(());
}
