use crate::point::Point;
use crate::vec3::Vec3;
use crate::{color::Color, ray::Ray};
use std::io::{BufWriter, Write};

mod color;
mod point;
mod ray;
mod vec3;

fn main() -> std::io::Result<()> {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;

    const fn calc_height() -> u32 {
        let height = IMAGE_WIDTH as f32 / ASPECT_RATIO;

        if height >= 1.0 { height as u32 } else { 1u32 }
    }

    const IMAGE_HEIGHT: u32 = calc_height();

    // Camera
    const FOCAL_LENGTH: f32 = 1.0;
    const VIEWPORT_HEIGHT: f32 = 2.0;
    const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * ((IMAGE_WIDTH as f32) / (IMAGE_HEIGHT as f32));
    const CAMERA_CENTER: Point = Point::new(0.0, 0.0, 0.0);

    // Viewport vectors, u horizontal, v vertical (down)
    const VIEWPORT_U: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    const VIEWPORT_V: Vec3 = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

    // Horizontal and vertical distance vectors between pixels
    // Not const because of impl div
    let PIXEL_DELTA_U: Vec3 = VIEWPORT_U / (IMAGE_WIDTH as f32);
    let PIXEL_DELTA_V: Vec3 = VIEWPORT_V / (IMAGE_HEIGHT as f32);

    // Location of upper left pixel
    let VIEWPORT_UPPER_LEFT: Point =
        CAMERA_CENTER - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - VIEWPORT_U / 2.0 - VIEWPORT_V / 2.0;

    let PIXEL00_LOC = VIEWPORT_UPPER_LEFT + (PIXEL_DELTA_U + PIXEL_DELTA_V) * 0.5;

    // Render

    let mut out = BufWriter::new(std::fs::File::create("render.ppm")?);
    writeln!(out, "P3")?;
    writeln!(out, "{IMAGE_WIDTH} {IMAGE_HEIGHT}")?;
    writeln!(out, "255")?;

    for i in 0..IMAGE_HEIGHT {
        print!("Rendering line: {}/{}\r", i + 1, IMAGE_HEIGHT);
        std::io::stdout().flush()?;
        for j in 0..IMAGE_WIDTH {
            let pixel_center: Point =
                (PIXEL00_LOC + (PIXEL_DELTA_U * j as f32) + (PIXEL_DELTA_V * i as f32)).into();

            let ray_direction = pixel_center - CAMERA_CENTER;

            let ray = Ray::new(pixel_center, ray_direction);

            let color = ray_color(ray);

            writeln!(out, "{color}")?;
        }
    }

    println!();

    out.flush()?;

    Ok(())
}

pub(crate) fn ray_color(ray: Ray) -> Color {
    if hit_sphere(Point::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction.unit();
    let a = (unit_direction.y + 1.0) * 0.5;
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

pub(crate) fn hit_sphere(center: Point, radius: f32, ray: Ray) -> bool {
    let origin_to_center = center - ray.origin;
    let a = ray.direction.dot(ray.direction);
    let b = -2.0 * ray.direction.dot(origin_to_center);
    let c = origin_to_center.dot(origin_to_center) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}
