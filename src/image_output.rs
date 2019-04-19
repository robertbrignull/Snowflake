use std::f64::INFINITY;
use std::path::Path;

use crate::data::Point;

const BORDER: f64 = 10.0;

pub fn save_png(points: Vec<Point>, output_filename: &str) {
    let mut top: f64 = INFINITY;
    let mut right: f64 = -INFINITY;
    let mut bottom: f64 = -INFINITY;
    let mut left: f64 = INFINITY;

    for point in &points {
        top = top.min(point.y);
        right = right.max(point.x);
        bottom = bottom.max(point.y);
        left = left.min(point.x);
    }

    let width = (right - left + BORDER * 2.0 + 1.0).ceil() as usize;
    let height = (bottom - top + BORDER * 2.0 + 1.0).ceil() as usize;
    let left = left - BORDER;
    let top = top - BORDER;

    let mut buffer: Vec<u8> = vec![0; width * height * 3];

    for point in &points {
        let x = (point.x - left) as usize;
        let y = (point.y - top) as usize;
        let i = (x + y * width) * 3;
        buffer[i] = 255;
        buffer[i + 1] = 255;
        buffer[i + 2] = 255;
    }

    // Save the buffer as "image.png"
    image::save_buffer(
        &Path::new(output_filename),
        &buffer,
        width as u32,
        height as u32,
        image::RGB(8))
        .expect("Unable to save image");
}
